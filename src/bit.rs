use css_bitvector_compiler::{
    LayoutFrame, NFA, Nfacell, Rule, Selector, SelectorId, SelectorManager, extract_path_from_command, generate_nfa, parse_css, parse_trace, rdtsc
};
use serde_json;
use std::{
    collections::{HashMap, HashSet},
    fs,
};
static mut MISS_CNT: usize = 0;
static mut STATE: usize = 0; // global state

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
}

impl DOMNode {
    fn set_dirty(&mut self) {
        self.dirty = true;
        self.recursive_dirty = true;
    }
}

#[derive(Debug, Default)]
pub struct DOM {
    pub nodes: HashMap<u64, DOMNode>,      // Arena: 所有节点都存储在这里
    pub selector_manager: SelectorManager, // 选择器管理器
    root_node: Option<u64>,
}

fn get_input(nfa: &NFA) -> Vec<bool> {
    let mut input = vec![false; unsafe { STATE } + 1];

    input[nfa.start_state.0] = true;
    input
}

impl DOM {
    /// 创建一个新的、空的 DOM。
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
        nfa: &NFA,
    ) -> u64 {
        let sm = &mut self.selector_manager;
        let tag_id = sm.get_or_create_type_id(tag_name);

        let mut class_ids = HashSet::new();
        for class in &classes {
            let class_id = sm.get_or_create_class_id(class);
            class_ids.insert(class_id);
        }

        let id_selector_id = html_id
            .as_ref()
            .map(|id| sm.get_or_create_id_selector_id(id));

        let mut new_node = DOMNode {
            tag_id,
            class_ids,
            id_selector_id,
            parent: parent_index,
            children: Vec::new(),
            dirty: true,
            recursive_dirty: true,
            output_state: vec![false; unsafe { STATE } + 1],
        };
        let o = self.new_output_state(&new_node, &get_input(nfa), nfa);
        new_node.output_state = o;
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
    pub fn node_matches_selector(&self, node: &DOMNode, selector_id: SelectorId) -> bool {
        if node.tag_id == selector_id {
            return true;
        }

        if node.class_ids.contains(&selector_id) {
            return true;
        }

        if let Some(id_sel_id) = node.id_selector_id
            && id_sel_id == selector_id
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
        let current_index = self.add_node(id, tag_name, classes.clone(), html_id, parent_index, nfa);
        // HACK
        if id == 5458 {
            if classes.contains(&"hidden".to_string()) {
                panic!()
            }
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
            current_idx = self.nodes[&current_idx].children[path_element as usize];
        }

        // 在指定位置插入新节点
        let new_node_idx = self.json_to_html_node(json_node, Some(current_idx), nfa);
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
        let node = &self.nodes[&node_idx];

        if self.nodes[&node_idx].dirty {
            unsafe {
                MISS_CNT += 1;
            }
            let new_output_state = self.new_output_state(node, input, nfa);
            if self.nodes[&node_idx].output_state != new_output_state {
                self.nodes.get_mut(&node_idx).unwrap().output_state = new_output_state;
                for child_idx in self.nodes[&node_idx].children.clone() {
                    self.nodes.get_mut(&child_idx).unwrap().set_dirty();
                }
            }
        } else {
            // Debug check: if not dirty, recomputing should not change output
            let original_output_state = self.nodes[&node_idx].output_state.clone();
            let new_output_state = self.new_output_state(node, input, nfa);
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
    /// 传播规则是这样的
    /// 对 NFA 来说, 每条边对应一个 Rule
    /// 用一个 Vec 收集这些 Rule, 下标对应 state 的下标, 表示哪些边已经被激活了.
    /// 当一个新的 input 传下来时, 已经亮的就不用检查了
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

    fn force_recalc(&mut self, node_idx: u64, input: &[bool], nfa: &NFA) {
        self.nodes.get_mut(&node_idx).unwrap().recursive_dirty = false;
        self.nodes.get_mut(&node_idx).unwrap().dirty = false;
        // unsafe {
        //     MISS_CNT += 1;
        // }
        let new_output_state = self.new_output_state(&self.nodes[&node_idx], input, nfa);
        self.nodes.get_mut(&node_idx).unwrap().output_state = new_output_state;
        for child_idx in self.nodes[&node_idx].children.clone() {
            self.nodes.get_mut(&child_idx).unwrap().set_dirty();
        }
        
        // Debug check: if not dirty, recomputing should not change output
        let original_output_state = self.nodes[&node_idx].output_state.clone();
        let new_output_state = self.new_output_state(&self.nodes[&node_idx], input, nfa);
        assert_eq!(
            original_output_state, new_output_state,
            "Node index {}: Output state changed when node was not dirty!",
            node_idx
        );
        

        // Recursively process children
        let children_indices = self.nodes[&node_idx].children.clone();
        let current_output_state = self.nodes[&node_idx].output_state.clone();
        for &child_idx in &children_indices {
            self.force_recalc(child_idx,&current_output_state, nfa);
        }

        // Reset dirty flags
        if let Some(node) = self.nodes.get_mut(&node_idx) {
            node.dirty = false;
            node.recursive_dirty = false;
        }
    }
}

/// 解析CSS选择器字符串并生成对应的选择器对象
pub fn parse_selector(selector_str: &str) -> Selector {
    let trimmed = selector_str.trim().to_lowercase();

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
            dom.json_to_html_node(node_data, None, nfa);
        }
        "add" => {
            let path = extract_path_from_command(&frame.command_data);
            let node_data = frame.command_data.get("node").unwrap();
            dom.add_node_by_path(&path, node_data, nfa);
            if std::env::var("WEBSITE_NAME").unwrap() == "testcase".to_string() {
                dbg!(&dom.nodes);
            }


                        // BEGIN HACK


            // END HACK
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
    // 1. 构建 DOM 树
    let mut dom = DOM::new();
    let selectors = parse_css(
        &std::fs::read_to_string(format!(
            "css-gen-op/{0}/{0}.css",
            std::env::var("WEBSITE_NAME").unwrap(),
        ))
        .unwrap(),
    );
    let mut s = unsafe {
         STATE
    };
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
    for f in parse_trace() {
        apply_frame(&mut dom, &f, &nfa);
    }
    let d = dom.get_root_node();
    dom.force_recalc(d, &get_input(&nfa), &nfa);
    
    let final_matches = collect_rule_matches(&dom, &nfa, &selectors);
    println!("final_rule_matches:");
    for (k, v) in final_matches {
        println!("{} -> {:?}", k, v);
    }
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
}
