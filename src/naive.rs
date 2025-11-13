use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
};

use css_bitvector_compiler::{
    Command, is_simple_selector, json_value_to_attr_string, parse_command,
    parse_css as shared_parse_css, report_skipped_selectors,
};

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
    let mut rules: Vec<CssRule> = shared_parse_css(css_content)
        .into_iter()
        .filter_map(|selector| convert_selector_string_to_rule(&selector))
        .collect();
    rules.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));
    rules.dedup();
    rules
}

fn partition_rules(rules: Vec<CssRule>) -> (Vec<CssRule>, Vec<String>) {
    let mut considered = Vec::new();
    let mut skipped = Vec::new();

    for rule in rules {
        let printable = rule.to_string();
        if is_simple_selector(&printable) {
            skipped.push(printable);
        } else {
            considered.push(rule);
        }
    }

    (considered, skipped)
}

fn convert_selector_string_to_rule(selector: &str) -> Option<CssRule> {
    let tokens = tokenize_rule(selector);
    let mut parts: Vec<SelectorPart> = Vec::new();
    let mut current_selector: Option<Selector> = None;
    let mut pending_combinator = Combinator::None;

    for token in tokens {
        match token {
            RuleToken::Selector(text) => {
                let selector = parse_simple_selector(&text)?;
                if let Some(prev) = current_selector.replace(selector) {
                    parts.push(SelectorPart {
                        selector: prev,
                        combinator: pending_combinator,
                    });
                    pending_combinator = Combinator::None;
                }
            }
            RuleToken::Combinator(Combinator::Child) => {
                if current_selector.is_some() {
                    pending_combinator = Combinator::Child;
                }
            }
            RuleToken::Combinator(Combinator::Descendant) => {
                if current_selector.is_some() && !matches!(pending_combinator, Combinator::Child) {
                    pending_combinator = Combinator::Descendant;
                }
            }
            RuleToken::Combinator(Combinator::None) => {}
        }
    }

    if let Some(selector) = current_selector {
        parts.push(SelectorPart {
            selector,
            combinator: Combinator::None,
        });
    }

    if parts.is_empty() {
        None
    } else {
        Some(CssRule::Complex { parts })
    }
}

fn parse_simple_selector(selector_str: &str) -> Option<Selector> {
    let trimmed = selector_str.trim();
    if trimmed.is_empty() {
        return None;
    }
    if trimmed.starts_with('.') {
        Some(Selector::Class(trimmed[1..].to_string()))
    } else if trimmed.starts_with('#') {
        Some(Selector::Id(trimmed[1..].to_string()))
    } else if trimmed.starts_with('[') {
        parse_attribute_selector(trimmed)
    } else if trimmed == "*" {
        Some(Selector::Type("*".to_string()))
    } else {
        Some(Selector::Type(trimmed.to_lowercase()))
    }
}

fn parse_attribute_selector(raw: &str) -> Option<Selector> {
    let raw = raw.trim();
    if !raw.starts_with('[') || !raw.ends_with(']') {
        return None;
    }
    let inner = &raw[1..raw.len() - 1];
    let mut parts = inner.splitn(2, '=');
    let name = parts.next()?.trim().to_lowercase();
    let value_part = parts.next()?.trim();

    if !value_part.starts_with('"') || !value_part.ends_with('"') || value_part.len() < 2 {
        return None;
    }
    let mut value = value_part[1..value_part.len() - 1].to_string();
    value = value.replace("\\\"", "\"");

    Some(Selector::AttributeEquals { name, value })
}

#[derive(Debug)]
enum RuleToken {
    Selector(String),
    Combinator(Combinator),
}

