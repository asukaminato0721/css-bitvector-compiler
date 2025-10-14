use css_bitvector_compiler::{
    AddNode, Command, LayoutFrame, NFA, Nfacell, Rule, Selector, SelectorId, SelectorManager,
    generate_nfa, json_value_to_attr_string, parse_css, parse_trace, rdtsc,
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
    pub tag_id: SelectorId,                  // 标签选择器ID
    pub class_ids: HashSet<SelectorId>,      // CSS类选择器ID集合
    pub id_selector_id: Option<SelectorId>,  // HTML ID选择器ID
    pub attributes: HashMap<String, String>, // 节点属性键值对（小写键）
    pub parent: Option<u64>,                 // 存储父节点在 arena 中的索引
    pub children: Vec<u64>,                  // 存储子节点在 arena 中的索引
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
        parent_index: Option<u64>,
        nfa: &NFA,
    ) -> u64 {
        let sm = &mut self.selector_manager;
        let tag_id = sm.get_or_create_type_id(&tag_name.to_lowercase());

        let mut class_ids = HashSet::new();
        for class in &classes {
            let class_id = sm.get_or_create_class_id(&class.to_lowercase());
            class_ids.insert(class_id);
        }

        let id_selector_id = html_id
            .as_ref()
            .map(|id| sm.get_or_create_id_selector_id(&id.to_lowercase()));

        let mut new_node = DOMNode {
            tag_id,
            class_ids,
            id_selector_id,
            attributes,
            parent: parent_index,
            children: Vec::new(),
            dirty: true,
            recursive_dirty: true,
            output_state: vec![false; unsafe { STATE } + 1],
        };
        let o = self.new_output_state(&new_node, &get_input(), nfa);
        new_node.output_state = o;
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
    /// 创建一个新的、空的 DOM。
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

        // 创建当前节点
        let current_index = self.add_node(
            id,
            tag_name,
            classes.clone(),
            html_id,
            attributes,
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

        // 移除目标节点
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

    fn node_id_by_path(&mut self, path: &[usize]) -> Option<u64> {
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
        let key_lower = key.to_lowercase();
        match key_lower.as_str() {
            "class" => {
                let mut new_class_ids = HashSet::new();
                if let Some(ref class_value) = new_value {
                    for class_name in class_value
                        .split_whitespace()
                        .filter(|name| !name.is_empty())
                    {
                        let class_id = self
                            .selector_manager
                            .get_or_create_class_id(&class_name.to_lowercase());
                        new_class_ids.insert(class_id);
                    }
                }
                if let Some(node) = self.nodes.get_mut(&node_idx) {
                    if let Some(ref val) = new_value {
                        node.attributes.insert(key_lower.clone(), val.clone());
                    } else {
                        node.attributes.remove(key_lower.as_str());
                    }
                    node.class_ids = new_class_ids;
                }
            }
            "id" => {
                let new_selector_id = new_value.as_ref().map(|value| {
                    self.selector_manager
                        .get_or_create_id_selector_id(&value.to_lowercase())
                });
                if let Some(node) = self.nodes.get_mut(&node_idx) {
                    if let Some(ref val) = new_value {
                        node.attributes.insert(key_lower.clone(), val.clone());
                    } else {
                        node.attributes.remove(key_lower.as_str());
                    }
                    node.id_selector_id = new_selector_id;
                }
            }
            _ => {
                if let Some(node) = self.nodes.get_mut(&node_idx) {
                    if let Some(ref val) = new_value {
                        node.attributes.insert(key_lower.clone(), val.clone());
                    } else {
                        node.attributes.remove(key_lower.as_str());
                    }
                }
            }
        }
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
        self.recompute_styles_recursive(root_node, nfa, input);
        debug_log(|| format!("recompute done {}", self.describe_node(root_node)));
    }
    fn recompute_styles_recursive(&mut self, node_idx: u64, nfa: &NFA, input: &[bool]) {
        let node_descriptor = self.describe_node(node_idx);
        let (was_recursive_dirty, was_dirty, previous_output, child_indices_snapshot) = {
            let node = &self.nodes[&node_idx];
            (
                node.recursive_dirty,
                node.dirty,
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
                let node = self.nodes.get(&node_idx).unwrap();
                self.new_output_state(node, input, nfa)
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
                {
                    let node = self.nodes.get_mut(&node_idx).unwrap();
                    node.output_state = new_output_state.clone();
                }
                for &child_idx in &child_indices_snapshot {
                    {
                        let child = self.nodes.get_mut(&child_idx).unwrap();
                        child.set_dirty();
                    }
                    let child_desc = self.describe_node(child_idx);
                    debug_log(|| {
                        format!(
                            "{} child {} marked dirty due to parent change",
                            node_descriptor, child_desc
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
                let node = self.nodes.get(&node_idx).unwrap();
                self.new_output_state(node, input, nfa)
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
        let current_output_state = self.nodes[&node_idx].output_state.clone();
        debug_log(|| {
            format!(
                "{} propagating to {} children",
                node_descriptor,
                child_indices_snapshot.len()
            )
        });
        for &child_idx in &child_indices_snapshot {
            self.recompute_styles_recursive(child_idx, nfa, &current_output_state);
        }

        // Reset dirty flags
        if let Some(node) = self.nodes.get_mut(&node_idx) {
            node.dirty = false;
            node.recursive_dirty = false;
        }
        debug_log(|| format!("{} finished; dirty flags cleared", node_descriptor));
    }
    /// 传播规则是这样的
    /// 对 NFA 来说, 每条边对应一个 Rule
    /// 用一个 Vec 收集这些 Rule, 下标对应 state 的下标, 表示哪些边已经被激活了.
    /// 当一个新的 input 传下来时, 已经亮的就不用检查了
    fn new_output_state(&self, node: &DOMNode, input: &[bool], nfa: &NFA) -> Vec<bool> {
        let mut new_state = vec![false; input.len()];

        for &rule in nfa.rules.iter() {
            match rule {
                Rule(None, None, Nfacell(_)) => {
                    unreachable!()
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

fn apply_frame(dom: &mut DOM, frame: &LayoutFrame, nfa: &NFA) {
    match frame.as_command() {
        Command::Init { node } => {
            dom.nodes.clear();
            dom.root_node = None;
            dom.json_to_html_node(node, None, nfa);
        }
        Command::Add { path, node } => {
            dom.add_node_by_path(&path, node, nfa);
            if std::env::var("WEBSITE_NAME").unwrap() == "testcase" {
                //     dbg!(&dom.nodes);
            }
        }
        Command::ReplaceValue {
            path,
            key,
            value,
            old_value,
        } => {
            let node_idx = dom.node_id_by_path(&path).unwrap();
            if let Some(old_value) = old_value {
                let expected = json_value_to_attr_string(old_value);
                let actual = dom
                    .nodes
                    .get(&node_idx)
                    .and_then(|node| node.attributes.get(&key.to_lowercase()))
                    .cloned()
                    .unwrap_or_default();
                assert_eq!(
                    actual, expected,
                    "existing attribute value mismatch for key {} at path {:?}",
                    key, path
                );
            }
            let new_value = value.map(json_value_to_attr_string);
            dom.update_attribute(node_idx, key, new_value);
            dom.set_node_dirty(node_idx);
        }
        Command::InsertValue { path, key, value } => {
            let node_idx = dom.node_id_by_path(&path).unwrap();
            let new_value = value.map(json_value_to_attr_string);
            dom.update_attribute(node_idx, key, new_value);
            dom.set_node_dirty(node_idx);
        }
        Command::DeleteValue {
            path,
            key,
            old_value,
        } => {
            let node_idx = dom.node_id_by_path(&path).unwrap();
            if let Some(old_value) = old_value {
                let expected = json_value_to_attr_string(old_value);
                let actual = dom
                    .nodes
                    .get(&node_idx)
                    .and_then(|node| node.attributes.get(&key.to_lowercase()))
                    .cloned()
                    .unwrap_or_default();
                assert_eq!(
                    actual, expected,
                    "existing attribute value mismatch for key {} at path {:?}",
                    key, path
                );
            }
            dom.update_attribute(node_idx, key, None);
            dom.set_node_dirty(node_idx);
        }
        Command::Recalculate => {
            let start = rdtsc();
            let mut input = vec![false; unsafe { STATE } + 1];
            input[nfa.start_state.unwrap_or_default().0] = true;
            dom.recompute_styles(nfa, &input);
            let end = rdtsc();
            println!("{}", end - start);
        }
        Command::Remove { path } => {
            dom.remove_node_by_path(&path);
        }
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
