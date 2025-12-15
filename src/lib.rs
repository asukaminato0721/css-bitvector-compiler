use lightningcss::{
    rules::CssRule,
    selector::{Combinator as LCombinator, Component as LComponent, Selector as LightningSelector},
    stylesheet::{ParserOptions, PrinterOptions, StyleSheet},
    traits::ToCss,
};
use parcel_selectors::attr::AttrSelectorOperator;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    fmt::Display,
};

pub mod runtime_shared;

// Helpers used by naive implementation
mod naive_util {
    use crate::extract_pseudoclasses;
    use std::collections::{HashMap, HashSet};

    #[derive(Debug)]
    pub struct BasicNode {
        pub tag_name: String,
        pub id: u64,
        pub attributes: HashMap<String, String>,
        pub classes: HashSet<String>,
        pub html_id: Option<String>,
        pub pseudo_classes: HashSet<String>,
    }

    pub fn basic_node_from_json(json_node: &serde_json::Value) -> BasicNode {
        let tag_name = json_node["name"].as_str().unwrap().to_string();
        let id = json_node["id"].as_u64().unwrap();
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

        let html_id = attributes.get("id").cloned();
        let class_attr = attributes.get("class").cloned().unwrap_or_default();
        let classes = class_attr
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect::<HashSet<_>>();
        let pseudo_classes = extract_pseudoclasses(json_node);

        BasicNode {
            tag_name,
            id,
            attributes,
            classes,
            html_id,
            pseudo_classes,
        }
    }
}
pub use naive_util::*;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SelectorPart {
    selector: Selector,
    combinator: Combinator,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Combinator {
    Descendant, // Space combinator
    Child,      // >
    None,       // The last selector has no combinator
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Selector {
    Type(String),
    Class(String),
    Id(String),
    AttributeEquals { name: String, value: String },
    Compound(CompoundSelector),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct CompoundSelector {
    pub tag: Option<String>,
    pub id: Option<String>,
    pub classes: BTreeSet<String>,
    pub attributes: Vec<(String, String)>,
    pub pseudos: BTreeSet<String>,
}

impl CompoundSelector {
    fn is_simple_class_only(&self) -> bool {
        self.id.is_none()
            && self.attributes.is_empty()
            && self.pseudos.is_empty()
            && self.classes.len() == 1
            && self.tag.is_none()
    }

    fn is_simple_tag_only(&self) -> bool {
        self.tag.as_deref().is_some()
            && self.id.is_none()
            && self.classes.is_empty()
            && self.attributes.is_empty()
            && self.pseudos.is_empty()
    }

    fn is_simple_id_only(&self) -> bool {
        self.id.is_some()
            && self.tag.is_none()
            && self.classes.is_empty()
            && self.attributes.is_empty()
            && self.pseudos.is_empty()
    }

    fn is_simple_attr_only(&self) -> bool {
        self.attributes.len() == 1
            && self.tag.is_none()
            && self.id.is_none()
            && self.classes.is_empty()
            && self.pseudos.is_empty()
    }
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
            Selector::Compound(compound) => write!(f, "{}", compound),
        }
    }
}

impl Display for CompoundSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = String::new();
        if let Some(tag) = &self.tag {
            text.push_str(tag);
        }
        if let Some(id) = &self.id {
            text.push('#');
            text.push_str(id);
        }
        for class in &self.classes {
            text.push('.');
            text.push_str(class);
        }
        for (name, value) in &self.attributes {
            text.push_str(&format!("[{}=\"{}\"]", name, value));
        }
        for pseudo in &self.pseudos {
            text.push(':');
            text.push_str(pseudo);
        }
        write!(f, "{}", text)
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

#[derive(Debug, Clone)]
pub enum Command<'a> {
    Init {
        node: &'a serde_json::Value,
    },
    Add {
        path: Vec<usize>,
        node: &'a serde_json::Value,
    },
    ReplaceValue {
        path: Vec<usize>,
        key: &'a str,
        value: Option<&'a serde_json::Value>,
        old_value: Option<&'a serde_json::Value>,
    },
    InsertValue {
        path: Vec<usize>,
        key: &'a str,
        value: Option<&'a serde_json::Value>,
    },
    DeleteValue {
        path: Vec<usize>,
        key: &'a str,
        old_value: Option<&'a serde_json::Value>,
    },
    Recalculate,
    Remove {
        path: Vec<usize>,
    },
}

