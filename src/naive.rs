use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fmt::{Debug, Display},
};

use css_bitvector_compiler::{
    CompoundSelector, PSEUDO_CLASS_FOCUS, PSEUDO_CLASS_FOCUS_ROOT, PSEUDO_CLASS_FOCUS_WITHIN,
    PSEUDO_CLASS_HOVER, PSEUDO_CLASS_HOVER_ROOT, ParsedSelectors, Selector, basic_node_from_json,
    derive_hover_state, drain_supported_pseudo_selectors, is_simple_selector,
    parse_css_with_pseudo, parse_selector, parse_trace, report_pseudo_selectors,
    report_skipped_selectors, report_unsupported_selectors,
    runtime_shared::{BasicDomOps, apply_frame_basic},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum CssRule {
    Complex {
        parts: Vec<SelectorPart>,
        source: String,
    },
}

impl Display for CssRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CssRule::Complex { source, .. } => write!(f, "{}", source),
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
            Combinator::AdjacentSibling => write!(f, " + "),
            Combinator::None => Ok(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Combinator {
    Descendant,      // Space combinator
    Child,           // >
    AdjacentSibling, // +
    None,            // The last selector has no combinator
}

impl Display for Combinator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Combinator::Descendant => write!(f, " "),
            Combinator::Child => write!(f, ">"),
            Combinator::AdjacentSibling => write!(f, "+"),
            Combinator::None => write!(f, ""),
        }
    }
}

fn parse_css_rules(
    css_content: &str,
) -> (Vec<CssRule>, BTreeMap<String, Vec<String>>, Vec<String>) {
    let ParsedSelectors {
        mut selectors,
        mut pseudo_selectors,
        unsupported_selectors,
    } = parse_css_with_pseudo(css_content);

    selectors.extend(drain_supported_pseudo_selectors(&mut pseudo_selectors));
    selectors.extend(drain_naive_specific_pseudos(&mut pseudo_selectors));
    selectors.sort();
    selectors.dedup();

    let mut rules: Vec<CssRule> = selectors
        .into_iter()
        .filter_map(|selector| convert_selector_string_to_rule(&selector))
        .collect();

    rules.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));
    rules.dedup();
    (rules, pseudo_selectors, unsupported_selectors)
}

const NAIVE_PSEUDO_KEYS: &[&str] = &[
    ":first-child",
    ":last-child",
    ":only-child",
    ":first-of-type",
    ":last-of-type",
    ":only-of-type",
    ":nth-child",
    ":nth-last-child",
    ":nth-of-type",
    ":nth-last-of-type",
];

const STATEFUL_PSEUDO_CLASSES: &[&str] = &[
    PSEUDO_CLASS_HOVER,
    PSEUDO_CLASS_FOCUS,
    PSEUDO_CLASS_FOCUS_WITHIN,
];

fn drain_naive_specific_pseudos(
    pseudo_selectors: &mut BTreeMap<String, Vec<String>>,
) -> Vec<String> {
    let mut collected = Vec::new();
    for key in NAIVE_PSEUDO_KEYS {
        if let Some(mut selectors) = pseudo_selectors.remove(*key) {
            collected.append(&mut selectors);
        }
    }
    collected
}

fn is_stateful_pseudo(pseudo: &str) -> bool {
    STATEFUL_PSEUDO_CLASSES
        .iter()
        .any(|stateful| *stateful == pseudo)
}

