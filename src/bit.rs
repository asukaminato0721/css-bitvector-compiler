use css_bitvector_compiler::{
    AddNode, CompoundSelector, LayoutFrame, NFA, Nfacell, PSEUDO_CLASS_FOCUS,
    PSEUDO_CLASS_FOCUS_ROOT, PSEUDO_CLASS_FOCUS_WITHIN, PSEUDO_CLASS_HOVER, ParsedSelectors, Rule,
    Selector, SelectorId, SelectorManager, derive_hover_state, drain_supported_pseudo_selectors,
    extract_pseudoclasses, generate_nfa, parse_css_with_pseudo, parse_trace,
    partition_simple_selectors, report_pseudo_selectors, report_skipped_selectors,
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
static mut STATE: usize = 0; // global state
static DEBUG_MODE: OnceLock<bool> = OnceLock::new();

fn debug_enabled() -> bool {
    *DEBUG_MODE.get_or_init(|| match std::env::var("BIT_DEBUG") {
        Ok(value) => {
            let lower = value.to_ascii_lowercase();
            matches!(lower.as_str(), "1" | "true" | "yes" | "on")
        }
        Err(_) => false,
    })
}

fn debug_log<F>(build: F)
where
    F: FnOnce() -> String,
{
    if debug_enabled() {
        eprintln!("[bit-debug] {}", build());
    }
}

fn format_bits(bits: &[bool]) -> String {
    bits.iter()
        .map(|bit| if *bit { '1' } else { '0' })
        .collect()
}

#[derive(Debug, Default)]
pub struct DOMNode {
    pub tag_id: SelectorId,                  // Tag selector ID
    pub class_ids: HashSet<SelectorId>,      // Collection of CSS class selector IDs
    pub id_selector_id: Option<SelectorId>,  // HTML ID selector ID
    pub attributes: HashMap<String, String>, // Node attribute key-value pairs (lowercase keys)
    pub pseudo_classes: HashSet<String>,
    pub computed_pseudo_classes: HashSet<String>,
    pub parent: Option<u64>, // Index of the parent node in the arena
    pub children: Vec<u64>,  // Indices of child nodes in the arena
    pub dirty: bool,
    pub recursive_dirty: bool,
    pub output_state: Vec<bool>,
}

impl DOMNode {
    fn set_dirty(&mut self) {
        self.dirty = true;
        self.recursive_dirty = true;
    }
}

#[derive(Debug, Default)]
pub struct DOM {
    pub nodes: HashMap<u64, DOMNode>,      // Arena storage for all nodes
    pub selector_manager: SelectorManager, // Selector manager
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

fn get_input() -> Vec<bool> {
    vec![false; unsafe { STATE } + 1]
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
        let tag_id = sm.get_or_create_type_id(&tag_name.to_lowercase());

        let mut class_ids = HashSet::new();
        for class in &classes {
            let class_id = sm.get_or_create_class_id(class);
            class_ids.insert(class_id);
        }

        let id_selector_id = html_id
            .as_ref()
            .map(|id| sm.get_or_create_id_selector_id(id));

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
            dirty: true,
            recursive_dirty: true,
            output_state: vec![false; unsafe { STATE } + 1],
        };
        let o = self.new_output_state(&new_node, &get_input(), nfa);
        new_node.output_state = o;
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
    /// Create a new empty DOM.
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
                node.set_dirty();
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
                node.set_dirty();
                changed = true;
            } else if !hover_active && had_hover {
                node.computed_pseudo_classes.remove(PSEUDO_CLASS_HOVER);
                node.set_dirty();
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
            let has_focus = node.computed_pseudo_classes.contains(PSEUDO_CLASS_FOCUS);
            if focus_root_active && !has_focus {
                node.computed_pseudo_classes
                    .insert(PSEUDO_CLASS_FOCUS.to_string());
                node.set_dirty();
                changed = true;
            } else if !focus_root_active && has_focus {
                node.computed_pseudo_classes.remove(PSEUDO_CLASS_FOCUS);
                node.set_dirty();
                changed = true;
            }

            let has_focus_within = node
                .computed_pseudo_classes
                .contains(PSEUDO_CLASS_FOCUS_WITHIN);
            if focus_within_active && !has_focus_within {
                node.computed_pseudo_classes
                    .insert(PSEUDO_CLASS_FOCUS_WITHIN.to_string());
                node.set_dirty();
                changed = true;
            } else if !focus_within_active && has_focus_within {
                node.computed_pseudo_classes
                    .remove(PSEUDO_CLASS_FOCUS_WITHIN);
                node.set_dirty();
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

        // Remove the target node
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
            self.nodes.remove(&removed_child_id);
        }
        self.set_node_dirty(cur_idx);
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
        let (was_recursive_dirty, was_dirty, previous_output, child_indices_snapshot) =
            match self.nodes.get(&node_idx) {
                Some(node) => (
                    node.recursive_dirty,
                    node.dirty,
                    node.output_state.clone(),
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
                "{} visit: dirty={} input={}",
                node_descriptor,
                was_dirty,
                format_bits(input)
            )
        });

        if was_dirty {
            unsafe {
                MISS_CNT += 1;
            }
            let new_output_state = {
                if let Some(node) = self.nodes.get(&node_idx) {
                    self.new_output_state(node, input, nfa)
                } else {
                    debug_log(|| {
                        format!(
                            "{} vanished before recompute; aborting dirty propagation",
                            node_descriptor
                        )
                    });
                    return;
                }
            };
            debug_log(|| {
                format!(
                    "{} recompute -> output={} (prev={})",
                    node_descriptor,
                    format_bits(&new_output_state),
                    format_bits(&previous_output)
                )
            });
            if previous_output != new_output_state {
                debug_log(|| {
                    format!(
                        "{} output changed; marking {} children dirty",
                        node_descriptor,
                        child_indices_snapshot.len()
                    )
                });
                if let Some(node) = self.nodes.get_mut(&node_idx) {
                    node.output_state = new_output_state.clone();
                } else {
                    debug_log(|| {
                        format!(
                            "{} missing before storing output_state; aborting child propagation",
                            node_descriptor
                        )
                    });
                    return;
                }
                let mut marked_children = Vec::new();
                for &child_idx in &child_indices_snapshot {
                    if let Some(child) = self.nodes.get_mut(&child_idx) {
                        child.set_dirty();
                        marked_children.push((child_idx, child.dirty));
                    }
                }
                for (child_idx, dirty_state) in marked_children {
                    let child_desc = self.describe_node(child_idx);
                    debug_log(|| {
                        format!(
                            "{} child {} marked dirty due to parent change -> dirty={}",
                            node_descriptor, child_desc, dirty_state
                        )
                    });
                }
            } else {
                debug_log(|| {
                    format!(
                        "{} output unchanged; children remain clean",
                        node_descriptor
                    )
                });
            }
        } else {
            // Debug check: if not dirty, recomputing should not change output
            debug_log(|| {
                format!(
                    "{} clean node; validating cached output={} with new input={}",
                    node_descriptor,
                    format_bits(&previous_output),
                    format_bits(input)
                )
            });
            let new_output_state = {
                if let Some(node) = self.nodes.get(&node_idx) {
                    self.new_output_state(node, input, nfa)
                } else {
                    debug_log(|| {
                        format!(
                            "{} missing before validation recompute; skipping check",
                            node_descriptor
                        )
                    });
                    return;
                }
            };
            debug_log(|| {
                format!(
                    "{} validation recompute -> output={}",
                    node_descriptor,
                    format_bits(&new_output_state)
                )
            });
            assert_eq!(
                previous_output, new_output_state,
                "{}: Output state changed when node was not dirty!",
                node_descriptor
            );
        }

        // Recursively process children
        let current_output_state = match self.nodes.get(&node_idx) {
            Some(node) => node.output_state.clone(),
            None => {
                debug_log(|| {
                    format!(
                        "{} removed before propagating to children; aborting subtree traversal",
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
            let child_needs_visit = self
                .nodes
                .get(&child_idx)
                .map(|child| child.recursive_dirty)
                .unwrap_or(false);
            if child_needs_visit {
                self.recompute_styles_recursive(child_idx, nfa, &current_output_state);
            }
        }

        // Reset dirty flags
        if let Some(node) = self.nodes.get_mut(&node_idx) {
            node.dirty = false;
            node.recursive_dirty = false;
        }
        debug_log(|| format!("{} finished; dirty flags cleared", node_descriptor));
    }
    /// Propagation follows these rules.
    /// For an NFA, each edge corresponds to a `Rule`.
    /// Collect the rules in a `Vec` indexed by state to track which edges are already active.
    /// When new input arrives, you can skip edges that are already active.
    fn new_output_state(&self, node: &DOMNode, input: &[bool], nfa: &NFA) -> Vec<bool> {
        let mut new_state = vec![false; input.len()];

        for &rule in nfa.rules.iter() {
            match rule {
                Rule(None, None, Nfacell(c)) => {
                    new_state[c] = true;
                }
                Rule(None, Some(Nfacell(b)), Nfacell(c)) => {
                    if input[b] {
                        new_state[c] = true;
                    }
                }
                Rule(Some(a), None, Nfacell(c)) => {
                    if self.node_matches_selector(node, a) {
                        new_state[c] = true;
                    }
                }
                Rule(Some(a), Some(Nfacell(b)), Nfacell(c)) => {
                    if self.node_matches_selector(node, a) && input[b] {
                        new_state[c] = true;
                    }
                }
            }
        }
        new_state
    }
}

impl css_bitvector_compiler::runtime_shared::FrameDom<DOMNode> for DOM {
    type AttrState = Vec<bool>;
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
            .map(|parent| parent.output_state.clone())
            .unwrap_or_else(|| make_root_input());
        (node.output_state.clone(), parent_bits)
    }
    fn recompute_attr_state(
        &self,
        node_idx: u64,
        parent_bits: &[bool],
        nfa: &NFA,
    ) -> Self::AttrState {
        let node = &self.nodes[&node_idx];
        self.new_output_state(node, parent_bits, nfa)
    }
}

fn apply_frame(dom: &mut DOM, frame: &LayoutFrame, nfa: &NFA) {
    let make_input = || get_input();
    let make_recalc_input = |nfa: &NFA| {
        let mut input = vec![false; unsafe { STATE } + 1];
        if let Some(start) = nfa.start_state {
            input[start.0] = true;
        }
        input
    };
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
            if node.output_state[state_index] {
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

fn main() {
    // 1. Build the DOM tree
    let mut dom = DOM::new();
    let ParsedSelectors {
        mut selectors,
        mut pseudo_selectors,
    } = parse_css_with_pseudo(
        &std::fs::read_to_string(format!(
            "css-gen-op/{0}/{0}.css",
            std::env::var("WEBSITE_NAME").unwrap(),
        ))
        .unwrap(),
    );
    selectors.extend(drain_supported_pseudo_selectors(&mut pseudo_selectors));
    selectors.sort();
    selectors.dedup();
    let (selectors, skipped_simple) = partition_simple_selectors(selectors);
    report_skipped_selectors("bit", &skipped_simple);
    report_pseudo_selectors("bit", &pseudo_selectors);
    // dbg!(&selectors);
    let mut s = unsafe { STATE };
    let nfa = generate_nfa(&selectors, &mut dom.selector_manager, &mut s);
    unsafe {
        STATE = s;
    }
    let _ = fs::write(
        format!(
            "css-gen-op/{0}/dot.dot",
            std::env::var("WEBSITE_NAME").unwrap(),
        ),
        nfa.to_dot(&dom.selector_manager),
    );

    // for Rule(a, b, c) in nfa.rules.iter() {
    //     println!(
    //         "{} {:?}  {:?}",
    //         dom.selector_manager.id_to_selector[&a.unwrap_or_default()],
    //         b,
    //         c
    //     );
    // }

    for f in parse_trace() {
        apply_frame(&mut dom, &f, &nfa);
    }
    let mut final_matches = collect_rule_matches(&dom, &nfa, &selectors)
        .into_iter()
        .collect::<Vec<_>>();
    final_matches.sort();
    println!("BEGIN");
    for (k, v) in final_matches {
        println!("{} -> {:?}", k.replace('>', " > "), v);
    }
    println!("END");
    dbg!(unsafe { MISS_CNT });
}

#[cfg(test)]
mod tests {
    use std::fs::write;

    use css_bitvector_compiler::generate_nfa;

    use super::*;
    #[test]
    fn test_generate_nfa() {
        // Reset global state for testing
        let mut s = 0;
        let mut selector_manager = SelectorManager::new();
        let selectors = ["div a", "p", "h1 > h2", "h1 h2", "div a p"].map(|x| x.into());

        let nfa = generate_nfa(&selectors, &mut selector_manager, &mut s);
        // dbg!(&nfa);
        let _ = write("./dot.dot", nfa.to_dot(&selector_manager));
        dbg!(nfa.rules);
    }

    #[test]
    fn node_matches_attribute_selector() {
        let mut dom = DOM::new();
        let attr_selector = Selector::AttributeEquals {
            name: "data-test".into(),
            value: "foo".into(),
        };
        let attr_id = dom.selector_manager.get_or_create_id(attr_selector.clone());
        let tag_id = dom
            .selector_manager
            .get_or_create_id(Selector::Type("div".into()));

        let node = DOMNode {
            tag_id,
            class_ids: HashSet::new(),
            id_selector_id: None,
            attributes: HashMap::from([("data-test".into(), "foo".into())]),
            pseudo_classes: HashSet::new(),
            computed_pseudo_classes: HashSet::new(),
            parent: None,
            children: Vec::new(),
            dirty: false,
            recursive_dirty: false,
            output_state: Vec::new(),
        };

        assert!(dom.node_matches_selector(&node, attr_id));

        let other_attr_id = dom
            .selector_manager
            .get_or_create_id(Selector::AttributeEquals {
                name: "data-test".into(),
                value: "bar".into(),
            });
        assert!(!dom.node_matches_selector(&node, other_attr_id));
    }

    #[test]
    fn debug_logs_skip_child_recompute_when_parent_change_is_irrelevant() {
        unsafe {
            MISS_CNT = 0;
            STATE = 0;
            std::env::set_var("BIT_DEBUG", "1");
        }

        let mut dom = DOM::new();
        let selectors = vec![".leaf".to_string()];
        let mut s = 0;
        let nfa = generate_nfa(&selectors, &mut dom.selector_manager, &mut s);
        unsafe {
            STATE = s;
        }

        let root_attributes = HashMap::from([("id".to_string(), "root".to_string())]);
        let root_id = dom.add_node(
            1,
            "A",
            Vec::<String>::new(),
            Some("root".to_string()),
            root_attributes,
            HashSet::new(),
            None,
            &nfa,
        );
        let child_attributes = HashMap::from([("class".to_string(), "leaf".to_string())]);
        let child_id = dom.add_node(
            2,
            "A",
            vec!["leaf".to_string()],
            None,
            child_attributes,
            HashSet::new(),
            Some(root_id),
            &nfa,
        );

        let initial_input = get_input();
        dom.recompute_styles(&nfa, &initial_input);

        let before = unsafe { MISS_CNT };

        let new_tag_id = dom.selector_manager.get_or_create_type_id("b");
        {
            let root = dom.nodes.get_mut(&root_id).unwrap();
            root.tag_id = new_tag_id;
        }
        dom.set_node_dirty(root_id);
        assert!(
            !dom.nodes.get(&child_id).unwrap().dirty,
            "child should remain clean before recompute"
        );

        let second_input = get_input();
        dom.recompute_styles(&nfa, &second_input);

        let after = unsafe { MISS_CNT };
        assert_eq!(
            after - before,
            1,
            "expected only the root node to recompute after the tag rename"
        );

        unsafe {
            std::env::remove_var("BIT_DEBUG");
        }
    }
}
