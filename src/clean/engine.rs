use super::{
    Combinator, CompiledProgram, Compound, DynamicPseudo, EngineKind, NodeId, Nth, RunError,
    SelectorChain, Trace, TraceCommand, TraceFrame,
};
use crate::{Node, attributes_to_string_map, rdtsc};
use std::collections::{BTreeMap, HashMap, HashSet};

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
pub(crate) struct NodeFacts {
    tag: String,
    classes: HashSet<String>,
    html_id: Option<String>,
    attributes: HashMap<String, String>,
    pub(crate) hover_root: bool,
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

pub(crate) fn facts_from_parts(tag: &str, mut attributes: HashMap<String, String>) -> NodeFacts {
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
    pub cycles: u64,
}

#[derive(Debug, Clone)]
pub struct RunResult {
    pub matches: BTreeMap<String, Vec<u64>>,
    pub stats: RunStats,
}

#[derive(Debug, Clone)]
pub struct Engine {
    kind: EngineKind,
    program: CompiledProgram,
    dom: Dom,
    stats: RunStats,
}

impl Engine {
    pub fn new(kind: EngineKind, program: CompiledProgram) -> Self {
        Self {
            kind,
            program,
            dom: Dom::default(),
            stats: RunStats::default(),
        }
    }

    pub fn run(mut self, trace: &Trace) -> Result<RunResult, RunError> {
        let start = rdtsc();
        for frame in &trace.frames {
            self.apply(frame)
                .map_err(|error| RunError::at(frame.frame_id, error.message))?;
        }
        self.stats.cycles = rdtsc().wrapping_sub(start);
        Ok(RunResult {
            matches: self.collect_matches(),
            stats: self.stats,
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

pub(crate) fn matches_nth(position: usize, nth: Nth) -> bool {
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
