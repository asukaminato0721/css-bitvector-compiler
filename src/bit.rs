use cssparser::{Parser, ParserInput, Token};
use std::{ascii::escape_default, collections::{HashMap, HashSet}};

use css_bitvector_compiler::Cache;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum CssRule {
    Simple(Selector),
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
    pub tag_name: String,
    pub id: u64,
    pub html_id: Option<String>,
    pub class: HashSet<String>,
    pub children: Vec<BitVectorHtmlNode>,
    pub parent: Option<*mut BitVectorHtmlNode>, // TODO: use u64 in future
    cache: BitVectorCache,
}

impl BitVectorHtmlNode {
    fn init(&mut self, hm: &HashMap<Selector, usize>) {
        let s = std::fs::read_to_string(format!(
            "css-gen-op/{}/command.json",
            std::env::var("WEBSITE_NAME").unwrap()
        ))
        .unwrap();
        let first_line = s.lines().next().unwrap();
        let trace_data: serde_json::Value = serde_json::from_str(first_line).unwrap();
        *self = self.json_to_html_node(&trace_data["node"], hm);
    }

    fn json_to_html_node(
        &mut self,
        json_node: &serde_json::Value,
        hm: &HashMap<Selector, usize>,
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
    fn match_css(&self, arr: &[bool], hm: &HashMap<Selector, usize>) -> bool {
        // self has tag_name, html_id, class: set<str>
        // html_id
        let is_id_match = if let Some(id) = &self.html_id {
            if let Some(v) = hm.get(&Selector::Id(id.to_string())) {
                arr[*v]
            } else {
                true
            }
        } else {
            true
        };
        let is_tag_match = if let Some(v) = hm.get(&Selector::Type(self.tag_name.clone())) {
            arr[*v]
        } else {true};
        let is_class_match = self.class.iter().map(|cls|
          todo!()
        );
        is_id_match && is_tag_match
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

fn main() {
    let css = parse_css(
        &std::fs::read_to_string(format!(
            "css-gen-op/{0}/{0}.css",
            std::env::var("WEBSITE_NAME").unwrap(),
        ))
        .unwrap(),
    );
    let mut hm = HashMap::new();
    dbg!(&css);
    for i in css {
        match i {
            CssRule::Descendant { selectors } => {
                for s in selectors {
                    if !hm.contains_key(&s) {
                        hm.insert(s, hm.len());
                    }
                }
            }
            CssRule::Simple(s) => {
                if !hm.contains_key(&s) {
                    hm.insert(s, hm.len());
                }
            }
        }
    }

    let mut bit = BitVectorHtmlNode::default();
    bit.init(&hm);
   // dbg!(bit);
}
