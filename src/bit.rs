use css_bitvector_compiler::{
    LayoutFrame, extract_path_from_command, parse_css, parse_trace, rdtsc,
};
use serde_json;
use std::collections::{HashMap, HashSet};
static mut MISS_CNT: usize = 0;
static mut STATE: usize = 0; // global state

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct Nfacell(pub usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct SelectorId(pub usize);

/// CSS选择器类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Selector {
    Type(String),
    Class(String),
    Id(String),
}

/// 标签名和选择器管理器，负责字符串选择器与ID之间的映射
#[derive(Debug)]
pub struct SelectorManager {
    /// 从选择器到ID的映射
    pub selector_to_id: HashMap<Selector, SelectorId>,
    /// 从ID到选择器的映射
    pub id_to_selector: HashMap<SelectorId, Selector>,
    /// 下一个可用的ID
    next_id: SelectorId,
}

impl SelectorManager {
    pub fn new() -> Self {
        let manager = SelectorManager {
            selector_to_id: HashMap::new(),
            id_to_selector: HashMap::new(),
            next_id: SelectorId(0),
        };
        manager
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

    /// 便捷方法：根据标签名获取或创建类型选择器ID
    pub fn get_or_create_type_id(&mut self, tag_name: &str) -> SelectorId {
        self.get_or_create_id(Selector::Type(tag_name.to_string()))
    }

    /// 便捷方法：根据类名获取或创建类选择器ID
    pub fn get_or_create_class_id(&mut self, class_name: &str) -> SelectorId {
        self.get_or_create_id(Selector::Class(class_name.to_string()))
    }

    /// 便捷方法：根据ID获取或创建ID选择器ID
    pub fn get_or_create_id_selector_id(&mut self, id_name: &str) -> SelectorId {
        self.get_or_create_id(Selector::Id(id_name.to_string()))
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
}

impl DOMNode {
    fn set_dirty(&mut self) {
        self.dirty = true;
        self.recursive_dirty = true;
    }
}

#[derive(Debug)]
pub struct DOM {
    pub nodes: HashMap<u64, DOMNode>,      // Arena: 所有节点都存储在这里
    pub selector_manager: SelectorManager, // 选择器管理器
    root_node: Option<u64>,
}

impl DOM {
    /// 创建一个新的、空的 DOM。
    pub fn new() -> Self {
        DOM {
            nodes: Default::default(),
            selector_manager: SelectorManager::new(),
            root_node: None,
        }
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
        // 获取或创建选择器ID
        let tag_id = self.selector_manager.get_or_create_type_id(tag_name);

        let mut class_ids = HashSet::new();
        for class in &classes {
            let class_id = self.selector_manager.get_or_create_class_id(class);
            class_ids.insert(class_id);
        }

        let id_selector_id = html_id
            .as_ref()
            .map(|id| self.selector_manager.get_or_create_id_selector_id(id));

        let new_node = DOMNode {
            tag_id,
            class_ids,
            id_selector_id,
            parent: parent_index,
            children: Vec::new(),
            dirty: true,
            recursive_dirty: true,
            output_state: vec![false; unsafe { STATE } + 1],
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
    pub fn node_matches_selector(&self, node_index: u64, selector_id: SelectorId) -> bool {
        if let Some(node) = self.nodes.get(&node_index) {
            // 检查是否匹配标签选择器
            if node.tag_id == selector_id {
                return true;
            }

            // 检查是否匹配类选择器
            if node.class_ids.contains(&selector_id) {
                return true;
            }

            // 检查是否匹配ID选择器
            if let Some(id_sel_id) = node.id_selector_id {
                if id_sel_id == selector_id {
                    return true;
                }
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
    pub fn add_node_by_path(&mut self, path: &[u64], json_node: &serde_json::Value) {
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
        self.nodes.entry(current_idx).and_modify(|x| {
            x.children
                .insert(insert_pos.try_into().unwrap(), new_node_idx)
        });
        self.set_node_dirty(current_idx);
    }

    /// 通过路径移除节点
    pub fn remove_node_by_path(&mut self, path: &[u64]) {
        let root_nodes = self.get_root_node();
        // 递归到目标父节点
        let mut cur_idx = root_nodes;
        for &path_element in &path[..path.len() - 1] {
            cur_idx = self.nodes[&cur_idx].children[path_element as usize];
        }

        // 移除目标节点
        let rm_pos = path[path.len() - 1];
        self.nodes
            .get_mut(&cur_idx)
            .unwrap()
            .children
            .remove(rm_pos.try_into().unwrap());
        self.nodes.remove(&rm_pos);
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
            let new_output_state = self.new_output_state(node_idx, input, nfa);
            if self.nodes[&node_idx].output_state != new_output_state {
                self.nodes.get_mut(&node_idx).unwrap().output_state = new_output_state;
                for child_idx in self.nodes[&node_idx].children.clone() {
                    self.nodes.get_mut(&child_idx).unwrap().set_dirty();
                }
            }
        } else {
            // Debug check: if not dirty, recomputing should not change output
            let original_output_state = self.nodes[&node_idx].output_state.clone();
            let new_output_state = self.new_output_state(node_idx, input, nfa);
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
    fn new_output_state(&self, node_idx: u64, input: &[bool], nfa: &NFA) -> Vec<bool> {
        let mut new_state = self.nodes[&node_idx].output_state.clone();
        for Rule(sel_opt, from_opt, to_state) in &nfa.rules {
            let Some(from_state) = from_opt else {
                continue;
            };
            if !input[from_state.0] {
                continue;
            }
            if nfa.is_accept_state(from_state) {
                continue;
            }
            if let Some(sel_id) = sel_opt {
                if self.node_matches_selector(node_idx, *sel_id) {
                    new_state[to_state.0] = true;
                }
            } else {
                new_state[to_state.0] = true;
            }
        }
        new_state
    }
}

/// 转移规则: (输入选择器, 当前状态, 下一个状态)
/// 其中输入选择器为 None 表示通配符/epsilon 或者特殊匹配；当前状态为 None 可用于起始逻辑
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rule(pub Option<SelectorId>, pub Option<Nfacell>, pub Nfacell);

/// 表示一个非确定性有限状态自动机 (NFA)。
#[derive(Debug, PartialEq, Eq)]
pub struct NFA {
    pub states: HashSet<Nfacell>,
    pub rules: Vec<Rule>,
    pub start_state: Nfacell,
    pub max_state_id: Nfacell,
    pub accept_states: Vec<Nfacell>,
}

impl NFA {
    /// 检查给定状态是否为接受状态（没有后继状态）
    pub fn is_accept_state(&self, state: &Nfacell) -> bool {
        // 如果没有任何以该状态为 from 的规则，则认为是接受状态
        !self
            .rules
            .iter()
            .any(|Rule(_, from, _)| from.as_ref() == Some(state))
    }

    /// 获取所有接受状态
    pub fn get_accept_states(&self) -> HashSet<Nfacell> {
        self.states
            .iter()
            .filter(|&state| self.is_accept_state(state))
            .copied()
            .collect()
    }
    // trans 方法在新结构下不再适用，如需要可实现查询功能
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

pub fn generate_nfa(selectors: &[String], selector_manager: &mut SelectorManager) -> NFA {
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

        // 从左往右处理选择器部分
        let mut i = 0;
        while i < parts.len() {
            let token = parts[i];
            let direct = token == ">";

            if direct {
                // 子代组合器，跳过它，下一个选择器是直接子元素
                i += 1;
                if i >= parts.len() {
                    break;
                }
            }
            let selector_str = if direct { parts[i] } else { token };
            // 判断当前处理的是否是最后一个选择器（忽略 '>' 符号）
            let mut k = i + 1;
            let mut has_more_selector = false;
            while k < parts.len() {
                if parts[k] != ">" { has_more_selector = true; break; }
                k += 1;
            }
            let is_last_selector = !has_more_selector;
            // 创建新状态
            let new_state = unsafe {
                STATE += 1;
                Nfacell(STATE)
            };
            states.insert(new_state);

            // 解析选择器并获取对应的ID
            let selector = parse_selector(selector_str);
            // '*' 直接转换为 None 代表通配符，否则分配 SelectorId
            match selector {
                Selector::Type(ref s) if s == "*" => {
                    rules.push(Rule(None, Some(cur), new_state));
                }
                other => {
                    let selector_id = selector_manager.get_or_create_id(other);
                    rules.push(Rule(Some(selector_id), Some(cur), new_state));
                }
            }
            if !direct && !is_last_selector {
                // 仅在不是最后一个选择器时添加自循环（用于后代组合器 * 匹配）
                rules.push(Rule(None, Some(cur), cur));
            }
            cur = new_state;
            i += 1;
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

/// 收集所有 rule -> [node id] 的匹配结果
pub fn collect_rule_matches(
    dom: &DOM,
    nfas: &NFA,
    selects: &[String],
) -> HashMap<String, Vec<u64>> {
    let mut res: HashMap<String, Vec<u64>> = HashMap::new();
    for (idx, rule) in selects.iter().enumerate() {
        let acc = nfas.accept_states[idx];
        for (node_id, node) in dom.nodes.iter() {
            if acc.0 < node.output_state.len() && node.output_state[acc.0] {
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
    let nfa = generate_nfa(&selectors, &mut dom.selector_manager);
    for f in parse_trace() {
        apply_frame(&mut dom, &f, &nfa);
    }
    let final_matches = collect_rule_matches(&dom, &nfa, &selectors);
    println!("final_rule_matches: {:#?}", final_matches);
    dbg!(unsafe { MISS_CNT });
}
