use css_bitvector_compiler::{
    AddNode, CompoundSelector, LayoutFrame, NFA, Nfacell, PSEUDO_CLASS_FOCUS,
    PSEUDO_CLASS_FOCUS_ROOT, PSEUDO_CLASS_FOCUS_WITHIN, PSEUDO_CLASS_HOVER, Rule, Selector,
    SelectorId, SelectorManager, derive_hover_state, extract_pseudoclasses, generate_nfa,
    parse_css_with_pseudo, parse_trace, partition_simple_selectors, report_pseudo_selectors,
    report_skipped_selectors,
    runtime_shared::{
        HasNodes, HasSelectorManager, NodeAttributes, apply_frame_common, update_attribute_common,
    },
};
use std::{
    collections::{HashMap, HashSet},
    fs,
    sync::OnceLock,
};
static mut MISS_CNT: usize = 0;
static mut INPUT_CHANGE_COUNT: usize = 0;
static mut INPUT_SKIP_COUNT: usize = 0;
static mut STATE: usize = 0; // global state
static DEBUG_MODE: OnceLock<bool> = OnceLock::new();

fn env_flag(name: &str) -> bool {
    match std::env::var(name) {
        Ok(value) => {
            let lower = value.to_ascii_lowercase();
            matches!(lower.as_str(), "1" | "true" | "yes" | "on")
        }
        Err(_) => false,
    }
}

fn debug_enabled() -> bool {
    *DEBUG_MODE.get_or_init(|| env_flag("BIT_DEBUG"))
}

fn debug_log<F>(build: F)
where
    F: FnOnce() -> String,
{
    if debug_enabled() {
        eprintln!("[rec-tri-debug] {}", build());
    }
}

fn format_bits(bits: &[bool]) -> String {
    bits.iter()
        .map(|bit| if *bit { '1' } else { '0' })
        .collect()
}

