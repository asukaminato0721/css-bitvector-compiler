use cssparser::{Parser, ParserInput, Token};
use std::fmt::Display;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SelectorPart {
    selector: Selector,
    combinator: Combinator,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Combinator {
    Descendant, // 空格
    Child,      // >
    None,       // 最后一个选择器没有组合器
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Selector {
    Type(String),
    Class(String),
    Id(String),
}

impl Display for Selector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Selector::Type(tag) => write!(f, "{}", tag),
            Selector::Class(class) => write!(f, ".{}", class),
            Selector::Id(id) => write!(f, "#{}", id),
        }
    }
}
impl Display for SelectorPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.selector, self.combinator)
    }
}

impl Display for Combinator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Combinator::Descendant => write!(f, " "),
            Combinator::Child => write!(f, ">"),
            Combinator::None => write!(f, ""),
        }
    }
}

pub trait Cache<HtmlNode> {
    fn dirtied(&mut self, path: &[u64]);
    fn recompute(&mut self, root: &mut HtmlNode);
}

#[inline(always)]
pub fn rdtsc() -> u64 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        std::arch::x86_64::_rdtsc()
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OState {
    OOne,
    OZero,
    OFromParent,
}

// lets generalize IState first - this is two separate but very similar optimization
// (also, you should tag our old commit before today's work, we want the old version to compare in benchmark)  // it's already in the commit.
// note that not all input state is used, some state are downright ignored.
// as an example, imagine we have a query A B, saying we should match a node satisfying predicate B,
// where parent satsify predicate A
// the code will look something like this:
// if (B(self)) {
//   if (parent_bitvector.A) {
//     self.out[AB] = 1;
//   }
// }
// in such case, you can see that we are not actually reading A, if branch is not entered
// so, suppose the parent A changed, we should do 0 work recomputing
// todo this, we have to update co/pute/ let me explain how this work with an example
//
// Export HtmlNode structure

// Common layout frame structure used across different implementations
#[derive(Debug, Clone)]
pub struct LayoutFrame {
    pub frame_id: usize,
    pub command_name: String,
    pub command_data: serde_json::Value,
}

/// Parse trace from command.json file
pub fn parse_trace() -> Vec<LayoutFrame> {
    let content = std::fs::read_to_string(format!(
        "css-gen-op/{0}/command.json",
        std::env::var("WEBSITE_NAME").unwrap()
    ))
    .unwrap();

    let mut frames = vec![];
    for (frame_id, line) in content.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let command_data = serde_json::from_str::<serde_json::Value>(line).unwrap();

        let command_name = command_data["name"].as_str().unwrap().to_string();
        if command_name.starts_with("layout_") {
            continue;
        }

        frames.push(LayoutFrame {
            frame_id,
            command_name,
            command_data,
        });
    }

    frames
}

/// Extract path from command data
pub fn extract_path_from_command(command_data: &serde_json::Value) -> Vec<usize> {
    command_data
        .get("path")
        .and_then(|p| p.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_u64().map(|x| x as usize))
                .collect::<Vec<_>>()
        })
        .unwrap()
}

pub fn parse_css(css_content: &str) -> Vec<String> {
    let mut rules = vec![];
    let mut input = ParserInput::new(css_content);
    let mut parser = Parser::new(&mut input);

    let mut selector_parts: Vec<SelectorPart> = vec![];
    let mut current_selector: Option<Selector> = None;
    let mut pending_combinator = Combinator::None;

    #[derive(PartialEq, Eq)]
    enum NextSelector {
        Class,
        Type,
    }
    let mut next_selector = NextSelector::Type;

    loop {
        let token = match parser.next_including_whitespace_and_comments() {
            Ok(token) => token,
            Err(_) => {
                // End of input, finalize any pending rule
                if let Some(selector) = current_selector.take() {
                    selector_parts.push(SelectorPart {
                        selector,
                        combinator: Combinator::None,
                    });
                }
                if !selector_parts.is_empty() {
                    // Convert selector parts to string
                    let rule_string = selector_parts
                        .iter()
                        .map(|part| part.to_string())
                        .collect::<String>();
                    rules.push(rule_string);
                }
                break;
            }
        };

        match token {
            Token::Comment(_) => continue,
            Token::WhiteSpace(_) => {
                if current_selector.is_some() && pending_combinator == Combinator::None {
                    pending_combinator = Combinator::Descendant;
                }
            }
            Token::Delim('.') => {
                next_selector = NextSelector::Class;
            }
            Token::Delim('>') => {
                if current_selector.is_some() {
                    pending_combinator = Combinator::Child;
                }
            }
            Token::IDHash(id) => {
                if let Some(prev_selector) = current_selector.take() {
                    selector_parts.push(SelectorPart {
                        selector: prev_selector,
                        combinator: pending_combinator.clone(),
                    });
                    pending_combinator = Combinator::None;
                }
                current_selector = Some(Selector::Id(id.to_string()));
                next_selector = NextSelector::Type;
            }
            Token::Ident(name) => {
                let s = match next_selector {
                    NextSelector::Class => Selector::Class(name.to_string()),
                    NextSelector::Type => Selector::Type(name.to_string().to_lowercase()),
                };
                if let Some(prev_selector) = current_selector.take() {
                    selector_parts.push(SelectorPart {
                        selector: prev_selector,
                        combinator: pending_combinator.clone(),
                    });
                    pending_combinator = Combinator::None;
                }
                current_selector = Some(s);
                next_selector = NextSelector::Type;
            }
            Token::CurlyBracketBlock => {
                if let Some(selector) = current_selector.take() {
                    selector_parts.push(SelectorPart {
                        selector,
                        combinator: Combinator::None,
                    });
                }
                if !selector_parts.is_empty() {
                    // Convert selector parts to string
                    let rule_string = selector_parts
                        .iter()
                        .map(|part| part.to_string())
                        .collect::<String>();
                    rules.push(rule_string);
                }
                selector_parts = vec![];
                current_selector = None;
                pending_combinator = Combinator::None;
                next_selector = NextSelector::Type;
            }
            _ => {
                // Any other token (like a comma) finalizes the current rule
                if let Some(selector) = current_selector.take() {
                    selector_parts.push(SelectorPart {
                        selector,
                        combinator: Combinator::None,
                    });
                }
                if !selector_parts.is_empty() {
                    // Convert selector parts to string
                    let rule_string = selector_parts
                        .iter()
                        .map(|part| part.to_string())
                        .collect::<String>();
                    rules.push(rule_string);
                }
                selector_parts = vec![];
                current_selector = None;
                pending_combinator = Combinator::None;
                next_selector = NextSelector::Type;
            }
        }
    }

    rules.sort();
    rules.dedup();
    rules
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct Nfacell(pub usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct SelectorId(pub usize);
/// 转移规则: (输入选择器, 当前状态, 下一个状态)
/// 其中输入选择器为 None 表示通配符/epsilon 或者特殊匹配；当前状态为 None 可用于起始逻辑
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rule(pub Option<SelectorId>, pub Option<Nfacell>, pub Nfacell);
