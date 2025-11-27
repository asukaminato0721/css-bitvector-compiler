use css_bitvector_compiler::{
    AddNode, CompoundSelector, LayoutFrame, NFA, Nfacell, PSEUDO_CLASS_FOCUS,
    PSEUDO_CLASS_FOCUS_ROOT, PSEUDO_CLASS_FOCUS_WITHIN, PSEUDO_CLASS_HOVER, Rule, Selector,
    SelectorId, SelectorManager, derive_hover_state, encode, extract_pseudoclasses, generate_nfa,
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
        eprintln!("[quad-debug] {}", build());
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
    pub tag_id: SelectorId,                       // 标签选择器ID
    pub class_ids: HashSet<SelectorId>,           // CSS类选择器ID集合
    pub id_selector_id: Option<SelectorId>,       // HTML ID选择器ID
    pub attributes: HashMap<String, String>,      // 节点属性键值对（小写键）
    pub pseudo_classes: HashSet<String>,          // 原始伪类标签集合
    pub computed_pseudo_classes: HashSet<String>, // 计算后的伪类状态
    pub parent: Option<u64>,                      // 存储父节点在 arena 中的索引
    pub children: Vec<u64>,                       // 存储子节点在 arena 中的索引
    pub dirty: DirtyState,
    pub recursive_dirty: bool,
    pub input_state: Vec<IState>,
    pub output_state: Vec<OState>,
}

