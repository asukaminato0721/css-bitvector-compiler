use std::collections::{HashMap, HashSet};

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
    pub selector_to_id: HashMap<Selector, usize>,
    /// 从ID到选择器的映射
    pub id_to_selector: HashMap<usize, Selector>,
    /// 下一个可用的ID
    next_id: usize,
}

impl SelectorManager {
    /// 创建一个新的选择器管理器，其中ID 0 保留给通配符 "*"
    pub fn new() -> Self {
        let mut manager = SelectorManager {
            selector_to_id: HashMap::new(),
            id_to_selector: HashMap::new(),
            next_id: 1, // 从1开始，因为0保留给通配符
        };

        // 预先注册通配符
        let wildcard_selector = Selector::Type("*".to_string());
        manager.selector_to_id.insert(wildcard_selector.clone(), 0);
        manager.id_to_selector.insert(0, wildcard_selector);

        manager
    }

    /// 获取选择器对应的ID，如果不存在则创建新的ID
    pub fn get_or_create_id(&mut self, selector: Selector) -> usize {
        if let Some(&id) = self.selector_to_id.get(&selector) {
            return id;
        }

        let id = self.next_id;
        self.selector_to_id.insert(selector.clone(), id);
        self.id_to_selector.insert(id, selector);
        self.next_id += 1;
        id
    }

    /// 根据ID获取选择器
    pub fn get_selector(&self, id: usize) -> Option<&Selector> {
        self.id_to_selector.get(&id)
    }

    /// 根据选择器获取ID
    pub fn get_id(&self, selector: &Selector) -> Option<usize> {
        self.selector_to_id.get(selector).copied()
    }

    /// 便捷方法：根据标签名获取或创建类型选择器ID
    pub fn get_or_create_type_id(&mut self, tag_name: &str) -> usize {
        self.get_or_create_id(Selector::Type(tag_name.to_string()))
    }

    /// 便捷方法：根据类名获取或创建类选择器ID
    pub fn get_or_create_class_id(&mut self, class_name: &str) -> usize {
        self.get_or_create_id(Selector::Class(class_name.to_string()))
    }

    /// 便捷方法：根据ID获取或创建ID选择器ID
    pub fn get_or_create_id_selector_id(&mut self, id_name: &str) -> usize {
        self.get_or_create_id(Selector::Id(id_name.to_string()))
    }
}

#[derive(Debug)]
pub struct DOMNode {
    pub tag_id: usize,                 // 标签选择器ID
    pub class_ids: HashSet<usize>,     // CSS类选择器ID集合
    pub id_selector_id: Option<usize>, // HTML ID选择器ID
    pub parent: Option<usize>,         // 存储父节点在 arena 中的索引
    pub children: Vec<usize>,          // 存储子节点在 arena 中的索引
}

#[derive(Debug)]
pub struct DOM {
    pub nodes: Vec<DOMNode>,               // Arena: 所有节点都存储在这里
    pub selector_manager: SelectorManager, // 选择器管理器
}

impl DOM {
    /// 创建一个新的、空的 DOM。
    pub fn new() -> Self {
        DOM {
            nodes: Vec::new(),
            selector_manager: SelectorManager::new(),
        }
    }

