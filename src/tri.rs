use css_bitvector_compiler::{
    LayoutFrame, Nfacell, Rule, SelectorId, extract_path_from_command, parse_css, parse_trace,
    rdtsc,
};
use serde_json;
use std::{
    collections::{HashMap, HashSet},
    fs,
};
static mut MISS_CNT: usize = 0;
static mut STATE: usize = 0; // global state

/// CSS选择器类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Selector {
    Type(String),
    Class(String),
    Id(String),
}

/// whether a part of input is: 1, 0, or unused
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IState {
    IOne,
    IZero,
    IUnused,
}

#[derive(Debug, Default)]
pub struct SelectorManager {
    pub selector_to_id: HashMap<Selector, SelectorId>,
    pub id_to_selector: HashMap<SelectorId, Selector>,
    next_id: SelectorId,
}

impl SelectorManager {
    /// 创建一个新的选择器管理器
    pub fn new() -> Self {
        Default::default()
    }

    /// 获取选择器对应的ID，如果不存在则创建新的ID
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
}

#[derive(Debug, Default)]
pub struct DOMNode {
    pub tag_id: SelectorId,                 // 标签选择器ID
    pub class_ids: HashSet<SelectorId>,     // CSS类选择器ID集合
    pub id_selector_id: Option<SelectorId>, // HTML ID选择器ID
    pub parent: Option<u64>,                // 存储父节点在 arena 中的索引
    pub children: Vec<u64>,                 // 存储子节点在 arena 中的索引
    pub dirty: bool,
    pub recursive_dirty: bool,
    pub output_state: Vec<bool>,
    pub tri_state: Vec<IState>,
}

impl DOMNode {
    fn set_dirty(&mut self) {
        self.dirty = true;
        self.recursive_dirty = true;
    }
}

#[derive(Debug, Default)]
pub struct DOM {
    pub nodes: HashMap<u64, DOMNode>, // Arena: 所有节点都存储在这里
    pub selector_manager: SelectorManager,
    root_node: Option<u64>,
}

impl DOM {
    pub fn new() -> Self {
        Default::default()
    }

    /// 向 DOM 中添加一个新节点。
    /// 返回新节点的索引。
    pub fn add_node(
        &mut self,
        id: u64,
        tag_name: &str,
        classes: Vec<String>,
        html_id: Option<String>,
        parent_index: Option<u64>,
    ) -> u64 {
        let sm = &mut self.selector_manager;
        let tag_id = sm.get_or_create_id(Selector::Type(tag_name.into()));

        let mut class_ids = HashSet::new();
        for class in &classes {
            let class_id = sm.get_or_create_id(Selector::Class(class.into()));
            class_ids.insert(class_id);
        }
        let id_selector_id = html_id
            .as_ref()
            .map(|id| sm.get_or_create_id(Selector::Id(id.into())));

        let new_node = DOMNode {
            tag_id,
            class_ids,
            id_selector_id,
            parent: parent_index,
            children: Vec::new(),
            dirty: true,
            recursive_dirty: true,
            output_state: vec![false; unsafe { STATE } + 1],
            tri_state: vec![IState::IUnused; unsafe { STATE } + 1],
        };

        self.nodes.insert(id, new_node);

        // 如果有父节点，将当前节点作为子节点添加到父节点的 children 列表中
        if let Some(p_idx) = parent_index {
            self.nodes
                .get_mut(&p_idx)
                .expect(&format!("{p_idx} not found"))
                .children
                .push(id);
        }

        id
    }

