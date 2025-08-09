use cssparser::{Parser, ParserInput, Token};
use std::collections::HashSet;

use css_bitvector_compiler::Cache;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum CssRule {
    Complex { parts: Vec<SelectorPart> },
}

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

fn parse_css(css_content: &str) -> Vec<CssRule> {
    let mut rules = Vec::new();
    let mut input = ParserInput::new(css_content);
    let mut parser = Parser::new(&mut input);

    let mut selector_parts: Vec<SelectorPart> = Vec::new();
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
                    rules.push(CssRule::Complex {
                        parts: selector_parts,
                    });
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
                    rules.push(CssRule::Complex {
                        parts: selector_parts,
                    });
                }
                selector_parts = Vec::new();
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
                    rules.push(CssRule::Complex {
                        parts: selector_parts,
                    });
                }
                selector_parts = Vec::new();
                current_selector = None;
                pending_combinator = Combinator::None;
                next_selector = NextSelector::Type;
            }
        }
    }

    rules.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));
    rules.dedup();
    rules
}

// note: do nt pull out bitvector result; - absvector will change that laters
// to other type struct NaiveCache {
// no dirty node anywhere, have to recompute from scratch
// bitvector result;
//}
impl NaiveHtmlNode {
    fn add_by_path(&mut self, path: &[usize], node: &serde_json::Value) {
        assert!(!path.is_empty());
        if path.len() == 1 {
            self.children.insert(path[0], self.json_to_node(node));
            return;
        }
        self.children[path[0]].add_by_path(&path[1..], node);
        self.fix_parent_pointers(); // TODO: optimize
    }
    fn remove_by_path(&mut self, path: &[usize]) {
        assert!(!path.is_empty());
        if path.len() == 1 {
            self.children.remove(path[0]);
            return;
        }
        self.children[path[0]].remove_by_path(&path[1..]);
    }

    fn json_to_node(&self, json_node: &serde_json::Value) -> Self {
        let mut node = Self::default();
        //  // dbg!(&json_node);
        node.tag_name = json_node["name"].as_str().unwrap().into();
        node.id = json_node["id"].as_u64().unwrap();
        node.html_id = json_node["attributes"]
            .as_object()
            .unwrap()
            .get("id")
            .and_then(|x| x.as_str())
            .map(String::from);
        // Add classes from attributes
        node.classes = json_node["attributes"]
            .as_object()
            .unwrap()
            .get("class")
            .map(|x| x.as_str().unwrap())
            .unwrap_or_default()
            .split_whitespace()
            .map(|x| x.into())
            .collect::<HashSet<String>>();

        // Add children recursively
        node.children = json_node["children"]
            .as_array()
            .unwrap()
            .into_iter()
            .map(|x| self.json_to_node(x))
            .collect();
        node
    }
    fn fix_parent_pointers(&mut self) {
        let self_ptr = self as *mut Self;
        for child in self.children.iter_mut() {
            child.parent = Some(self_ptr);
            child.fix_parent_pointers();
        }
    }
    fn matches_simple_selector(&self, selector: &Selector) -> bool {
        match selector {
            Selector::Type(tag) => self.tag_name.to_lowercase() == tag.to_lowercase(),
            Selector::Class(class) => self.classes.contains(class),
            Selector::Id(id) => {
                if let Some(ref html_id) = self.html_id {
                    html_id == id
                } else {
                    false
                }
            }
        }
    }

    fn matches_complex_selector(&self, parts: &[SelectorPart]) -> bool {
        if parts.is_empty() {
            return true;
        }

        let last_part = &parts[parts.len() - 1];
        if !self.matches_simple_selector(&last_part.selector) {
            return false;
        }

        if parts.len() == 1 {
            return true;
        }

        match self.parent {
            None => false,
            Some(parent_ptr) => {
                let parent = unsafe { &*parent_ptr };
                let remaining_parts = &parts[..parts.len() - 1];

                // 获取前一个组合器（这是连接当前选择器和父级的组合器）
                let combinator = if parts.len() >= 2 {
                    &parts[parts.len() - 2].combinator
                } else {
                    &Combinator::None
                };

                match combinator {
                    Combinator::None => {
                        // 这不应该发生在中间部分
                        parent.matches_complex_selector_recursive(remaining_parts)
                    }
                    Combinator::Child => {
                        // 直接子代：父节点必须精确匹配剩余的选择器
                        parent.matches_complex_selector(remaining_parts)
                    }
                    Combinator::Descendant => {
                        // 后代：可以在祖先链上任意位置匹配
                        parent.matches_complex_selector_recursive(remaining_parts)
                    }
                }
            }
        }
    }