fn tokenize_rule(selector: &str) -> Vec<RuleToken> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_brackets = false;
    let mut quote_char: Option<char> = None;
    let mut pending_descendant = false;

    let push_selector = |buf: &mut String, tokens: &mut Vec<RuleToken>| {
        if !buf.trim().is_empty() {
            tokens.push(RuleToken::Selector(buf.trim().to_string()));
        }
        buf.clear();
    };

    for ch in selector.chars() {
        match ch {
            '"' | '\'' => {
                if quote_char == Some(ch) {
                    quote_char = None;
                } else if quote_char.is_none() {
                    quote_char = Some(ch);
                }
                current.push(ch);
            }
            '[' if quote_char.is_none() => {
                in_brackets = true;
                current.push(ch);
            }
            ']' if quote_char.is_none() => {
                in_brackets = false;
                current.push(ch);
            }
            '>' if quote_char.is_none() && !in_brackets => {
                push_selector(&mut current, &mut tokens);
                pending_descendant = false;
                if matches!(tokens.last(), Some(RuleToken::Selector(_))) {
                    tokens.push(RuleToken::Combinator(Combinator::Child));
                }
            }
            c if c.is_whitespace() && quote_char.is_none() && !in_brackets => {
                if !current.is_empty() {
                    push_selector(&mut current, &mut tokens);
                }
                pending_descendant =
                    matches!(tokens.last(), Some(RuleToken::Selector(_))) || pending_descendant;
            }
            _ => {
                if pending_descendant {
                    if matches!(tokens.last(), Some(RuleToken::Selector(_))) {
                        tokens.push(RuleToken::Combinator(Combinator::Descendant));
                    }
                    pending_descendant = false;
                }
                current.push(ch);
            }
        }
    }

    if pending_descendant && matches!(tokens.last(), Some(RuleToken::Selector(_))) {
        tokens.push(RuleToken::Combinator(Combinator::Descendant));
    }

    if !current.is_empty() {
        tokens.push(RuleToken::Selector(current.trim().to_string()));
    }

    tokens
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

#[derive(Debug, Default)]
struct SimpleDomNode {
    tag_name: String,
    id: u64,
    html_id: Option<String>,
    attributes: HashMap<String, String>,
    classes: HashSet<String>,
    parent: Option<u64>,
    children: Vec<u64>,
}

impl SimpleDomNode {
    fn from_json(json_node: &serde_json::Value) -> Self {
        let mut node = SimpleDomNode::default();
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
        node.classes = class_attr
            .split_whitespace()
            .map(|x| x.to_string())
            .collect();
        node.attributes = attributes;
        node
    }

    fn set_attribute(&mut self, key: &str, new_value: Option<String>) {
        let key_lower = key.to_lowercase();
        match key_lower.as_str() {
            "class" => {
                self.classes.clear();
                if let Some(ref value) = new_value {
                    for class_name in value.split_whitespace().filter(|name| !name.is_empty()) {
                        self.classes.insert(class_name.to_string());
                    }
                    self.attributes.insert(key_lower, value.clone());
                } else {
                    self.attributes.remove("class");
                }
            }
            "id" => {
                if let Some(ref value) = new_value {
                    self.html_id = Some(value.clone());
                    self.attributes.insert(key_lower, value.clone());
                } else {
                    self.html_id = None;
                    self.attributes.remove("id");
                }
            }
            _ => {
                if let Some(value) = new_value {
                    self.attributes.insert(key_lower, value);
                } else {
                    self.attributes.remove(&key_lower);
                }
            }
        }
    }
}

#[derive(Debug, Default)]
struct SimpleDom {
    nodes: HashMap<u64, SimpleDomNode>,
    root_id: Option<u64>,
}

impl SimpleDom {
    fn init(&mut self, root: &serde_json::Value) {
        self.nodes.clear();
        self.root_id = Some(self.build_subtree(root, None));
    }

    fn build_subtree(&mut self, node_json: &serde_json::Value, parent: Option<u64>) -> u64 {
        let node_id = node_json["id"].as_u64().unwrap();
        let mut node = SimpleDomNode::from_json(node_json);
        node.parent = parent;
        self.nodes.insert(node_id, node);
        if let Some(children) = node_json["children"].as_array() {
            for child in children {
                let child_id = self.build_subtree(child, Some(node_id));
                if let Some(parent_node) = self.nodes.get_mut(&node_id) {
                    parent_node.children.push(child_id);
                }
            }
        }
        node_id
    }

