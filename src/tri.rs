use css_bitvector_compiler::Cache;
use cssparser::{Parser, ParserInput, Token};
use std::collections::HashSet;

/// whether a part of input is: 1, 0, or unused

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum SimpleSelector {
    Type(String),
    Class(String),
    Id(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CssRule {
    Simple(SimpleSelector),
    Compound { selectors: Vec<SimpleSelector> },
}

fn parse_css(css_content: &str) -> Vec<CssRule> {
    let mut rules = Vec::new();
    let mut input = ParserInput::new(css_content);
    let mut parser = Parser::new(&mut input);

    let mut expecting_rule_body = false;
    let mut selector_chain: Vec<SimpleSelector> = Vec::new();
    let mut current_selector: Option<SimpleSelector> = None;

    while !parser.is_exhausted() {
        let Ok(token) = parser.next() else {
            break;
        };

        if expecting_rule_body {
            match token {
                Token::CurlyBracketBlock => {
                    if let Some(selector) = current_selector.take() {
                        selector_chain.push(selector);
                    }
                    if !selector_chain.is_empty() {
                        if selector_chain.len() == 1 {
                            rules.push(CssRule::Simple(selector_chain.into_iter().next().unwrap()));
                        } else {
                            rules.push(CssRule::Compound {
                                selectors: selector_chain,
                            });
                        }
                    }

                    selector_chain = Vec::new();
                    expecting_rule_body = false;
                }
                _ => {
                    expecting_rule_body = false;
                    current_selector = None;
                    selector_chain.clear();
                }
            }
        } else {
            match token {
                Token::Ident(name) => {
                    let type_name = name.to_string().to_lowercase();

                    if let Some(prev_selector) = current_selector.take() {
                        selector_chain.push(prev_selector);
                    }

                    current_selector = Some(SimpleSelector::Type(type_name));
                }
                Token::IDHash(id) => {
                    if let Some(prev_selector) = current_selector.take() {
                        selector_chain.push(prev_selector);
                    }

                    current_selector = Some(SimpleSelector::Id(id.to_string()));
                }
                Token::Delim('.') => {
                    if let Ok(Token::Ident(class_name)) = parser.next() {
                        if let Some(prev_selector) = current_selector.take() {
                            selector_chain.push(prev_selector);
                        }

                        current_selector = Some(SimpleSelector::Class(class_name.to_string()));
                    }
                }
                Token::CurlyBracketBlock => {
                    if let Some(selector) = current_selector.take() {
                        if selector_chain.is_empty() {
                            rules.push(CssRule::Simple(selector));
                        } else {
                            selector_chain.push(selector);
                            rules.push(CssRule::Compound {
                                selectors: selector_chain,
                            });
                        }
                    }
                    selector_chain = Vec::new();
                }
                Token::WhiteSpace(_) => {
                    if current_selector.is_some() {
                        expecting_rule_body = true;
                    }
                }
                _ => {
                    current_selector = None;
                    selector_chain.clear();
                }
            }
        }
    }

    rules.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));
    rules.dedup();
    rules
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IState {
    IOne,
    IZero,
    IUnused,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OState {
    OOne,
    OZero,
    OFromParent,
}
#[derive(Debug, Default)]

struct TriVectorCache {
    dirtynode: bool,
    parent: Vec<IState>,
    result: Vec<bool>,
}

#[derive(Debug, Default)]

struct TriVectorHtmlNode {
    pub tag_name: String,
    pub id: u64,
    pub html_id: Option<String>,
    pub classes: HashSet<String>,
    pub children: Vec<TriVectorHtmlNode>,
    pub parent: Option<*mut TriVectorHtmlNode>, // TODO: use u64 in future
    cache: TriVectorCache,
}

impl TriVectorHtmlNode {
    fn init(&mut self) {
        let s = std::fs::read_to_string(format!(
            "css-gen-op/{}/command.json",
            std::env::var("WEBSITE_NAME").unwrap()
        ))
        .unwrap();
        let first_line = s.lines().next().unwrap();
        let trace_data: serde_json::Value = serde_json::from_str(first_line).unwrap();
        *self = self.json_dom_to_html_node(&trace_data["node"]);
    }

    fn json_dom_to_html_node(&mut self, json_node: &serde_json::Value) -> Self {
        let mut node = Self::default();
        //  dbg!(&json_node);
        node.tag_name = json_node["name"].as_str().unwrap().into();
        node.id = json_node["id"].as_u64().unwrap();
        node.html_id = {
            let attributes = json_node["attributes"].as_object().unwrap();
            attributes
                .get("id")
                .and_then(|x| x.as_str())
                .map(String::from)
        };
        // Add classes from attributes
        node.classes = {
            let attributes = json_node["attributes"].as_object().unwrap();
            let class_str = attributes
                .get("class")
                .map(|x| x.as_str().unwrap())
                .unwrap_or_default();
            class_str
                .split_whitespace()
                .map(|x| x.into())
                .collect::<HashSet<String>>()
        };

        // Add children recursively
        node.children = {
            let children = json_node["children"].as_array().unwrap();
            children
                .into_iter()
                .map(|x| self.json_dom_to_html_node(x))
                .collect()
        };
        node.fix_parent_pointers();
        node
    }
    fn fix_parent_pointers(&mut self) {
        let self_ptr = self as *mut Self;
        for child in self.children.iter_mut() {
            child.parent = Some(self_ptr);
            child.fix_parent_pointers();
        }
    }
}

impl Cache<TriVectorHtmlNode> for TriVectorHtmlNode {
    fn dirtied(&mut self, path: &[u64]) {
        if path.is_empty() {
            self.cache.dirtynode = true;
            return;
        }
        self.dirtied(&path[1..]);
    }
    fn recompute(&mut self, root: &mut TriVectorHtmlNode) {
        unimplemented!()
    }
}

fn main() {
    let mut tri = TriVectorHtmlNode::default();
    tri.init();
    let css = parse_css(
        &std::fs::read_to_string(format!(
            "css-gen-op/{0}/{0}.css",
            std::env::var("WEBSITE_NAME").unwrap(),
        ))
        .unwrap(),
    );
    dbg!(&tri);
    dbg!(&css);
}