/// whether a part of input is: 1, 0, or unused
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IState {
    IOne,
    IZero,
    IUnused,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OState {
    OOne,
    OZero,
    OFromParent(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DirtyState {
    #[default]
    Clean,
    InputChanged,
    NodeChanged,
}

impl DirtyState {
    fn label(self) -> &'static str {
        match self {
            DirtyState::Clean => "clean",
            DirtyState::InputChanged => "input_changed",
            DirtyState::NodeChanged => "node_changed",
        }
    }
}

#[derive(Debug, Default)]
pub struct DOMNode {
    pub tag_id: SelectorId,                       // Tag selector ID
    pub class_ids: HashSet<SelectorId>,           // Collection of CSS class selector IDs
    pub id_selector_id: Option<SelectorId>,       // HTML ID selector ID
    pub attributes: HashMap<String, String>,      // Node attribute key-value pairs (lowercase keys)
    pub pseudo_classes: HashSet<String>,          // Original pseudo-class set
    pub computed_pseudo_classes: HashSet<String>, // Computed pseudo-class states
    pub parent: Option<u64>,                      // Index of the parent node in the arena
    pub children: Vec<u64>,                       // Indices of child nodes in the arena
    pub dirty: DirtyState,
    pub recursive_dirty: bool,
    pub output_bits: Vec<bool>,
    pub quad_output: Vec<OState>,
    pub parent_dependencies: Vec<Vec<usize>>,
    pub tri_state: Vec<IState>,
}

fn format_tri_state(tri: &[IState]) -> String {
    tri.iter()
        .map(|state| match state {
            IState::IOne => '1',
            IState::IZero => '0',
            IState::IUnused => '_',
        })
        .collect()
}

fn format_output_state(states: &[OState]) -> String {
    states
        .iter()
        .map(|state| match state {
            OState::OOne => "1".to_string(),
            OState::OZero => "0".to_string(),
            OState::OFromParent(idx) => format!("P{}", idx),
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn ensure_needed_outputs_stable(
    node_descriptor: &str,
    needed_outputs: &[bool],
    previous_bits: &[bool],
    new_bits: &[bool],
    previous_quad: &[OState],
    new_quad: &[OState],
) {
    for (idx, needed) in needed_outputs.iter().copied().enumerate() {
        if !needed {
            continue;
        }
        if previous_bits[idx] != new_bits[idx] {
            panic!(
                "{} needed output[{}] changed despite tri reuse (prev={} new={})",
                node_descriptor, idx, previous_bits[idx], new_bits[idx]
            );
        }
        if previous_quad[idx] != new_quad[idx] {
            panic!(
                "{} needed quad state[{}] changed despite tri reuse (prev={} new={})",
                node_descriptor,
                idx,
                format_output_state(&[previous_quad[idx]]),
                format_output_state(&[new_quad[idx]])
            );
        }
    }
}

impl DOMNode {
    fn mark_node_changed(&mut self) {
        self.dirty = DirtyState::NodeChanged;
        self.recursive_dirty = true;
    }

    fn mark_input_changed(&mut self) {
        if self.dirty != DirtyState::NodeChanged {
            self.dirty = DirtyState::InputChanged;
        }
        self.recursive_dirty = true;
    }

    fn clear_dirty(&mut self) {
        self.dirty = DirtyState::Clean;
        self.recursive_dirty = false;
    }
}

#[derive(Debug, Default)]
pub struct DOM {
    pub nodes: HashMap<u64, DOMNode>, // Arena storage for all nodes
    pub selector_manager: SelectorManager,
    root_node: Option<u64>,
}

impl NodeAttributes for DOMNode {
    fn attributes(&mut self) -> &mut HashMap<String, String> {
        &mut self.attributes
    }
    fn class_ids(&mut self) -> &mut HashSet<SelectorId> {
        &mut self.class_ids
    }
    fn id_selector_id(&mut self) -> &mut Option<SelectorId> {
        &mut self.id_selector_id
    }
    fn pseudo_classes(&mut self) -> &mut HashSet<String> {
        &mut self.pseudo_classes
    }
}

impl HasSelectorManager for DOM {
    fn selector_manager(&mut self) -> &mut SelectorManager {
        &mut self.selector_manager
    }
}

impl HasNodes<DOMNode> for DOM {
    fn nodes_mut(&mut self) -> &mut HashMap<u64, DOMNode> {
        &mut self.nodes
    }
}

impl AddNode for DOM {
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
    ) -> u64 {
        let sm = &mut self.selector_manager;
        let tag_id = sm.get_or_create_id(Selector::Type(tag_name.to_lowercase()));

        let mut class_ids = HashSet::new();
        for class in &classes {
            let class_id = sm.get_or_create_id(Selector::Class(class.clone()));
            class_ids.insert(class_id);
        }
        let id_selector_id = html_id
            .as_ref()
            .map(|id| sm.get_or_create_id(Selector::Id(id.clone())));

        let parent_hover_active = parent_index
            .and_then(|pid| self.nodes.get(&pid))
            .map(|parent| parent.computed_pseudo_classes.contains(PSEUDO_CLASS_HOVER))
            .unwrap_or(false);

        let mut computed_pseudo_classes = HashSet::new();
        if derive_hover_state(&pseudo_classes, parent_hover_active) {
            computed_pseudo_classes.insert(PSEUDO_CLASS_HOVER.to_string());
        }

        let mut new_node = DOMNode {
            tag_id,
            class_ids,
            id_selector_id,
            attributes,
            pseudo_classes,
            computed_pseudo_classes,
            parent: parent_index,
            children: Vec::new(),
            dirty: DirtyState::NodeChanged,
            recursive_dirty: true,
            output_bits: vec![false; unsafe { STATE } + 1],
            quad_output: vec![OState::OZero; unsafe { STATE } + 1],
            parent_dependencies: vec![Vec::new(); unsafe { STATE } + 1],
            tri_state: vec![IState::IUnused; unsafe { STATE } + 1],
        };
        let (output_bits, quad_output, dependencies) = self.new_output_state(&new_node, &get_input(), nfa);
        new_node.output_bits = output_bits;
        new_node.quad_output = quad_output;
        new_node.parent_dependencies = dependencies;
        self.nodes.insert(id, new_node);

        // Add the current node as a child of its parent if one exists
        if let Some(p_idx) = parent_index {
            self.nodes
                .get_mut(&p_idx)
                .unwrap_or_else(|| panic!("{p_idx} not found"))
                .children
                .push(id);
        }
        id
    }
}

impl DOM {
    pub fn new() -> Self {
        Default::default()
    }

    fn describe_node(&self, node_idx: u64) -> String {
        if let Some(node) = self.nodes.get(&node_idx) {
            let tag = self
                .selector_manager
                .id_to_selector
                .get(&node.tag_id)
                .map(|selector| selector.to_string())
                .unwrap_or_else(|| format!("sid:{}", node.tag_id.0));
            let mut class_parts = node
                .class_ids
                .iter()
                .filter_map(|cid| {
                    self.selector_manager
                        .id_to_selector
                        .get(cid)
                        .map(|selector| selector.to_string())
                })
                .collect::<Vec<_>>();
            class_parts.sort();
            let id_part = node
                .id_selector_id
                .and_then(|sid| {
                    self.selector_manager
                        .id_to_selector
                        .get(&sid)
                        .map(|selector| selector.to_string())
                })
                .unwrap_or_default();
            let mut descriptor = format!("<{}", tag);
            if !id_part.is_empty() {
                descriptor.push(' ');
                descriptor.push_str(&id_part);
            }
            if !class_parts.is_empty() {
                descriptor.push(' ');
                descriptor.push_str(&class_parts.join(""));
            }
            descriptor.push('>');
            format!("node {} {}", node_idx, descriptor)
        } else {
            format!("node {} <unknown>", node_idx)
        }
    }
    /// Check whether a node matches the given selector ID.
    pub fn node_matches_selector(&self, node: &DOMNode, selector_id: SelectorId) -> bool {
        match self.selector_manager.id_to_selector.get(&selector_id) {
            Some(Selector::Type(_)) => node.tag_id == selector_id,
            Some(Selector::Class(_)) => node.class_ids.contains(&selector_id),
            Some(Selector::Id(_)) => node.id_selector_id == Some(selector_id),
            Some(Selector::AttributeEquals { name, value }) => node
                .attributes
                .get(name)
                .map(|v| v == value)
                .unwrap_or(false),
            Some(Selector::Compound(compound)) => self.node_matches_compound(node, compound),
            None => false,
        }
    }

    fn node_matches_compound(&self, node: &DOMNode, compound: &CompoundSelector) -> bool {
        if let Some(tag) = &compound.tag {
            if !self.node_has_tag(node, tag) {
                return false;
            }
        }
        if let Some(id_value) = &compound.id {
            if !self.node_has_id(node, id_value) {
                return false;
            }
        }
        for class_name in &compound.classes {
            if !self.node_has_class(node, class_name) {
                return false;
            }
        }
        for (name, value) in &compound.attributes {
            if !node
                .attributes
                .get(name)
                .map(|v| v == value)
                .unwrap_or(false)
            {
                return false;
            }
        }
        for pseudo in &compound.pseudos {
            if !node.computed_pseudo_classes.contains(pseudo) {
                return false;
            }
        }
        true
    }

    fn node_has_class(&self, node: &DOMNode, class_name: &str) -> bool {
        let selector = Selector::Class(class_name.to_string());
        match self.selector_manager.get_id(&selector) {
            Some(class_id) => node.class_ids.contains(&class_id),
            None => false,
        }
    }

    fn node_has_id(&self, node: &DOMNode, id_value: &str) -> bool {
        let selector = Selector::Id(id_value.to_string());
        match self.selector_manager.get_id(&selector) {
            Some(id) => node.id_selector_id == Some(id),
            None => false,
        }
    }

    fn node_has_tag(&self, node: &DOMNode, tag_name: &str) -> bool {
        if tag_name == "*" {
            return true;
        }
        let selector = Selector::Type(tag_name.to_string());
        match self.selector_manager.get_id(&selector) {
            Some(tag_id) => node.tag_id == tag_id,
            None => false,
        }
    }
    pub fn get_root_node(&mut self) -> u64 {
        if let Some(r) = self.root_node {
            if self.nodes.contains_key(&r) {
                return r;
            }
            self.root_node = None;
        }
        let root = self
            .nodes
            .iter()
            .find(|(_, node)| node.parent.is_none())
            .map(|(idx, _)| *idx)
            .unwrap_or_else(|| panic!("DOM has no root node"));
        self.root_node = Some(root);
        root
    }

    /// Mark the specified node dirty and propagate the recursive_dirty flag upward.
    pub fn set_node_dirty(&mut self, node_idx: u64) {
        let parent_idx = match self.nodes.get_mut(&node_idx) {
            Some(node) => {
                node.mark_node_changed();
                node.parent
            }
            None => return,
        };
        self.propagate_recursive_dirty(parent_idx);
    }

    fn propagate_recursive_dirty(&mut self, mut current_idx: Option<u64>) {
        while let Some(parent_idx) = current_idx {
            let parent_node = match self.nodes.get_mut(&parent_idx) {
                Some(node) => node,
                None => break,
            };

            if parent_node.recursive_dirty {
                break;
            }
            parent_node.recursive_dirty = true;
            current_idx = parent_node.parent;
        }
    }

    fn refresh_computed_pseudos(&mut self, node_idx: u64) {
        let (parent_idx, parent_hover) = match self.nodes.get(&node_idx) {
            Some(node) => {
                let parent_hover = node
                    .parent
                    .and_then(|pid| self.nodes.get(&pid))
                    .map(|parent| parent.computed_pseudo_classes.contains(PSEUDO_CLASS_HOVER))
                    .unwrap_or(false);
                (node.parent, parent_hover)
            }
            None => return,
        };

        let mut changed = false;
        if let Some(node) = self.nodes.get_mut(&node_idx) {
            let hover_active = derive_hover_state(&node.pseudo_classes, parent_hover);
            let had_hover = node.computed_pseudo_classes.contains(PSEUDO_CLASS_HOVER);
            if hover_active && !had_hover {
                node.computed_pseudo_classes
                    .insert(PSEUDO_CLASS_HOVER.to_string());
                node.mark_node_changed();
                changed = true;
            } else if !hover_active && had_hover {
                node.computed_pseudo_classes.remove(PSEUDO_CLASS_HOVER);
                node.mark_node_changed();
                changed = true;
            }
        } else {
            return;
        }

        if changed {
            self.propagate_recursive_dirty(parent_idx);
        }
    }

    fn recompute_focus_states(&mut self, node_idx: u64) -> bool {
        let (child_indices, parent_idx, focus_root_active) = match self.nodes.get(&node_idx) {
            Some(node) => {
                let active = node.pseudo_classes.contains(PSEUDO_CLASS_FOCUS_ROOT)
                    || node.pseudo_classes.contains(PSEUDO_CLASS_FOCUS);
                (node.children.clone(), node.parent, active)
            }
            None => return false,
        };

        let mut focus_within_active = focus_root_active;
        for child_idx in child_indices {
            if self.recompute_focus_states(child_idx) {
                focus_within_active = true;
            }
        }

        let mut changed = false;
        if let Some(node) = self.nodes.get_mut(&node_idx) {
            let mut update_flag = |pseudo: &str, active: bool| -> bool {
                let has_flag = node.computed_pseudo_classes.contains(pseudo);
                if active && !has_flag {
                    node.computed_pseudo_classes.insert(pseudo.to_string());
                    true
                } else if !active && has_flag {
                    node.computed_pseudo_classes.remove(pseudo);
                    true
                } else {
                    false
                }
            };

            let focus_changed = update_flag(PSEUDO_CLASS_FOCUS, focus_root_active);
            let focus_within_changed = update_flag(PSEUDO_CLASS_FOCUS_WITHIN, focus_within_active);

            if focus_changed || focus_within_changed {
                node.mark_node_changed();
                changed = true;
            }
        } else {
            return focus_within_active;
        }

        if changed {
            self.propagate_recursive_dirty(parent_idx);
        }

        focus_within_active
    }
    pub fn json_to_html_node(
        &mut self,
        json_node: &serde_json::Value,
        parent_index: Option<u64>,
        nfa: &NFA,
    ) -> u64 {
        let tag_name = json_node["name"].as_str().unwrap();
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
            .collect::<Vec<String>>();
        let pseudo_classes = extract_pseudoclasses(json_node);

        // Create the current node
        let current_index = self.add_node(
            id,
            tag_name,
            classes.clone(),
            html_id,
            attributes,
            pseudo_classes,
            parent_index,
            nfa,
        );
        // HACK
        if id == 5458 && classes.contains(&"hidden".to_string()) {
            panic!()
        }
        //
        // Recursively process child nodes
        if let Some(children_array) = json_node["children"].as_array() {
            for child_json in children_array {
                self.json_to_html_node(child_json, Some(current_index), nfa);
            }
        }
        current_index
    }

    /// Add a node specified by a path.
    pub fn add_node_by_path(&mut self, path: &[usize], json_node: &serde_json::Value, nfa: &NFA) {
        assert!(!path.is_empty());
        let root_node = self.get_root_node();

        let mut current_idx = root_node;

        // Walk the path to the target parent node
        for &path_element in &path[..path.len() - 1] {
            current_idx = self.nodes[&current_idx].children[path_element];
        }

        // Insert the new node at the specified position
        let new_node_idx = self.json_to_html_node(json_node, Some(current_idx), nfa);
        let insert_pos = path[path.len() - 1];
        if let Some(parent) = self.nodes.get_mut(&current_idx) {
            assert_eq!(parent.children.last().copied(), Some(new_node_idx));
            parent.children.pop();
            parent.children.insert(insert_pos, new_node_idx);
        }
        self.set_node_dirty(current_idx);
    }

    /// Remove a node specified by a path.
    pub fn remove_node_by_path(&mut self, path: &[usize]) {
        let root_nodes = self.get_root_node();
        // Descend to the target parent node
        let mut cur_idx = root_nodes;
        for &path_idx in &path[..path.len() - 1] {
            cur_idx = self.nodes[&cur_idx].children[path_idx];
        }

        let rm_pos = path[path.len() - 1];
        let removed_child_id = self
            .nodes
            .get_mut(&cur_idx)
            .unwrap()
            .children
            .remove(rm_pos);
        let should_remove = self
            .nodes
            .get(&removed_child_id)
            .map(|node| node.parent == Some(cur_idx))
            .unwrap_or(true);
        if should_remove {
            self.remove_subtree(removed_child_id);
        }
        self.set_node_dirty(cur_idx);
    }

    fn remove_subtree(&mut self, node_id: u64) {
        if let Some(node) = self.nodes.remove(&node_id) {
            for child in node.children {
                self.remove_subtree(child);
            }
        }
    }
    pub fn node_id_by_path(&mut self, path: &[usize]) -> Option<u64> {
        if self.nodes.is_empty() {
            return None;
        }
        let mut current_idx = self.get_root_node();
        for &segment in path {
            let node = self.nodes.get(&current_idx)?;
            current_idx = *node.children.get(segment)?;
        }
        Some(current_idx)
    }

    fn update_attribute(&mut self, node_idx: u64, key: &str, new_value: Option<String>) {
        update_attribute_common(self, node_idx, key, new_value);
    }
    pub fn recompute_styles(&mut self, nfa: &NFA, input: &[bool]) {
        let root_node = self.get_root_node();
        debug_log(|| {
            format!(
                "recompute start {}; input={}",
                self.describe_node(root_node),
                format_bits(input)
            )
        });
        self.recompute_focus_states(root_node);
        self.recompute_styles_recursive(root_node, nfa, input);
        debug_log(|| format!("recompute done {}", self.describe_node(root_node)));
    }
    fn recompute_styles_recursive(&mut self, node_idx: u64, nfa: &NFA, input: &[bool]) {
        let node_descriptor = self.describe_node(node_idx);
        self.refresh_computed_pseudos(node_idx);
        let (
            was_recursive_dirty,
            dirty_state,
            previous_output_bits,
            previous_quad_output,
            previous_tri,
            child_indices_snapshot,
        ) = match self.nodes.get(&node_idx) {
            Some(node) => (
                node.recursive_dirty,
                node.dirty,
                node.output_bits.clone(),
                node.quad_output.clone(),
                node.tri_state.clone(),
                node.children.clone(),
            ),
            None => {
                debug_log(|| format!("{} missing; skipping recompute", node_descriptor));
                return;
            }
        };

        if !was_recursive_dirty {
            debug_log(|| {
                format!(
                    "{} ignored: recursive_dirty=false, input={}",
                    node_descriptor,
                    format_bits(input)
                )
            });
            return;
        }

        debug_log(|| {
            format!(
                "{} visit: dirty={} input={} cached_output={} cached_quad={} tri={}",
                node_descriptor,
                dirty_state.label(),
                format_bits(input),
                format_bits(&previous_output_bits),
                format_output_state(&previous_quad_output),
                format_tri_state(&previous_tri)
            )
        });

        let mut should_mark_children = false;
        match dirty_state {
            DirtyState::Clean => {
                debug_log(|| format!("{} clean validation start", node_descriptor));
                let (new_output_bits, new_quad_output, _new_dependencies) =
                    match self.nodes.get(&node_idx) {
                    Some(node) => self.new_output_state(node, input, nfa),
                    None => {
                        debug_log(|| {
                            format!(
                                "{} vanished during clean validation; skipping",
                                node_descriptor
                            )
                        });
                        return;
                    }
                };
                debug_log(|| {
                    format!(
                        "{} validation -> output={} quad={}",
                        node_descriptor,
                        format_bits(&new_output_bits),
                        format_output_state(&new_quad_output)
                    )
                });
                let needed_outputs = self.compute_needed_outputs(node_idx, nfa);
                ensure_needed_outputs_stable(
                    &node_descriptor,
                    &needed_outputs,
                    &previous_output_bits,
                    &new_output_bits,
                    &previous_quad_output,
                    &new_quad_output,
                );
            }
            DirtyState::InputChanged => {
                let need_re = !input.iter().copied().zip(previous_tri.iter().copied()).all(
                    |(input_bit, state)| {
                        matches!(
                            (input_bit, state),
                            (false, IState::IZero) | (true, IState::IOne) | (_, IState::IUnused)
                        )
                    },
                );
                unsafe {
                    INPUT_CHANGE_COUNT += 1;
                }

                debug_log(|| {
                    format!(
                        "{} input_changed need_recompute={} tri={}",
                        node_descriptor,
                        need_re,
                        format_tri_state(&previous_tri)
                    )
                });

                if need_re {
                    unsafe {
                        MISS_CNT += 1;
                    }
                    let (new_output_state, new_quad_output, new_dependencies) =
                        match self.nodes.get(&node_idx) {
                        Some(node) => self.new_output_state(node, input, nfa),
                        None => {
                            debug_log(|| {
                                format!(
                                    "{} missing before recompute; aborting input_changed branch",
                                    node_descriptor
                                )
                            });
                            return;
                        }
                    };
                    let output_changed = new_output_state != previous_output_bits;
                    if let Some(node) = self.nodes.get_mut(&node_idx) {
                        node.output_bits = new_output_state.clone();
                        node.quad_output = new_quad_output.clone();
                        node.parent_dependencies = new_dependencies.clone();
                    } else {
                        debug_log(|| {
                            format!(
                                "{} missing before storing recompute output; aborting",
                                node_descriptor
                            )
                        });
                        return;
                    }
                    debug_log(|| {
                        format!(
                            "{} recompute -> output={} (prev={}) quad={}",
                            node_descriptor,
                            format_bits(&new_output_state),
                            format_bits(&previous_output_bits),
                            format_output_state(&new_quad_output)
                        )
                    });
                    if output_changed {
                        should_mark_children = true;
                    }
                } else {
                    unsafe {
                        INPUT_SKIP_COUNT += 1;
                    }
                    let (new_output, new_quad, _validation_dependencies) =
                        match self.nodes.get(&node_idx) {
                        Some(node) => self.new_output_state(node, input, nfa),
                        None => {
                            debug_log(|| {
                                format!(
                                    "{} missing before input reuse validation; skipping",
                                    node_descriptor
                                )
                            });
                            return;
                        }
                    };
                    debug_log(|| {
                        format!(
                            "{} input reused; output stays {} tri stays {}",
                            node_descriptor,
                            format_bits(&previous_output_bits),
                            format_tri_state(&previous_tri)
                        )
                    });
                    let needed_outputs = self.compute_needed_outputs(node_idx, nfa);
                    ensure_needed_outputs_stable(
                        &node_descriptor,
                        &needed_outputs,
                        &previous_output_bits,
                        &new_output,
                        &previous_quad_output,
                        &new_quad,
                    );
                }
            }
            DirtyState::NodeChanged => {
                unsafe {
                    MISS_CNT += 1;
                }
                let (new_output_state, new_quad_state, new_dependencies) =
                    match self.nodes.get(&node_idx) {
                    Some(node) => self.new_output_state(node, input, nfa),
                    None => {
                        debug_log(|| {
                            format!(
                                "{} missing before node_changed recompute; skipping",
                                node_descriptor
                            )
                        });
                        return;
                    }
                };
                let output_changed = new_output_state != previous_output_bits;
                debug_log(|| {
                    format!(
                        "{} recompute (node_changed) -> output={} (prev={}) quad={}",
                        node_descriptor,
                        format_bits(&new_output_state),
                        format_bits(&previous_output_bits),
                        format_output_state(&new_quad_state)
                    )
                });
                if let Some(node) = self.nodes.get_mut(&node_idx) {
                    node.output_bits = new_output_state.clone();
                    node.quad_output = new_quad_state.clone();
                    node.parent_dependencies = new_dependencies.clone();
                } else {
                    debug_log(|| {
                        format!(
                            "{} missing before storing node_changed output; aborting",
                            node_descriptor
                        )
                    });
                    return;
                }
                if output_changed {
                    should_mark_children = true;
                }
            }
        }

        if should_mark_children {
            debug_log(|| {
                format!(
                    "{} marking {} children input_changed",
                    node_descriptor,
                    child_indices_snapshot.len()
                )
            });
            let mut marked_children = Vec::new();
            for &child_idx in &child_indices_snapshot {
                if let Some(child) = self.nodes.get_mut(&child_idx) {
                    child.mark_input_changed();
                    let dirty_label = child.dirty.label();
                    marked_children.push((child_idx, dirty_label));
                }
            }
            for (child_idx, dirty_label) in marked_children {
                let child_desc = self.describe_node(child_idx);
                debug_log(|| {
                    format!(
                        "{} child {} marked input_changed -> dirty={}",
                        node_descriptor, child_desc, dirty_label
                    )
                });
            }
        } else {
            debug_log(|| format!("{} children remain clean", node_descriptor));
        }

        let current_output_bits = match self.nodes.get(&node_idx) {
            Some(node) => node.output_bits.clone(),
            None => {
                debug_log(|| {
                    format!(
                        "{} removed before propagating children; aborting traversal",
                        node_descriptor
                    )
                });
                return;
            }
        };
        debug_log(|| {
            format!(
                "{} propagating to {} children",
                node_descriptor,
                child_indices_snapshot.len()
            )
        });
        for &child_idx in &child_indices_snapshot {
            let child_needs_visit = if should_mark_children {
                self.nodes.contains_key(&child_idx)
            } else {
                self.nodes
                    .get(&child_idx)
                    .map(|child| child.recursive_dirty)
                    .unwrap_or(false)
            };
            if child_needs_visit {
                self.recompute_styles_recursive(child_idx, nfa, &current_output_bits);
            }
        }

        let new_tri_state = self.recompute_tri_state(node_idx, input, nfa);
        let tri_changed = new_tri_state != previous_tri;
        if tri_changed {
            debug_log(|| {
                format!(
                    "{} tri updated -> {} (prev={})",
                    node_descriptor,
                    format_tri_state(&new_tri_state),
                    format_tri_state(&previous_tri)
                )
            });
        }
        if let Some(node) = self.nodes.get_mut(&node_idx) {
            node.tri_state = new_tri_state;
            node.clear_dirty();
        }
        debug_log(|| format!("{} finished; dirty flags cleared", node_descriptor));
    }

    fn new_output_state(
        &self,
        node: &DOMNode,
        input: &[bool],
        nfa: &NFA,
    ) -> (Vec<bool>, Vec<OState>, Vec<Vec<usize>>) {
        let mut quad_state = vec![OState::OZero; input.len()];
        let mut parent_dependencies: Vec<Vec<usize>> = vec![Vec::new(); input.len()];
        let mut propagate_rules = Vec::new();

        for &rule in nfa.rules.iter() {
            match rule {
                Rule(None, None, Nfacell(target)) => {
                    quad_state[target] = OState::OOne;
                }
                Rule(Some(selector_id), None, Nfacell(target)) => {
                    if self.node_matches_selector(node, selector_id) {
                        quad_state[target] = OState::OOne;
                    }
                }
                Rule(_, Some(_), _) => {
                    propagate_rules.push(rule);
                }
            }
        }

        for &Rule(selector_opt, parent_opt, Nfacell(target_idx)) in &propagate_rules {
            let Some(Nfacell(parent_idx)) = parent_opt else {
                continue;
            };
            match selector_opt {
                None => {
                    if matches!(quad_state[target_idx], OState::OZero) {
                        if !parent_dependencies[target_idx].contains(&parent_idx) {
                            parent_dependencies[target_idx].push(parent_idx);
                        }
                        if input[parent_idx] {
                            quad_state[target_idx] = OState::OFromParent(parent_idx);
                        } else {
                            quad_state[target_idx] = OState::OZero;
                        }
                    }
                }
                Some(selector_id) => {
                    if self.node_matches_selector(node, selector_id) {
                        if !parent_dependencies[target_idx].contains(&parent_idx) {
                            parent_dependencies[target_idx].push(parent_idx);
                        }
                        if input[parent_idx] {
                            quad_state[target_idx] = OState::OFromParent(parent_idx);
                        } else {
                            quad_state[target_idx] = OState::OZero;
                        }
                    } else if matches!(quad_state[target_idx], OState::OFromParent(_)) {
                        quad_state[target_idx] = OState::OZero;
                    }
                }
            }
        }

        let output_bits = self.materialize(input, &quad_state);
        (output_bits, quad_state, parent_dependencies)
    }

    fn materialize(&self, input: &[bool], output: &[OState]) -> Vec<bool> {
        output
            .iter()
            .map(|state| match state {
                OState::OOne => true,
                OState::OZero => false,
                OState::OFromParent(idx) => input[*idx],
            })
            .collect()
    }

    fn compute_needed_outputs(&self, node_idx: u64, nfa: &NFA) -> Vec<bool> {
        let mut needed = vec![false; unsafe { STATE } + 1];
        for &Nfacell(state_idx) in &nfa.accept_states {
            needed[state_idx] = true;
        }

        if let Some(node) = self.nodes.get(&node_idx) {
            for &child_idx in &node.children {
                if let Some(child) = self.nodes.get(&child_idx) {
                    for (state_idx, usage) in child.tri_state.iter().enumerate() {
                        if !matches!(usage, IState::IUnused) {
                            needed[state_idx] = true;
                        }
                    }
                }
            }
        }

        needed
    }

    fn derive_tri_state(
        &self,
        needed_outputs: &[bool],
        dependencies: &[Vec<usize>],
        parent_input: &[bool],
    ) -> Vec<IState> {
        let mut tri_state = vec![IState::IUnused; parent_input.len()];
        for (state_idx, needed) in needed_outputs.iter().copied().enumerate() {
            if !needed {
                continue;
            }
            if let Some(parent_list) = dependencies.get(state_idx) {
                for &parent_idx in parent_list {
                    if parent_idx >= parent_input.len() {
                        continue;
                    }
                    let value = if parent_input[parent_idx] {
                        IState::IOne
                    } else {
                        IState::IZero
                    };
                    tri_state[parent_idx] = value;
                }
            }
        }
        tri_state
    }

    fn recompute_tri_state(&self, node_idx: u64, parent_input: &[bool], nfa: &NFA) -> Vec<IState> {
        let needed_outputs = self.compute_needed_outputs(node_idx, nfa);
        let node = self
            .nodes
            .get(&node_idx)
            .unwrap_or_else(|| panic!("node {} missing during tri recompute", node_idx));
        self.derive_tri_state(&needed_outputs, &node.parent_dependencies, parent_input)
    }
}

impl css_bitvector_compiler::runtime_shared::FrameDom<DOMNode> for DOM {
    type AttrState = (Vec<bool>, Vec<IState>);
    fn reset_dom(&mut self) {
        self.nodes.clear();
        self.root_node = None;
    }
    fn json_to_html_node(&mut self, node: &serde_json::Value, parent: Option<u64>, nfa: &NFA) {
        self.json_to_html_node(node, parent, nfa);
    }
    fn add_node_by_path(&mut self, path: &[usize], node: &serde_json::Value, nfa: &NFA) {
        self.add_node_by_path(path, node, nfa);
    }
    fn remove_node_by_path(&mut self, path: &[usize]) {
        self.remove_node_by_path(path);
    }
    fn node_id_by_path(&mut self, path: &[usize]) -> Option<u64> {
        self.node_id_by_path(path)
    }
    fn set_node_dirty(&mut self, node_idx: u64) {
        self.set_node_dirty(node_idx);
    }
    fn recompute_styles(&mut self, nfa: &NFA, input: &[bool]) {
        self.recompute_styles(nfa, input);
    }
    fn attr_state_and_parent_input<F>(
        &self,
        node_idx: u64,
        make_root_input: &F,
    ) -> (Self::AttrState, Vec<bool>)
    where
        F: Fn() -> Vec<bool>,
    {
        let node = &self.nodes[&node_idx];
        let parent_bits = node
            .parent
            .and_then(|pid| self.nodes.get(&pid))
            .map(|parent| parent.output_bits.clone())
            .unwrap_or_else(|| make_root_input());
        (
            (node.output_bits.clone(), node.tri_state.clone()),
            parent_bits,
        )
    }
    fn recompute_attr_state(
        &self,
        node_idx: u64,
        parent_bits: &[bool],
        nfa: &NFA,
    ) -> Self::AttrState {
        let node = &self.nodes[&node_idx];
        let (output_bits, _quad_output, dependencies) = self.new_output_state(node, parent_bits, nfa);
        let needed_outputs = self.compute_needed_outputs(node_idx, nfa);
        let tri_state = self.derive_tri_state(&needed_outputs, &dependencies, parent_bits);
        (output_bits, tri_state)
    }
}

fn get_input() -> Vec<bool> {
    vec![false; unsafe { STATE } + 1]
}

fn apply_frame(dom: &mut DOM, frame: &LayoutFrame, nfa: &NFA) {
    let make_input = || get_input();
    let make_recalc_input = |_nfa: &NFA| get_input();
    apply_frame_common(dom, frame, nfa, make_input, make_recalc_input);
}

pub fn collect_rule_matches(
    dom: &DOM,
    nfas: &NFA,
    selects: &[String],
) -> HashMap<String, Vec<u64>> {
    let mut res: HashMap<String, Vec<u64>> = HashMap::new();

    for (node_id, node) in dom.nodes.iter() {
        for (idx, &Nfacell(state_index)) in nfas.accept_states.iter().enumerate() {
            if node.output_bits[state_index] {
                let rule = &selects[idx];
                res.entry(rule.clone()).or_default().push(*node_id);
            }
        }
    }

    for v in res.values_mut() {
        v.sort_unstable();
    }
    res
}

fn matches_grouped_by_node(rule_matches: &HashMap<String, Vec<u64>>) -> HashMap<u64, Vec<String>> {
    let mut by_node: HashMap<u64, Vec<String>> = HashMap::new();
    for (selector, node_ids) in rule_matches {
        for &node_id in node_ids {
            by_node.entry(node_id).or_default().push(selector.clone());
        }
    }
    for selectors in by_node.values_mut() {
        selectors.sort();
    }
    by_node
}

fn count_node_match_changes(
    previous: &HashMap<u64, Vec<String>>,
    current: &HashMap<u64, Vec<String>>,
) -> usize {
    let mut changed = 0;
    for (&node_id, previous_matches) in previous {
        if current.get(&node_id) != Some(previous_matches) {
            changed += 1;
        }
    }
    for (&node_id, current_matches) in current {
        if !previous.contains_key(&node_id) && !current_matches.is_empty() {
            changed += 1;
        }
    }
    changed
}
fn main() {
    let mut dom = DOM::new();
    let website_name = std::env::var("WEBSITE_NAME").unwrap();
    let log_match_deltas = env_flag("TRI_LOG_MATCH_DELTAS");
    let parsed = parse_css_with_pseudo(
        &std::fs::read_to_string(format!("css-gen-op/{0}/{0}.css", website_name)).unwrap(),
    );
    let (selectors, skipped_simple) = partition_simple_selectors(parsed.selectors);
    report_skipped_selectors("rec_tri", &skipped_simple);
    report_pseudo_selectors("rec_tri", &parsed.pseudo_selectors);
    let mut s = unsafe { STATE };
    let nfa = generate_nfa(&selectors, &mut dom.selector_manager, &mut s);
    unsafe {
        STATE = s;
    }
    let _ = fs::write(
        format!("css-gen-op/{0}/dot_rec_tri.dot", website_name),
        nfa.to_dot(&dom.selector_manager),
    );

    let mut prev_node_matches = if log_match_deltas {
        Some(HashMap::<u64, Vec<String>>::new())
    } else {
        None
    };
    let mut cached_rule_matches: Option<HashMap<String, Vec<u64>>> = None;

    for f in parse_trace() {
        let before_miss = if log_match_deltas {
            unsafe { MISS_CNT }
        } else {
            0
        };
        apply_frame(&mut dom, &f, &nfa);
        if log_match_deltas {
            let after_miss = unsafe { MISS_CNT };
            let rule_matches = collect_rule_matches(&dom, &nfa, &selectors);
            let node_matches = matches_grouped_by_node(&rule_matches);
            let changed_nodes = prev_node_matches
                .as_ref()
                .map(|previous| count_node_match_changes(previous, &node_matches))
                .unwrap_or_else(|| node_matches.len());
            println!(
                "[rec-tri-match] frame_id={} command={} miss_delta={} node_match_changes={} total_misses={}",
                f.frame_id,
                f.command_name,
                after_miss - before_miss,
                changed_nodes,
                after_miss
            );
            prev_node_matches = Some(node_matches);
            cached_rule_matches = Some(rule_matches);
        }
    }

    let mut final_matches = if let Some(matches) = cached_rule_matches {
        matches.into_iter().collect::<Vec<_>>()
    } else {
        collect_rule_matches(&dom, &nfa, &selectors)
            .into_iter()
            .collect::<Vec<_>>()
    };
    final_matches.sort();
    println!("BEGIN");
    for (k, v) in final_matches {
        println!("{} -> {:?}", k.replace('>', " > "), v);
    }
    println!("END");
    dbg!(unsafe { MISS_CNT });
    dbg!(unsafe { INPUT_CHANGE_COUNT });
    dbg!(unsafe { INPUT_SKIP_COUNT });
}