    fn node_id_by_path(&self, path: &[usize]) -> Option<u64> {
        let mut current = self.root_id?;
        if path.is_empty() {
            return Some(current);
        }
        for &segment in path {
            let node = self.nodes.get(&current)?;
            current = *node.children.get(segment)?;
        }
        Some(current)
    }

    fn add_by_path(&mut self, path: &[usize], json_node: &serde_json::Value) {
        if path.is_empty() {
            return;
        }
        let insert_pos = *path.last().unwrap();
        let parent_path = &path[..path.len() - 1];
        let parent_id = self
            .node_id_by_path(parent_path)
            .unwrap_or_else(|| panic!("invalid parent path {:?} for add", parent_path));
        let new_root_id = self.build_subtree(json_node, Some(parent_id));
        if let Some(parent_node) = self.nodes.get_mut(&parent_id) {
            if insert_pos <= parent_node.children.len() {
                parent_node.children.insert(insert_pos, new_root_id);
            } else {
                parent_node.children.push(new_root_id);
            }
        }
    }

    fn remove_by_path(&mut self, path: &[usize]) {
        if path.is_empty() {
            return;
        }
        let parent_path = &path[..path.len() - 1];
        let child_idx = *path.last().unwrap();
        if let Some(parent_id) = self.node_id_by_path(parent_path) {
            if let Some(parent_node) = self.nodes.get_mut(&parent_id) {
                if child_idx < parent_node.children.len() {
                    let child_id = parent_node.children.remove(child_idx);
                    self.nodes.remove(&child_id);
                }
            }
        }
    }

    fn set_attribute(&mut self, path: &[usize], key: &str, new_value: Option<String>) {
        if let Some(node_id) = self.node_id_by_path(path) {
            if let Some(node) = self.nodes.get_mut(&node_id) {
                node.set_attribute(key, new_value);
            }
        }
    }

    fn assert_attribute_value(&self, path: &[usize], key: &str, expected: &str) {
        if let Some(node_id) = self.node_id_by_path(path) {
            if let Some(node) = self.nodes.get(&node_id) {
                let actual = node
                    .attributes
                    .get(&key.to_lowercase())
                    .cloned()
                    .unwrap_or_default();
                assert_eq!(
                    actual, expected,
                    "existing attribute value mismatch for key {} at path {:?}",
                    key, path
                );
            }
        }
    }

    fn matches_simple_selector(&self, node_id: u64, selector: &Selector) -> bool {
        let Some(node) = self.nodes.get(&node_id) else {
            return false;
        };
        match selector {
            Selector::Type(tag) => {
                if tag == "*" {
                    true
                } else {
                    node.tag_name.to_lowercase() == tag.to_lowercase()
                }
            }
            Selector::Class(class) => node.classes.contains(class),
            Selector::Id(id) => node.html_id.as_deref() == Some(id.as_str()),
            Selector::AttributeEquals { name, value } => node
                .attributes
                .get(name)
                .map(|v| v == value)
                .unwrap_or(false),
        }
    }

    fn matches_complex_selector(&self, node_id: u64, parts: &[SelectorPart]) -> bool {
        if parts.is_empty() {
            return true;
        }

        let last_part = &parts[parts.len() - 1];
        if !self.matches_simple_selector(node_id, &last_part.selector) {
            return false;
        }

        if parts.len() == 1 {
            return true;
        }

        let combinator = if parts.len() >= 2 {
            &parts[parts.len() - 2].combinator
        } else {
            &Combinator::None
        };

        let parent_id = self.nodes.get(&node_id).and_then(|node| node.parent);
        match combinator {
            Combinator::None => parent_id
                .map(|pid| self.matches_complex_selector_recursive(pid, &parts[..parts.len() - 1]))
                .unwrap_or(false),
            Combinator::Child => parent_id
                .map(|pid| self.matches_complex_selector(pid, &parts[..parts.len() - 1]))
                .unwrap_or(false),
            Combinator::Descendant => parent_id
                .map(|pid| self.matches_complex_selector_recursive(pid, &parts[..parts.len() - 1]))
                .unwrap_or(false),
        }
    }