pub fn parse_command<'a>(
    command_name: &'a str,
    command_data: &'a serde_json::Value,
) -> Command<'a> {
    match command_name {
        "init" => {
            let node = command_data.get("node").unwrap();
            Command::Init { node }
        }
        "add" => {
            let node = command_data.get("node").unwrap();
            let path = extract_path_from_command(command_data);
            Command::Add { path, node }
        }
        "replace_value" => {
            if command_data.get("type").and_then(|v| v.as_str()) != Some("attributes") {
                unreachable!();
            }
            let path = extract_path_from_command(command_data);
            let key = command_data.get("key").and_then(|v| v.as_str()).unwrap();
            let value = command_data.get("value");
            let old_value = command_data.get("old_value");
            Command::ReplaceValue {
                path,
                key,
                value,
                old_value,
            }
        }
        "insert_value" => {
            if command_data.get("type").and_then(|v| v.as_str()) != Some("attributes") {
                unreachable!();
            }
            let path = extract_path_from_command(command_data);
            let key = command_data.get("key").and_then(|v| v.as_str()).unwrap();
            let value = command_data.get("value");
            Command::InsertValue { path, key, value }
        }
        "delete_value" => {
            if command_data.get("type").and_then(|v| v.as_str()) != Some("attributes") {
                unreachable!();
            }
            let path = extract_path_from_command(command_data);
            let key = command_data.get("key").and_then(|v| v.as_str()).unwrap();
            let old_value = command_data.get("old_value");
            Command::DeleteValue {
                path,
                key,
                old_value,
            }
        }
        "recalculate" => Command::Recalculate,
        "remove" => {
            let path = extract_path_from_command(command_data);
            Command::Remove { path }
        }
        _ => unreachable!(),
    }
}

impl LayoutFrame {
    pub fn as_command(&self) -> Command<'_> {
        parse_command(&self.command_name, &self.command_data)
    }
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

#[derive(Debug, Default, Clone)]
pub struct ParsedSelectors {
    pub selectors: Vec<String>,
    pub pseudo_selectors: BTreeMap<String, Vec<String>>,
    pub unsupported_selectors: Vec<String>,
}

pub fn parse_css(css_content: &str) -> Vec<String> {
    parse_css_with_pseudo(css_content).selectors
}

pub fn drain_supported_pseudo_selectors(
    pseudo_selectors: &mut BTreeMap<String, Vec<String>>,
) -> Vec<String> {
    let keys: Vec<String> = pseudo_selectors
        .keys()
        .filter(|name| is_supported_pseudo_class(name))
        .cloned()
        .collect();
    let mut collected = Vec::new();
    for key in keys {
        if let Some(mut selectors) = pseudo_selectors.remove(&key) {
            collected.append(&mut selectors);
        }
    }
    collected
}

pub fn parse_css_with_pseudo(css_content: &str) -> ParsedSelectors {
    let mut parser_options = ParserOptions::default();
    parser_options.error_recovery = true;

    let stylesheet = match StyleSheet::parse(css_content, parser_options) {
        Ok(sheet) => sheet,
        Err(_) => return ParsedSelectors::default(),
    };

    let mut selectors = Vec::new();
    let mut pseudo_selectors: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut unsupported_selectors = Vec::new();

    for rule in stylesheet.rules.0 {
        if let CssRule::Style(style_rule) = rule {
            for selector in style_rule.selectors.0 {
                match lightning_selector_to_rule_string(&selector) {
                    SelectorConversionResult::Keep(s) => selectors.push(s),
                    SelectorConversionResult::RecordPseudo { selector, pseudos } => {
                        for pseudo in pseudos {
                            pseudo_selectors
                                .entry(pseudo)
                                .or_default()
                                .push(selector.clone());
                        }
                    }
                    SelectorConversionResult::Skip => {
                        unsupported_selectors.push(selector_to_string(&selector));
                    }
                }
            }
        }
    }

    selectors.sort();
    selectors.dedup();
    for selectors in pseudo_selectors.values_mut() {
        selectors.sort();
        selectors.dedup();
    }
    unsupported_selectors.sort();
    unsupported_selectors.dedup();

    let mut supported_with_pseudo = Vec::new();
    pseudo_selectors.retain(|pseudo, sels| {
        if is_supported_pseudo_class(pseudo) {
            supported_with_pseudo.extend(sels.iter().cloned());
            false
        } else {
            true
        }
    });
    selectors.extend(supported_with_pseudo);
    selectors.sort();
    selectors.dedup();

    ParsedSelectors {
        selectors,
        pseudo_selectors,
        unsupported_selectors,
    }
}

