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
    pub tag_name: String,         // 标签名
    pub classes: HashSet<String>, // CSS类列表
    pub html_id: Option<String>,  // HTML ID属性
    pub parent: Option<usize>,    // 存储父节点在 arena 中的索引
    pub children: Vec<usize>,     // 存储子节点在 arena 中的索引
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

        let new_node = DOMNode {
            tag_name: tag_name.to_string(),
            classes: classes.into_iter().collect(),
            html_id,
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

    /// 检查节点是否匹配给定的选择器
    pub fn node_matches_selector(&self, node_index: usize, selector: &Selector) -> bool {
        if let Some(node) = self.nodes.get(node_index) {
            match selector {
                Selector::Type(tag) => node.tag_name.to_lowercase() == tag.to_lowercase(),
                Selector::Class(class) => node.classes.contains(class),
                Selector::Id(id) => {
                    if let Some(ref html_id) = node.html_id {
                        html_id == id
                    } else {
                        false
                    }
                }
            }
        } else {
            false
        }
    }
}

/// 辅助函数，用于在 DOM 树中查找所有匹配特定选择器的节点的索引。
pub fn find_nodes_by_selector(dom: &DOM, selector: &Selector) -> Vec<usize> {
    dom.nodes
        .iter()
        .enumerate()
        .filter(|(index, _)| dom.node_matches_selector(*index, selector))
        .map(|(index, _)| index)
        .collect()
}

/// 便捷函数：根据标签名查找节点
pub fn find_nodes_by_tag(dom: &DOM, tag_name: &str) -> Vec<usize> {
    find_nodes_by_selector(dom, &Selector::Type(tag_name.to_string()))
}

/// 便捷函数：根据类名查找节点
pub fn find_nodes_by_class(dom: &DOM, class_name: &str) -> Vec<usize> {
    find_nodes_by_selector(dom, &Selector::Class(class_name.to_string()))
}

