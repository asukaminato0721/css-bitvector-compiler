use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct DOMNode {
    pub tag_name: String,
    pub parent: Option<usize>, // 存储父节点在 arena 中的索引
    pub children: Vec<usize>,  // 存储子节点在 arena 中的索引
}

#[derive(Debug)]
pub struct DOM {
    pub nodes: Vec<DOMNode>, // Arena: 所有节点都存储在这里
}

impl DOM {
    /// 创建一个新的、空的 DOM。
    pub fn new() -> Self {
        DOM { nodes: Vec::new() }
    }

    /// 向 DOM 中添加一个新节点。
    /// 返回新节点的索引。
    pub fn add_node(&mut self, tag_name: &str, parent_index: Option<usize>) -> usize {
        let new_node_index = self.nodes.len();

        let new_node = DOMNode {
            tag_name: tag_name.to_string(),
            parent: parent_index,
            children: Vec::new(),
        };

        self.nodes.push(new_node);

        // 如果有父节点，将当前节点作为子节点添加到父节点的 children 列表中
        if let Some(p_idx) = parent_index {
            if let Some(parent_node) = self.nodes.get_mut(p_idx) {
                parent_node.children.push(new_node_index);
            }
        }

        new_node_index
    }
}

/// 辅助函数，用于在 DOM 树中查找所有特定标签的节点的索引。
pub fn find_nodes_by_tag(dom: &DOM, tag_name: &str) -> Vec<usize> {
    dom.nodes
        .iter()
        .enumerate() // 获取 (index, node) 对
        .filter(|(_, node)| node.tag_name == tag_name)
        .map(|(index, _)| index)
        .collect()
}

/// 表示一个非确定性有限状态自动机 (NFA)。
#[derive(Debug, PartialEq, Eq)]
pub struct NFA {
    /// NFA 中所有状态的集合。
    pub states: HashSet<usize>,
    /// 转移函数，格式为: {当前状态: {输入符号: 下一个状态}}
    pub transitions: HashMap<usize, HashMap<String, usize>>,
    /// 起始状态。
    pub start_state: usize,
}

pub fn generate_nfa(selector: &str) -> NFA {
    let t = selector.replace('>', " > ");
    let mut parts: Vec<&str> = t.split_whitespace().collect();

    // 如果选择器为空，则返回一个永远不会匹配的空NFA。
    if parts.is_empty() {
        return NFA {
            states: HashSet::new(),
            transitions: HashMap::new(),
            start_state: 0,
        };
    }

    // 2. 弹出目标标签。它定义了我们要检查的节点类型，但不属于祖先匹配规则。
    parts.pop();

    // --- NFA 初始化 ---
    let start_state = 0;
    let mut states: HashSet<usize> = [start_state].into_iter().collect();
    let mut transitions = HashMap::<_, HashMap<_, usize>>::new();

    let mut state_counter = 1;
    // `from_state` 是状态链中较靠近起始状态（即选择器右侧）的状态。
    let mut from_state = start_state;

    // 3. 从右向左遍历规则（通过从Vec末尾pop）。
    while let Some(part) = parts.pop() {
        let (tag_name, is_descendant) = if part == ">" {
            // 如果是子代组合器，它修饰的是它左边的标签，所以我们再pop一次。
            // `unwrap` 在这里是可接受的，因为一个格式正确的选择器在 `>` 前面一定有内容。
            let tag = parts
                .pop()
                .expect("Invalid selector: '>' must be preceded by a tag.");
            (tag, false) // 标记为非后代（即子代）
        } else {
            // 如果是标签名，则意味着是隐式的后代组合器。
            (part, true) // 标记为后代
        };

        // --- 为这条规则构建NFA片段 ---
        let to_state = state_counter;
        states.insert(to_state);

        // 创建从上一个状态到新状态的转移。
        // `entry().or_default()` 是一个非常方便的模式，用于处理嵌套的HashMap。
        transitions
            .entry(from_state)
            .or_default()
            .insert(tag_name.to_string(), to_state);

        // 5. 添加传播规则：如果是后代选择器，就在新状态上添加一个自循环。
        if is_descendant {
            // 关键修复：通配跳过应加在"当前(from_state)"上，表示在匹配到该标签前可跳过任意祖先
            transitions
                .entry(from_state)
                .or_default()
                .insert("*".to_string(), from_state);
        }

        // 为下一次迭代更新状态
        from_state = to_state;
        state_counter += 1;
    }

    NFA {
        states,
        transitions,
        start_state,
    }
}