    /// 向 DOM 中添加一个新节点。
    /// 返回新节点的索引。
    pub fn add_node(
        &mut self,
        tag_name: &str,
        classes: Vec<String>,
        html_id: Option<String>,
        parent_index: Option<usize>,
    ) -> usize {
        let new_node_index = self.nodes.len();

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

    /// 便捷方法：添加只有标签名的节点
    pub fn add_simple_node(&mut self, tag_name: &str, parent_index: Option<usize>) -> usize {
        self.add_node(tag_name, vec![], None, parent_index)
    }

    /// 检查节点是否匹配给定的选择器ID（优化版本，使用usize比较）
    pub fn node_matches_selector(&self, node_index: usize, selector_id: usize) -> bool {
        if let Some(node) = self.nodes.get(node_index) {
            // 通配符匹配所有节点
            if selector_id == 0 {
                return true;
            }

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

    /// 获取所有根节点（没有父节点的节点）
    pub fn get_root_nodes(&self) -> Vec<usize> {
        self.nodes
            .iter()
            .enumerate()
            .filter(|(_, node)| node.parent.is_none())
            .map(|(idx, _)| idx)
            .collect()
    }
}

/// 辅助函数，用于在 DOM 树中查找所有匹配特定选择器的节点的索引。
pub fn find_nodes_by_selector_id(dom: &DOM, selector_id: usize) -> Vec<usize> {
    dom.nodes
        .iter()
        .enumerate()
        .filter(|(index, _)| dom.node_matches_selector(*index, selector_id))
        .map(|(index, _)| index)
        .collect()
}

/// 便捷函数：根据标签名查找节点
pub fn find_nodes_by_tag(dom: &DOM, tag_name: &str) -> Vec<usize> {
    let selector = Selector::Type(tag_name.to_string());
    if let Some(selector_id) = dom.selector_manager.get_id(&selector) {
        find_nodes_by_selector_id(dom, selector_id)
    } else {
        vec![]
    }
}

/// 便捷函数：根据类名查找节点
pub fn find_nodes_by_class(dom: &DOM, class_name: &str) -> Vec<usize> {
    let selector = Selector::Class(class_name.to_string());
    if let Some(selector_id) = dom.selector_manager.get_id(&selector) {
        find_nodes_by_selector_id(dom, selector_id)
    } else {
        vec![]
    }
}

/// 便捷函数：根据ID查找节点
pub fn find_nodes_by_id(dom: &DOM, id_name: &str) -> Vec<usize> {
    let selector = Selector::Id(id_name.to_string());
    if let Some(selector_id) = dom.selector_manager.get_id(&selector) {
        find_nodes_by_selector_id(dom, selector_id)
    } else {
        vec![]
    }
}

/// 表示一个非确定性有限状态自动机 (NFA)。
#[derive(Debug, PartialEq, Eq)]
pub struct NFA {
    /// NFA 中所有状态的集合。
    pub states: HashSet<usize>,
    /// 转移函数，格式为: {当前状态: {输入选择器ID: 下一个状态}}
    pub transitions: HashMap<usize, HashMap<usize, usize>>,
    /// 起始状态。
    pub start_state: usize,
}
impl NFA {
    /// 检查给定状态是否为接受状态（没有后继状态）
    pub fn is_accept_state(&self, state: usize) -> bool {
        !self.transitions.contains_key(&state)
            || self
                .transitions
                .get(&state)
                .map_or(true, |trans| trans.is_empty())
    }

    /// 获取所有接受状态
    pub fn get_accept_states(&self) -> HashSet<usize> {
        self.states
            .iter()
            .filter(|&&state| self.is_accept_state(state))
            .copied()
            .collect()
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

pub fn generate_nfa(selector: &str, selector_manager: &mut SelectorManager) -> NFA {
    let t = selector.replace('>', " > ");
    let parts: Vec<&str> = t.split_whitespace().collect();

    // 如果选择器为空，则返回一个永远不会匹配的空NFA。
    if parts.is_empty() {
        return NFA {
            states: HashSet::new(),
            transitions: HashMap::new(),
            start_state: 0,
        };
    }

    // --- NFA 初始化 ---
    let start_state = 0;
    let mut states: HashSet<usize> = [start_state].into_iter().collect();
    let mut transitions = HashMap::<_, HashMap<_, usize>>::new();

    let mut state_counter = 1;
    let mut current_state = start_state;

    // 从左往右处理选择器部分
    let mut i = 0;
    while i < parts.len() {
        let part = parts[i];

        if part == ">" {
            // 子代组合器，跳过它，下一个选择器是直接子元素
            i += 1;
            if i >= parts.len() {
                break;
            }
            let selector_str = parts[i];

            // 创建新状态
            let new_state = state_counter;
            states.insert(new_state);

            // 解析选择器并获取对应的ID
            let selector = parse_selector(selector_str);
            let selector_id = selector_manager.get_or_create_id(selector);

            // 创建直接转移（不允许跳过中间节点）
            transitions
                .entry(current_state)
                .or_default()
                .insert(selector_id, new_state);

            current_state = new_state;
            state_counter += 1;
        } else {
            // 后代选择器（隐式或显式）
            let selector_str = part;

            // 创建新状态
            let new_state = state_counter;
            states.insert(new_state);

            // 解析选择器并获取对应的ID
            let selector = parse_selector(selector_str);
            let selector_id = selector_manager.get_or_create_id(selector);

            // 创建转移到新状态
            transitions
                .entry(current_state)
                .or_default()
                .insert(selector_id, new_state);

            // 添加自循环，允许跳过不匹配的中间节点（通配符）
            transitions
                .entry(current_state)
                .or_default()
                .insert(0, current_state);

            current_state = new_state;
            state_counter += 1;
        }

        i += 1;
    }

    NFA {
        states,
        transitions,
        start_state,
    }
}

/// 新的 NFA 匹配引擎。
///
/// 从根节点开始，向下遍历所有子节点，并根据 NFA 规则转换状态。
///
/// # Arguments
/// * `nfa` - 用于匹配的 NFA。
/// * `dom` - 包含所有节点的 DOM 结构。
///
pub fn nfa_match(nfa: &NFA, dom: &DOM) -> Vec<usize> {
    let mut matches = HashSet::new(); // 使用 HashSet 避免重复
    let root_nodes = dom.get_root_nodes();

    for root_idx in root_nodes {
        nfa_match_recursive(nfa, dom, root_idx, nfa.start_state, &mut matches);
    }

    matches.into_iter().collect()
}

/// 递归匹配函数（完全优化版本）
fn nfa_match_recursive(
    nfa: &NFA,
    dom: &DOM,
    node_idx: usize,
    current_state: usize,
    matches: &mut HashSet<usize>,
) {
    // 检查当前节点是否可以进行状态转移
    if let Some(state_transitions) = nfa.transitions.get(&current_state) {
        let mut state_advanced = false;

        // 尝试匹配所有可能的选择器（除了通配符）
        for (&selector_id, &next_state) in state_transitions {
            if selector_id == 0 {
                continue; // 先跳过通配符，稍后处理
            }

            // 直接使用selector_id进行匹配，无需字符串比较
            if dom.node_matches_selector(node_idx, selector_id) {
                // 匹配成功，转移到下一个状态
                if nfa.is_accept_state(next_state) {
                    // 如果下一个状态是接受状态，记录匹配
                    matches.insert(node_idx);
                }

                // 继续从新状态匹配子节点
                for &child_idx in &dom.nodes[node_idx].children {
                    nfa_match_recursive(nfa, dom, child_idx, next_state, matches);
                }

                state_advanced = true;
            }
        }

        // 如果没有状态推进，并且有通配符转移，则使用通配符继续在当前状态
        if !state_advanced && state_transitions.contains_key(&0) {
            for &child_idx in &dom.nodes[node_idx].children {
                nfa_match_recursive(nfa, dom, child_idx, current_state, matches);
            }
        }
    }

    // 如果没有转移规则，继续遍历子节点（保持当前状态）
    if !nfa.transitions.contains_key(&current_state) {
        for &child_idx in &dom.nodes[node_idx].children {
            nfa_match_recursive(nfa, dom, child_idx, current_state, matches);
        }
    }
}
fn main() {
    // 1. 构建 DOM 树
    let mut dom = DOM::new();

    // 根节点 <body> 是索引 0
    let body_idx = dom.add_simple_node("body", None);

    // <div class="container"> 是 <body> 的子节点
    let div_1_idx = dom.add_node("div", vec!["container".to_string()], None, Some(body_idx));

    // <p class="text"> 是 <body> 的子节点
    let p_1_idx = dom.add_node("p", vec!["text".to_string()], None, Some(body_idx));

    // <p> 是 <div> 的子节点
    let p_2_idx = dom.add_simple_node("p", Some(div_1_idx));

    // <section id="main"> 是 <div> 的子节点
    let section_idx = dom.add_node("section", vec![], Some("main".to_string()), Some(div_1_idx));

    // <p class="highlight"> 是 <section> 的子节点
    let p_3_idx = dom.add_node("p", vec!["highlight".to_string()], None, Some(section_idx));

    println!("DOM 结构:");
    for (idx, node) in dom.nodes.iter().enumerate() {
        // 根据ID获取对应的字符串用于显示
        let tag_name = dom
            .selector_manager
            .get_selector(node.tag_id)
            .map(|s| match s {
                Selector::Type(name) => name.as_str(),
                _ => "unknown",
            })
            .unwrap_or("unknown");

        let classes_str = if node.class_ids.is_empty() {
            "无".to_string()
        } else {
            node.class_ids
                .iter()
                .filter_map(|&id| dom.selector_manager.get_selector(id))
                .filter_map(|s| match s {
                    Selector::Class(name) => Some(name.as_str()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join(" ")
        };

        let id_str = if let Some(id_sel_id) = node.id_selector_id {
            dom.selector_manager
                .get_selector(id_sel_id)
                .and_then(|s| match s {
                    Selector::Id(name) => Some(name.as_str()),
                    _ => None,
                })
                .unwrap_or("无")
        } else {
            "无"
        };

        let parent_tag = node
            .parent
            .and_then(|p_idx| dom.nodes.get(p_idx))
            .and_then(|p_node| dom.selector_manager.get_selector(p_node.tag_id))
            .map(|s| match s {
                Selector::Type(name) => name.as_str(),
                _ => "unknown",
            })
            .unwrap_or("None");

        println!(
            "索引 {}: <{}> [类: {}] [ID: {}] [父节点: <{}>]",
            idx, tag_name, classes_str, id_str, parent_tag
        );
    }

    println!("\n选择器ID映射:");
    for (selector, &id) in &dom.selector_manager.selector_to_id {
        println!("{:?} -> ID {}", selector, id);
    }

    // 2. 定义我们要测试的选择器，包括类选择器
    let selectors_to_test = [
        "div > p", // 标签选择器
        "div p",
        "body .container > p",   // 混合选择器
        ".highlight",            // 类选择器
        "#main p",               // ID选择器 + 标签选择器
        ".container .highlight", // 类选择器组合
        "section .highlight",    // 标签 + 类选择器
        "div section",
    ];

    // 3. 循环测试每个选择器
    for selector in selectors_to_test {
        println!("\n// === 测试选择器: \"{}\" === //", selector);

        // 3.1 动态生成 NFA
        let nfa = generate_nfa(selector, &mut dom.selector_manager);
        println!("NFA 状态: {:?}", nfa.states);
        println!("NFA 接受状态: {:?}", nfa.get_accept_states());
        println!("NFA 转移:");
        for (from_state, transitions) in &nfa.transitions {
            for (selector_id, to_state) in transitions {
                if let Some(sel) = dom.selector_manager.get_selector(*selector_id) {
                    println!(
                        "  状态 {} --[{:?}(ID:{})]-> 状态 {}",
                        from_state, sel, selector_id, to_state
                    );
                } else {
                    println!(
                        "  状态 {} --[通配符(ID:{})]-> 状态 {}",
                        from_state, selector_id, to_state
                    );
                }
            }
        }

        // 3.2
        let matched_nodes = nfa_match(&nfa, &dom);

        println!("匹配结果:");
        if matched_nodes.is_empty() {
            println!("  无匹配节点");
        } else {
            for &node_idx in &matched_nodes {
                let node = dom.nodes.get(node_idx).unwrap();

                let tag_name = dom
                    .selector_manager
                    .get_selector(node.tag_id)
                    .map(|s| match s {
                        Selector::Type(name) => name.as_str(),
                        _ => "unknown",
                    })
                    .unwrap_or("unknown");

                let classes_str = if node.class_ids.is_empty() {
                    "无".to_string()
                } else {
                    node.class_ids
                        .iter()
                        .filter_map(|&id| dom.selector_manager.get_selector(id))
                        .filter_map(|s| match s {
                            Selector::Class(name) => Some(name.as_str()),
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                        .join(" ")
                };

                let id_str = if let Some(id_sel_id) = node.id_selector_id {
                    dom.selector_manager
                        .get_selector(id_sel_id)
                        .and_then(|s| match s {
                            Selector::Id(name) => Some(name.as_str()),
                            _ => None,
                        })
                        .unwrap_or("无")
                } else {
                    "无"
                };

                let parent_tag = node
                    .parent
                    .and_then(|p_idx| dom.nodes.get(p_idx))
                    .and_then(|p_node| dom.selector_manager.get_selector(p_node.tag_id))
                    .map(|s| match s {
                        Selector::Type(name) => name.as_str(),
                        _ => "unknown",
                    })
                    .unwrap_or("None");

                println!(
                    "  节点 <{}> [类: {}] [ID: {}] (索引 {}) [父节点: <{}>]",
                    tag_name, classes_str, id_str, node_idx, parent_tag
                );
            }
        }
    }
}