/// Returns true if the selector is a single "simple" selector without combinators
/// (e.g. `a`, `.button`, `#header`). Those selectors consist only of alphanumeric
/// characters or the symbols `-`, `_`, `.`, and `#`.
pub fn is_simple_selector(selector: &str) -> bool {
    let trimmed = selector.trim();
    if trimmed.is_empty() {
        return false;
    }
    if trimmed.starts_with('@') || trimmed.contains(',') {
        return false;
    }

    trimmed
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | '#'))
}

/// Splits selectors into the ones we keep and the simple ones we skip.
pub fn partition_simple_selectors(selectors: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut considered = Vec::new();
    let mut skipped = Vec::new();

    for selector in selectors {
        if is_simple_selector(&selector) {
            skipped.push(selector);
        } else {
            considered.push(selector);
        }
    }

    (considered, skipped)
}

/// Prints the selectors we currently skip so runs can surface missing coverage.
pub fn report_skipped_selectors(label: &str, selectors: &[String]) {
    if selectors.is_empty() {
        println!("NOT_CONSIDERED[{label}] none");
        return;
    }

    println!("NOT_CONSIDERED[{label}] {} selector(s)", selectors.len());
    for selector in selectors {
        println!("NOT_CONSIDERED[{label}] {selector}");
    }
}

pub fn report_pseudo_selectors(label: &str, selectors: &BTreeMap<String, Vec<String>>) {
    if selectors.is_empty() {
        println!("PSEUDO_SKIPPED[{label}] none");
        return;
    }

    let mut entries: Vec<_> = selectors.iter().collect();
    entries.sort_by(|a, b| b.1.len().cmp(&a.1.len()).then_with(|| a.0.cmp(b.0)));

    for (pseudo, sels) in entries {
        println!(
            "PSEUDO_SKIPPED[{label}] {pseudo} -> {} selector(s)",
            sels.len()
        );
        for example in sels.iter().take(5) {
            println!("PSEUDO_SKIPPED[{label}]    eg {example}");
        }
        if sels.len() > 5 {
            println!("PSEUDO_SKIPPED[{label}]    ...");
        }
    }
}

pub fn report_unsupported_selectors(label: &str, selectors: &[String]) {
    if selectors.is_empty() {
        println!("UNSUPPORTED[{label}] none");
        return;
    }

    println!("UNSUPPORTED[{label}] {} selector(s)", selectors.len());
    for sel in selectors.iter().take(25) {
        println!("UNSUPPORTED[{label}] {sel}");
    }
    if selectors.len() > 25 {
        println!("UNSUPPORTED[{label}] ...");
    }
}

fn is_supported_pseudo_class(name: &str) -> bool {
    matches!(
        normalize_pseudo_name(name),
        "hover" | "focus" | "focus-within"
    )
}

fn normalize_pseudo_name(name: &str) -> &str {
    name.trim_start_matches(':')
}

enum ComponentConversion {
    Keep(Selector),
    Skip,
    Abort,
}

enum SelectorConversionResult {
    Keep(String),
    RecordPseudo {
        selector: String,
        pseudos: Vec<String>,
    },
    Skip,
}

fn selector_to_string(selector: &LightningSelector) -> String {
    selector
        .to_css_string(PrinterOptions::default())
        .unwrap_or_else(|_| format!("{:?}", selector))
}

fn record_pseudo_selector(selector: &LightningSelector) -> SelectorConversionResult {
    let selector_string = selector_to_string(selector);
    let mut pseudos = extract_pseudo_tokens(&selector_string);
    if pseudos.is_empty() {
        pseudos.push("<pseudo>".to_string());
    }
    SelectorConversionResult::RecordPseudo {
        selector: selector_string,
        pseudos,
    }
}

