use lightningcss::{
    rules::CssRule,
    selector::{
        Combinator as CssCombinator, Component as CssComponent, PseudoClass,
        Selector as CssSelector,
    },
    stylesheet::{ParserOptions, PrinterOptions, StyleSheet},
    traits::ToCss,
};
use parcel_selectors::{attr::AttrSelectorOperator, parser::NthType};
use serde::Deserialize;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    error::Error,
    fmt::{self, Display},
    fs,
    path::{Path, PathBuf},
};

use crate::{Node, attributes_to_string_map, rdtsc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StateId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineKind {
    Naive,
    Bit,
    Tri,
    RecursiveTri,
    ExperimentalQuad,
}

impl EngineKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::Naive => "naive",
            Self::Bit => "bit",
            Self::Tri => "tri",
            Self::RecursiveTri => "rec_tri",
            Self::ExperimentalQuad => "quad",
        }
    }

    fn uses_dependencies(self) -> bool {
        matches!(self, Self::Tri | Self::RecursiveTri)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Combinator {
    Descendant,
    Child,
    AdjacentSibling,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Nth {
    pub a: i32,
    pub b: i32,
    pub of_type: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DynamicPseudo {
    Hover,
    Focus,
    FocusWithin,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Compound {
    pub tag: Option<String>,
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub attributes: Vec<(String, String)>,
    pub pseudos: Vec<DynamicPseudo>,
    pub nth: Vec<Nth>,
}

impl Compound {
    fn is_empty(&self) -> bool {
        self.tag.is_none()
            && self.id.is_none()
            && self.classes.is_empty()
            && self.attributes.is_empty()
            && self.pseudos.is_empty()
            && self.nth.is_empty()
    }

    fn is_trivial_single(&self) -> bool {
        self.attributes.is_empty() && self.pseudos.is_empty() && self.nth.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectorChain {
    pub text: String,
    pub compounds: Vec<Compound>,
    /// `combinators[i]` connects compounds `i` and `i + 1`.
    pub combinators: Vec<Combinator>,
    state_offset: usize,
}

impl SelectorChain {
    fn state(&self, part: usize) -> StateId {
        StateId(self.state_offset + part)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsupportedSelector {
    pub selector: String,
    pub reason: String,
}

#[derive(Debug, Clone, Default)]
pub struct SelectorReport {
    pub skipped_simple: Vec<String>,
    pub unsupported: Vec<UnsupportedSelector>,
    pub unsupported_pseudos: BTreeMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Default)]
pub struct CompiledProgram {
    pub selectors: Vec<SelectorChain>,
    pub report: SelectorReport,
    pub state_count: usize,
}

impl CompiledProgram {
    pub fn compile(css: &str) -> Result<Self, CompileError> {
        let options = ParserOptions {
            error_recovery: true,
            ..ParserOptions::default()
        };
        let sheet = StyleSheet::parse(css, options)
            .map_err(|error| CompileError(format!("unable to parse stylesheet: {error:?}")))?;

        let mut parsed = Vec::new();
        let mut report = SelectorReport::default();
        for rule in sheet.rules.0 {
            let CssRule::Style(style) = rule else {
                continue;
            };
            for selector in style.selectors.0 {
                let text = selector_text(&selector);
                match parse_selector(&selector, text.clone()) {
                    Ok(chain)
                        if chain.combinators.is_empty()
                            && chain.compounds[0].is_trivial_single() =>
                    {
                        report.skipped_simple.push(text);
                    }
                    Ok(chain) => parsed.push(chain),
                    Err(reason) => {
                        if let Some(pseudo) = reason.strip_prefix("unsupported pseudo-class ") {
                            report
                                .unsupported_pseudos
                                .entry(pseudo.to_string())
                                .or_default()
                                .push(text.clone());
                        }
                        report.unsupported.push(UnsupportedSelector {
                            selector: text,
                            reason,
                        });
                    }
                }
            }
        }

        parsed.sort_by(|a, b| a.text.cmp(&b.text));
        parsed.dedup_by(|a, b| a.text == b.text);
        report.skipped_simple.sort();
        report.skipped_simple.dedup();
        report
            .unsupported
            .sort_by(|a, b| a.selector.cmp(&b.selector).then(a.reason.cmp(&b.reason)));
        report
            .unsupported
            .dedup_by(|a, b| a.selector == b.selector && a.reason == b.reason);
        for selectors in report.unsupported_pseudos.values_mut() {
            selectors.sort();
            selectors.dedup();
        }

        let mut state_count = 0;
        for selector in &mut parsed {
            selector.state_offset = state_count;
            state_count += selector.compounds.len();
        }
        Ok(Self {
            selectors: parsed,
            report,
            state_count,
        })
    }

    pub fn to_dot(&self) -> String {
        let mut dot = String::from("digraph SelectorProgram {\n  rankdir=LR;\n");
        for (rule_index, selector) in self.selectors.iter().enumerate() {
            for (part, compound) in selector.compounds.iter().enumerate() {
                let state = selector.state(part).0;
                let label = format!("{}:{} {:?}", rule_index, part, compound).replace('"', "\\\"");
                let shape = if part + 1 == selector.compounds.len() {
                    "doublecircle"
                } else {
                    "circle"
                };
                dot.push_str(&format!("  s{state} [shape={shape}, label=\"{label}\"];\n"));
                if part > 0 {
                    let previous = selector.state(part - 1).0;
                    dot.push_str(&format!(
                        "  s{previous} -> s{state} [label=\"{:?}\"];\n",
                        selector.combinators[part - 1]
                    ));
                }
            }
        }
        dot.push_str("}\n");
        dot
    }
}

#[derive(Debug, Clone)]
pub struct CompileError(pub String);

impl Display for CompileError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl Error for CompileError {}

fn selector_text(selector: &CssSelector) -> String {
    selector
        .to_css_string(PrinterOptions::default())
        .unwrap_or_else(|_| format!("{selector:?}"))
}

fn parse_selector(selector: &CssSelector, text: String) -> Result<SelectorChain, String> {
    let mut compounds = vec![Compound::default()];
    let mut combinators = Vec::new();

    for component in selector.iter_raw_parse_order_from(0) {
        match component {
            CssComponent::Combinator(css_combinator) => {
                if compounds.last().is_some_and(Compound::is_empty) {
                    continue;
                }
                let combinator = match css_combinator {
                    CssCombinator::Descendant => Combinator::Descendant,
                    CssCombinator::Child => Combinator::Child,
                    CssCombinator::NextSibling => Combinator::AdjacentSibling,
                    CssCombinator::LaterSibling => {
                        return Err("unsupported combinator general-sibling".into());
                    }
                    _ => return Err(format!("unsupported combinator {css_combinator:?}")),
                };
                combinators.push(combinator);
                compounds.push(Compound::default());
            }
            CssComponent::ExplicitUniversalType => {
                compounds.last_mut().unwrap().tag = Some("*".into());
            }
            CssComponent::LocalName(name) => {
                compounds.last_mut().unwrap().tag = Some(name.name.as_ref().to_ascii_lowercase());
            }
            CssComponent::ID(id) => compounds.last_mut().unwrap().id = Some(id.to_string()),
            CssComponent::Class(class) => {
                compounds
                    .last_mut()
                    .unwrap()
                    .classes
                    .push(class.to_string());
            }
            CssComponent::AttributeInNoNamespace {
                local_name,
                operator: AttrSelectorOperator::Equal,
                value,
                ..
            } => compounds
                .last_mut()
                .unwrap()
                .attributes
                .push((local_name.as_ref().to_ascii_lowercase(), value.to_string())),
            CssComponent::Nth(data) => {
                let of_type = match data.ty {
                    NthType::Child | NthType::OfType => data.ty == NthType::OfType,
                    NthType::LastChild | NthType::LastOfType => {
                        return Err("unsupported pseudo-class :last-*".into());
                    }
                    _ => return Err("unsupported pseudo-class :only-*".into()),
                };
                compounds.last_mut().unwrap().nth.push(Nth {
                    a: data.a,
                    b: data.b,
                    of_type,
                });
            }
            CssComponent::NonTSPseudoClass(pseudo) => {
                let pseudo = match pseudo {
                    PseudoClass::Hover => DynamicPseudo::Hover,
                    PseudoClass::Focus => DynamicPseudo::Focus,
                    PseudoClass::FocusWithin => DynamicPseudo::FocusWithin,
                    _ => return Err("unsupported pseudo-class <dynamic>".into()),
                };
                compounds.last_mut().unwrap().pseudos.push(pseudo);
            }
            CssComponent::NthOf(_) => return Err("unsupported pseudo-class :nth-*(of S)".into()),
            CssComponent::Negation(_) => return Err("unsupported pseudo-class :not".into()),
            CssComponent::Is(_) => return Err("unsupported pseudo-class :is".into()),
            CssComponent::Where(_) => return Err("unsupported pseudo-class :where".into()),
            CssComponent::Has(_) => return Err("unsupported pseudo-class :has".into()),
            CssComponent::PseudoElement(_) => {
                return Err("unsupported pseudo-element".into());
            }
            CssComponent::AttributeInNoNamespaceExists { .. }
            | CssComponent::AttributeInNoNamespace { .. }
            | CssComponent::AttributeOther(_) => {
                return Err("unsupported attribute operator".into());
            }
            other => return Err(format!("unsupported selector component {other:?}")),
        }
    }

    if compounds.last().is_some_and(Compound::is_empty) {
        compounds.pop();
    }
    if compounds.is_empty() || combinators.len() + 1 != compounds.len() {
        return Err("malformed selector chain".into());
    }
    for compound in &mut compounds {
        compound.classes.sort();
        compound.classes.dedup();
        compound.attributes.sort();
        compound.attributes.dedup();
        compound.pseudos.sort_by_key(|pseudo| *pseudo as u8);
        compound.pseudos.dedup();
    }
    Ok(SelectorChain {
        text,
        compounds,
        combinators,
        state_offset: 0,
    })
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "name", rename_all = "snake_case")]
pub enum TraceCommand {
    Init {
        node: Node,
    },
    Add {
        path: Vec<usize>,
        node: Node,
    },
    Remove {
        path: Vec<usize>,
    },
    Replace {
        path: Vec<usize>,
        node: Node,
    },
    ReplaceValue {
        path: Vec<usize>,
        #[serde(rename = "type")]
        value_type: Option<String>,
        key: String,
        value: Option<serde_json::Value>,
        old_value: Option<serde_json::Value>,
    },
    InsertValue {
        path: Vec<usize>,
        #[serde(rename = "type")]
        value_type: Option<String>,
        key: String,
        value: Option<serde_json::Value>,
    },
    DeleteValue {
        path: Vec<usize>,
        #[serde(rename = "type")]
        value_type: Option<String>,
        key: String,
        old_value: Option<serde_json::Value>,
    },
    Recalculate,
}

impl TraceCommand {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Init { .. } => "init",
            Self::Add { .. } => "add",
            Self::Remove { .. } => "remove",
            Self::Replace { .. } => "replace",
            Self::ReplaceValue { .. } => "replace_value",
            Self::InsertValue { .. } => "insert_value",
            Self::DeleteValue { .. } => "delete_value",
            Self::Recalculate => "recalculate",
        }
    }
}

#[derive(Debug, Clone)]
pub struct TraceFrame {
    pub frame_id: usize,
    pub command: TraceCommand,
}

#[derive(Debug, Clone, Default)]
pub struct Trace {
    pub frames: Vec<TraceFrame>,
}

impl Trace {
    pub fn parse(path: &Path) -> Result<Self, RunError> {
        let content = fs::read_to_string(path)
            .map_err(|error| RunError::new(format!("cannot read {}: {error}", path.display())))?;
        let mut frames = Vec::new();
        for (frame_id, line) in content.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            let header: TraceHeader = serde_json::from_str(line).map_err(|error| {
                RunError::at(frame_id, format!("invalid command header: {error}"))
            })?;
            if header.name.starts_with("layout_") {
                continue;
            }
            let command = serde_json::from_str(line).map_err(|error| {
                RunError::at(
                    frame_id,
                    format!("invalid command `{}`: {error}", header.name),
                )
            })?;
            frames.push(TraceFrame { frame_id, command });
        }
        Ok(Self { frames })
    }
}

#[derive(Deserialize)]
struct TraceHeader {
    name: String,
}

#[derive(Debug, Clone)]
pub struct RunError {
    message: String,
}

impl RunError {
    fn new(message: String) -> Self {
        Self { message }
    }

    fn at(frame_id: usize, message: String) -> Self {
        Self::new(format!("frame {frame_id}: {message}"))
    }
}

impl Display for RunError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for RunError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Dirty {
    #[default]
    Clean,
    InputChanged,
    NodeChanged,
}

impl Dirty {
    fn merge(self, other: Self) -> Self {
        if self as u8 >= other as u8 {
            self
        } else {
            other
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Requirement {
    #[default]
    Unused,
    Zero,
    One,
}

impl Requirement {
    fn from_bit(bit: bool) -> Self {
        if bit { Self::One } else { Self::Zero }
    }

    fn accepts(self, bit: bool) -> bool {
        matches!(self, Self::Unused | Self::Zero if !bit)
            || matches!(self, Self::Unused | Self::One if bit)
    }
}

#[derive(Debug, Clone, Default)]
struct NodeState {
    output: Vec<bool>,
    matches: Vec<bool>,
    parent_input: Vec<bool>,
    sibling_input: Vec<bool>,
    require_parent: Vec<Requirement>,
    require_sibling: Vec<Requirement>,
    dirty: Dirty,
    subtree_dirty: bool,
}

#[derive(Debug, Clone, Default)]
struct NodeFacts {
    tag: String,
    classes: HashSet<String>,
    html_id: Option<String>,
    attributes: HashMap<String, String>,
    hover_root: bool,
    focus_root: bool,
    pseudos: HashSet<DynamicPseudo>,
}

#[derive(Debug, Clone, Default)]
struct DomNode {
    facts: NodeFacts,
    parent: Option<NodeId>,
    children: Vec<NodeId>,
    state: NodeState,
}

#[derive(Debug, Clone, Default)]
pub struct Dom {
    nodes: HashMap<NodeId, DomNode>,
    root: Option<NodeId>,
}

impl Dom {
    fn clear(&mut self) {
        self.nodes.clear();
        self.root = None;
    }

    fn build_subtree(
        &mut self,
        node: &Node,
        parent: Option<NodeId>,
        program: &CompiledProgram,
    ) -> Result<NodeId, RunError> {
        let id = NodeId(node.id);
        // The trace generator can encode a move as an add of an existing ID.
        // Keep the old parent reference until its later remove command, while
        // replacing the arena entry with the node at its new location.
        if self.nodes.contains_key(&id) {
            self.remove_subtree(id);
        }
        let attributes = attributes_to_string_map(&node.attributes);
        let facts = facts_from_parts(&node.tag_name, attributes);
        self.nodes.insert(
            id,
            DomNode {
                facts,
                parent,
                children: Vec::new(),
                state: NodeState {
                    output: vec![false; program.state_count],
                    matches: vec![false; program.selectors.len()],
                    parent_input: vec![false; program.state_count],
                    sibling_input: vec![false; program.state_count],
                    require_parent: vec![Requirement::Unused; program.state_count],
                    require_sibling: vec![Requirement::Unused; program.state_count],
                    dirty: Dirty::NodeChanged,
                    subtree_dirty: true,
                },
            },
        );
        for child in &node.children {
            let child_id = self.build_subtree(child, Some(id), program)?;
            self.nodes.get_mut(&id).unwrap().children.push(child_id);
        }
        Ok(id)
    }

    fn id_at_path(&self, path: &[usize]) -> Result<NodeId, RunError> {
        let mut current = self
            .root
            .ok_or_else(|| RunError::new("DOM has no root".into()))?;
        for &index in path {
            current = *self
                .nodes
                .get(&current)
                .and_then(|node| node.children.get(index))
                .ok_or_else(|| RunError::new(format!("invalid DOM path {path:?}")))?;
        }
        Ok(current)
    }

    fn mark(&mut self, id: NodeId, dirty: Dirty) {
        let mut current = Some(id);
        let mut first = true;
        while let Some(node_id) = current {
            let Some(node) = self.nodes.get_mut(&node_id) else {
                break;
            };
            if first {
                node.state.dirty = node.state.dirty.merge(dirty);
                first = false;
            }
            node.state.subtree_dirty = true;
            current = node.parent;
        }
    }

    fn following_siblings(&self, id: NodeId) -> Vec<NodeId> {
        let Some(parent) = self.nodes.get(&id).and_then(|node| node.parent) else {
            return Vec::new();
        };
        let children = &self.nodes[&parent].children;
        children
            .iter()
            .position(|child| *child == id)
            .map(|index| children[index + 1..].to_vec())
            .unwrap_or_default()
    }

    fn next_sibling(&self, id: NodeId) -> Option<NodeId> {
        self.following_siblings(id).first().copied()
    }

    fn previous_sibling(&self, id: NodeId) -> Option<NodeId> {
        let parent = self.nodes.get(&id)?.parent?;
        let siblings = &self.nodes.get(&parent)?.children;
        let index = siblings.iter().position(|sibling| *sibling == id)?;
        index.checked_sub(1).map(|previous| siblings[previous])
    }

    fn remove_subtree(&mut self, id: NodeId) {
        if let Some(node) = self.nodes.remove(&id) {
            for child in node.children {
                self.remove_subtree(child);
            }
        }
    }

    fn position(&self, id: NodeId) -> Option<(usize, usize)> {
        let node = self.nodes.get(&id)?;
        let parent = node.parent?;
        let siblings = &self.nodes.get(&parent)?.children;
        let mut all = 0;
        let mut of_type = 0;
        for sibling in siblings {
            all += 1;
            if self.nodes.get(sibling)?.facts.tag == node.facts.tag {
                of_type += 1;
            }
            if *sibling == id {
                return Some((all, of_type));
            }
        }
        None
    }

    fn previous_sibling_output(&self, id: NodeId, state_count: usize) -> Vec<bool> {
        let Some(parent) = self.nodes.get(&id).and_then(|node| node.parent) else {
            return vec![false; state_count];
        };
        let siblings = &self.nodes[&parent].children;
        let Some(index) = siblings.iter().position(|sibling| *sibling == id) else {
            return vec![false; state_count];
        };
        if index == 0 {
            vec![false; state_count]
        } else {
            self.nodes
                .get(&siblings[index - 1])
                .map(|node| node.state.output.clone())
                .unwrap_or_else(|| vec![false; state_count])
        }
    }

    fn refresh_pseudos(&mut self) {
        let Some(root) = self.root else { return };
        self.refresh_hover(root, false);
        self.refresh_focus(root);
    }

    fn refresh_hover(&mut self, id: NodeId, parent_hover: bool) {
        let (active, children, changed) = {
            let Some(node) = self.nodes.get_mut(&id) else {
                return;
            };
            let active = parent_hover || node.facts.hover_root;
            let changed = set_pseudo(&mut node.facts.pseudos, DynamicPseudo::Hover, active);
            (active, node.children.clone(), changed)
        };
        if changed {
            self.mark(id, Dirty::NodeChanged);
        }
        for child in children {
            self.refresh_hover(child, active);
        }
    }

    fn refresh_focus(&mut self, id: NodeId) -> bool {
        let (focus, children) = {
            let Some(node) = self.nodes.get(&id) else {
                return false;
            };
            (node.facts.focus_root, node.children.clone())
        };
        let mut within = focus;
        for child in children {
            within |= self.refresh_focus(child);
        }
        let changed = {
            let node = self.nodes.get_mut(&id).unwrap();
            set_pseudo(&mut node.facts.pseudos, DynamicPseudo::Focus, focus)
                | set_pseudo(&mut node.facts.pseudos, DynamicPseudo::FocusWithin, within)
        };
        if changed {
            self.mark(id, Dirty::NodeChanged);
        }
        within
    }
}

fn set_pseudo(set: &mut HashSet<DynamicPseudo>, pseudo: DynamicPseudo, active: bool) -> bool {
    if active {
        set.insert(pseudo)
    } else {
        set.remove(&pseudo)
    }
}

fn facts_from_parts(tag: &str, mut attributes: HashMap<String, String>) -> NodeFacts {
    let classes = attributes
        .get("class")
        .map(|value| value.split_whitespace().map(str::to_string).collect())
        .unwrap_or_default();
    let html_id = attributes.get("id").cloned();
    let hover_root = take_boolean_alias(&mut attributes, &["is_hover_root", "is_hovered_root"]);
    let focus_root = take_boolean_alias(&mut attributes, &["is_focus_root"]);
    NodeFacts {
        tag: tag.to_ascii_lowercase(),
        classes,
        html_id,
        attributes,
        hover_root,
        focus_root,
        pseudos: HashSet::new(),
    }
}

fn take_boolean_alias(attributes: &mut HashMap<String, String>, keys: &[&str]) -> bool {
    keys.iter().any(|key| {
        attributes
            .get(*key)
            .is_some_and(|value| value.eq_ignore_ascii_case("true"))
    })
}

#[derive(Debug, Clone, Default)]
pub struct RunStats {
    pub recomputed_nodes: usize,
    pub input_changes: usize,
    pub input_skips: usize,
    pub visited_nodes: usize,
    pub match_changes: usize,
    pub cycles: u64,
}

#[derive(Debug, Clone)]
pub struct FrameStats {
    pub frame_id: usize,
    pub command: &'static str,
    pub miss_delta: usize,
    pub node_match_changes: usize,
    pub total_misses: usize,
}

#[derive(Debug, Clone)]
pub struct RunResult {
    pub matches: BTreeMap<String, Vec<u64>>,
    pub stats: RunStats,
    pub frames: Vec<FrameStats>,
}

#[derive(Debug, Clone)]
pub struct Engine {
    kind: EngineKind,
    program: CompiledProgram,
    dom: Dom,
    stats: RunStats,
    frame_stats: Vec<FrameStats>,
    track_frame_stats: bool,
}

impl Engine {
    pub fn new(kind: EngineKind, program: CompiledProgram) -> Self {
        Self {
            kind,
            program,
            dom: Dom::default(),
            stats: RunStats::default(),
            frame_stats: Vec::new(),
            track_frame_stats: false,
        }
    }

    pub fn with_frame_stats(mut self, enabled: bool) -> Self {
        self.track_frame_stats = enabled;
        self
    }

    pub fn run(mut self, trace: &Trace) -> Result<RunResult, RunError> {
        let start = rdtsc();
        let mut previous = BTreeMap::new();
        for frame in &trace.frames {
            let before = self.stats.recomputed_nodes;
            self.apply(frame)
                .map_err(|error| RunError::at(frame.frame_id, error.message))?;
            if self.track_frame_stats {
                let current = self.collect_matches();
                let changed = count_node_match_changes(&previous, &current);
                self.stats.match_changes += changed;
                self.frame_stats.push(FrameStats {
                    frame_id: frame.frame_id,
                    command: frame.command.name(),
                    miss_delta: self.stats.recomputed_nodes - before,
                    node_match_changes: changed,
                    total_misses: self.stats.recomputed_nodes,
                });
                previous = current;
            }
        }
        self.stats.cycles = rdtsc().wrapping_sub(start);
        Ok(RunResult {
            matches: self.collect_matches(),
            stats: self.stats,
            frames: self.frame_stats,
        })
    }

    fn apply(&mut self, frame: &TraceFrame) -> Result<(), RunError> {
        match &frame.command {
            TraceCommand::Init { node } => {
                self.dom.clear();
                let root = self.dom.build_subtree(node, None, &self.program)?;
                self.dom.root = Some(root);
            }
            TraceCommand::Add { path, node } => self.add(path, node)?,
            TraceCommand::Remove { path } => self.remove(path)?,
            TraceCommand::Replace { path, node } => {
                self.remove(path)?;
                self.add(path, node)?;
            }
            TraceCommand::ReplaceValue {
                path,
                value_type,
                key,
                value,
                old_value,
            } => {
                ensure_attribute_type(value_type)?;
                self.update_attribute(path, key, value.as_ref(), old_value.as_ref())?;
            }
            TraceCommand::InsertValue {
                path,
                value_type,
                key,
                value,
            } => {
                ensure_attribute_type(value_type)?;
                self.update_attribute(path, key, value.as_ref(), None)?;
            }
            TraceCommand::DeleteValue {
                path,
                value_type,
                key,
                old_value,
            } => {
                ensure_attribute_type(value_type)?;
                self.update_attribute(path, key, None, old_value.as_ref())?;
            }
            TraceCommand::Recalculate => {}
        }
        self.dom.refresh_pseudos();
        self.recompute();
        Ok(())
    }

    fn add(&mut self, path: &[usize], node: &Node) -> Result<(), RunError> {
        let (&position, parent_path) = path
            .split_last()
            .ok_or_else(|| RunError::new("cannot add a second root".into()))?;
        let parent = self.dom.id_at_path(parent_path)?;
        let new_id = self.dom.build_subtree(node, Some(parent), &self.program)?;
        let children = &mut self.dom.nodes.get_mut(&parent).unwrap().children;
        if position > children.len() {
            self.dom.remove_subtree(new_id);
            return Err(RunError::new(format!("invalid insertion path {path:?}")));
        }
        children.insert(position, new_id);
        let following = children[position + 1..].to_vec();
        self.dom.mark(new_id, Dirty::NodeChanged);
        for sibling in following {
            self.dom.mark(sibling, Dirty::NodeChanged);
        }
        Ok(())
    }

    fn remove(&mut self, path: &[usize]) -> Result<(), RunError> {
        let (&position, parent_path) = path
            .split_last()
            .ok_or_else(|| RunError::new("cannot remove the root".into()))?;
        let parent = self.dom.id_at_path(parent_path)?;
        let removed = {
            let children = &mut self.dom.nodes.get_mut(&parent).unwrap().children;
            if position >= children.len() {
                return Err(RunError::new(format!("invalid removal path {path:?}")));
            }
            children.remove(position)
        };
        let owns_arena_node = self
            .dom
            .nodes
            .get(&removed)
            .is_none_or(|node| node.parent == Some(parent));
        if owns_arena_node {
            self.dom.remove_subtree(removed);
        }
        let following = self.dom.nodes[&parent].children[position..].to_vec();
        for sibling in following {
            self.dom.mark(sibling, Dirty::NodeChanged);
        }
        Ok(())
    }

    fn update_attribute(
        &mut self,
        path: &[usize],
        key: &str,
        value: Option<&serde_json::Value>,
        expected_old: Option<&serde_json::Value>,
    ) -> Result<(), RunError> {
        let id = self.dom.id_at_path(path)?;
        let key = key.to_ascii_lowercase();
        let old = self.dom.nodes[&id].facts.attributes.get(&key).cloned();
        let old_output = self.dom.nodes[&id].state.output.clone();
        let old_matches = self.dom.nodes[&id].state.matches.clone();
        if let Some(expected) = expected_old {
            let expected = scalar_attribute(expected)?;
            if old.as_deref().unwrap_or_default() != expected {
                return Err(RunError::new(format!(
                    "attribute `{key}` mismatch at {path:?}: expected `{expected}`, got `{}`",
                    old.unwrap_or_default()
                )));
            }
        }
        let value = value.map(scalar_attribute).transpose()?;
        let node = self.dom.nodes.get_mut(&id).unwrap();
        match key.as_str() {
            "class" => {
                node.facts.classes = value
                    .as_deref()
                    .map(|value| value.split_whitespace().map(str::to_string).collect())
                    .unwrap_or_default();
            }
            "id" => node.facts.html_id = value.clone(),
            "is_hover_root" | "is_hovered_root" => {
                node.facts.hover_root = value
                    .as_deref()
                    .is_some_and(|value| value.eq_ignore_ascii_case("true"));
            }
            "is_focus_root" => {
                node.facts.focus_root = value
                    .as_deref()
                    .is_some_and(|value| value.eq_ignore_ascii_case("true"));
            }
            _ => {}
        }
        if let Some(value) = value {
            node.facts.attributes.insert(key.clone(), value);
        } else {
            node.facts.attributes.remove(&key);
        }

        let pseudo_event = matches!(
            key.as_str(),
            "is_hover_root" | "is_hovered_root" | "is_focus_root"
        );
        if !pseudo_event && self.dom.nodes[&id].state.dirty == Dirty::Clean {
            let parent_input = self.dom.nodes[&id]
                .parent
                .and_then(|parent| self.dom.nodes.get(&parent))
                .map(|parent| parent.state.output.clone())
                .unwrap_or_else(|| vec![false; self.program.state_count]);
            let sibling_input = self
                .dom
                .previous_sibling_output(id, self.program.state_count);
            let evaluation = self.evaluate(id, &parent_input, &sibling_input);
            if evaluation.output == old_output && evaluation.matches == old_matches {
                let state = &mut self.dom.nodes.get_mut(&id).unwrap().state;
                state.require_parent = evaluation.require_parent;
                state.require_sibling = evaluation.require_sibling;
                state.parent_input = parent_input;
                state.sibling_input = sibling_input;
                return Ok(());
            }
        }
        self.dom.mark(id, Dirty::NodeChanged);
        Ok(())
    }

    fn recompute(&mut self) {
        if let Some(root) = self.dom.root {
            self.recompute_node(root);
        }
    }

    fn recompute_node(&mut self, id: NodeId) {
        if !self
            .dom
            .nodes
            .get(&id)
            .is_some_and(|node| node.state.subtree_dirty)
        {
            return;
        }
        self.stats.visited_nodes += 1;
        let dirty = self.dom.nodes[&id].state.dirty;
        if dirty != Dirty::Clean {
            let parent_input = self.dom.nodes[&id]
                .parent
                .map(|parent| self.dom.nodes[&parent].state.output.clone())
                .unwrap_or_else(|| vec![false; self.program.state_count]);
            let sibling_input = self
                .dom
                .previous_sibling_output(id, self.program.state_count);
            let can_skip = dirty == Dirty::InputChanged
                && self.kind.uses_dependencies()
                && requirements_accept(&self.dom.nodes[&id].state.require_parent, &parent_input)
                && requirements_accept(&self.dom.nodes[&id].state.require_sibling, &sibling_input);
            if dirty == Dirty::InputChanged {
                self.stats.input_changes += 1;
            }
            if can_skip {
                self.stats.input_skips += 1;
                let state = &mut self.dom.nodes.get_mut(&id).unwrap().state;
                state.parent_input = parent_input;
                state.sibling_input = sibling_input;
                state.dirty = Dirty::Clean;
            } else {
                self.stats.recomputed_nodes += 1;
                let evaluation = self.evaluate(id, &parent_input, &sibling_input);
                let old_output = self.dom.nodes[&id].state.output.clone();
                {
                    let state = &mut self.dom.nodes.get_mut(&id).unwrap().state;
                    state.output = evaluation.output;
                    state.matches = evaluation.matches;
                    state.require_parent = evaluation.require_parent;
                    state.require_sibling = evaluation.require_sibling;
                    state.parent_input = parent_input;
                    state.sibling_input = sibling_input;
                    state.dirty = Dirty::Clean;
                }
                if old_output != self.dom.nodes[&id].state.output {
                    let new_output = self.dom.nodes[&id].state.output.clone();
                    let children = self.dom.nodes[&id].children.clone();
                    for child in children {
                        let recursive_skip = self.kind == EngineKind::RecursiveTri
                            && self.dom.nodes.get(&child).is_some_and(|node| {
                                node.state.dirty == Dirty::Clean
                                    && requirements_accept(&node.state.require_parent, &new_output)
                            });
                        if recursive_skip {
                            self.dom.nodes.get_mut(&child).unwrap().state.parent_input =
                                new_output.clone();
                            self.stats.input_skips += 1;
                        } else {
                            self.dom.mark(child, Dirty::InputChanged);
                        }
                    }
                    if let Some(next) = self.dom.next_sibling(id) {
                        let recursive_skip = self.kind == EngineKind::RecursiveTri
                            && self.dom.nodes.get(&next).is_some_and(|node| {
                                node.state.dirty == Dirty::Clean
                                    && requirements_accept(&node.state.require_sibling, &new_output)
                            });
                        if recursive_skip {
                            self.dom.nodes.get_mut(&next).unwrap().state.sibling_input = new_output;
                            self.stats.input_skips += 1;
                        } else {
                            self.dom.mark(next, Dirty::InputChanged);
                        }
                    }
                }
            }
        }

        let children = self.dom.nodes[&id].children.clone();
        for child in children {
            self.recompute_node(child);
        }
        self.dom.nodes.get_mut(&id).unwrap().state.subtree_dirty = false;
    }

    fn evaluate(&self, id: NodeId, parent: &[bool], sibling: &[bool]) -> Evaluation {
        let mut output = vec![false; self.program.state_count];
        let mut matches = vec![false; self.program.selectors.len()];
        let mut require_parent = vec![Requirement::Unused; self.program.state_count];
        let mut require_sibling = vec![Requirement::Unused; self.program.state_count];

        for (selector_index, selector) in self.program.selectors.iter().enumerate() {
            for part in 0..selector.compounds.len() {
                let state = selector.state(part).0;
                let local_match = self.matches_compound(id, &selector.compounds[part]);
                let raw = if part == 0 {
                    local_match
                } else {
                    let previous = selector.state(part - 1).0;
                    match selector.combinators[part - 1] {
                        Combinator::Descendant | Combinator::Child => {
                            require_parent[previous] = Requirement::from_bit(parent[previous]);
                            local_match && parent[previous]
                        }
                        Combinator::AdjacentSibling => {
                            require_sibling[previous] = Requirement::from_bit(sibling[previous]);
                            local_match && sibling[previous]
                        }
                    }
                };
                let propagate = selector
                    .combinators
                    .get(part)
                    .is_some_and(|combinator| *combinator == Combinator::Descendant);
                let carried = propagate && parent[state];
                if propagate {
                    require_parent[state] = Requirement::from_bit(parent[state]);
                }
                output[state] = raw || carried;
                if part + 1 == selector.compounds.len() {
                    matches[selector_index] = raw;
                }
            }
            if self.kind == EngineKind::Naive {
                matches[selector_index] =
                    self.matches_selector_part(id, selector, selector.compounds.len() - 1);
            }
        }
        Evaluation {
            output,
            matches,
            require_parent,
            require_sibling,
        }
    }

    fn matches_compound(&self, id: NodeId, compound: &Compound) -> bool {
        let node = &self.dom.nodes[&id];
        if compound
            .tag
            .as_ref()
            .is_some_and(|tag| tag != "*" && *tag != node.facts.tag)
        {
            return false;
        }
        if compound
            .id
            .as_ref()
            .is_some_and(|html_id| node.facts.html_id.as_ref() != Some(html_id))
        {
            return false;
        }
        if !compound
            .classes
            .iter()
            .all(|class| node.facts.classes.contains(class))
        {
            return false;
        }
        if !compound.attributes.iter().all(|(name, value)| {
            node.facts
                .attributes
                .get(name)
                .is_some_and(|actual| actual == value)
        }) {
            return false;
        }
        if !compound
            .pseudos
            .iter()
            .all(|pseudo| node.facts.pseudos.contains(pseudo))
        {
            return false;
        }
        let position = self.dom.position(id);
        compound.nth.iter().all(|nth| {
            position.is_some_and(|(all, of_type)| {
                matches_nth(if nth.of_type { of_type } else { all }, *nth)
            })
        })
    }

    fn matches_selector_part(&self, id: NodeId, selector: &SelectorChain, part: usize) -> bool {
        if !self.matches_compound(id, &selector.compounds[part]) {
            return false;
        }
        if part == 0 {
            return true;
        }
        match selector.combinators[part - 1] {
            Combinator::Child => self.dom.nodes[&id]
                .parent
                .is_some_and(|parent| self.matches_selector_part(parent, selector, part - 1)),
            Combinator::AdjacentSibling => self
                .dom
                .previous_sibling(id)
                .is_some_and(|sibling| self.matches_selector_part(sibling, selector, part - 1)),
            Combinator::Descendant => {
                let mut ancestor = self.dom.nodes[&id].parent;
                while let Some(id) = ancestor {
                    if self.matches_selector_part(id, selector, part - 1) {
                        return true;
                    }
                    ancestor = self.dom.nodes[&id].parent;
                }
                false
            }
        }
    }

    fn collect_matches(&self) -> BTreeMap<String, Vec<u64>> {
        let mut result: BTreeMap<String, Vec<u64>> = BTreeMap::new();
        for (id, node) in &self.dom.nodes {
            for (selector_index, matched) in node.state.matches.iter().enumerate() {
                if *matched {
                    result
                        .entry(self.program.selectors[selector_index].text.clone())
                        .or_default()
                        .push(id.0);
                }
            }
        }
        for ids in result.values_mut() {
            ids.sort_unstable();
        }
        result
    }
}

struct Evaluation {
    output: Vec<bool>,
    matches: Vec<bool>,
    require_parent: Vec<Requirement>,
    require_sibling: Vec<Requirement>,
}

fn requirements_accept(requirements: &[Requirement], input: &[bool]) -> bool {
    requirements
        .iter()
        .copied()
        .zip(input.iter().copied())
        .all(|(requirement, bit)| requirement.accepts(bit))
}

fn matches_nth(position: usize, nth: Nth) -> bool {
    let position = position as i64;
    let a = i64::from(nth.a);
    let b = i64::from(nth.b);
    if a == 0 {
        return position == b;
    }
    let delta = position - b;
    delta % a == 0 && delta / a >= 0
}

fn ensure_attribute_type(value_type: &Option<String>) -> Result<(), RunError> {
    if value_type.as_deref() == Some("attributes") {
        Ok(())
    } else {
        Err(RunError::new(format!(
            "unsupported value type {:?}",
            value_type
        )))
    }
}

fn scalar_attribute(value: &serde_json::Value) -> Result<String, RunError> {
    match value {
        serde_json::Value::String(value) => Ok(value.clone()),
        serde_json::Value::Number(value) => Ok(value.to_string()),
        serde_json::Value::Bool(value) => Ok(value.to_string()),
        serde_json::Value::Null => Ok(String::new()),
        serde_json::Value::Array(_) | serde_json::Value::Object(_) => Err(RunError::new(
            "attribute values must be JSON scalars".into(),
        )),
    }
}

fn count_node_match_changes(
    previous: &BTreeMap<String, Vec<u64>>,
    current: &BTreeMap<String, Vec<u64>>,
) -> usize {
    let previous_by_node = matches_by_node(previous);
    let current_by_node = matches_by_node(current);
    previous_by_node
        .keys()
        .chain(current_by_node.keys())
        .copied()
        .collect::<HashSet<_>>()
        .into_iter()
        .filter(|id| previous_by_node.get(id) != current_by_node.get(id))
        .count()
}

fn matches_by_node(matches: &BTreeMap<String, Vec<u64>>) -> HashMap<u64, Vec<&str>> {
    let mut result: HashMap<u64, Vec<&str>> = HashMap::new();
    for (selector, ids) in matches {
        for id in ids {
            result.entry(*id).or_default().push(selector);
        }
    }
    for selectors in result.values_mut() {
        selectors.sort_unstable();
    }
    result
}

#[derive(Debug, Clone)]
pub struct SiteInput {
    pub name: String,
    pub css_path: PathBuf,
    pub trace_path: PathBuf,
}

impl SiteInput {
    pub fn named(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            css_path: PathBuf::from(format!("css-gen-op/{name}/{name}.css")),
            trace_path: PathBuf::from(format!("css-gen-op/{name}/command.json")),
            name,
        }
    }

    pub fn from_environment() -> Result<Self, RunError> {
        std::env::var("WEBSITE_NAME")
            .map(Self::named)
            .map_err(|_| RunError::new("WEBSITE_NAME is not set".into()))
    }
}

pub fn load_site(input: &SiteInput) -> Result<(CompiledProgram, Trace), RunError> {
    let css = fs::read_to_string(&input.css_path).map_err(|error| {
        RunError::new(format!("cannot read {}: {error}", input.css_path.display()))
    })?;
    let program = CompiledProgram::compile(&css)
        .map_err(|error| RunError::new(format!("{}: {error}", input.css_path.display())))?;
    let trace = Trace::parse(&input.trace_path)?;
    Ok((program, trace))
}

pub fn run_site(
    kind: EngineKind,
    input: &SiteInput,
) -> Result<(CompiledProgram, RunResult), RunError> {
    let (program, trace) = load_site(input)?;
    let result = Engine::new(kind, program.clone()).run(&trace)?;
    Ok((program, result))
}

pub fn run_site_with_frame_stats(
    kind: EngineKind,
    input: &SiteInput,
) -> Result<(CompiledProgram, RunResult), RunError> {
    let (program, trace) = load_site(input)?;
    let result = Engine::new(kind, program.clone())
        .with_frame_stats(true)
        .run(&trace)?;
    Ok((program, result))
}

pub fn report_selectors(label: &str, report: &SelectorReport) {
    if report.skipped_simple.is_empty() {
        println!("NOT_CONSIDERED[{label}] none");
    } else {
        println!(
            "NOT_CONSIDERED[{label}] {} selector(s)",
            report.skipped_simple.len()
        );
        for selector in &report.skipped_simple {
            println!("NOT_CONSIDERED[{label}] {selector}");
        }
    }
    if report.unsupported_pseudos.is_empty() {
        println!("PSEUDO_SKIPPED[{label}] none");
    } else {
        for (pseudo, selectors) in &report.unsupported_pseudos {
            println!(
                "PSEUDO_SKIPPED[{label}] {pseudo} -> {} selector(s)",
                selectors.len()
            );
            for selector in selectors.iter().take(5) {
                println!("PSEUDO_SKIPPED[{label}]    eg {selector}");
            }
        }
    }
    if report.unsupported.is_empty() {
        println!("UNSUPPORTED[{label}] none");
    } else {
        println!(
            "UNSUPPORTED[{label}] {} selector(s)",
            report.unsupported.len()
        );
        for selector in report.unsupported.iter().take(25) {
            println!(
                "UNSUPPORTED[{label}] {} ({})",
                selector.selector, selector.reason
            );
        }
    }
}

pub fn binary_main(kind: EngineKind, dot_name: Option<&str>) -> Result<(), Box<dyn Error>> {
    let input = SiteInput::from_environment()?;
    let log_frame_stats = env_flag("TRI_LOG_MATCH_DELTAS");
    let (program, result) = if log_frame_stats {
        run_site_with_frame_stats(kind, &input)?
    } else {
        run_site(kind, &input)?
    };
    report_selectors(kind.label(), &program.report);
    if let Some(dot_name) = dot_name.filter(|_| !env_flag("CSS_BV_NO_DOT")) {
        fs::write(
            format!("css-gen-op/{}/{dot_name}", input.name),
            program.to_dot(),
        )?;
    }
    if log_frame_stats {
        for frame in &result.frames {
            println!(
                "[{}-match] frame_id={} command={} miss_delta={} node_match_changes={} total_misses={}",
                kind.label().replace('_', "-"),
                frame.frame_id,
                frame.command,
                frame.miss_delta,
                frame.node_match_changes,
                frame.total_misses
            );
        }
    }
    println!("BEGIN");
    for (selector, ids) in &result.matches {
        println!("{selector} -> {ids:?}");
    }
    println!("END");
    if kind != EngineKind::Naive {
        eprintln!("unsafe {{ MISS_CNT }} = {}", result.stats.recomputed_nodes);
    }
    if kind.uses_dependencies() {
        eprintln!(
            "unsafe {{ INPUT_CHANGE_COUNT }} = {}",
            result.stats.input_changes
        );
        eprintln!(
            "unsafe {{ INPUT_SKIP_COUNT }} = {}",
            result.stats.input_skips
        );
    }
    Ok(())
}

pub fn env_flag(name: &str) -> bool {
    std::env::var(name).is_ok_and(|value| {
        matches!(
            value.to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(id: u64, tag: &str, class: Option<&str>, children: Vec<Node>) -> Node {
        let mut attributes = HashMap::new();
        if let Some(class) = class {
            attributes.insert("class".into(), serde_json::Value::String(class.into()));
        }
        Node {
            id,
            tag_name: tag.into(),
            node_type: Some("element".into()),
            attributes,
            children,
            extra: HashMap::new(),
        }
    }

    fn compile_rule(rule: &str) -> CompiledProgram {
        CompiledProgram::compile(&format!("{rule} {{ color: red }}")).unwrap()
    }

    #[test]
    fn compiles_chain_in_dom_order() {
        let program = compile_rule("div > .item + span:hover");
        let selector = &program.selectors[0];
        assert_eq!(selector.compounds[0].tag.as_deref(), Some("div"));
        assert_eq!(selector.compounds[1].classes, ["item"]);
        assert_eq!(selector.compounds[2].tag.as_deref(), Some("span"));
        assert_eq!(
            selector.combinators,
            [Combinator::Child, Combinator::AdjacentSibling]
        );
    }

    #[test]
    fn supports_forward_nth_and_rejects_last() {
        let program =
            CompiledProgram::compile("li:first-child {} li:nth-child(2n+1) {} li:last-child {}")
                .unwrap();
        assert_eq!(program.selectors.len(), 2);
        assert!(
            program
                .report
                .unsupported
                .iter()
                .any(|item| item.selector.contains("last-child"))
        );
    }

    #[test]
    fn nth_formula_handles_positive_and_negative_steps() {
        assert!(matches_nth(
            5,
            Nth {
                a: 2,
                b: 1,
                of_type: false
            }
        ));
        assert!(!matches_nth(
            4,
            Nth {
                a: 2,
                b: 1,
                of_type: false
            }
        ));
        assert!(matches_nth(
            2,
            Nth {
                a: -1,
                b: 3,
                of_type: false
            }
        ));
        assert!(!matches_nth(
            4,
            Nth {
                a: -1,
                b: 3,
                of_type: false
            }
        ));
    }

    #[test]
    fn hover_aliases_are_equivalent() {
        let first = facts_from_parts(
            "div",
            HashMap::from([("is_hover_root".into(), "true".into())]),
        );
        let second = facts_from_parts(
            "div",
            HashMap::from([("is_hovered_root".into(), "true".into())]),
        );
        assert!(first.hover_root);
        assert!(second.hover_root);
    }

    #[test]
    fn engines_agree_on_sibling_and_nth_invalidation() {
        let program = compile_rule(".lead:first-child + .target");
        let trace = Trace {
            frames: vec![
                TraceFrame {
                    frame_id: 0,
                    command: TraceCommand::Init {
                        node: node(
                            1,
                            "main",
                            None,
                            vec![
                                node(2, "div", Some("lead"), vec![]),
                                node(3, "div", Some("target"), vec![]),
                            ],
                        ),
                    },
                },
                TraceFrame {
                    frame_id: 1,
                    command: TraceCommand::Add {
                        path: vec![0],
                        node: node(4, "div", Some("prefix"), vec![]),
                    },
                },
            ],
        };
        let naive = Engine::new(EngineKind::Naive, program.clone())
            .run(&trace)
            .unwrap();
        let mut tri_stats = None;
        for kind in [EngineKind::Bit, EngineKind::Tri, EngineKind::RecursiveTri] {
            let result = Engine::new(kind, program.clone()).run(&trace).unwrap();
            assert_eq!(result.matches, naive.matches, "engine {kind:?}");
            if kind == EngineKind::Tri {
                tri_stats = Some(result.stats);
            } else if kind == EngineKind::RecursiveTri {
                let tri = tri_stats.as_ref().unwrap();
                assert!(result.stats.visited_nodes <= tri.visited_nodes);
                assert!(result.stats.recomputed_nodes <= tri.recomputed_nodes);
            }
        }
        assert!(naive.matches.is_empty());
    }

    #[test]
    fn checked_in_testcase_has_engine_parity() {
        let input = SiteInput::named("testcase");
        let (program, trace) = load_site(&input).unwrap();
        let naive = Engine::new(EngineKind::Naive, program.clone())
            .run(&trace)
            .unwrap();
        for kind in [EngineKind::Bit, EngineKind::Tri, EngineKind::RecursiveTri] {
            let result = Engine::new(kind, program.clone()).run(&trace).unwrap();
            assert_eq!(result.matches, naive.matches, "engine {kind:?}");
        }
    }

    #[test]
    #[ignore = "full corpus regression; run explicitly before release"]
    fn checked_in_corpus_has_engine_parity() {
        for site in [
            "a_to_b",
            "amazon",
            "bilibili",
            "bing",
            "bootstrap",
            "google",
            "netflix",
            "testcase",
            "tiktok",
            "whatsapp",
            "wikipedia",
            "yahoo",
            "youtube",
        ] {
            eprintln!("checking corpus site {site}");
            let (program, trace) = load_site(&SiteInput::named(site)).unwrap();
            let naive = Engine::new(EngineKind::Naive, program.clone())
                .run(&trace)
                .unwrap();
            for kind in [EngineKind::Bit, EngineKind::Tri, EngineKind::RecursiveTri] {
                let result = Engine::new(kind, program.clone()).run(&trace).unwrap();
                assert_eq!(
                    result.matches, naive.matches,
                    "site {site}, engine {kind:?}"
                );
            }
        }
    }
}
