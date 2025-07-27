use cssparser::{Parser, ParserInput, Token};
use std::collections::HashSet;

use css_bitvector_compiler::Cache;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CssRule {
    Simple(Selector),
    Descendant { selectors: Vec<Selector> },
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Selector {
    Type(String),
    Class(String),
    Id(String),
}

pub fn parse_css(css_content: &str) -> Vec<CssRule> {
    let mut rules = Vec::new();
    let mut input = ParserInput::new(css_content);
    let mut parser = Parser::new(&mut input);

    let mut expecting_rule_body = false;
    let mut selector_chain: Vec<Selector> = Vec::new();
    let mut current_selector: Option<Selector> = None;

    while let Ok(token) = parser.next() {
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
                            rules.push(CssRule::Descendant {
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

                    current_selector = Some(Selector::Type(type_name));
                }
                Token::IDHash(id) => {
                    if let Some(prev_selector) = current_selector.take() {
                        selector_chain.push(prev_selector);
                    }

                    current_selector = Some(Selector::Id(id.to_string()));
                }
                Token::Delim('.') => {
                    if let Ok(Token::Ident(class_name)) = parser.next() {
                        if let Some(prev_selector) = current_selector.take() {
                            selector_chain.push(prev_selector);
                        }

                        current_selector = Some(Selector::Class(class_name.to_string()));
                    }
                }
                Token::CurlyBracketBlock => {
                    if let Some(selector) = current_selector.take() {
                        if selector_chain.is_empty() {
                            rules.push(CssRule::Simple(selector));
                        } else {
                            selector_chain.push(selector);
                            rules.push(CssRule::Descendant {
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
        //  dbg!(&json_node);
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
    fn matches_descendant_selector(&self, selectors: &[Selector]) -> bool {
        match (self.parent, selectors.len()) {
            (_, 0) => true,
            (None, 1) => self.matches_simple_selector(selectors.last().unwrap()),
            (None, 2..) => false,
            (Some(p), 1..) => {
                let p = unsafe { &*p };
                if self.matches_simple_selector(selectors.last().unwrap()) {
                    p.matches_descendant_selector(&selectors[..selectors.len() - 1])
                } else {
                    p.matches_descendant_selector(selectors)
                }
            }
        }
    }
    fn matches_css_rule(&self, rule: &CssRule) -> bool {
        match rule {
            CssRule::Simple(selector) => self.matches_simple_selector(selector),
            CssRule::Descendant { selectors } => self.matches_descendant_selector(selectors),
        }
    }
    fn collect_matches(&self, rule: &CssRule, matches: &mut Vec<u64>) {
        if self.matches_css_rule(rule) {
            matches.push(self.id);
        }
        for child in &self.children {
            child.collect_matches(rule, matches);
        }
    }
    fn print_css_matches(&self, rules: &[CssRule]) {
        for rule in rules {
            let mut matches = Vec::new();
            self.collect_matches(rule, &mut matches);
            println!("{:?} -> {:?}", rule, matches);
        }
    }
}
#[derive(Debug, Default)]

struct NaiveHtmlNode {
    pub tag_name: String,
    pub id: u64,
    pub html_id: Option<String>,
    pub classes: HashSet<String>,
    pub children: Vec<NaiveHtmlNode>,
    pub parent: Option<*mut NaiveHtmlNode>, // TODO: use u64 in future
}

impl Cache<NaiveHtmlNode> for NaiveHtmlNode {
    fn dirtied(&mut self, _: &[u64]) {}
    fn recompute(&mut self, _: &mut NaiveHtmlNode) {}
}

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
            dbg!(frame.frame_id, frame.command_name.as_str());
            *tree = tree.json_to_node(frame.command_data.get("node").unwrap());
            tree.fix_parent_pointers();
        }
        "add" => {
            dbg!(frame.frame_id, frame.command_name.as_str());
            let path = extract_path_from_command(&frame.command_data);
            if path.is_empty() {
                return;
            }
            tree.add_by_path(&path, frame.command_data.get("node").unwrap());
            tree.fix_parent_pointers(); // TODO: optimize
        }
        "replace_value" | "insert_value" => {
            dbg!(frame.frame_id, frame.command_name.as_str());
        }
        "recalculate" => {
            dbg!(frame.frame_id, frame.command_name.as_str());
        }
        "remove" => {
            dbg!(frame.frame_id, frame.command_name.as_str());
            let path = extract_path_from_command(&frame.command_data);
            tree.remove_by_path(&path);
        }
        _ => {
            dbg!(frame.frame_id, frame.command_name.as_str());
        }
    }
}

// 分离 3 种不同的 node, naive , bit, tri
// 对每种 node, 实现一个公共的 trait, recompute, dirtied.
// recompute 是实际做计算的
// dirtied 只是做脏标记
fn main() {
    let mut naive = NaiveHtmlNode::default();
    let css = parse_css(
        &std::fs::read_to_string(format!(
            "css-gen-op/{0}/{0}.css",
            std::env::var("WEBSITE_NAME").unwrap(),
        ))
        .unwrap(),
    );
    // dbg!(&naive);
    //  dbg!(&css);
    let trace = parse_trace();

    for i in &trace {
        apply_frame(&mut naive, &i);
    }
    naive.print_css_matches(&css);
    //  dbg!(trace);
}
