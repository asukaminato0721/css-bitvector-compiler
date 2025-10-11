use cssparser::{ParseError, Parser, ParserInput, Token};
use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
};

use css_bitvector_compiler::{Cache, json_value_to_attr_string};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum CssRule {
    Complex { parts: Vec<SelectorPart> },
}

impl Display for CssRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CssRule::Complex { parts } => {
                for part in parts {
                    write!(f, "{}", part)?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SelectorPart {
    selector: Selector,
    combinator: Combinator,
}

impl Display for SelectorPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.selector)?;
        match self.combinator {
            Combinator::Descendant => write!(f, " "),
            Combinator::Child => write!(f, " > "),
            Combinator::None => Ok(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Combinator {
    Descendant, // 空格
    Child,      // >
    None,       // 最后一个选择器没有组合器
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Selector {
    Type(String),
    Class(String),
    Id(String),
    AttributeEquals { name: String, value: String },
}

impl Display for Selector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Selector::Type(tag) => write!(f, "{}", tag),
            Selector::Class(class) => write!(f, ".{}", class),
            Selector::Id(id) => write!(f, "#{}", id),
            Selector::AttributeEquals { name, value } => {
                write!(f, "[{}=\"{}\"]", name, value)
            }
        }
    }
}

fn parse_css(css_content: &str) -> Vec<CssRule> {
    let mut rules = vec![];
    let mut input = ParserInput::new(css_content);
    let mut parser = Parser::new(&mut input);

    let mut selector_parts: Vec<SelectorPart> = vec![];
    let mut current_selector: Option<Selector> = None;
    let mut pending_combinator = Combinator::None;
    let mut skip_next_simple_selector = false;
    let mut skip_at_rule = false;

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

        if skip_at_rule {
            match token {
                Token::CurlyBracketBlock | Token::Semicolon => {
                    skip_at_rule = false;
                    continue;
                }
                _ => continue,
            }
        }

        match token {
            Token::Comment(_) => continue,
            Token::WhiteSpace(_) => {
                if skip_next_simple_selector {
                    skip_next_simple_selector = false;
                }
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
            Token::AtKeyword(_) => {
                selector_parts.clear();
                current_selector = None;
                pending_combinator = Combinator::None;
                skip_next_simple_selector = false;
                skip_at_rule = true;
                continue;
            }
            Token::Colon => {
                skip_next_simple_selector = true;
                continue;
            }
            Token::Function(_) => {
                if skip_next_simple_selector {
                    let _ = parser.parse_nested_block(|nested| -> Result<(), ParseError<'_, ()>> {
                        while nested.next_including_whitespace_and_comments().is_ok() {}
                        Ok(())
                    });
                    skip_next_simple_selector = false;
                    next_selector = NextSelector::Type;
                    continue;
                }
            }
            Token::ParenthesisBlock => {
                if skip_next_simple_selector {
                    skip_next_simple_selector = false;
                    next_selector = NextSelector::Type;
                    continue;
                }
            }
            Token::SquareBracketBlock => {
                if let Some(prev_selector) = current_selector.take() {
                    selector_parts.push(SelectorPart {
                        selector: prev_selector,
                        combinator: pending_combinator.clone(),
                    });
                }
                pending_combinator = Combinator::None;
                if let Some(attribute_selector) = parse_attribute_selector_block(&mut parser) {
                    current_selector = Some(attribute_selector);
                }
                next_selector = NextSelector::Type;
            }
            Token::IDHash(id) => {
                if skip_next_simple_selector {
                    skip_next_simple_selector = false;
                    next_selector = NextSelector::Type;
                    continue;
                }
                if let Some(prev_selector) = current_selector.take() {
                    selector_parts.push(SelectorPart {
                        selector: prev_selector,
                        combinator: pending_combinator.clone(),
                    });
                    pending_combinator = Combinator::None;
                }
                current_selector = Some(Selector::Id(id.to_lowercase().to_string()));
                next_selector = NextSelector::Type;
            }
            Token::Ident(name) => {
                if skip_next_simple_selector {
                    skip_next_simple_selector = false;
                    next_selector = NextSelector::Type;
                    continue;
                }
                let s = match next_selector {
                    NextSelector::Class => Selector::Class(name.to_lowercase().to_string()),
                    NextSelector::Type => Selector::Type(name.to_lowercase().to_string()),
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
                    rules.push(CssRule::Complex {
                        parts: selector_parts,
                    });
                }
                selector_parts = vec![];
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

fn parse_attribute_selector_block(parser: &mut Parser) -> Option<Selector> {
    parser
        .parse_nested_block(|nested| -> Result<Selector, ParseError<'_, ()>> {
            nested.skip_whitespace();
            let name = nested
                .expect_ident_cloned()
                .map_err(ParseError::from)?
                .to_ascii_lowercase();
            nested.skip_whitespace();
            nested.expect_delim('=').map_err(ParseError::from)?;
            nested.skip_whitespace();
            let value = nested
                .expect_string_cloned()
                .map_err(ParseError::from)?
                .to_string();
            nested.skip_whitespace();
            Ok(Selector::AttributeEquals { name, value })
        })
        .ok()
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
            self.children.insert(path[0], Self::json_to_node(node));
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

    fn node_mut_by_path(&mut self, path: &[usize]) -> Option<&mut Self> {
        if path.is_empty() {
            return Some(self);
        }
        let mut current = self;
        for &idx in path {
            current = current.children.get_mut(idx)?;
        }
        Some(current)
    }

    fn set_attribute(&mut self, key: &str, new_value: Option<String>) {
        let key_lower = key.to_lowercase();
        match key_lower.as_str() {
            "class" => {
                let mut new_classes = HashSet::new();
                if let Some(ref value) = new_value {
                    for class_name in value.split_whitespace().filter(|name| !name.is_empty()) {
                        new_classes.insert(class_name.to_string());
                    }
                }
                self.classes = new_classes;
            }
            "id" => {
                self.html_id = new_value.clone();
            }
            _ => {}
        }

        if let Some(value) = new_value {
            self.attributes.insert(key_lower, value);
        } else {
            self.attributes.remove(&key_lower);
        }
    }

    fn json_to_node(json_node: &serde_json::Value) -> Self {
        let mut node = Self::default();
        //  // dbg!(&json_node);
        node.tag_name = json_node["name"].as_str().unwrap().into();
        node.id = json_node["id"].as_u64().unwrap();
        let attributes = json_node["attributes"]
            .as_object()
            .map(|attrs| {
                attrs
                    .iter()
                    .filter_map(|(name, value)| match value {
                        serde_json::Value::String(s) => Some((name.to_lowercase(), s.to_string())),
                        serde_json::Value::Number(n) => Some((name.to_lowercase(), n.to_string())),
                        serde_json::Value::Bool(b) => Some((name.to_lowercase(), b.to_string())),
                        _ => None,
                    })
                    .collect::<HashMap<_, _>>()
            })
            .unwrap_or_default();
        node.html_id = attributes.get("id").cloned();
        let class_attr = attributes.get("class").cloned().unwrap_or_default();
        // Add classes from attributes
        node.classes = class_attr
            .split_whitespace()
            .map(|x| x.into())
            .collect::<HashSet<String>>();
        node.attributes = attributes;

        // Add children recursively
        node.children = json_node["children"]
            .as_array()
            .unwrap()
            .iter()
            .map(Self::json_to_node)
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
            Selector::AttributeEquals { name, value } => self
                .attributes
                .get(name)
                .map(|v| v == value)
                .unwrap_or(false),
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
            let mut matches = vec![];
            self.collect_matches(rule, &mut matches);
            if matches.is_empty() {
                continue;
            }
            matches.sort_unstable();
            matches.dedup();
            println!("{} -> {:?}", rule, matches);
        }
    }
}
#[derive(Debug, Default)]

struct NaiveHtmlNode {
    tag_name: String,
    id: u64,
    html_id: Option<String>,
    attributes: HashMap<String, String>,
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
            *tree = NaiveHtmlNode::json_to_node(frame.command_data.get("node").unwrap());
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
        "replace_value" => {
            if frame.command_data["type"].as_str() != Some("attributes") {
                return;
            }
            let path = extract_path_from_command(&frame.command_data);
            let key = frame.command_data["key"].as_str().unwrap();
            let node = tree
                .node_mut_by_path(&path)
                .unwrap_or_else(|| panic!("invalid path {:?} for replace_value", path));
            if let Some(old_value) = frame.command_data.get("old_value") {
                let expected = json_value_to_attr_string(old_value);
                let actual = node
                    .attributes
                    .get(&key.to_lowercase())
                    .cloned()
                    .unwrap_or_default();
                debug_assert_eq!(
                    actual, expected,
                    "existing attribute value mismatch for key {} at path {:?}",
                    key, path
                );
            }
            let new_value = frame
                .command_data
                .get("value")
                .map(json_value_to_attr_string);
            node.set_attribute(key, new_value);
        }
        "insert_value" => {
            if frame.command_data["type"].as_str() != Some("attributes") {
                return;
            }
            let path = extract_path_from_command(&frame.command_data);
            let key = frame.command_data["key"].as_str().unwrap();
            let node = tree
                .node_mut_by_path(&path)
                .unwrap_or_else(|| panic!("invalid path {:?} for insert_value", path));
            let new_value = frame
                .command_data
                .get("value")
                .map(json_value_to_attr_string);
            node.set_attribute(key, new_value);
        }
        "delete_value" => {
            if frame.command_data["type"].as_str() != Some("attributes") {
                return;
            }
            let path = extract_path_from_command(&frame.command_data);
            let key = frame.command_data["key"].as_str().unwrap();
            let node = tree
                .node_mut_by_path(&path)
                .unwrap_or_else(|| panic!("invalid path {:?} for delete_value", path));
            if let Some(old_value) = frame.command_data.get("old_value") {
                let expected = json_value_to_attr_string(old_value);
                let actual = node
                    .attributes
                    .get(&key.to_lowercase())
                    .cloned()
                    .unwrap_or_default();
                debug_assert_eq!(
                    actual, expected,
                    "existing attribute value mismatch for key {} at path {:?}",
                    key, path
                );
            }
            node.set_attribute(key, None);
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

fn main() {
    let mut naive = NaiveHtmlNode::default();
    let mut css = parse_css(
        &std::fs::read_to_string(format!(
            "css-gen-op/{0}/{0}.css",
            std::env::var("WEBSITE_NAME").unwrap(),
        ))
        .unwrap(),
    );
    // dbg!(&naive);
    // dbg!(&css);
    let trace = parse_trace();

    for i in &trace {
        apply_frame(&mut naive, i);
    }
    println!("BEGIN");
    naive.print_css_matches(&mut css);
    println!("END");
    //  // dbg!(trace);
    //// dbg!(naive);
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base_case() {
        let s = "div h1 > h2 p .a > .b #c";
        dbg!(parse_css(s));
    }

    #[test]
    fn parse_attribute_selector() {
        let rules = parse_css(r#"[data-role="hero"] { color: red; }"#);
        assert_eq!(rules.len(), 1);
        match &rules[0] {
            CssRule::Complex { parts } => {
                assert_eq!(parts.len(), 1);
                match &parts[0].selector {
                    Selector::AttributeEquals { name, value } => {
                        assert_eq!(name, "data-role");
                        assert_eq!(value, "hero");
                    }
                    other => panic!("unexpected selector: {:?}", other),
                }
            }
        }
    }

    #[test]
    fn matches_attribute_selector_on_node() {
        let mut node = NaiveHtmlNode::default();
        node.attributes.insert("data-id".into(), "item-1".into());
        let selector = Selector::AttributeEquals {
            name: "data-id".into(),
            value: "item-1".into(),
        };
        assert!(node.matches_simple_selector(&selector));

        let mismatch = Selector::AttributeEquals {
            name: "data-id".into(),
            value: "item-2".into(),
        };
        assert!(!node.matches_simple_selector(&mismatch));
    }

    #[test]
    fn parse_css_skips_pseudo_classes() {
        let rules = parse_css(".wrapper .item:hover strong { font-weight: bold; }");
        assert_eq!(rules.len(), 1);
        let CssRule::Complex { parts } = &rules[0];
        assert_eq!(parts.len(), 3);
        match (&parts[0].selector, &parts[0].combinator) {
            (Selector::Class(class), Combinator::Descendant) => assert_eq!(class, "wrapper"),
            other => panic!("unexpected first part: {:?}", other),
        }
        match (&parts[1].selector, &parts[1].combinator) {
            (Selector::Class(class), Combinator::Descendant) => assert_eq!(class, "item"),
            other => panic!("unexpected second part: {:?}", other),
        }
        match (&parts[2].selector, &parts[2].combinator) {
            (Selector::Type(tag), Combinator::None) => assert_eq!(tag, "strong"),
            other => panic!("unexpected third part: {:?}", other),
        }
    }
}