fn format_input_state(states: &[IState]) -> String {
    states
        .iter()
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
    pub nodes: HashMap<u64, DOMNode>, // Arena: 所有节点都存储在这里
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
            output_state: vec![OState::OZero; unsafe { STATE } + 1],
            input_state: vec![IState::IUnused; unsafe { STATE } + 1],
        };
        let (input, output) = self.new_output_state(&new_node, &get_input(), nfa);
        new_node.input_state = input;
        new_node.output_state = output;
        self.nodes.insert(id, new_node);

        // 如果有父节点，将当前节点作为子节点添加到父节点的 children 列表中
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

    /// 检查节点是否匹配给定的选择器ID
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
    pub fn get_root_node(&mut self) -> u64 {
        if let Some(r) = self.root_node {
            return r;
        }
        self.root_node = Some(
            self.nodes
                .iter()
                .filter(|(_, node)| node.parent.is_none())
                .map(|(idx, _)| *idx)
                .take(1)
                .next()
                .unwrap(),
        );
        self.root_node.unwrap()
    }

    /// 设置指定节点为脏状态，并向上传播recursive_dirty位
    pub fn set_node_dirty(&mut self, node_idx: u64) {
        let parent_idx = {
            let node = self.nodes.get_mut(&node_idx).unwrap();
            node.mark_node_changed();
            node.parent
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

    fn node_matches_compound(&self, node: &DOMNode, compound: &CompoundSelector) -> bool {
        if let Some(tag) = &compound.tag {
            if !self.node_has_tag(node, tag) {
                return false;
            }
        }
        if let Some(id_name) = &compound.id {
            if !self.node_has_id(node, id_name) {
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

    fn refresh_computed_pseudos(&mut self, node_idx: u64) {
        let (parent_idx, parent_hover) = {
            let node = &self.nodes[&node_idx];
            let parent_hover = node
                .parent
                .and_then(|pid| self.nodes.get(&pid))
                .map(|parent| parent.computed_pseudo_classes.contains(PSEUDO_CLASS_HOVER))
                .unwrap_or(false);
            (node.parent, parent_hover)
        };

        let mut changed = false;
        {
            let node = self.nodes.get_mut(&node_idx).unwrap();
            let hover_active = derive_hover_state(&node.pseudo_classes, parent_hover);
            let had_hover = node.computed_pseudo_classes.contains(PSEUDO_CLASS_HOVER);
            if hover_active && !had_hover {
                node.computed_pseudo_classes
                    .insert(PSEUDO_CLASS_HOVER.to_string());
                changed = true;
            } else if !hover_active && had_hover {
                node.computed_pseudo_classes.remove(PSEUDO_CLASS_HOVER);
                changed = true;
            }
            if changed {
                if node.dirty == DirtyState::Clean {
                    node.mark_input_changed();
                } else {
                    node.recursive_dirty = true;
                }
            }
        }

        if changed {
            self.propagate_recursive_dirty(parent_idx);
        }
    }

    fn recompute_focus_states(&mut self, node_idx: u64) -> bool {
        let (child_indices, parent_idx, focus_root_active) = {
            let node = &self.nodes[&node_idx];
            let active = node.pseudo_classes.contains(PSEUDO_CLASS_FOCUS_ROOT)
                || node.pseudo_classes.contains(PSEUDO_CLASS_FOCUS);
            (node.children.clone(), node.parent, active)
        };

        let mut focus_within_active = focus_root_active;
        for child_idx in child_indices {
            if self.recompute_focus_states(child_idx) {
                focus_within_active = true;
            }
        }

        let mut changed = false;
        {
            let node = self.nodes.get_mut(&node_idx).unwrap();
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
                if node.dirty == DirtyState::Clean {
                    node.mark_input_changed();
                } else {
                    node.recursive_dirty = true;
                }
                changed = true;
            }
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

        // 创建当前节点
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
        // 递归处理子节点
        if let Some(children_array) = json_node["children"].as_array() {
            for child_json in children_array {
                self.json_to_html_node(child_json, Some(current_index), nfa);
            }
        }
        current_index
    }

    /// 通过路径添加节点
    pub fn add_node_by_path(&mut self, path: &[usize], json_node: &serde_json::Value, nfa: &NFA) {
        assert!(!path.is_empty());
        let root_node = self.get_root_node();

        let mut current_idx = root_node;

        // 遍历路径到目标父节点
        for &path_element in &path[..path.len() - 1] {
            current_idx = self.nodes[&current_idx].children[path_element];
        }

        // 在指定位置插入新节点
        let new_node_idx = self.json_to_html_node(json_node, Some(current_idx), nfa);
        let insert_pos = path[path.len() - 1];
        if let Some(parent) = self.nodes.get_mut(&current_idx) {
            assert_eq!(parent.children.last().copied(), Some(new_node_idx));
            parent.children.pop();
            parent.children.insert(insert_pos, new_node_idx);
        }
        self.set_node_dirty(current_idx);
    }

    /// 通过路径移除节点
    pub fn remove_node_by_path(&mut self, path: &[usize]) {
        let root_nodes = self.get_root_node();
        // 递归到目标父节点
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
        self.nodes.remove(&removed_child_id);
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
    fn materialize(&self, input: &[bool], output: &[OState]) -> Vec<bool> {
        (output)
            .iter()
            .map(|p| match p {
                OState::OFromParent(index) => input[*index],
                OState::OOne => true,
                OState::OZero => false,
            })
            .collect()
    }
    fn recompute_styles_recursive(&mut self, node_idx: u64, nfa: &NFA, input: &[bool]) {
        let node_descriptor = self.describe_node(node_idx);
        self.refresh_computed_pseudos(node_idx);
        let (
            was_recursive_dirty,
            dirty_state,
            previous_input_state,
            previous_output_state,
            child_indices_snapshot,
        ) = {
            let node = &self.nodes[&node_idx];
            (
                node.recursive_dirty,
                node.dirty,
                node.input_state.clone(),
                node.output_state.clone(),
                node.children.clone(),
            )
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
                "{} visit: dirty={} input={} cached_input={} cached_output={}",
                node_descriptor,
                dirty_state.label(),
                format_bits(input),
                format_input_state(&previous_input_state),
                format_output_state(&previous_output_state)
            )
        });

        let mut should_mark_children = false;
        match dirty_state {
            DirtyState::Clean => {
                debug_log(|| format!("{} clean validation start", node_descriptor));
                let (new_input, new_output) =
                    self.new_output_state(&self.nodes[&node_idx], input, nfa);
                debug_log(|| {
                    format!(
                        "{} validation -> input={} output={}",
                        node_descriptor,
                        format_input_state(&new_input),
                        format_output_state(&new_output)
                    )
                });
                assert_eq!(
                    previous_input_state,
                    new_input,
                    "input is {:?}
old_tri is {:?}
old_output is {:?}
new_output is {:?}
new_tri is {:?}

                   ",
                    encode(input),
                    encode(&previous_input_state),
                    encode(&previous_output_state),
                    encode(&new_output),
                    encode(&new_input)
                );
            }
            DirtyState::InputChanged => {
                let need_re = !input
                    .iter()
                    .copied()
                    .zip(previous_input_state.iter().copied())
                    .all(|(input_bit, state)| {
                        matches!(
                            (input_bit, state),
                            (false, IState::IZero) | (true, IState::IOne) | (_, IState::IUnused)
                        )
                    });
                unsafe {
                    INPUT_CHANGE_COUNT += 1;
                }

                debug_log(|| {
                    format!(
                        "{} input_changed need_recompute={} input_state={}",
                        node_descriptor,
                        need_re,
                        format_input_state(&previous_input_state)
                    )
                });

                if need_re {
                    unsafe {
                        MISS_CNT += 1;
                    }
                    let (new_input_state, new_output_state) =
                        self.new_output_state(&self.nodes[&node_idx], input, nfa);
                    debug_log(|| {
                        format!(
                            "{} recompute -> input={} (prev={}) output={} (prev={})",
                            node_descriptor,
                            format_input_state(&new_input_state),
                            format_input_state(&previous_input_state),
                            format_output_state(&new_output_state),
                            format_output_state(&previous_output_state)
                        )
                    });
                    {
                        let node = self.nodes.get_mut(&node_idx).unwrap();
                        node.output_state = new_output_state.clone();
                        node.input_state = new_input_state.clone();
                    }
                    should_mark_children = true;
                } else {
                    unsafe {
                        INPUT_SKIP_COUNT += 1;
                    }
                    let (new_input, new_output) =
                        self.new_output_state(&self.nodes[&node_idx], input, nfa);
                    debug_log(|| {
                        format!(
                            "{} input reused; output stays {} input stays {}",
                            node_descriptor,
                            format_output_state(&previous_output_state),
                            format_input_state(&previous_input_state)
                        )
                    });
                    assert_eq!(
                        previous_input_state,
                        new_input,
                        "input is {:?}
old_tri is {:?}
old_output is {:?}
new_output is {:?}
new_tri is {:?}

                   ",
                        encode(input),
                        encode(&previous_input_state),
                        encode(&previous_output_state),
                        encode(&new_output),
                        encode(&new_input)
                    );
                }
            }
            DirtyState::NodeChanged => {
                unsafe {
                    MISS_CNT += 1;
                }
                let (new_input_state, new_output_state) =
                    self.new_output_state(&self.nodes[&node_idx], input, nfa);
                debug_log(|| {
                    format!(
                        "{} recompute (node_changed) -> input={} (prev={}) output={} (prev={})",
                        node_descriptor,
                        format_input_state(&new_input_state),
                        format_input_state(&previous_input_state),
                        format_output_state(&new_output_state),
                        format_output_state(&previous_output_state)
                    )
                });
                {
                    let node = self.nodes.get_mut(&node_idx).unwrap();
                    node.output_state = new_output_state.clone();
                    node.input_state = new_input_state.clone();
                }
                should_mark_children = true;
            }
        }

        // Propagate dirty state to children if this node's output changed.
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
                let child = self.nodes.get_mut(&child_idx).unwrap();
                child.mark_input_changed();
                let dirty_label = child.dirty.label();
                marked_children.push((child_idx, dirty_label));
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

        let output_state_snapshot = self.nodes[&node_idx].output_state.clone();
        let current_output_state = self.materialize(input, &output_state_snapshot);
        debug_log(|| {
            format!(
                "{} propagating to {} children with materialized output={}",
                node_descriptor,
                child_indices_snapshot.len(),
                format_bits(&current_output_state)
            )
        });
        for &child_idx in &child_indices_snapshot {
            self.recompute_styles_recursive(child_idx, nfa, &current_output_state);
        }

        // Reset dirty flags
        if let Some(node) = self.nodes.get_mut(&node_idx) {
            node.clear_dirty();
        }
        debug_log(|| format!("{} finished; dirty flags cleared", node_descriptor));
    }
    fn new_output_state(
        &self,
        node: &DOMNode,
        input: &[bool],
        nfa: &NFA,
    ) -> (Vec<IState>, Vec<OState>) {
        let mut new_state = vec![OState::OZero; input.len()];

        struct Read {
            input: Vec<bool>,
            pub tri: Vec<IState>,
        }
        impl Read {
            fn new(v: &[bool]) -> Self {
                let l = v.len();
                Self {
                    input: v.into(),
                    tri: vec![IState::IUnused; l],
                }
            }
            fn get(&mut self, idx: usize) -> bool {
                self.tri[idx] = if self.input[idx] {
                    IState::IOne
                } else {
                    IState::IZero
                };
                self.input[idx]
            }
        }
        let mut input = Read::new(input);
        let mut propagate_rules: Vec<Rule> = Vec::new();

        for &rule in nfa.rules.iter() {
            match rule {
                Rule(None, None, Nfacell(c)) => {
                    new_state[c] = OState::OOne;
                }
                Rule(Some(selector_id), None, Nfacell(c)) => {
                    if self.node_matches_selector(node, selector_id) {
                        new_state[c] = OState::OOne;
                    }
                }
                Rule(_, Some(_), _) => {
                    propagate_rules.push(rule);
                }
            }
        }

        for &Rule(selector_opt, parent_opt, Nfacell(target_idx)) in &propagate_rules {
            let Some(Nfacell(parent_idx)) = parent_opt else {
                unreachable!("propagate_rules should only contain transitions with predecessors");
            };

            match selector_opt {
                None => {
                    if matches!(new_state[target_idx], OState::OZero) {
                        let _ = input.get(parent_idx);
                        new_state[target_idx] = OState::OFromParent(parent_idx);
                    }
                }
                Some(selector_id) => {
                    if self.node_matches_selector(node, selector_id) {
                        let parent_active = input.get(parent_idx);
                        if parent_active {
                            new_state[target_idx] = OState::OFromParent(parent_idx);
                        } else {
                            new_state[target_idx] = OState::OZero;
                        }
                    } else if matches!(new_state[target_idx], OState::OFromParent(_)) {
                        new_state[target_idx] = OState::OZero;
                    }
                }
            }
        }

        for &Nfacell(state_idx) in &nfa.accept_states {
            if let OState::OFromParent(parent_idx) = new_state[state_idx] {
                let parent_active = input.get(parent_idx);
                new_state[state_idx] = if parent_active {
                    OState::OOne
                } else {
                    OState::OZero
                };
            }
        }

        (input.tri, new_state)
    }
}

impl css_bitvector_compiler::runtime_shared::FrameDom<DOMNode> for DOM {
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
    dom: &mut DOM,
    nfas: &NFA,
    selects: &[String],
) -> HashMap<String, Vec<u64>> {
    let mut res: HashMap<String, Vec<u64>> = HashMap::new();

    let mut state_cache: HashMap<u64, Vec<bool>> = HashMap::new();

    fn materialize_node(
        dom: &DOM,
        node_idx: u64,
        cache: &mut HashMap<u64, Vec<bool>>,
    ) -> Vec<bool> {
        if let Some(existing) = cache.get(&node_idx) {
            return existing.clone();
        }

        let node = &dom.nodes[&node_idx];
        let parent_state = if let Some(parent_idx) = node.parent {
            if dom.nodes.contains_key(&parent_idx) {
                materialize_node(dom, parent_idx, cache)
            } else {
                vec![false; unsafe { STATE } + 1]
            }
        } else {
            vec![false; unsafe { STATE } + 1]
        };

        let current_state = dom.materialize(&parent_state, &node.output_state);
        cache.insert(node_idx, current_state.clone());
        current_state
    }

    // Prime root cache if possible; this also ensures STATE has been initialised.
    let _ = dom.get_root_node();

    for (&node_id, _) in dom.nodes.iter() {
        let current_state = materialize_node(dom, node_id, &mut state_cache);
        for (idx, &Nfacell(state_index)) in nfas.accept_states.iter().enumerate() {
            if current_state[state_index] {
                let rule = &selects[idx];
                res.entry(rule.clone()).or_default().push(node_id);
            }
        }
    }

    for v in res.values_mut() {
        v.dedup();
        v.sort_unstable();
    }
    res
}
fn main() {
    let mut dom = DOM::new();
    let parsed = parse_css_with_pseudo(
        &std::fs::read_to_string(format!(
            "css-gen-op/{0}/{0}.css",
            std::env::var("WEBSITE_NAME").unwrap(),
        ))
        .unwrap(),
    );
    let (selectors, skipped_simple) = partition_simple_selectors(parsed.selectors);
    report_skipped_selectors("quad", &skipped_simple);
    report_pseudo_selectors("quad", &parsed.pseudo_selectors);
    let mut s = unsafe { STATE };
    let nfa = generate_nfa(&selectors, &mut dom.selector_manager, &mut s);
    unsafe {
        STATE = s;
    }
    let _ = fs::write(
        format!(
            "css-gen-op/{0}/dot_quad.dot",
            std::env::var("WEBSITE_NAME").unwrap(),
        ),
        nfa.to_dot(&dom.selector_manager),
    );
    // dbg!(&nfa);
    for f in parse_trace() {
        apply_frame(&mut dom, &f, &nfa);
    }

    let mut final_matches = collect_rule_matches(&mut dom, &nfa, &selectors)
        .into_iter()
        .collect::<Vec<_>>();
    final_matches.sort();
    println!("BEGIN");
    for (k, mut v) in final_matches {
        v.dedup();
        println!("{} -> {:?}", k.replace('>', " > "), v);
    }
    println!("END");
    dbg!(unsafe { MISS_CNT });
    dbg!(unsafe { INPUT_CHANGE_COUNT });
    dbg!(unsafe { INPUT_SKIP_COUNT });
}