    fn matches_complex_selector_recursive(&self, parts: &[SelectorPart]) -> bool {
        if self.matches_complex_selector(parts) {
            return true;
        }

        match self.parent {
            None => false,
            Some(parent_ptr) => {
                let parent = unsafe { &*parent_ptr };
                parent.matches_complex_selector_recursive(parts)
            }
        }
    }

    fn matches_css_rule(&self, CssRule::Complex { parts }: &CssRule) -> bool {
        self.matches_complex_selector(parts)
    }
    fn collect_matches(&self, rule: &CssRule, matches: &mut Vec<u64>) {
        if self.matches_css_rule(rule) {
            matches.push(self.id);
        }
        for child in &self.children {
            child.collect_matches(rule, matches);
        }
    }
    fn print_css_matches(&self, rules: &mut [CssRule]) {
        rules.sort_by_key(|x| format!("{x:?}"));
        for rule in rules {
            let mut matches = Vec::new();
            self.collect_matches(rule, &mut matches);
            if matches.is_empty() {
                continue;
            }
            matches.sort_unstable();
            matches.dedup();
            println!("{:?} -> {:?}", rule, matches);
        }
    }
}
#[derive(Debug, Default)]

struct NaiveHtmlNode {
    tag_name: String,
    id: u64,
    html_id: Option<String>,
    classes: HashSet<String>,
    children: Vec<NaiveHtmlNode>,
    parent: Option<*mut NaiveHtmlNode>, // TODO: use u64 in future
}

impl Cache<NaiveHtmlNode> for NaiveHtmlNode {
    fn dirtied(&mut self, _: &[u64]) {}
    fn recompute(&mut self, _: &mut NaiveHtmlNode) {}
}
#[allow(unused)]
#[derive(Debug, Clone)]
struct LayoutFrame {
    pub frame_id: usize,
    pub command_name: String,
    pub command_data: serde_json::Value,
}

fn parse_trace() -> Vec<LayoutFrame> {
    let content = std::fs::read_to_string(format!(
        "css-gen-op/{0}/command.json",
        std::env::var("WEBSITE_NAME").unwrap()
    ))
    .unwrap();

    let mut frames = Vec::new();
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

fn extract_path_from_command(command_data: &serde_json::Value) -> Vec<usize> {
    command_data
        .get("path")
        .and_then(|p| p.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_u64())
                .map(|v| v as usize)
                .collect::<Vec<_>>()
        })
        .unwrap()
}

fn apply_frame(tree: &mut NaiveHtmlNode, frame: &LayoutFrame) {
    match frame.command_name.as_str() {
        "init" => {
            // dbg!(frame.frame_id, frame.command_name.as_str());
            *tree = tree.json_to_node(frame.command_data.get("node").unwrap());
            tree.fix_parent_pointers();
        }
        "add" => {
            // dbg!(frame.frame_id, frame.command_name.as_str());
            let path = extract_path_from_command(&frame.command_data);
            if path.is_empty() {
                return;
            }
            tree.add_by_path(&path, frame.command_data.get("node").unwrap());
            tree.fix_parent_pointers(); // TODO: optimize
        }
        "replace_value" | "insert_value" => {
            // dbg!(frame.frame_id, frame.command_name.as_str());
        }
        "recalculate" => {
            // dbg!(frame.frame_id, frame.command_name.as_str());
        }
        "remove" => {
            // dbg!(frame.frame_id, frame.command_name.as_str());
            let path = extract_path_from_command(&frame.command_data);
            tree.remove_by_path(&path);
        }
        _ => {
            // dbg!(frame.frame_id, frame.command_name.as_str());
        }
    }
}

// 分离 3 种不同的 node, naive , bit, tri
// 对每种 node, 实现一个公共的 trait, recompute, dirtied.
// recompute 是实际做计算的
// dirtied 只是做脏标记
fn main() {
    let mut naive = NaiveHtmlNode::default();
    let mut css = parse_css(
        &std::fs::read_to_string(format!(
            "css-gen-op/{0}/{0}.css",
            std::env::var("WEBSITE_NAME").unwrap(),
        ))
        .unwrap(),
    );
    dbg!(&naive);
    dbg!(&css);
    let trace = parse_trace();

    for i in &trace {
        apply_frame(&mut naive, &i);
    }
    naive.print_css_matches(&mut css);
    //  // dbg!(trace);
    //// dbg!(naive);
}
