use css_bitvector_compiler::{
    AddNode, LayoutFrame, NFA, Nfacell, Rule, Selector, SelectorId, SelectorManager,
    extract_path_from_command, generate_nfa, parse_css, parse_trace, rdtsc,
};
use std::{
    collections::{HashMap, HashSet},
    fs,
};
static mut MISS_CNT: usize = 0;
static mut STATE: usize = 0; // global state

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

/// whether a part of input is: 1, 0, or unused

#[derive(Debug, Default)]
pub struct DOMNode {
    pub tag_id: SelectorId,                 // 标签选择器ID
    pub class_ids: HashSet<SelectorId>,     // CSS类选择器ID集合
    pub id_selector_id: Option<SelectorId>, // HTML ID选择器ID
    pub parent: Option<u64>,                // 存储父节点在 arena 中的索引
    pub children: Vec<u64>,                 // 存储子节点在 arena 中的索引
    pub input_state: Vec<IState>,
    pub output_state: Vec<OState>,
    pub recursive_tri_input: Vec<IState>,
}

impl DOMNode {}

#[derive(Debug, Default)]
pub struct DOM {
    pub nodes: HashMap<u64, DOMNode>, // Arena: 所有节点都存储在这里
    pub selector_manager: SelectorManager,
    root_node: Option<u64>,
}