fn lightning_selector_to_rule_string(selector: &LightningSelector) -> SelectorConversionResult {
    let mut selector_parts: Vec<SelectorPart> = Vec::new();
    let mut current_selector: Option<Selector> = None;
    let mut pending_combinator = Combinator::None;

    for component in selector.iter_raw_parse_order_from(0) {
        match component {
            LComponent::Combinator(combinator) => match combinator {
                LCombinator::Descendant => {
                    if current_selector.is_some() && matches!(pending_combinator, Combinator::None)
                    {
                        pending_combinator = Combinator::Descendant;
                    }
                }
                LCombinator::Child => {
                    if current_selector.is_some() {
                        pending_combinator = Combinator::Child;
                    }
                }
                LCombinator::PseudoElement => {
                    return record_pseudo_selector(selector);
                }
                _ => break,
            },
            LComponent::Negation(_)
            | LComponent::Root
            | LComponent::Empty
            | LComponent::Scope
            | LComponent::Nth(_)
            | LComponent::NthOf(_)
            | LComponent::NonTSPseudoClass(_)
            | LComponent::Slotted(_)
            | LComponent::Part(_)
            | LComponent::Host(_)
            | LComponent::Where(_)
            | LComponent::Is(_)
            | LComponent::Any(_, _)
            | LComponent::Has(_)
            | LComponent::PseudoElement(_) => {
                return record_pseudo_selector(selector);
            }
            _ => match convert_component(component) {
                ComponentConversion::Keep(selector) => {
                    if let Some(prev_selector) = current_selector.take() {
                        selector_parts.push(SelectorPart {
                            selector: prev_selector,
                            combinator: pending_combinator.clone(),
                        });
                        pending_combinator = Combinator::None;
                    }
                    current_selector = Some(selector);
                }
                ComponentConversion::Skip => {}
                ComponentConversion::Abort => {
                    return SelectorConversionResult::Skip;
                }
            },
        }
    }

    if let Some(selector) = current_selector {
        selector_parts.push(SelectorPart {
            selector,
            combinator: Combinator::None,
        });
    }

    if selector_parts.is_empty() {
        SelectorConversionResult::Skip
    } else {
        SelectorConversionResult::Keep(
            selector_parts
                .iter()
                .map(|part| part.to_string())
                .collect::<String>(),
        )
    }
}

fn convert_component(component: &LComponent) -> ComponentConversion {
    match component {
        LComponent::LocalName(local_name) => {
            ComponentConversion::Keep(Selector::Type(local_name.name.as_ref().to_lowercase()))
        }
        LComponent::ExplicitUniversalType => {
            ComponentConversion::Keep(Selector::Type("*".to_string()))
        }
        LComponent::ID(id) => ComponentConversion::Keep(Selector::Id(id.to_string())),
        LComponent::Class(class) => ComponentConversion::Keep(Selector::Class(class.to_string())),
        LComponent::AttributeInNoNamespace {
            local_name,
            operator,
            value,
            ..
        } => {
            if matches!(operator, AttrSelectorOperator::Equal) {
                ComponentConversion::Keep(Selector::AttributeEquals {
                    name: local_name.as_ref().to_ascii_lowercase(),
                    value: value.to_string(),
                })
            } else {
                ComponentConversion::Skip
            }
        }
        LComponent::AttributeInNoNamespaceExists { .. } | LComponent::AttributeOther(_) => {
            ComponentConversion::Skip
        }
        LComponent::Negation(_)
        | LComponent::Root
        | LComponent::Empty
        | LComponent::Scope
        | LComponent::Nth(_)
        | LComponent::NthOf(_)
        | LComponent::NonTSPseudoClass(_)
        | LComponent::Slotted(_)
        | LComponent::Part(_)
        | LComponent::Host(_)
        | LComponent::Where(_)
        | LComponent::Is(_)
        | LComponent::Any(_, _)
        | LComponent::Has(_)
        | LComponent::PseudoElement(_) => ComponentConversion::Abort,
        LComponent::ExplicitAnyNamespace
        | LComponent::ExplicitNoNamespace
        | LComponent::DefaultNamespace(..)
        | LComponent::Namespace(..)
        | LComponent::Nesting => ComponentConversion::Abort,
        LComponent::Combinator(_) => ComponentConversion::Skip,
    }
}