    /// 检查节点是否匹配给定的选择器ID
    pub fn node_matches_selector(&self, node_index: u64, SelectorId(sid): SelectorId) -> bool {
        if let Some(node) = self.nodes.get(&node_index) {
            if node.tag_id == SelectorId(sid) {
                return true;
            }

            if node.class_ids.contains(&SelectorId(sid)) {
                return true;
            }

            if let Some(id_sel_id) = node.id_selector_id
                && id_sel_id == SelectorId(sid)
            {
                return true;
            }

            false
        } else {
            false
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
        return self.root_node.unwrap();
    }

    /// 设置指定节点为脏状态，并向上传播recursive_dirty位
    pub fn set_node_dirty(&mut self, node_idx: u64) {
        let node = self.nodes.get_mut(&node_idx).unwrap();
        node.set_dirty();

        // 向上传播 recursive_dirty
        let mut current_idx = node.parent;
        while let Some(parent_idx) = current_idx {
            let parent_node = self.nodes.get_mut(&parent_idx).unwrap();

            if parent_node.recursive_dirty {
                break; // 如果父节点已经设置了recursive_dirty，停止传播
            }
            parent_node.recursive_dirty = true;
            current_idx = parent_node.parent;
        }
    }
    fn json_to_html_node(
        &mut self,
        json_node: &serde_json::Value,
        parent_index: Option<u64>,
    ) -> u64 {
        let tag_name = json_node["name"].as_str().unwrap();
        let id = json_node["id"].as_u64().unwrap();
        let html_id = json_node["attributes"]
            .as_object()
            .and_then(|attrs| attrs.get("id"))
            .and_then(|id| id.as_str())
            .map(String::from);

        let classes = json_node["attributes"]
            .as_object()
            .and_then(|attrs| attrs.get("class"))
            .and_then(|class| class.as_str())
            .unwrap_or_default()
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<String>>();

        // 创建当前节点
        let current_index = self.add_node(id, tag_name, classes, html_id, parent_index);

        // 递归处理子节点
        if let Some(children_array) = json_node["children"].as_array() {
            for child_json in children_array {
                self.json_to_html_node(child_json, Some(current_index));
            }
        }
        current_index
    }

    /// 通过路径添加节点
    pub fn add_node_by_path(&mut self, path: &[usize], json_node: &serde_json::Value) {
        assert!(!path.is_empty());
        let root_node = self.get_root_node();

        let mut current_idx = root_node;

        // 遍历路径到目标父节点
        for &path_element in &path[..path.len() - 1] {
            current_idx = self.nodes[&current_idx].children[path_element as usize];
        }

        // 在指定位置插入新节点
        let new_node_idx = self.json_to_html_node(json_node, Some(current_idx));
        let insert_pos = path[path.len() - 1];
        if let Some(parent) = self.nodes.get_mut(&current_idx) {
            debug_assert_eq!(parent.children.last().copied(), Some(new_node_idx));
            parent.children.pop();
            parent.children.insert(insert_pos as usize, new_node_idx);
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

        // 移除目标节点
        let rm_pos = path[path.len() - 1];
        self.nodes
            .get_mut(&cur_idx)
            .unwrap()
            .children
            .remove(rm_pos.try_into().unwrap());
        let rm_pos = path[path.len() - 1] as usize;
        let removed_child_id = self
            .nodes
            .get_mut(&cur_idx)
            .unwrap()
            .children
            .remove(rm_pos);
        self.nodes.remove(&removed_child_id);
        self.set_node_dirty(cur_idx);
    }
    pub fn recompute_styles(&mut self, nfa: &NFA, input: &[bool]) {
        let root_node = self.get_root_node();
        self.recompute_styles_recursive(root_node, nfa, input);
    }
    fn recompute_styles_recursive(&mut self, node_idx: u64, nfa: &NFA, input: &[bool]) {
        if !self.nodes[&node_idx].recursive_dirty {
            return;
        }

        if self.nodes[&node_idx].dirty {
            unsafe {
                MISS_CNT += 1;
            }
            let (new_output_state, new_tri_state) = self.new_output_state(node_idx, input, nfa);
            let need_re = !input.iter().zip(new_tri_state).all(|x: (&bool, IState)| {
                matches!(
                    x,
                    (&false, IState::IZero) | (&true, IState::IOne) | (_, IState::IUnused)
                )
            });

            if need_re {
                self.nodes.get_mut(&node_idx).unwrap().output_state = new_output_state;
                for child_idx in self.nodes[&node_idx].children.clone() {
                    self.nodes.get_mut(&child_idx).unwrap().set_dirty(); // recompute
                }
            }
        } else {
            // Debug check: if not dirty, recomputing should not change output
            let original_output_state = self.nodes[&node_idx].output_state.clone();
            let (new_output_state, _) = self.new_output_state(node_idx, input, nfa);
            assert_eq!(
                original_output_state, new_output_state,
                "Node index {}: Output state changed when node was not dirty!",
                node_idx
            );
        }

        // Recursively process children
        let children_indices = self.nodes[&node_idx].children.clone();
        let current_output_state = self.nodes[&node_idx].output_state.clone();
        for &child_idx in &children_indices {
            self.recompute_styles_recursive(child_idx, nfa, &current_output_state);
        }

        // Reset dirty flags
        if let Some(node) = self.nodes.get_mut(&node_idx) {
            node.dirty = false;
            node.recursive_dirty = false;
        }
    }
    fn new_output_state(
        &self,
        node_idx: u64,
        input: &[bool],
        nfa: &NFA,
    ) -> (Vec<bool>, Vec<IState>) {
        let mut new_state = input.to_vec();

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
                return self.input[idx];
            }
        }
        let mut input = Read::new(input);

        for &rule in nfa.rules.iter() {
            match rule {
                Rule(None, None, Nfacell(c)) => {
                    new_state[c] = true;
                }
                Rule(None, Some(Nfacell(b)), Nfacell(c)) => {
                    if input.get(b) {
                        new_state[c] = true;
                    }
                }
                Rule(Some(a), None, Nfacell(c)) => {
                    if self.node_matches_selector(node_idx, a) {
                        new_state[c] = true;
                    }
                }
                Rule(Some(a), Some(Nfacell(b)), Nfacell(c)) => {
                    if !input.get(b) {
                        continue;
                    }
                    if self.node_matches_selector(node_idx, a) {
                        new_state[c] = true;
                    }
                }
            }
        }
        (new_state, input.tri)
    }
}
fn escape_dot_label(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}
#[derive(Debug, PartialEq, Eq)]
pub struct NFA {
    /// NFA 中所有状态的集合。
    pub states: HashSet<Nfacell>,
    /// 规则列表： (可选谓词, 可选前驱状态, 后继状态)
    pub rules: Vec<Rule>,
    /// 起始状态。
    pub start_state: Nfacell,
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