    fn matches_complex_selector_recursive(&self, node_id: u64, parts: &[SelectorPart]) -> bool {
        if self.matches_complex_selector(node_id, parts) {
            return true;
        }
        if let Some(parent_id) = self.nodes.get(&node_id).and_then(|node| node.parent) {
            self.matches_complex_selector_recursive(parent_id, parts)
        } else {
            false
        }
    }

    fn matches_css_rule(&self, node_id: u64, rule: &CssRule) -> bool {
        match rule {
            CssRule::Complex { parts } => self.matches_complex_selector(node_id, parts),
        }
    }

    fn collect_rule_matches(&self, rule: &CssRule) -> Vec<u64> {
        self.nodes
            .keys()
            .copied()
            .filter(|node_id| self.matches_css_rule(*node_id, rule))
            .collect()
    }

    fn print_css_matches(&self, rules: &mut [CssRule]) {
        rules.sort_by_key(|x| format!("{x:?}"));
        for rule in rules.iter() {
            let mut matches = self.collect_rule_matches(rule);
            if matches.is_empty() {
                continue;
            }
            matches.sort_unstable();
            matches.dedup();
            println!("{} -> {:?}", rule, matches);
        }
    }
}

fn apply_frame(dom: &mut SimpleDom, frame: &LayoutFrame) {
    match parse_command(&frame.command_name, &frame.command_data) {
        Command::Init { node } => {
            dom.init(node);
        }
        Command::Add { path, node } => {
            dom.add_by_path(&path, node);
        }
        Command::ReplaceValue {
            path,
            key,
            value,
            old_value,
        } => {
            if let Some(old_value) = old_value {
                dom.assert_attribute_value(&path, key, &json_value_to_attr_string(old_value));
            }
            let new_value = value.map(json_value_to_attr_string);
            dom.set_attribute(&path, key, new_value);
        }
        Command::InsertValue { path, key, value } => {
            let new_value = value.map(json_value_to_attr_string);
            dom.set_attribute(&path, key, new_value);
        }
        Command::DeleteValue {
            path,
            key,
            old_value,
        } => {
            if let Some(old_value) = old_value {
                dom.assert_attribute_value(&path, key, &json_value_to_attr_string(old_value));
            }
            dom.set_attribute(&path, key, None);
        }
        Command::Recalculate => {}
        Command::Remove { path } => {
            dom.remove_by_path(&path);
        }
    }
}

fn main() {
    let mut dom = SimpleDom::default();
    let (mut css, skipped_simple) = partition_rules(parse_css(
        &std::fs::read_to_string(format!(
            "css-gen-op/{0}/{0}.css",
            std::env::var("WEBSITE_NAME").unwrap(),
        ))
        .unwrap(),
    ));
    report_skipped_selectors("naive", &skipped_simple);
    let trace = parse_trace();

    for frame in &trace {
        apply_frame(&mut dom, frame);
    }
    println!("BEGIN");
    dom.print_css_matches(&mut css);
    println!("END");
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
        let mut dom = SimpleDom::default();
        let mut node = SimpleDomNode::default();
        node.id = 1;
        node.attributes.insert("data-id".into(), "item-1".into());
        dom.nodes.insert(1, node);
        dom.root_id = Some(1);

        let selector = Selector::AttributeEquals {
            name: "data-id".into(),
            value: "item-1".into(),
        };
        assert!(dom.matches_simple_selector(1, &selector));

        let mismatch = Selector::AttributeEquals {
            name: "data-id".into(),
            value: "item-2".into(),
        };
        assert!(!dom.matches_simple_selector(1, &mismatch));
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