fn extract_pseudo_tokens(selector: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = selector.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == ':' {
            let mut end = i + 1;
            if end < chars.len() && chars[end] == ':' {
                end += 1;
            }
            while end < chars.len() {
                let c = chars[end];
                if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                    end += 1;
                } else {
                    break;
                }
            }
            if end > i + 1 {
                let mut token: String = chars[i..end].iter().collect();
                token.make_ascii_lowercase();
                tokens.push(token);
            }
            if end < chars.len() && chars[end] == '(' {
                let mut depth = 1;
                let mut j = end + 1;
                while j < chars.len() && depth > 0 {
                    match chars[j] {
                        '(' => depth += 1,
                        ')' => depth -= 1,
                        _ => {}
                    }
                    j += 1;
                }
                i = j;
            } else {
                i = end;
            }
            continue;
        }
        i += 1;
    }
    tokens.sort();
    tokens.dedup();
    tokens
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct Nfacell(pub usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct SelectorId(pub usize);
/// Transition rule: (input selector, current state, next state)
/// When the input selector is None it represents a wildcard/epsilon or special match; a current
/// state of None can be used for start logic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rule(pub Option<SelectorId>, pub Option<Nfacell>, pub Nfacell);

#[derive(Debug, PartialEq, Eq)]
pub struct NFA {
    /// Set of all NFA states.
    pub states: HashSet<Option<Nfacell>>,
    /// Rule list: (optional predicate, optional predecessor state, successor state)
    pub rules: Vec<Rule>,
    /// Start state.
    pub start_state: Option<Nfacell>,
    pub max_state_id: Nfacell,
    // for print match
    pub accept_states: Vec<Nfacell>,
}

impl NFA {
    pub fn is_accept_state(&self, state: Nfacell) -> bool {
        !self
            .rules
            .iter()
            .any(|Rule(_, prev, _)| *prev == Some(state))
    }

    pub fn get_accept_states(&self) -> HashSet<Option<Nfacell>> {
        self.states
            .iter()
            .filter(|&&state| self.is_accept_state(state.unwrap()))
            .copied()
            .collect()
    }
    pub fn to_dot(&self, sm: &SelectorManager) -> String {
        let mut s = String::new();
        s.push_str("digraph NFA {\n");
        s.push_str("  rankdir=LR;\n");
        s.push_str("  node [shape=circle, fontsize=10];\n");

        // Start marker
        s.push_str("  __start [shape=point, label=\"\"];\n");
        s.push_str(&format!("  __start -> {:?};\n", self.start_state));

        // States
        for st in &self.states {
            s.push_str(&format!(
                "  {} [label=\"{}\"];\n",
                st.unwrap_or_default().0,
                st.unwrap_or_default().0
            ));
        }

        // Accept states styling
        if !self.accept_states.is_empty() {
            s.push_str("  { node [shape=doublecircle]; ");
            for st in &self.accept_states {
                s.push_str(&format!("{} ", st.0));
            }
            s.push_str("}\n");
        }

        // Add self-loop on the zero node
        let zero_node = self.start_state.unwrap_or_default().0;
        s.push_str(&format!(
            "  {} -> {} [label=\"*\"];\n",
            zero_node, zero_node
        ));

        // Edges
        for Rule(selector_opt, from_opt, to) in &self.rules {
            let from = from_opt.unwrap_or(self.start_state.unwrap_or_default()).0;
            let label = match selector_opt {
                None => "*".to_string(),
                Some(sel_id) => match sm.id_to_selector.get(sel_id) {
                    Some(Selector::Type(t)) => t.clone(),
                    Some(Selector::Class(c)) => format!(".{}", c),
                    Some(Selector::Id(i)) => format!("#{}", i),
                    Some(Selector::AttributeEquals { name, value }) => {
                        format!("[{}=\"{}\"]", name, value)
                    }
                    Some(Selector::Compound(comp)) => comp.to_string(),
                    None => format!("sid:{}", sel_id.0),
                },
            };
            s.push_str(&format!(
                "  {} -> {} [label=\"{}\"];\n",
                from,
                to.0,
                escape_dot_label(&label)
            ));
        }
        s.push_str("}\n");
        s
    }
}

fn escape_dot_label(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

#[derive(Debug, Default)]
pub struct SelectorManager {
    pub selector_to_id: HashMap<Selector, SelectorId>,
    pub id_to_selector: HashMap<SelectorId, Selector>,
    next_id: SelectorId,
}

impl SelectorManager {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_or_create_id(&mut self, selector: Selector) -> SelectorId {
        if let Some(&id) = self.selector_to_id.get(&selector) {
            return id;
        }

        let id = self.next_id;
        self.selector_to_id.insert(selector.clone(), id);
        self.id_to_selector.insert(id, selector);
        self.next_id = SelectorId(self.next_id.0 + 1);
        id
    }

    /// Get the ID for a selector.
    pub fn get_id(&self, selector: &Selector) -> Option<SelectorId> {
        self.selector_to_id.get(selector).copied()
    }

    pub fn get_or_create_type_id(&mut self, tag_name: &str) -> SelectorId {
        self.get_or_create_id(Selector::Type(tag_name.to_string()))
    }

    pub fn get_or_create_class_id(&mut self, class_name: &str) -> SelectorId {
        self.get_or_create_id(Selector::Class(class_name.to_string()))
    }

    pub fn get_or_create_id_selector_id(&mut self, id_name: &str) -> SelectorId {
        self.get_or_create_id(Selector::Id(id_name.to_string()))
    }
}

/// Encodes a slice of elements of type T using Run-Length Encoding.
///
/// # Arguments
///
/// * `data` - A slice of elements to be encoded.
///
/// # Type Parameters
///
/// * `T` - The type of the elements in the slice. It must implement the `Copy` and `PartialEq` traits.
///
/// # Returns
///
/// A `Vec<(T, usize)>` where each tuple represents a run of an element and its count.
pub fn encode<T>(data: &[T]) -> Vec<(T, usize)>
where
    T: Copy + PartialEq,
{
    if data.is_empty() {
        return Vec::new();
    }

    let mut encoded = Vec::new();
    let mut current_item = data[0];
    let mut count = 1;

    for &item in &data[1..] {
        if item == current_item {
            count += 1;
        } else {
            encoded.push((current_item, count));
            current_item = item;
            count = 1;
        }
    }

    encoded.push((current_item, count));
    encoded
}

pub fn generate_nfa(selectors: &[String], sm: &mut SelectorManager, state: &mut usize) -> NFA {
    *state = 0;
    let start_state = Option::<Nfacell>::None;
    let mut states: HashSet<Option<Nfacell>> = [start_state].into_iter().collect();
    let mut rules: Vec<Rule> = Vec::new();
    let mut accept_states: Vec<Nfacell> = Vec::with_capacity(selectors.len());

    for rule in selectors {
        let t = rule.replace('>', " > ");
        let parts: Vec<&str> = t.split_whitespace().collect();
        let mut cur = start_state;

        let mut i = 0;
        while i < parts.len() {
            if parts[i] == ">" {
                i += 1;
                continue;
            }
            let selector_str = parts[i];

            // Look ahead to find the next selector and whether the combinator is direct (>)
            let mut next_selector_index = i + 1;
            let mut next_is_direct = false;
            if next_selector_index < parts.len() && parts[next_selector_index] == ">" {
                next_is_direct = true;
                next_selector_index += 1;
            }
            let has_next_selector = next_selector_index < parts.len();

            // Create new state and edge for current selector
            *state += 1;

            let new_state = Nfacell(*state);
            states.insert(Some(new_state));

            let selector = parse_selector(selector_str);
            match selector {
                Selector::Type(ref s) if s == "*" => {
                    rules.push(Rule(None, cur, new_state));
                }
                other => {
                    let selector_id = sm.get_or_create_id(other);
                    rules.push(Rule(Some(selector_id), cur, new_state));
                }
            }

            // Add self-loop only for descendant combinators (a b), not for child (a > b)
            if has_next_selector && !next_is_direct {
                rules.push(Rule(None, Some(new_state), new_state));
            }

            cur = Some(new_state);
            i = next_selector_index;
        }
        accept_states.push(cur.unwrap());
    }
    NFA {
        states,
        rules,
        start_state,
        max_state_id: Nfacell(*state),
        accept_states,
    }
}

/// Parse a CSS selector string and produce the corresponding selector object.
pub fn parse_selector(selector_str: &str) -> Selector {
    let trimmed = selector_str.trim();
    if trimmed.is_empty() {
        return Selector::Type("*".to_string());
    }

    let mut compound = CompoundSelector::default();
    let mut pos = 0;
    let len = trimmed.len();

    while pos < len {
        let ch = trimmed[pos..].chars().next().unwrap();
        match ch {
            '.' => {
                pos += ch.len_utf8();
                let (class_name, next_pos) = consume_identifier(trimmed, pos);
                if !class_name.is_empty() {
                    compound.classes.insert(class_name.to_ascii_lowercase());
                }
                pos = next_pos;
            }
            '#' => {
                pos += ch.len_utf8();
                let (id_name, next_pos) = consume_identifier(trimmed, pos);
                if !id_name.is_empty() {
                    compound.id = Some(id_name);
                }
                pos = next_pos;
            }
            ':' => {
                let (pseudo, next_pos) = consume_pseudo(trimmed, pos);
                if !pseudo.is_empty() {
                    compound.pseudos.insert(pseudo);
                }
                pos = next_pos;
            }
            '[' => {
                if let Some((attribute, next_pos)) = consume_attribute(trimmed, pos) {
                    compound.attributes.push(attribute);
                    pos = next_pos;
                } else {
                    break;
                }
            }
            '*' => {
                compound.tag = Some("*".to_string());
                pos += ch.len_utf8();
            }
            _ => {
                let (tag_name, next_pos) = consume_identifier(trimmed, pos);
                if !tag_name.is_empty() {
                    compound.tag = Some(tag_name.to_ascii_lowercase());
                }
                pos = next_pos;
            }
        }
    }

    compound.attributes.sort();

    if compound.is_simple_class_only() {
        let class_name = compound.classes.iter().next().cloned().unwrap_or_default();
        Selector::Class(class_name)
    } else if compound.is_simple_id_only() {
        Selector::Id(compound.id.unwrap())
    } else if compound.is_simple_attr_only() {
        let (name, value) = compound.attributes.into_iter().next().unwrap();
        Selector::AttributeEquals { name, value }
    } else if compound.is_simple_tag_only() {
        Selector::Type(compound.tag.unwrap())
    } else {
        Selector::Compound(compound)
    }
}

fn consume_identifier(selector: &str, start: usize) -> (String, usize) {
    let mut pos = start;
    let mut ident = String::new();
    while pos < selector.len() {
        let ch = selector[pos..].chars().next().unwrap();
        if matches!(ch, '.' | '#' | ':' | '[' | ']' | ' ' | '>') {
            break;
        }
        ident.push(ch);
        pos += ch.len_utf8();
    }
    (ident, pos)
}

fn consume_attribute(selector: &str, start: usize) -> Option<((String, String), usize)> {
    let remainder = &selector[start..];
    let closing = remainder.find(']')?;
    let end = start + closing + 1;
    let slice = &selector[start..end];
    if let Some(Selector::AttributeEquals { name, value }) = parse_attribute_selector(slice) {
        Some(((name, value), end))
    } else {
        None
    }
}

fn consume_pseudo(selector: &str, start: usize) -> (String, usize) {
    let mut idx = start;
    let mut colon_count = 0;
    while idx < selector.len() {
        let ch = selector[idx..].chars().next().unwrap();
        if ch == ':' {
            colon_count += 1;
            idx += ch.len_utf8();
        } else {
            break;
        }
    }

    let mut name = String::new();
    let mut paren_depth = 0;
    while idx < selector.len() {
        let ch = selector[idx..].chars().next().unwrap();
        if paren_depth == 0 && matches!(ch, '.' | '#' | ':' | '[' | ']') {
            break;
        }
        if ch == '(' {
            paren_depth += 1;
        } else if ch == ')' && paren_depth > 0 {
            paren_depth -= 1;
        }
        name.push(ch);
        idx += ch.len_utf8();
    }

    if colon_count == 0 || name.is_empty() {
        return (String::new(), idx);
    }

    let mut pseudo = name.to_ascii_lowercase();
    if colon_count >= 2 {
        pseudo = format!("::{}", pseudo);
    }
    (pseudo, idx)
}

pub fn json_value_to_attr_string(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Array(_) | serde_json::Value::Object(_) => value.to_string(),
        serde_json::Value::Null => String::new(),
    }
}

pub const PSEUDO_CLASS_HOVER: &str = "hover";
pub const PSEUDO_CLASS_HOVER_ROOT: &str = "hover-root";
pub const PSEUDO_CLASS_FOCUS: &str = "focus";
pub const PSEUDO_CLASS_FOCUS_ROOT: &str = "focus-root";
pub const PSEUDO_CLASS_FOCUS_WITHIN: &str = "focus-within";

pub fn derive_hover_state(pseudo_flags: &HashSet<String>, parent_hover: bool) -> bool {
    parent_hover
        || pseudo_flags.contains(PSEUDO_CLASS_HOVER_ROOT)
        || pseudo_flags.contains(PSEUDO_CLASS_HOVER)
}

pub fn extract_pseudoclasses(node: &serde_json::Value) -> HashSet<String> {
    fn collect_from_value(value: &serde_json::Value, target: &mut HashSet<String>) {
        match value {
            serde_json::Value::Array(items) => {
                for item in items {
                    collect_from_value(item, target);
                }
            }
            serde_json::Value::Object(map) => {
                for (key, val) in map {
                    let normalized = key.to_ascii_lowercase();
                    match val {
                        serde_json::Value::Bool(active) => {
                            if *active {
                                target.insert(normalized.clone());
                            }
                        }
                        serde_json::Value::String(s) => {
                            if !s.is_empty() {
                                target.insert(s.to_ascii_lowercase());
                            } else {
                                target.insert(normalized.clone());
                            }
                        }
                        _ => {}
                    }
                }
            }
            serde_json::Value::String(s) => {
                if !s.is_empty() {
                    target.insert(s.to_ascii_lowercase());
                }
            }
            _ => {}
        }
    }

    let mut result = HashSet::new();
    for key in [
        "pseudoclasses",
        "pseudo_classes",
        "pseudoclass",
        "pseudo_class",
    ] {
        if let Some(value) = node.get(key) {
            collect_from_value(value, &mut result);
        }
    }
    if let Some(attrs) = node.get("attributes").and_then(|attrs| attrs.as_object()) {
        if attrs
            .get("is_hovered_root")
            .and_then(|value| value.as_bool())
            .unwrap_or(false)
        {
            result.insert(PSEUDO_CLASS_HOVER_ROOT.to_string());
        }
        if attrs
            .get("is_focus_root")
            .and_then(|value| value.as_bool())
            .unwrap_or(false)
        {
            result.insert(PSEUDO_CLASS_FOCUS_ROOT.to_string());
        }
    }
    result
}

pub trait AddNode {
    /// Add a new node to the DOM.
    /// Returns the index of the new node.
    fn add_node(
        &mut self,
        id: u64,
        tag_name: &str,
        classes: Vec<String>,
        html_id: Option<String>,
        attributes: HashMap<String, String>,
        pseudo_classes: HashSet<String>,
        parent_index: Option<u64>,
        nfa: &NFA,
    ) -> u64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_css_handles_attribute_selector() {
        let selectors = parse_css(r#"[data-test="value"] { color: red; }"#);
        assert_eq!(selectors, vec![r#"[data-test="value"]"#.to_string()]);
    }

    #[test]
    fn parse_css_skips_media_queries() {
        let selectors = parse_css(
            r#"@media screen and (max-width: 600px) {
                .hidden { display: none; }
            }
            .visible { display: block; }"#,
        );
        assert_eq!(selectors, vec![".visible".to_string()]);
    }

    #[test]
    fn parse_selector_returns_attribute_variant() {
        match parse_selector(r#"[data-id="item-1"]"#) {
            Selector::AttributeEquals { name, value } => {
                assert_eq!(name, "data-id");
                assert_eq!(value, "item-1");
            }
            other => panic!("expected attribute selector, got {:?}", other),
        }
    }

    #[test]
    fn parse_selector_handles_class_and_pseudo() {
        match parse_selector(".foo:hover") {
            Selector::Compound(compound) => {
                assert!(compound.classes.contains("foo"));
                assert!(compound.pseudos.contains("hover"));
            }
            other => panic!("expected compound selector, got {:?}", other),
        }
    }
}