impl AddNode for DOM {
    fn add_node(
        &mut self,
        id: u64,
        tag_name: &str,
        classes: Vec<String>,
        html_id: Option<String>,
        parent_index: Option<u64>,
        nfa: &NFA,
    ) -> u64 {
        let sm = &mut self.selector_manager;
        let tag_id = sm.get_or_create_id(Selector::Type(tag_name.to_lowercase()));

        let mut class_ids = HashSet::new();
        for class in &classes {
            let class_id = sm.get_or_create_id(Selector::Class(class.to_lowercase()));
            class_ids.insert(class_id);
        }
        let id_selector_id = html_id
            .as_ref()
            .map(|id| sm.get_or_create_id(Selector::Id(id.to_lowercase())));

        let mut new_node = DOMNode {
            tag_id,
            class_ids,
            id_selector_id,
            parent: parent_index,
            children: Vec::new(),
            output_state: vec![OState::OZero; unsafe { STATE } + 1],
            input_state: vec![IState::IUnused; unsafe { STATE } + 1],
            recursive_tri_input: vec![IState::IUnused; unsafe { STATE } + 1],
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
    /// 检查节点是否匹配给定的选择器ID
    pub fn node_matches_selector(&self, node: &DOMNode, SelectorId(sid): SelectorId) -> bool {
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

    /// 向上传播节点的 IState 信息，通知祖先节点需要重新计算
    pub fn set_node_dirty(&mut self, node_idx: u64) {
        self.propagate_input_state_up_from(node_idx);
    }

    fn propagate_input_state_up_from(&mut self, node_idx: u64) {
        let (mut current_parent, mut propagated_state, mut propagated_tri) = {
            let node = self.nodes.get(&node_idx).unwrap();
            (
                node.parent,
                node.input_state.clone(),
                node.recursive_tri_input.clone(),
            )
        };

        while let Some(parent_idx) = current_parent {
            let (next_parent, next_state, next_tri) = {
                let parent_node = self.nodes.get_mut(&parent_idx).unwrap();
                let mut tri_changed = false;
                let mut state_changed = false;
                if parent_node.recursive_tri_input != propagated_tri {
                    parent_node
                        .recursive_tri_input
                        .clone_from_slice(&propagated_tri);
                    tri_changed = true;
                }
                for (parent_state, child_state) in parent_node
                    .input_state
                    .iter_mut()
                    .zip(propagated_state.iter())
                {
                    if matches!(*parent_state, IState::IUnused)
                        && matches!(*child_state, IState::IOne | IState::IZero)
                    {
                        *parent_state = *child_state;
                        state_changed = true;
                    }
                }
                if !(tri_changed || state_changed) {
                    return;
                }
                let next_state = parent_node.input_state.clone();
                let next_tri = parent_node.recursive_tri_input.clone();
                (parent_node.parent, next_state, next_tri)
            };

            propagated_state = next_state;
            propagated_tri = next_tri;
            current_parent = next_parent;
        }
    }
    fn json_to_html_node(
        &mut self,
        json_node: &serde_json::Value,
        parent_index: Option<u64>,
        nfa: &NFA,
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
        let current_index =
            self.add_node(id, tag_name, classes.clone(), html_id, parent_index, nfa);
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
            debug_assert_eq!(parent.children.last().copied(), Some(new_node_idx));
            parent.children.pop();
            parent.children.insert(insert_pos, new_node_idx);
        }
        self.set_node_dirty(new_node_idx);
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
    pub fn recompute_styles(&mut self, nfa: &NFA, input: &[bool]) {
        let root_node = self.get_root_node();
        self.recompute_styles_recursive(root_node, nfa, input);
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
        let need_recompute = {
            let node = self.nodes.get(&node_idx).unwrap();
            let tri_mismatch = node.recursive_tri_input != node.input_state;
            let value_mismatch = input
                .iter()
                .zip(node.input_state.iter())
                .any(|(&val, tri)| {
                    matches!((val, *tri), (true, IState::IZero) | (false, IState::IOne))
                });
            tri_mismatch || value_mismatch
        };

        if need_recompute {
            unsafe {
                MISS_CNT += 1;
            }
            let (new_input_state, new_output_state) = {
                let node = self.nodes.get(&node_idx).unwrap();
                self.new_output_state(node, input, nfa)
            };
            {
                let node = self.nodes.get_mut(&node_idx).unwrap();
                debug_assert_eq!(node.recursive_tri_input.len(), new_input_state.len());
                node.output_state = new_output_state;
                node.input_state = new_input_state.clone();
                node.recursive_tri_input.clone_from_slice(&new_input_state);
            }
            self.propagate_input_state_up_from(node_idx);
        }

        let (children_indices, node_output_state, desired_tri_from_parent) = {
            let node = self.nodes.get(&node_idx).unwrap();
            (
                node.children.clone(),
                node.output_state.clone(),
                node.recursive_tri_input.clone(),
            )
        };
        let current_output_state = self.materialize(input, &node_output_state);
        for child_idx in children_indices {
            let tri_changed = {
                let child = self.nodes.get(&child_idx).unwrap();
                child.recursive_tri_input != desired_tri_from_parent
            };
            if tri_changed {
                let child = self.nodes.get_mut(&child_idx).unwrap();
                debug_assert_eq!(
                    child.recursive_tri_input.len(),
                    desired_tri_from_parent.len()
                );
                child
                    .recursive_tri_input
                    .clone_from_slice(&desired_tri_from_parent);
            }
            self.recompute_styles_recursive(child_idx, nfa, &current_output_state);
        }
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
                Rule(None, None, Nfacell(_)) => {
                    unreachable!()
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

fn get_input() -> Vec<bool> {
    vec![false; unsafe { STATE } + 1]
}

fn apply_frame(dom: &mut DOM, frame: &LayoutFrame, nfa: &NFA) {
    match frame.command_name.as_str() {
        "init" => {
            let node_data = frame.command_data.get("node").unwrap();
            dom.nodes.clear();
            dom.root_node = None;
            dom.json_to_html_node(node_data, None, nfa);
            dom.recompute_styles(nfa, &get_input()); // 
        }
        "add" => {
            let path = extract_path_from_command(&frame.command_data);
            let node_data = frame.command_data.get("node").unwrap();
            dom.add_node_by_path(&path, node_data, nfa);
            dom.recompute_styles(nfa, &get_input()); // 
        }
        "replace_value" | "insert_value" => {}
        "recalculate" => {
            // Perform CSS matching using NFA
            let start = rdtsc();

            dom.recompute_styles(nfa, &get_input());

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
    let selectors = parse_css(
        &std::fs::read_to_string(format!(
            "css-gen-op/{0}/{0}.css",
            std::env::var("WEBSITE_NAME").unwrap(),
        ))
        .unwrap(),
    );
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
    dbg!(&nfa);
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
}