    pub fn get_accept_states(&self) -> HashSet<Nfacell> {
        self.states
            .iter()
            .filter(|&&state| self.is_accept_state(state))
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
        s.push_str(&format!("  __start -> {};\n", self.start_state.0));

        // States
        for st in &self.states {
            s.push_str(&format!("  {} [label=\"{}\"];\n", st.0, st.0));
        }

        // Accept states styling
        if !self.accept_states.is_empty() {
            s.push_str("  { node [shape=doublecircle]; ");
            for st in &self.accept_states {
                s.push_str(&format!("{} ", st.0));
            }
            s.push_str("}\n");
        }

        // Edges
        for Rule(selector_opt, from_opt, to) in &self.rules {
            let from = from_opt.unwrap_or(self.start_state).0;
            let label = match selector_opt {
                None => "*".to_string(),
                Some(sel_id) => match sm.id_to_selector.get(sel_id) {
                    Some(Selector::Type(t)) => t.clone(),
                    Some(Selector::Class(c)) => format!(".{}", c),
                    Some(Selector::Id(i)) => format!("#{}", i),
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

/// 解析CSS选择器字符串并生成对应的选择器对象
pub fn parse_selector(selector_str: &str) -> Selector {
    let trimmed = selector_str.trim();

    if trimmed.starts_with('.') {
        // 类选择器
        Selector::Class(trimmed[1..].to_string())
    } else if trimmed.starts_with('#') {
        // ID选择器
        Selector::Id(trimmed[1..].to_string())
    } else {
        // 标签选择器
        Selector::Type(trimmed.to_string())
    }
}

fn apply_frame(dom: &mut DOM, frame: &LayoutFrame, nfa: &NFA) {
    match frame.command_name.as_str() {
        "init" => {
            let node_data = frame.command_data.get("node").unwrap();
            dom.nodes.clear();
            dom.root_node = None;
            dom.json_to_html_node(node_data, None);
        }
        "add" => {
            let path = extract_path_from_command(&frame.command_data);
            let node_data = frame.command_data.get("node").unwrap();
            dom.add_node_by_path(&path, node_data);
        }
        "replace_value" | "insert_value" => {}
        "recalculate" => {
            // Perform CSS matching using NFA
            let start = rdtsc();
            let mut input = vec![false; unsafe { STATE } + 1];

            input[nfa.start_state.0] = true;
            dom.recompute_styles(nfa, &input);

            let end = rdtsc();
            println!("{}", end - start);
        }
        "remove" => {
            // Remove node at specified path
            let path = extract_path_from_command(&frame.command_data);
            dom.remove_node_by_path(&path);
        }
        _ => {}
    }
}

pub fn generate_nfa(selectors: &[String], sm: &mut SelectorManager) -> NFA {
    unsafe {
        STATE = 0;
    };
    let start_state = unsafe {
        STATE += 1;
        Nfacell(STATE)
    };
    let mut states: HashSet<Nfacell> = [start_state].into_iter().collect();
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
            let new_state = unsafe {
                STATE += 1;
                Nfacell(STATE)
            };
            states.insert(new_state);

            let selector = parse_selector(selector_str);
            match selector {
                Selector::Type(ref s) if s == "*" => {
                    rules.push(Rule(None, Some(cur), new_state));
                }
                other => {
                    let selector_id = sm.get_or_create_id(other);
                    rules.push(Rule(Some(selector_id), Some(cur), new_state));
                }
            }

            // Add self-loop only for descendant combinators (a b), not for child (a > b)
            if has_next_selector && !next_is_direct {
                rules.push(Rule(None, Some(new_state), new_state));
            }

            cur = new_state;
            i = next_selector_index;
        }
        accept_states.push(cur);
    }
    NFA {
        states,
        rules,
        start_state,
        max_state_id: Nfacell(unsafe { STATE }),
        accept_states,
    }
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
    let mut dom = DOM::new();
    let selectors = parse_css(
        &std::fs::read_to_string(format!(
            "css-gen-op/{0}/{0}.css",
            std::env::var("WEBSITE_NAME").unwrap(),
        ))
        .unwrap(),
    );
    let nfa = generate_nfa(&selectors, &mut dom.selector_manager);
    let _ = fs::write(
        format!(
            "css-gen-op/{0}/dot_tri.dot",
            std::env::var("WEBSITE_NAME").unwrap(),
        ),
        nfa.to_dot(&dom.selector_manager),
    );
    for f in parse_trace() {
        apply_frame(&mut dom, &f, &nfa);
    }
    let final_matches = collect_rule_matches(&dom, &nfa, &selectors);
    println!("final_rule_matches: {:#?}", final_matches);
    dbg!(unsafe { MISS_CNT });
}