/// NFA 匹配引擎。
///
/// 从目标节点的父节点开始，向上遍历祖先链，并根据 NFA 规则转换状态。
///
/// # Arguments
/// * `nfa` - 用于匹配的 NFA。
/// * `dom` - 包含所有节点的 DOM 结构。
/// * `target_node_index` - 我们要检查是否匹配的节点的索引。
///
pub fn nfa_match(nfa: &NFA, dom: &DOM, target_node_index: usize) -> bool {
    let mut current_state = nfa.start_state;

    // 定义"真实后继"：有至少一个非 "*" 的出边
    let mut has_real_successor = nfa
        .transitions
        .get(&current_state)
        .map(|m| m.keys().any(|k| k != "*"))
        .unwrap_or(false);

    // 若起始状态就没有真实后继，说明无需匹配任何祖先，直接成功
    if !has_real_successor {
        return true;
    }

    // 从目标节点的父节点开始向上遍历
    let mut current_node_opt = dom
        .nodes
        .get(target_node_index)
        .and_then(|node| node.parent);

    while let Some(node_index) = current_node_opt {
        let ancestor_node = &dom.nodes[node_index];
        let input_symbol = &ancestor_node.tag_name;

        // 当前状态的转移
        match nfa.transitions.get(&current_state) {
            Some(state_transitions) => {
                // 优先匹配精确标签
                if let Some(&next_state) = state_transitions.get(input_symbol) {
                    current_state = next_state;

                    // 成功转移后，若新状态没有"真实后继"，则匹配完成
                    has_real_successor = nfa
                        .transitions
                        .get(&current_state)
                        .map(|m| m.keys().any(|k| k != "*"))
                        .unwrap_or(false);

                    if !has_real_successor {
                        return true;
                    }

                    // 继续向上
                    current_node_opt = ancestor_node.parent;
                    continue;
                }

                // 没有精确匹配，但允许通配符"跳过"则跳过当前祖先
                if state_transitions.contains_key("*") {
                    current_node_opt = ancestor_node.parent;
                    continue;
                }

                // 既没有精确匹配，也没有"*"，说明此处必须是紧邻父代；失败
                return false;
            }
            None => {
                // 没有更多转移，说明需求已满足
                return true;
            }
        }
    }

    // 走到根也未满足所有必需的规则
    false
}

fn main() {
    // 1. 构建 DOM 树
    let mut dom = DOM::new();
    // 根节点 <body> 是索引 0
    let body_idx = dom.add_node("body", None);
    // <div> 是 <body> 的子节点
    let div_1_idx = dom.add_node("div", Some(body_idx));
    // <p> 是 <body> 的子节点
    dom.add_node("p", Some(body_idx));
    // <p> 是 <div> 的子节点
    dom.add_node("p", Some(div_1_idx));
    // <section> 是 <div> 的子节点
    let section_idx = dom.add_node("section", Some(div_1_idx));
    // <p> 是 <section> 的子节点
    dom.add_node("p", Some(section_idx));
    dbg!(&dom);

    // 2. 定义我们要测试的选择器
    let selectors_to_test = ["div > p", "div p", "body div > p", "section p"];

    // 3. 循环测试每个选择器
    for selector in selectors_to_test {
        println!("\n// === 测试选择器: \"{}\" === //", selector);

        // 3.1 动态生成 NFA
        let nfa = generate_nfa(selector);
        dbg!(&nfa);

        // 3.2 识别目标节点标签
        let t = selector.replace('>', " > ");
        let target_tag = t.split_whitespace().last().unwrap();
        println!("目标节点: <{}>\n", target_tag);

        // 3.3 在 DOM 树中查找所有目标节点并进行匹配
        let nodes_to_check = find_nodes_by_tag(&dom, target_tag);

        for node_idx in nodes_to_check {
            let node = dom.nodes.get(node_idx).unwrap();
            let parent_tag = node
                .parent
                .and_then(|p_idx| dom.nodes.get(p_idx))
                .map_or("None", |p_node| &p_node.tag_name);

            let is_match = nfa_match(&nfa, &dom, node_idx);

            println!(
                "节点 <{}> (索引 {}) [父节点: <{}>] 是否匹配?  {}",
                node.tag_name,
                node_idx,
                parent_tag,
                if is_match { "是" } else { "否" }
            );
        }
    }
}