fn convert_selector_string_to_rule(selector: &str) -> Option<CssRule> {
    let tokens = tokenize_rule(selector);
    let mut parts: Vec<SelectorPart> = Vec::new();
    let mut current_selector: Option<Selector> = None;
    let mut pending_combinator = Combinator::None;

    for token in tokens {
        match token {
            RuleToken::Selector(text) => {
                if text.trim().is_empty() {
                    continue;
                }
                let selector = parse_selector(&text);
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
            RuleToken::Combinator(Combinator::AdjacentSibling) => {
                if current_selector.is_some() {
                    pending_combinator = Combinator::AdjacentSibling;
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
        Some(CssRule::Complex {
            parts,
            source: selector.trim().to_string(),
        })
    }
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
            '+' if quote_char.is_none() && !in_brackets => {
                push_selector(&mut current, &mut tokens);
                pending_descendant = false;
                if matches!(tokens.last(), Some(RuleToken::Selector(_))) {
                    tokens.push(RuleToken::Combinator(Combinator::AdjacentSibling));
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

#[derive(Debug, Default)]
struct SimpleDomNode {
    pub tag_name: String,
    pub id: u64,
    pub html_id: Option<String>,
    pub attributes: HashMap<String, String>,
    pub classes: HashSet<String>,
    pub pseudo_classes: HashSet<String>,
    pub computed_pseudo_classes: HashSet<String>,
    pub parent: Option<u64>,
    pub children: Vec<u64>,
}

impl SimpleDomNode {
    fn from_json(json_node: &serde_json::Value) -> Self {
        let basic = basic_node_from_json(json_node);
        SimpleDomNode {
            tag_name: basic.tag_name,
            id: basic.id,
            html_id: basic.html_id,
            attributes: basic.attributes,
            classes: basic.classes,
            pseudo_classes: basic.pseudo_classes,
            ..Default::default()
        }
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
            "is_hovered_root" | "is_focus_root" => {
                let pseudo = if key_lower == "is_hovered_root" {
                    PSEUDO_CLASS_HOVER_ROOT
                } else {
                    PSEUDO_CLASS_FOCUS_ROOT
                };
                let should_set = new_value
                    .as_deref()
                    .map(|value| value.eq_ignore_ascii_case("true"))
                    .unwrap_or(false);
                if should_set {
                    self.pseudo_classes.insert(pseudo.to_string());
                } else {
                    self.pseudo_classes.remove(pseudo);
                }
                if let Some(value) = new_value.clone() {
                    self.attributes.insert(key_lower, value);
                } else {
                    self.attributes.remove(key_lower.as_str());
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

    fn remove_subtree(&mut self, node_id: u64) {
        if let Some(node) = self.nodes.remove(&node_id) {
            for child_id in node.children {
                self.remove_subtree(child_id);
            }
        }
    }

    fn remove_by_path(&mut self, path: &[usize]) {
        if path.is_empty() {
            self.nodes.clear();
            self.root_id = None;
            return;
        }
        let parent_path = &path[..path.len() - 1];
        let child_idx = *path.last().unwrap();
        if let Some(parent_id) = self.node_id_by_path(parent_path) {
            let child_id = {
                let parent_node_opt = self.nodes.get_mut(&parent_id);
                if let Some(parent_node) = parent_node_opt {
                    if child_idx < parent_node.children.len() {
                        Some(parent_node.children.remove(child_idx))
                    } else {
                        None
                    }
                } else {
                    None
                }
            };
            if let Some(child_id) = child_id {
                let should_remove = self
                    .nodes
                    .get(&child_id)
                    .map(|node| node.parent == Some(parent_id))
                    .unwrap_or(true);
                if should_remove {
                    self.remove_subtree(child_id);
                }
            }
        }
    }

    fn set_attribute(&mut self, path: &[usize], key: &str, new_value: Option<String>) {
        if let Some(node_id) = self.node_id_by_path(path)
            && let Some(node) = self.nodes.get_mut(&node_id)
        {
            node.set_attribute(key, new_value);
        }
    }

    fn assert_attribute_value(&self, path: &[usize], key: &str, expected: &str) {
        if let Some(node_id) = self.node_id_by_path(path)
            && let Some(node) = self.nodes.get(&node_id)
        {
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

    fn recompute_pseudo_states(&mut self) {
        if let Some(root_id) = self.root_id {
            self.refresh_pseudo_recursive(root_id, false);
        }
    }

    fn refresh_pseudo_recursive(&mut self, node_id: u64, parent_hover: bool) -> bool {
        let mut hover_active = parent_hover;
        let mut focus_root_active = false;
        if let Some(node) = self.nodes.get_mut(&node_id) {
            hover_active = derive_hover_state(&node.pseudo_classes, parent_hover);
            if hover_active {
                node.computed_pseudo_classes
                    .insert(PSEUDO_CLASS_HOVER.to_string());
            } else {
                node.computed_pseudo_classes.remove(PSEUDO_CLASS_HOVER);
            }

            focus_root_active = node.pseudo_classes.contains(PSEUDO_CLASS_FOCUS_ROOT)
                || node.pseudo_classes.contains(PSEUDO_CLASS_FOCUS);
        }
        let mut focus_within_active = focus_root_active;
        if let Some(children) = self.nodes.get(&node_id).map(|n| n.children.clone()) {
            for child_id in children {
                if self.refresh_pseudo_recursive(child_id, hover_active) {
                    focus_within_active = true;
                }
            }
        }
        if let Some(node) = self.nodes.get_mut(&node_id) {
            if focus_root_active {
                node.computed_pseudo_classes
                    .insert(PSEUDO_CLASS_FOCUS.to_string());
            } else {
                node.computed_pseudo_classes.remove(PSEUDO_CLASS_FOCUS);
            }
            if focus_within_active {
                node.computed_pseudo_classes
                    .insert(PSEUDO_CLASS_FOCUS_WITHIN.to_string());
            } else {
                node.computed_pseudo_classes
                    .remove(PSEUDO_CLASS_FOCUS_WITHIN);
            }
        }
        focus_within_active
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
                    node.tag_name.eq_ignore_ascii_case(tag)
                }
            }
            Selector::Class(class) => node.classes.contains(class),
            Selector::Id(id) => node.html_id.as_deref() == Some(id.as_str()),
            Selector::AttributeEquals { name, value } => node
                .attributes
                .get(name)
                .map(|v| v == value)
                .unwrap_or(false),
            Selector::Compound(compound) => self.matches_compound_selector(node_id, node, compound),
        }
    }

    fn matches_compound_selector(
        &self,
        node_id: u64,
        node: &SimpleDomNode,
        compound: &CompoundSelector,
    ) -> bool {
        if let Some(tag) = &compound.tag
            && tag != "*"
            && !node.tag_name.eq_ignore_ascii_case(tag)
        {
            return false;
        }
        if let Some(id_value) = &compound.id
            && node.html_id.as_deref() != Some(id_value.as_str())
        {
            return false;
        }
        for class_name in &compound.classes {
            if !node.classes.contains(class_name) {
                return false;
            }
        }
        for (name, value) in &compound.attributes {
            if node
                .attributes
                .get(name)
                .map(|v| v == value)
                .unwrap_or(false)
            {
                continue;
            }
            return false;
        }
        for pseudo in &compound.pseudos {
            if is_stateful_pseudo(pseudo) {
                if !node.computed_pseudo_classes.contains(pseudo) {
                    return false;
                }
            } else if !self.matches_structural_pseudo(node_id, node, pseudo) {
                return false;
            }
        }
        true
    }

    fn matches_structural_pseudo(&self, node_id: u64, node: &SimpleDomNode, pseudo: &str) -> bool {
        match pseudo {
            "first-child" => self
                .sibling_index(node_id, |_| true)
                .map(|(index, _)| index == 1)
                .unwrap_or(false),
            "last-child" => self
                .sibling_index(node_id, |_| true)
                .map(|(index, total)| index == total)
                .unwrap_or(false),
            "only-child" => self
                .sibling_index(node_id, |_| true)
                .map(|(_, total)| total == 1)
                .unwrap_or(false),
            "first-of-type" => {
                let target = node.tag_name.to_ascii_lowercase();
                self.sibling_index(node_id, |child| {
                    child.tag_name.eq_ignore_ascii_case(&target)
                })
                .map(|(index, _)| index == 1)
                .unwrap_or(false)
            }
            "last-of-type" => {
                let target = node.tag_name.to_ascii_lowercase();
                self.sibling_index(node_id, |child| {
                    child.tag_name.eq_ignore_ascii_case(&target)
                })
                .map(|(index, total)| index == total)
                .unwrap_or(false)
            }
            "only-of-type" => {
                let target = node.tag_name.to_ascii_lowercase();
                self.sibling_index(node_id, |child| {
                    child.tag_name.eq_ignore_ascii_case(&target)
                })
                .map(|(_, total)| total == 1)
                .unwrap_or(false)
            }
            _ => {
                if let Some(arg) = nth_argument(pseudo, "nth-child(") {
                    self.matches_nth_with_predicate(node_id, |_| true, arg, false)
                } else if let Some(arg) = nth_argument(pseudo, "nth-last-child(") {
                    self.matches_nth_with_predicate(node_id, |_| true, arg, true)
                } else if let Some(arg) = nth_argument(pseudo, "nth-of-type(") {
                    let target = node.tag_name.to_ascii_lowercase();
                    self.matches_nth_with_predicate(
                        node_id,
                        |child| child.tag_name.eq_ignore_ascii_case(&target),
                        arg,
                        false,
                    )
                } else if let Some(arg) = nth_argument(pseudo, "nth-last-of-type(") {
                    let target = node.tag_name.to_ascii_lowercase();
                    self.matches_nth_with_predicate(
                        node_id,
                        |child| child.tag_name.eq_ignore_ascii_case(&target),
                        arg,
                        true,
                    )
                } else {
                    false
                }
            }
        }
    }

    fn matches_nth_with_predicate<F>(
        &self,
        node_id: u64,
        predicate: F,
        argument: &str,
        from_end: bool,
    ) -> bool
    where
        F: Fn(&SimpleDomNode) -> bool,
    {
        let Some(expr) = parse_nth_expression(argument) else {
            return false;
        };
        let Some((index, total)) = self.sibling_index(node_id, predicate) else {
            return false;
        };
        let position = if from_end { total - index + 1 } else { index };
        matches_nth_value(position, expr)
    }

    fn sibling_index<F>(&self, node_id: u64, predicate: F) -> Option<(usize, usize)>
    where
        F: Fn(&SimpleDomNode) -> bool,
    {
        let node = self.nodes.get(&node_id)?;
        let parent_id = node.parent?;
        let parent = self.nodes.get(&parent_id)?;
        let mut current_index = 0usize;
        let mut total = 0usize;
        for &child_id in &parent.children {
            let Some(child_node) = self.nodes.get(&child_id) else {
                continue;
            };
            if predicate(child_node) {
                total += 1;
                if child_id == node_id {
                    current_index = total;
                }
            }
        }
        if current_index == 0 {
            None
        } else {
            Some((current_index, total))
        }
    }

    fn previous_sibling_id(&self, node_id: u64) -> Option<u64> {
        let node = self.nodes.get(&node_id)?;
        let parent_id = node.parent?;
        let parent = self.nodes.get(&parent_id)?;
        let mut prev = None;
        for &child_id in &parent.children {
            if child_id == node_id {
                return prev;
            }
            prev = Some(child_id);
        }
        None
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
            Combinator::AdjacentSibling => self
                .previous_sibling_id(node_id)
                .map(|sid| self.matches_complex_selector(sid, &parts[..parts.len() - 1]))
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
            CssRule::Complex { parts, .. } => self.matches_complex_selector(node_id, parts),
        }
    }

    fn collect_rule_matches(&self, rule: &CssRule) -> Vec<u64> {
        self.nodes
            .keys()
            .copied()
            .filter(|node_id| self.matches_css_rule(*node_id, rule))
            .collect()
    }

    fn print_css_matches(&mut self, rules: &mut [CssRule]) {
        self.recompute_pseudo_states();
        rules.sort_by_key(|x| format!("{x:?}"));
        for rule in rules.iter() {
            let mut matches = self.collect_rule_matches(rule);
            if matches.is_empty() {
                continue;
            }
            matches.sort_unstable();
            matches.dedup();
            let printable = rule.to_string().replace('>', " > ");
            println!("{} -> {:?}", printable, matches);
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct NthExpression {
    step: i64,
    offset: i64,
}

fn nth_argument<'a>(pseudo: &'a str, prefix: &str) -> Option<&'a str> {
    pseudo
        .strip_prefix(prefix)
        .and_then(|rest| rest.strip_suffix(')'))
        .map(|arg| arg.trim())
        .filter(|arg| !arg.is_empty())
}

fn parse_nth_expression(input: &str) -> Option<NthExpression> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }
    let lowered = trimmed.to_ascii_lowercase();
    if lowered == "odd" {
        return Some(NthExpression { step: 2, offset: 1 });
    }
    if lowered == "even" {
        return Some(NthExpression { step: 2, offset: 0 });
    }
    let normalized: String = lowered.chars().filter(|c| !c.is_whitespace()).collect();
    if normalized.is_empty() {
        return None;
    }
    if let Some(n_pos) = normalized.find('n') {
        let (a_part, b_part) = normalized.split_at(n_pos);
        let step = if a_part.is_empty() || a_part == "+" {
            1
        } else if a_part == "-" {
            -1
        } else {
            a_part.parse().ok()?
        };
        let offset = if b_part.len() == 1 {
            0
        } else {
            b_part[1..].parse().ok()?
        };
        Some(NthExpression { step, offset })
    } else {
        Some(NthExpression {
            step: 0,
            offset: normalized.parse().ok()?,
        })
    }
}

fn matches_nth_value(position: usize, expr: NthExpression) -> bool {
    let index = position as i64;
    if index < 1 {
        return false;
    }
    let step = expr.step;
    let offset = expr.offset;
    if step == 0 {
        return index == offset && offset >= 1;
    }
    if step > 0 {
        if index < offset {
            return false;
        }
        let diff = index - offset;
        diff % step == 0
    } else {
        let mut value = offset;
        while value >= 1 {
            if value == index {
                return true;
            }
            value += step;
        }
        false
    }
}

impl BasicDomOps for SimpleDom {
    fn init(&mut self, root: &serde_json::Value) {
        SimpleDom::init(self, root);
    }
    fn add_by_path(&mut self, path: &[usize], node: &serde_json::Value) {
        self.add_by_path(path, node);
    }
    fn set_attribute(&mut self, path: &[usize], key: &str, new_value: Option<String>) {
        self.set_attribute(path, key, new_value);
    }
    fn assert_attribute_value(&self, path: &[usize], key: &str, expected: &str) {
        self.assert_attribute_value(path, key, expected);
    }
    fn remove_by_path(&mut self, path: &[usize]) {
        self.remove_by_path(path);
    }
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

fn main() {
    let mut dom = SimpleDom::default();
    let (rules, pseudo_selectors, unsupported_selectors) = parse_css_rules(
        &std::fs::read_to_string(format!(
            "css-gen-op/{0}/{0}.css",
            std::env::var("WEBSITE_NAME").unwrap(),
        ))
        .unwrap(),
    );
    let (mut css, skipped_simple) = partition_rules(rules);
    report_skipped_selectors("naive", &skipped_simple);
    report_pseudo_selectors("naive", &pseudo_selectors);
    report_unsupported_selectors("naive", &unsupported_selectors);
    let trace = parse_trace();

    for frame in &trace {
        apply_frame_basic(&mut dom, frame);
    }
    println!("BEGIN");
    dom.print_css_matches(&mut css);
    println!("END");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        let s = "div h1 > h2 p .a > .b #c";
        dbg!(parse_css_rules(s).0);
    }

    #[test]
    fn parse_attribute_selector() {
        let (rules, _, _) = parse_css_rules(r#"[data-role="hero"] { color: red; }"#);
        assert_eq!(rules.len(), 1);
        match &rules[0] {
            CssRule::Complex { parts, .. } => {
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
    fn parse_css_handles_pseudo_classes() {
        let (rules, pseudo, _) =
            parse_css_rules(".wrapper .item:hover strong { font-weight: bold; }");
        assert!(pseudo.get(":hover").is_none());
        assert_eq!(rules.len(), 1);
        match &rules[0] {
            CssRule::Complex { parts, .. } => {
                assert_eq!(parts.len(), 3);
                match &parts[1].selector {
                    Selector::Compound(comp) => {
                        assert!(comp.classes.contains("item"));
                        assert!(comp.pseudos.contains("hover"));
                    }
                    other => panic!("expected compound selector, got {:?}", other),
                }
            }
        }
    }

    #[test]
    fn parse_css_keeps_descendant_hover_selector() {
        let (rules, pseudo, _) = parse_css_rules(".wrapper :hover { color: blue; }");
        assert!(pseudo.get(":hover").is_none());
        assert_eq!(rules.len(), 1, "hover selector should be preserved");
        match &rules[0] {
            CssRule::Complex { parts, .. } => {
                assert_eq!(parts.len(), 2);
                match &parts[1].selector {
                    Selector::Compound(comp) => {
                        assert!(comp.pseudos.contains("hover"));
                    }
                    other => panic!("expected compound selector, got {:?}", other),
                }
            }
        }
    }

    #[test]
    fn matches_adjacent_sibling_and_first_child() {
        let (mut rules, _, _) = parse_css_rules(".lead:first-child + .target { color: red; }");
        assert_eq!(rules.len(), 1);
        let rule = rules.pop().unwrap();

        let mut dom = SimpleDom::default();
        let mut root = SimpleDomNode::default();
        root.id = 1;
        root.tag_name = "div".into();
        root.children = vec![2, 3];
        dom.root_id = Some(1);

        let mut first = SimpleDomNode::default();
        first.id = 2;
        first.tag_name = "div".into();
        first.parent = Some(1);
        first.classes.insert("lead".into());

        let mut second = SimpleDomNode::default();
        second.id = 3;
        second.tag_name = "div".into();
        second.parent = Some(1);
        second.classes.insert("target".into());

        dom.nodes.insert(1, root);
        dom.nodes.insert(2, first);
        dom.nodes.insert(3, second);

        let matches = dom.collect_rule_matches(&rule);
        assert_eq!(matches, vec![3]);
    }

    #[test]
    fn supports_nth_child_and_of_type() {
        let (mut nth_child_rules, _, _) = parse_css_rules("span:nth-child(2) { color: red; }");
        let nth_child_rule = nth_child_rules.pop().unwrap();

        let (mut nth_of_type_rules, _, _) = parse_css_rules("div:nth-of-type(2) { color: blue; }");
        let nth_of_type_rule = nth_of_type_rules.pop().unwrap();

        let mut dom = SimpleDom::default();
        let mut root = SimpleDomNode::default();
        root.id = 1;
        root.tag_name = "ul".into();
        root.children = vec![2, 3, 4];
        dom.root_id = Some(1);

        let mut first = SimpleDomNode::default();
        first.id = 2;
        first.tag_name = "div".into();
        first.parent = Some(1);

        let mut second = SimpleDomNode::default();
        second.id = 3;
        second.tag_name = "span".into();
        second.parent = Some(1);

        let mut third = SimpleDomNode::default();
        third.id = 4;
        third.tag_name = "div".into();
        third.parent = Some(1);

        dom.nodes.insert(1, root);
        dom.nodes.insert(2, first);
        dom.nodes.insert(3, second);
        dom.nodes.insert(4, third);

        assert!(dom.matches_css_rule(3, &nth_child_rule));
        assert!(!dom.matches_css_rule(2, &nth_child_rule));

        assert!(dom.matches_css_rule(4, &nth_of_type_rule));
        assert!(!dom.matches_css_rule(2, &nth_of_type_rule));
    }
}