/// 便捷函数：根据ID查找节点
pub fn find_nodes_by_id(dom: &DOM, id_name: &str) -> Vec<usize> {
    find_nodes_by_selector(dom, &Selector::Id(id_name.to_string()))
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
    let mut parts: Vec<&str> = t.split_whitespace().collect();

    // 如果选择器为空，则返回一个永远不会匹配的空NFA。
    if parts.is_empty() {
        return NFA {
            states: HashSet::new(),
            transitions: HashMap::new(),
            start_state: 0,
        };
    }

    // 2. 弹出目标选择器。它定义了我们要检查的节点类型，但不属于祖先匹配规则。
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
        let (selector_str, is_descendant) = if part == ">" {
            // 如果是子代组合器，它修饰的是它左边的选择器，所以我们再pop一次。
            // `unwrap` 在这里是可接受的，因为一个格式正确的选择器在 `>` 前面一定有内容。
            let sel = parts
                .pop()
                .expect("Invalid selector: '>' must be preceded by a selector.");
            (sel, false) // 标记为非后代（即子代）
        } else {
            // 如果是选择器，则意味着是隐式的后代组合器。
            (part, true) // 标记为后代
        };

        // --- 为这条规则构建NFA片段 ---
        let to_state = state_counter;
        states.insert(to_state);

        // 解析选择器并获取对应的ID
        let selector = parse_selector(selector_str);
        let selector_id = selector_manager.get_or_create_id(selector);

        // 创建从上一个状态到新状态的转移。
        // `entry().or_default()` 是一个非常方便的模式，用于处理嵌套的HashMap。
        transitions
            .entry(from_state)
            .or_default()
            .insert(selector_id, to_state);

        // 5. 添加传播规则：如果是后代选择器，就在新状态上添加一个自循环。
        if is_descendant {
            // 关键修复：通配跳过应加在"当前(from_state)"上，表示在匹配到该选择器前可跳过任意祖先
            // 通配符 "*" 的ID是0
            transitions
                .entry(from_state)
                .or_default()
                .insert(0, from_state);
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

    // 定义"真实后继"：有至少一个非 "0"（通配符ID）的出边
    let mut has_real_successor = nfa
        .transitions
        .get(&current_state)
        .map(|m| m.keys().any(|&k| k != 0))
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

        // 当前状态的转移
        match nfa.transitions.get(&current_state) {
            Some(state_transitions) => {
                let mut found_match = false;

                // 检查所有可能的选择器匹配
                for (&selector_id, &next_state) in state_transitions {
                    if selector_id == 0 {
                        // 跳过通配符，稍后处理
                        continue;
                    }

                    if let Some(selector) = dom.selector_manager.get_selector(selector_id) {
                        if dom.node_matches_selector(node_index, selector) {
                            // 找到匹配的选择器
                            current_state = next_state;
                            found_match = true;

                            // 成功转移后，若新状态没有"真实后继"，则匹配完成
                            has_real_successor = nfa
                                .transitions
                                .get(&current_state)
                                .map(|m| m.keys().any(|&k| k != 0))
                                .unwrap_or(false);

                            if !has_real_successor {
                                return true;
                            }

                            break;
                        }
                    }
                }

                if found_match {
                    // 找到了精确匹配，继续向上
                    current_node_opt = ancestor_node.parent;
                    continue;
                }

                // 没有精确匹配，但允许通配符"跳过"（ID为0）则跳过当前祖先
                if state_transitions.contains_key(&0) {
                    current_node_opt = ancestor_node.parent;
                    continue;
                }

                // 既没有精确匹配，也没有通配符，说明此处必须是紧邻父代；失败
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
        let classes_str = if node.classes.is_empty() {
            "无".to_string()
        } else {
            node.classes.iter().cloned().collect::<Vec<_>>().join(" ")
        };

        let id_str = node.html_id.as_deref().unwrap_or("无");

        let n = "None".to_string();
        let parent_tag = node
            .parent
            .and_then(|p_idx| dom.nodes.get(p_idx))
            .map(|p_node| &p_node.tag_name)
            .unwrap_or(&n);

        println!(
            "索引 {}: <{}> [类: {}] [ID: {}] [父节点: <{}>]",
            idx, node.tag_name, classes_str, id_str, parent_tag
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
    ];

    // 3. 循环测试每个选择器
    for selector in selectors_to_test {
        println!("\n// === 测试选择器: \"{}\" === //", selector);

        // 3.1 动态生成 NFA
        let nfa = generate_nfa(selector, &mut dom.selector_manager);
        println!("NFA 状态: {:?}", nfa.states);
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

        // 3.2 识别目标选择器
        let t = selector.replace('>', " > ");
        let target_selector_str = t.split_whitespace().last().unwrap();
        let target_selector = parse_selector(target_selector_str);
        println!("目标选择器: {:?}\n", target_selector);

        // 3.3 在 DOM 树中查找所有目标节点并进行匹配
        let nodes_to_check = find_nodes_by_selector(&dom, &target_selector);

        for node_idx in nodes_to_check {
            let node = dom.nodes.get(node_idx).unwrap();
            let classes_str = if node.classes.is_empty() {
                "无".to_string()
            } else {
                node.classes.iter().cloned().collect::<Vec<_>>().join(" ")
            };
            let id_str = node.html_id.as_deref().unwrap_or("无");
            let n = "None".to_string();
            let parent_tag = node
                .parent
                .and_then(|p_idx| dom.nodes.get(p_idx))
                .map(|p_node| &p_node.tag_name)
                .unwrap_or(&n);

            let is_match = nfa_match(&nfa, &dom, node_idx);

            println!(
                "节点 <{}> [类: {}] [ID: {}] (索引 {}) [父节点: <{}>] 是否匹配?  {}",
                node.tag_name,
                classes_str,
                id_str,
                node_idx,
                parent_tag,
                if is_match { "是" } else { "否" }
            );
        }
    }
}
