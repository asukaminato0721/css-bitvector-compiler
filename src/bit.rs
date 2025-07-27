use cssparser::{Parser, ParserInput, Token};
use std::collections::{HashMap, HashSet};

use css_bitvector_compiler::Cache;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum CssRule {
    Descendant { selectors: Vec<Selector> },
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Selector {
    Type(String),
    Class(String),
    Id(String),
}

#[derive(Debug, Default)]
struct BitVectorCache {
    dirtynode: bool,
    result: Vec<bool>,
}

#[derive(Debug, Default)]

struct BitVectorHtmlNode {
    tag_name: String,
    id: u64,
    html_id: Option<String>,
    class: HashSet<String>,
    children: Vec<BitVectorHtmlNode>,
    input_state: Vec<bool>,
    output_state: Vec<bool>,
    parent: Option<*mut BitVectorHtmlNode>, // TODO: use u64 in future
    cache: BitVectorCache,
    dirty: bool,
}

impl BitVectorHtmlNode {
    fn json_to_html_node(
        &mut self,
        json_node: &serde_json::Value,
        hm: &HashMap<CssRule, usize>,
    ) -> Self {
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
        node.class = json_node["attributes"]
            .as_object()
            .unwrap()
            .get("class")
            .map(|x| x.as_str().unwrap())
            .unwrap_or_default()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect();

        // Add children recursively
        node.children = {
            let children = json_node["children"].as_array().unwrap();
            children
                .into_iter()
                .map(|x| self.json_to_html_node(x, &hm))
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
    fn matches_simple_selector(&self, selector: &Selector) -> bool {
        match selector {
            Selector::Type(tag) => self.tag_name.to_lowercase() == tag.to_lowercase(),
            Selector::Class(class) => self.class.contains(class),
            Selector::Id(id) => {
                if let Some(ref html_id) = self.html_id {
                    html_id == id
                } else {
                    false
                }
            }
        }
    }
    /// first match is strict match, after can have loose match, so split into 2 func
    fn matches_descendant_selector_after(&self, selectors: &[Selector]) -> bool {
        match (self.parent, selectors.len()) {
            (_, 0) => true,
            (None, 1) => self.matches_simple_selector(selectors.last().unwrap()),
            (None, 2..) => false,
            (Some(p), 1..) => {
                let p = unsafe { &*p };
                if self.matches_simple_selector(selectors.last().unwrap()) {
                    p.matches_descendant_selector_after(&selectors[..selectors.len() - 1])
                } else {
                    p.matches_descendant_selector_after(selectors)
                }
            }
        }
    }
    fn matches_descendant_selector(&self, selectors: &[Selector]) -> bool {
        match (self.parent, selectors.len()) {
            (_, 0) => true,
            (None, 1) => self.matches_simple_selector(&selectors[0]),
            (None, 2..) => false,
            (Some(p), 1..) => {
                let p = unsafe { &*p };
                if self.matches_simple_selector(selectors.last().unwrap()) {
                    p.matches_descendant_selector_after(&selectors[..selectors.len() - 1])
                } else {
                    false
                }
            }
        }
    }
    fn matches_css_rule(&self, CssRule::Descendant { selectors }: &CssRule) -> bool {
        self.matches_descendant_selector(selectors)
    }
    fn collect_matches(&self, rule: &CssRule, matches: &mut HashSet<u64>) {
        if self.matches_css_rule(rule) {
            matches.insert(self.id);
        }
        for child in &self.children {
            child.collect_matches(rule, matches);
        }
    }
    fn print_css_matches(&self, rules: &[CssRule]) {
        for rule in rules {
            let mut matches = HashSet::new();
            self.collect_matches(rule, &mut matches);
            println!("{:?} -> {:?}", rule, matches);
        }
    }
    fn add_by_path(
        &mut self,
        path: &[usize],
        node: &serde_json::Value,
        hm: &HashMap<CssRule, usize>,
    ) {
        assert!(!path.is_empty());
        if path.len() == 1 {
            let n = self.json_to_html_node(node, &hm);
            self.children.insert(path[0], n);
            self.dirty = true;
            let mut cur: *mut BitVectorHtmlNode = self;
            unsafe {
                while let Some(parent_ptr) = (*cur).parent {
                    (*parent_ptr).dirty = true;
                    cur = parent_ptr;
                }
            }
            return;
        }
        self.children[path[0]].add_by_path(&path[1..], node, &hm);
        self.fix_parent_pointers(); // TODO: optimize
    }
    fn remove_by_path(&mut self, path: &[usize]) {
        assert!(!path.is_empty());
        if path.len() == 1 {
            self.children.remove(path[0]);
            self.dirty = true;
            let mut cur: *mut BitVectorHtmlNode = self;
            unsafe {
                while let Some(parent_ptr) = (*cur).parent {
                    (*parent_ptr).dirty = true;
                    cur = parent_ptr;
                }
            }
            return;
        }
        self.children[path[0]].remove_by_path(&path[1..]);
    }
}

impl Cache<BitVectorHtmlNode> for BitVectorHtmlNode {
    fn dirtied(&mut self, path: &[u64]) {
        if path.is_empty() {
            self.cache.dirtynode = true;
            return;
        }
        self.dirtied(&path[1..]);
    }
    fn recompute(&mut self, root: &mut BitVectorHtmlNode) {
        unimplemented!()
    }
}

fn parse_css(css_content: &str) -> Vec<CssRule> {
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
                        rules.push(CssRule::Descendant {
                            selectors: selector_chain,
                        });
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
                        selector_chain.push(selector);
                        rules.push(CssRule::Descendant {
                            selectors: selector_chain,
                        });
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
fn apply_frame(tree: &mut BitVectorHtmlNode, frame: &LayoutFrame, hm: &HashMap<CssRule, usize>) {
    match frame.command_name.as_str() {
        "init" => {
            dbg!(frame.frame_id, frame.command_name.as_str());
            *tree = tree.json_to_html_node(frame.command_data.get("node").unwrap(), &hm);
            tree.fix_parent_pointers();
        }
        "add" => {
            dbg!(frame.frame_id, frame.command_name.as_str());
            let path = extract_path_from_command(&frame.command_data);
            if path.is_empty() {
                return;
            }
            tree.add_by_path(&path, frame.command_data.get("node").unwrap(), &hm);
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
fn main() {
    let css = parse_css(
        &std::fs::read_to_string(format!(
            "css-gen-op/{0}/{0}.css",
            std::env::var("WEBSITE_NAME").unwrap(),
        ))
        .unwrap(),
    );
    //dbg!(&css);
    let hm = {
        let mut hm = HashMap::new();
        for CssRule::Descendant { selectors } in &css {
            let mut v = vec![];
            for s in selectors {
                v.push(s.clone());
                let ss = CssRule::Descendant {
                    selectors: v.clone(),
                };
                if !hm.contains_key(&ss) {
                    hm.insert(ss, hm.len());
                }
            }
        }
        hm
    };
    dbg!(&hm);

    let mut bit = BitVectorHtmlNode::default();
    let trace = parse_trace();
    for i in &trace {
        apply_frame(&mut bit, &i, &hm);
    }
    bit.print_css_matches(&css);
    //dbg!(bit);
}
