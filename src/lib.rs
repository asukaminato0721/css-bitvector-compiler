use lightningcss::{
    rules::CssRule,
    selector::{Combinator as LCombinator, Component as LComponent, Selector as LightningSelector},
    stylesheet::{ParserOptions, StyleSheet},
};
use parcel_selectors::attr::AttrSelectorOperator;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};
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
pub enum Selector {
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

pub fn parse_css(css_content: &str) -> Vec<String> {
    let mut selectors = Vec::new();
    let mut parser_options = ParserOptions::default();
    parser_options.error_recovery = true;

    let stylesheet = match StyleSheet::parse(css_content, parser_options) {
        Ok(sheet) => sheet,
        Err(_) => return selectors,
    };

    for rule in stylesheet.rules.0 {
        if let CssRule::Style(style_rule) = rule {
            selectors.extend(
                style_rule
                    .selectors
                    .0
                    .into_iter()
                    .filter_map(|selector| lightning_selector_to_rule_string(&selector)),
            );
        }
    }

    selectors.sort();
    selectors.dedup();
    selectors
}

enum ComponentConversion {
    Keep(Selector),
    Skip,
    Abort,
}

fn lightning_selector_to_rule_string(selector: &LightningSelector) -> Option<String> {
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
                LCombinator::PseudoElement => break,
                _ => break,
            },
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
                ComponentConversion::Abort => return None,
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
        None
    } else {
        Some(
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
        | LComponent::PseudoElement(_) => ComponentConversion::Skip,
        LComponent::ExplicitAnyNamespace
        | LComponent::ExplicitNoNamespace
        | LComponent::DefaultNamespace(..)
        | LComponent::Namespace(..)
        | LComponent::Nesting => ComponentConversion::Abort,
        LComponent::Combinator(_) => ComponentConversion::Skip,
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct Nfacell(pub usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct SelectorId(pub usize);
/// 转移规则: (输入选择器, 当前状态, 下一个状态)
/// 其中输入选择器为 None 表示通配符/epsilon 或者特殊匹配；当前状态为 None 可用于起始逻辑
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rule(pub Option<SelectorId>, pub Option<Nfacell>, pub Nfacell);

#[derive(Debug, PartialEq, Eq)]
pub struct NFA {
    /// NFA 中所有状态的集合。
    pub states: HashSet<Option<Nfacell>>,
    /// 规则列表： (可选谓词, 可选前驱状态, 后继状态)
    pub rules: Vec<Rule>,
    /// 起始状态。
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

    /// 根据选择器获取ID
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

/// 解析CSS选择器字符串并生成对应的选择器对象
pub fn parse_selector(selector_str: &str) -> Selector {
    let trimmed = selector_str.trim();

    if trimmed.starts_with('.') {
        // 类选择器
        Selector::Class(trimmed[1..].to_string())
    } else if trimmed.starts_with('#') {
        // ID选择器
        Selector::Id(trimmed[1..].to_string())
    } else if let Some(attribute_selector) = parse_attribute_selector(trimmed) {
        attribute_selector
    } else {
        // 标签选择器
        Selector::Type(trimmed.to_string())
    }
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

pub trait AddNode {
    /// 向 DOM 中添加一个新节点。
    /// 返回新节点的索引。
    fn add_node(
        &mut self,
        id: u64,
        tag_name: &str,
        classes: Vec<String>,
        html_id: Option<String>,
        attributes: HashMap<String, String>,
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
}
