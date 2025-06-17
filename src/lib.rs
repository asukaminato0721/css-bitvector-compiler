// Library exports for css-bitvector-compiler
// This allows examples to use the types and functions as a library

use std::collections::{HashMap, HashSet};

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::_rdtsc;

// RDTSC 时间测量工具
#[inline(always)]
pub fn rdtsc() -> u64 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        _rdtsc()
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        // 对于非 x86_64 架构，回退到 nanos
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
    }
}

// 计算两个 RDTSC 读数之间的 CPU 周期数
pub fn cycles_to_duration(start_cycles: u64, end_cycles: u64) -> u64 {
    end_cycles.saturating_sub(start_cycles)
}

// All types are now defined directly in this file

// Export Google trace types
#[derive(Debug, Clone)]
pub struct GoogleNode {
    pub id: Option<u32>,
    pub name: String,
    pub node_type: String,
    pub namespace: Option<String>,
    pub attributes: std::collections::HashMap<String, String>,
    pub properties: std::collections::HashMap<String, String>,
    pub visible: bool,
    pub children: Vec<GoogleNode>,
}

impl GoogleNode {
    pub fn from_json(value: &serde_json::Value) -> Option<Self> {
        let obj = value.as_object()?;

        Some(GoogleNode {
            id: obj.get("id").and_then(|v| v.as_u64()).map(|v| v as u32),
            name: obj
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            node_type: obj
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            namespace: obj
                .get("namespace")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            attributes: obj
                .get("attributes")
                .and_then(|v| v.as_object())
                .map(|attrs| {
                    attrs
                        .iter()
                        .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                        .collect()
                })
                .unwrap_or_default(),
            properties: obj
                .get("properties")
                .and_then(|v| v.as_object())
                .map(|props| {
                    props
                        .iter()
                        .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                        .collect()
                })
                .unwrap_or_default(),
            visible: obj.get("visible").and_then(|v| v.as_bool()).unwrap_or(true),
            children: obj
                .get("children")
                .and_then(|v| v.as_array())
                .map(|children| children.iter().filter_map(GoogleNode::from_json).collect())
                .unwrap_or_default(),
        })
    }

    pub fn to_html_node(&self) -> HtmlNode {
        let mut node = HtmlNode::new(&self.name);

        if let Some(id) = &self.id {
            node.id = Some(id.to_string());
        }

        // Extract classes from attributes
        if let Some(class_attr) = self.attributes.get("class") {
            node.classes = class_attr
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
        }

        // Convert children
        for child in &self.children {
            node.children.push(child.to_html_node());
        }

        node
    }

    pub fn count_nodes(&self) -> usize {
        1 + self
            .children
            .iter()
            .map(|child| child.count_nodes())
            .sum::<usize>()
    }
}

// Export BitVector
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitVector {
    pub bits: u64,
}

impl Default for BitVector {
    fn default() -> Self {
        Self::new()
    }
}

impl BitVector {
    pub fn new() -> Self {
        BitVector { bits: 0 }
    }

    pub fn from_u64(bits: u64) -> Self {
        BitVector { bits }
    }

    pub fn set_bit(&mut self, pos: usize) {
        self.bits |= 1 << pos;
    }

    pub fn clear_bit(&mut self, pos: usize) {
        self.bits &= !(1 << pos);
    }

    pub fn is_bit_set(&self, pos: usize) -> bool {
        (self.bits & (1 << pos)) != 0
    }

    pub fn or_assign(&mut self, other: BitVector) {
        self.bits |= other.bits;
    }

    pub fn and(&self, other: BitVector) -> BitVector {
        BitVector {
            bits: self.bits & other.bits,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.bits == 0
    }

    pub fn as_u64(&self) -> u64 {
        self.bits
    }

    pub fn has_any_bits(&self, mask: BitVector) -> bool {
        (self.bits & mask.bits) != 0
    }
}

impl std::fmt::Binary for BitVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:016b}", self.bits)
    }
}

// Export selector matching index
#[derive(Debug, Clone)]
pub struct SelectorMatchingIndex {
    pub tag_rules: HashMap<String, Vec<(usize, NFAInstruction)>>,
    pub class_rules: HashMap<String, Vec<(usize, NFAInstruction)>>,
    pub id_rules: HashMap<String, Vec<(usize, NFAInstruction)>>,
    pub parent_dependent_rules: Vec<(usize, NFAInstruction)>,
}

impl Default for SelectorMatchingIndex {
    fn default() -> Self {
        Self::new()
    }
}

impl SelectorMatchingIndex {
    pub fn new() -> Self {
        Self {
            tag_rules: HashMap::new(),
            class_rules: HashMap::new(),
            id_rules: HashMap::new(),
            parent_dependent_rules: Vec::new(),
        }
    }

    pub fn add_rule(&mut self, rule_id: usize, instruction: NFAInstruction) {
        match &instruction {
            NFAInstruction::CheckAndSetBit { selector, .. } => match selector {
                SimpleSelector::Type(tag) => {
                    self.tag_rules
                        .entry(tag.clone())
                        .or_default()
                        .push((rule_id, instruction));
                }
                SimpleSelector::Class(class) => {
                    self.class_rules
                        .entry(class.clone())
                        .or_default()
                        .push((rule_id, instruction));
                }
                SimpleSelector::Id(id) => {
                    self.id_rules
                        .entry(id.clone())
                        .or_default()
                        .push((rule_id, instruction));
                }
            },
            NFAInstruction::CheckParentAndSetBit { .. } => {
                self.parent_dependent_rules.push((rule_id, instruction));
            }
            NFAInstruction::PropagateToChildren { .. } => {
                // These are processed separately after matching
            }
        }
    }

    pub fn get_matching_rules(&self, node: &HtmlNode) -> Vec<&NFAInstruction> {
        let mut matching_rules = Vec::new();

        // Check tag rules
        if let Some(tag_rules) = self.tag_rules.get(&node.tag_name) {
            for (_, instruction) in tag_rules {
                matching_rules.push(instruction);
            }
        }

        // Check class rules
        for class in &node.classes {
            if let Some(class_rules) = self.class_rules.get(class) {
                for (_, instruction) in class_rules {
                    matching_rules.push(instruction);
                }
            }
        }

        // Check id rules
        if let Some(id) = &node.id {
            if let Some(id_rules) = self.id_rules.get(id) {
                for (_, instruction) in id_rules {
                    matching_rules.push(instruction);
                }
            }
        }

        matching_rules
    }

    pub fn get_parent_dependent_rules(&self) -> &[(usize, NFAInstruction)] {
        &self.parent_dependent_rules
    }
}

// Export HtmlNode structure
#[derive(Debug, Clone)]
pub struct HtmlNode {
    pub tag_name: String,
    pub id: Option<String>,
    pub classes: HashSet<String>,
    pub children: Vec<HtmlNode>,
    pub css_match_bitvector: BitVector,
    pub is_self_dirty: bool,
    pub has_dirty_descendant: bool,
    pub cached_parent_state: Option<BitVector>,
    pub cached_node_intrinsic: Option<BitVector>,
    pub cached_child_states: Option<BitVector>,
    pub parent: Option<*mut HtmlNode>,
}

impl HtmlNode {
    pub fn new(tag_name: &str) -> Self {
        HtmlNode {
            tag_name: tag_name.to_lowercase(),
            id: None,
            classes: HashSet::new(),
            children: Vec::new(),
            css_match_bitvector: BitVector::new(),
            is_self_dirty: true,
            has_dirty_descendant: false,
            cached_parent_state: None,
            cached_node_intrinsic: None,
            cached_child_states: None,
            parent: None,
        }
    }

    pub fn with_id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self.mark_dirty();
        self
    }

    pub fn with_class(mut self, class: &str) -> Self {
        self.classes.insert(class.to_string());
        self.mark_dirty();
        self
    }

    pub fn add_child(mut self, child: HtmlNode) -> Self {
        self.children.push(child);
        self
    }

    fn fix_parent_pointers(&mut self) {
        let self_ptr = self as *mut HtmlNode;
        for child in self.children.iter_mut() {
            child.parent = Some(self_ptr);
        }
        // Fix children's parent pointers recursively in a separate loop
        for child in self.children.iter_mut() {
            child.fix_parent_pointers();
        }
    }

    /// Mark this node as dirty and notify ancestors
    pub fn mark_dirty(&mut self) {
        self.is_self_dirty = true;
        self.cached_node_intrinsic = None;
        self.set_summary_bit_on_ancestors();
    }

    /// Notify ancestors that they have a dirty descendant
    fn set_summary_bit_on_ancestors(&mut self) {
        if let Some(parent_ptr) = self.parent {
            unsafe {
                let parent = &mut *parent_ptr;
                parent.set_summary_bit();
            }
        }
    }

    /// Set summary bit and propagate upward
    pub fn set_summary_bit(&mut self) {
        if self.has_dirty_descendant {
            return;
        }

        self.has_dirty_descendant = true;

        if let Some(parent_ptr) = self.parent {
            unsafe {
                let parent = &mut *parent_ptr;
                parent.set_summary_bit();
            }
        }
    }

    /// Find all dirty nodes in subtree and clean them up
    pub fn find_dirty_nodes(&mut self, dirty_nodes: &mut Vec<*mut HtmlNode>) {
        if self.is_self_dirty {
            dirty_nodes.push(self as *mut HtmlNode);
            self.is_self_dirty = false;
        }

        if self.has_dirty_descendant {
            for child in &mut self.children {
                child.find_dirty_nodes(dirty_nodes);
            }
            self.has_dirty_descendant = false;
        }
    }

    /// Recursively find all dirty nodes regardless of summary bits
    pub fn find_all_dirty_nodes_recursive(&mut self, dirty_nodes: &mut Vec<*mut HtmlNode>) {
        if self.is_self_dirty {
            dirty_nodes.push(self as *mut HtmlNode);
            self.is_self_dirty = false;
        }

        for child in &mut self.children {
            child.find_all_dirty_nodes_recursive(dirty_nodes);
        }

        self.has_dirty_descendant = false;
    }

    /// Collect all dirty nodes
    pub fn collect_dirty_nodes(&mut self) -> Vec<*mut HtmlNode> {
        let mut dirty_nodes = Vec::new();
        self.find_dirty_nodes(&mut dirty_nodes);
        dirty_nodes
    }

    /// Process all dirty nodes with a closure
    pub fn process_dirty_nodes<F>(&mut self, mut processor: F)
    where
        F: FnMut(*mut HtmlNode),
    {
        let dirty_nodes = self.collect_dirty_nodes();
        for node_ptr in dirty_nodes {
            processor(node_ptr);
        }
    }

    /// Check if subtree has dirty nodes
    pub fn has_dirty_nodes(&self) -> bool {
        self.is_self_dirty || self.has_dirty_descendant
    }

    pub fn needs_any_recomputation(&self, new_parent_state: BitVector) -> bool {
        self.is_self_dirty
            || self.has_dirty_descendant
            || self.cached_parent_state.is_none()
            || self.cached_parent_state.unwrap() != new_parent_state
    }

    pub fn mark_clean(&mut self) {
        self.is_self_dirty = false;
        self.has_dirty_descendant = false;
    }

    pub fn mark_child_dirty_by_index(&mut self, child_index: usize) -> bool {
        if child_index >= self.children.len() {
            return false;
        }

        self.children[child_index].mark_dirty();
        true
    }

    pub fn mark_node_dirty_by_path(&mut self, path: &[usize]) -> bool {
        if path.is_empty() {
            self.mark_dirty();
            return true;
        }

        let first_index = path[0];
        if first_index >= self.children.len() {
            return false;
        }

        let success = self.children[first_index].mark_node_dirty_by_path(&path[1..]);
        if success {
            // Mark this node as having dirty descendant
            self.has_dirty_descendant = true;
        }
        success
    }

    pub fn init_parent_pointers(&mut self) {
        self.parent = None;
        self.fix_parent_pointers();
    }

    pub fn find_deep_node_mut(&mut self, target_depth: usize) -> Option<&mut HtmlNode> {
        if target_depth == 0 {
            return Some(self);
        }

        for child in &mut self.children {
            if let Some(found) = child.find_deep_node_mut(target_depth - 1) {
                return Some(found);
            }
        }

        None
    }
}

// Export CSS types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SimpleSelector {
    Type(String),
    Class(String),
    Id(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CssRule {
    Simple(SimpleSelector),
    Child {
        parent_selector: SimpleSelector,
        child_selector: SimpleSelector,
    },
}

#[derive(Debug, Clone)]
pub enum NFAInstruction {
    CheckAndSetBit {
        selector: SimpleSelector,
        bit_pos: usize,
    },
    CheckParentAndSetBit {
        parent_state_bit: usize,
        child_selector: SimpleSelector,
        result_bit: usize,
    },
    PropagateToChildren {
        match_bit: usize,
        active_bit: usize,
    },
}

// Export TreeNFAProgram
#[derive(Debug, Default)]
pub struct TreeNFAProgram {
    pub instructions: Vec<NFAInstruction>,
    pub state_names: HashMap<usize, String>,
    pub total_bits: usize,
}

impl TreeNFAProgram {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_instruction(&mut self, instruction: NFAInstruction) {
        self.instructions.push(instruction);
    }

    pub fn set_state_name(&mut self, bit_pos: usize, name: String) {
        self.state_names.insert(bit_pos, name);
        if bit_pos >= self.total_bits {
            self.total_bits = bit_pos + 1;
        }
    }

    pub fn generate_rust_code(&self) -> String {
        let mut code = String::new();

        code.push_str("// Generated Tree NFA Program with Incremental Processing\n");
        code.push_str(
            "// This program processes HTML nodes and computes CSS matches with caching\n\n",
        );

        // Generate incremental processing function
        code.push_str("pub fn process_node_generated_incremental(\n");
        code.push_str("    node: &mut HtmlNode,\n");
        code.push_str("    parent_state: BitVector,\n");
        code.push_str(") -> BitVector { // returns child_states\n");
        code.push_str("    // Check if we need to recompute\n");
        code.push_str("    if !node.needs_any_recomputation(parent_state) {\n");
        code.push_str("        // Return cached result - entire subtree can be skipped\n");
        code.push_str("        return node.cached_child_states.unwrap_or_default();\n");
        code.push_str("    }\n\n");

        code.push_str("    // Recompute node intrinsic matches if needed\n");
        code.push_str("    if node.cached_node_intrinsic.is_none() || node.is_self_dirty {\n");
        code.push_str("        let mut intrinsic_matches = BitVector::new();\n\n");

        // Generate intrinsic selector checks
        for (i, instruction) in self.instructions.iter().enumerate() {
            if let NFAInstruction::CheckAndSetBit { selector, bit_pos } = instruction {
                code.push_str(&format!(
                    "        // Instruction {}: {:?}\n",
                    i, instruction
                ));
                let selector_str = match selector {
                    SimpleSelector::Type(tag) => {
                        format!("SimpleSelector::Type(\"{}\".to_string())", tag)
                    }
                    SimpleSelector::Class(class) => {
                        format!("SimpleSelector::Class(\"{}\".to_string())", class)
                    }
                    SimpleSelector::Id(id) => {
                        format!("SimpleSelector::Id(\"{}\".to_string())", id)
                    }
                };
                code.push_str(&format!(
                    "        if node_matches_selector_generated(node, &{}) {{\n",
                    selector_str
                ));
                code.push_str(&format!(
                    "            intrinsic_matches.set_bit({}); // {}\n",
                    bit_pos,
                    self.state_names
                        .get(bit_pos)
                        .unwrap_or(&format!("bit_{}", bit_pos))
                ));
                code.push_str("        }\n\n");
            }
        }

        code.push_str("        node.cached_node_intrinsic = Some(intrinsic_matches);\n");
        code.push_str("    }\n\n");

        code.push_str("    // Start with cached intrinsic matches\n");
        code.push_str("    let mut current_matches = node.cached_node_intrinsic.unwrap();\n");
        code.push_str("    let mut child_states = BitVector::new();\n\n");

        // Generate parent-dependent rules
        code.push_str("    // Apply parent-dependent rules\n");
        for (i, instruction) in self.instructions.iter().enumerate() {
            match instruction {
                NFAInstruction::CheckParentAndSetBit {
                    parent_state_bit,
                    child_selector,
                    result_bit,
                } => {
                    code.push_str(&format!("    // Instruction {}: {:?}\n", i, instruction));
                    let child_selector_str = match child_selector {
                        SimpleSelector::Type(tag) => {
                            format!("SimpleSelector::Type(\"{}\".to_string())", tag)
                        }
                        SimpleSelector::Class(class) => {
                            format!("SimpleSelector::Class(\"{}\".to_string())", class)
                        }
                        SimpleSelector::Id(id) => {
                            format!("SimpleSelector::Id(\"{}\".to_string())", id)
                        }
                    };
                    code.push_str(&format!("    if parent_state.is_bit_set({}) && node_matches_selector_generated(node, &{}) {{\n", 
                        parent_state_bit, child_selector_str));
                    code.push_str(&format!(
                        "        current_matches.set_bit({}); // {}\n",
                        result_bit,
                        self.state_names
                            .get(result_bit)
                            .unwrap_or(&format!("bit_{}", result_bit))
                    ));
                    code.push_str("    }\n\n");
                }
                NFAInstruction::PropagateToChildren {
                    match_bit,
                    active_bit,
                } => {
                    code.push_str(&format!("    // Instruction {}: {:?}\n", i, instruction));
                    code.push_str(&format!(
                        "    if current_matches.is_bit_set({}) {{\n",
                        match_bit
                    ));
                    code.push_str(&format!(
                        "        child_states.set_bit({}); // {}\n",
                        active_bit,
                        self.state_names
                            .get(active_bit)
                            .unwrap_or(&format!("bit_{}", active_bit))
                    ));
                    code.push_str("    }\n\n");
                }
                _ => {} // CheckAndSetBit already handled above in intrinsic section
            }
        }

        code.push_str("    // Cache results\n");
        code.push_str("    node.css_match_bitvector = current_matches;\n");
        code.push_str("    node.cached_parent_state = Some(parent_state);\n");
        code.push_str("    node.cached_child_states = Some(child_states);\n");
        code.push_str("    node.mark_clean();\n\n");
        code.push_str("    child_states\n");
        code.push_str("}\n\n");

        // Generate helper function
        code.push_str("pub fn node_matches_selector_generated(node: &HtmlNode, selector: &SimpleSelector) -> bool {\n");
        code.push_str("    match selector {\n");
        code.push_str("        SimpleSelector::Type(tag) => node.tag_name == *tag,\n");
        code.push_str("        SimpleSelector::Class(class) => node.classes.contains(class),\n");
        code.push_str("        SimpleSelector::Id(id) => node.id.as_deref() == Some(id),\n");
        code.push_str("    }\n");
        code.push_str("}\n\n");

        code
    }

    pub fn print_program(&self) {
        println!("=== Generated Tree NFA Program ===");
        println!("Total bits used: {}", self.total_bits);
        println!("\nState mapping:");
        for i in 0..self.total_bits {
            if let Some(name) = self.state_names.get(&i) {
                println!("  Bit {}: {}", i, name);
            }
        }

        println!("\nInstructions:");
        for (i, instruction) in self.instructions.iter().enumerate() {
            println!("  {}: {:?}", i, instruction);
        }
        println!("===================================\n");
    }
}

// Export CSS Compiler
pub struct CssCompiler {
    pub bit_counter: usize,
    pub state_mapping: HashMap<String, usize>,
}

impl Default for CssCompiler {
    fn default() -> Self {
        Self::new()
    }
}

impl CssCompiler {
    pub fn new() -> Self {
        CssCompiler {
            bit_counter: 0,
            state_mapping: HashMap::new(),
        }
    }

    pub fn allocate_bit(&mut self, state_name: String) -> usize {
        if let Some(&existing_bit) = self.state_mapping.get(&state_name) {
            existing_bit
        } else {
            let bit_pos = self.bit_counter;
            self.state_mapping.insert(state_name, bit_pos);
            self.bit_counter += 1;
            bit_pos
        }
    }

    pub fn compile_css_rules(&mut self, rules: &[CssRule]) -> TreeNFAProgram {
        let mut program = TreeNFAProgram::new();

        // First pass: allocate bits for all selectors
        for rule in rules {
            match rule {
                CssRule::Simple(selector) => {
                    let match_state = format!("match_{:?}", selector);
                    let active_state = format!("active_{:?}", selector);

                    let match_bit = self.allocate_bit(match_state.clone());
                    let active_bit = self.allocate_bit(active_state.clone());

                    program.set_state_name(match_bit, match_state);
                    program.set_state_name(active_bit, active_state);
                }
                CssRule::Child {
                    parent_selector,
                    child_selector,
                } => {
                    // Ensure parent has active state
                    let parent_active_state = format!("active_{:?}", parent_selector);
                    let parent_active_bit = self.allocate_bit(parent_active_state.clone());
                    program.set_state_name(parent_active_bit, parent_active_state);

                    // Allocate bit for child rule match
                    let child_match_state =
                        format!("match_{:?}_gt_{:?}", parent_selector, child_selector);
                    let child_match_bit = self.allocate_bit(child_match_state.clone());
                    program.set_state_name(child_match_bit, child_match_state);
                }
            }
        }

        // Second pass: generate instructions
        for rule in rules {
            match rule {
                CssRule::Simple(selector) => {
                    let match_state = format!("match_{:?}", selector);
                    let active_state = format!("active_{:?}", selector);

                    let match_bit = self.state_mapping[&match_state];
                    let active_bit = self.state_mapping[&active_state];

                    // Generate instruction to check and set match bit
                    program.add_instruction(NFAInstruction::CheckAndSetBit {
                        selector: selector.clone(),
                        bit_pos: match_bit,
                    });

                    // Generate instruction to propagate to children
                    program.add_instruction(NFAInstruction::PropagateToChildren {
                        match_bit,
                        active_bit,
                    });
                }
                CssRule::Child {
                    parent_selector,
                    child_selector,
                } => {
                    let parent_active_state = format!("active_{:?}", parent_selector);
                    let child_match_state =
                        format!("match_{:?}_gt_{:?}", parent_selector, child_selector);

                    let parent_active_bit = self.state_mapping[&parent_active_state];
                    let child_match_bit = self.state_mapping[&child_match_state];

                    // Generate instruction to check parent state and set child match bit
                    program.add_instruction(NFAInstruction::CheckParentAndSetBit {
                        parent_state_bit: parent_active_bit,
                        child_selector: child_selector.clone(),
                        result_bit: child_match_bit,
                    });
                }
            }
        }

        program
    }
}

// Helper functions for parsing
pub fn parse_basic_css(css_content: &str) -> Vec<CssRule> {
    let mut rules = Vec::new();

    let lines: Vec<&str> = css_content.lines().collect();
    let mut current_selector = String::new();

    for line in lines {
        let line = line.trim();
        if line.is_empty() || line.starts_with("/*") {
            continue;
        }

        if line.contains('{') && !line.contains('}') {
            current_selector = line.split('{').next().unwrap_or("").trim().to_string();
        } else if line.contains('}') && !current_selector.is_empty() {
            if current_selector.starts_with('.') {
                let class_name = current_selector[1..].to_string();
                if !class_name.contains(' ') && !class_name.contains(':') {
                    rules.push(CssRule::Simple(SimpleSelector::Class(class_name)));
                }
            } else if current_selector.starts_with('#') {
                let id_name = current_selector[1..].to_string();
                if !id_name.contains(' ') && !id_name.contains(':') {
                    rules.push(CssRule::Simple(SimpleSelector::Id(id_name)));
                }
            } else if !current_selector.contains(' ')
                && !current_selector.contains(':')
                && !current_selector.contains('.')
                && !current_selector.contains('#')
            {
                rules.push(CssRule::Simple(SimpleSelector::Type(
                    current_selector.to_lowercase(),
                )));
            }
            current_selector.clear();
        }
    }

    // Add some common Google selectors
    rules.extend([
        CssRule::Simple(SimpleSelector::Type("div".to_string())),
        CssRule::Simple(SimpleSelector::Type("span".to_string())),
        CssRule::Simple(SimpleSelector::Type("a".to_string())),
        CssRule::Simple(SimpleSelector::Type("input".to_string())),
        CssRule::Simple(SimpleSelector::Class("gbts".to_string())),
        CssRule::Simple(SimpleSelector::Class("gbmt".to_string())),
        CssRule::Simple(SimpleSelector::Class("lsb".to_string())),
        CssRule::Simple(SimpleSelector::Id("gb".to_string())),
        CssRule::Simple(SimpleSelector::Id("gbz".to_string())),
    ]);

    rules
}

// DOM creation helper functions
pub fn load_dom_from_file() -> HtmlNode {
    // Try to read Google trace data from file
    let json_data =
        std::fs::read_to_string("css-gen-op/command.json").expect("fail to read command.json");

    // Get the first line which should be the init command
    let first_line = json_data
        .lines()
        .next()
        .expect("File is empty or cannot read first line");

    // Parse the JSON to get the DOM tree
    let trace_data: serde_json::Value =
        serde_json::from_str(first_line).expect("Failed to parse command.json");

    // Check if it's an init command
    if trace_data["name"] != "init" {
        println!("⚠️ Expected init command, using mock data");
    }

    // Extract the node from init command
    let google_node_data = &trace_data["node"];

    // Convert JSON DOM to HtmlNode and initialize parent pointers
    let mut root = convert_json_dom_to_html_node(google_node_data);
    root.init_parent_pointers();
    root
}

pub fn convert_json_dom_to_html_node(json_node: &serde_json::Value) -> HtmlNode {
    let name = json_node["name"].as_str().unwrap_or("unknown");
    let mut node = HtmlNode::new(name);

    // Set ID if present
    if let Some(id) = json_node["id"].as_u64() {
        node = node.with_id(&id.to_string());
    }

    // Add classes from attributes
    if let Some(attributes) = json_node["attributes"].as_object() {
        if let Some(class_attr) = attributes.get("class") {
            if let Some(class_str) = class_attr.as_str() {
                for class_name in class_str.split_whitespace() {
                    node = node.with_class(class_name);
                }
            }
        }
    }

    // Add children recursively
    if let Some(children) = json_node["children"].as_array() {
        for child_json in children {
            let child_node = convert_json_dom_to_html_node(child_json);
            node = node.add_child(child_node);
        }
    }

    // Initialize parent pointers for the complete tree
    node.init_parent_pointers();
    node
}

// Utility functions
pub fn count_matches(node: &HtmlNode) -> usize {
    let current = if node.css_match_bitvector.as_u64() != 0 {
        1
    } else {
        0
    };
    current + node.children.iter().map(count_matches).sum::<usize>()
}

pub fn count_total_nodes(node: &HtmlNode) -> usize {
    1 + node.children.iter().map(count_total_nodes).sum::<usize>()
}

// Helper function for generated code
pub fn node_matches_selector_generated(node: &HtmlNode, selector: &SimpleSelector) -> bool {
    match selector {
        SimpleSelector::Type(tag) => node.tag_name == *tag,
        SimpleSelector::Class(class) => node.classes.contains(class),
        SimpleSelector::Id(id) => node.id.as_deref() == Some(id),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dirty_marking_python_style() {
        // Create a tree: root -> child1 -> grandchild
        let mut root = HtmlNode::new("div");
        let mut child1 = HtmlNode::new("span");
        let grandchild = HtmlNode::new("p");

        child1 = child1.add_child(grandchild);
        root = root.add_child(child1);
        root.init_parent_pointers();

        // Clean all initial dirty state (nodes are dirty from construction)
        let mut initial_dirty = Vec::new();
        root.find_all_dirty_nodes_recursive(&mut initial_dirty);

        // Now nothing should be dirty
        assert!(!root.has_dirty_nodes());
        assert!(!root.is_self_dirty);
        assert!(!root.has_dirty_descendant);

        // Mark the grandchild as dirty
        if let Some(grandchild) = root.children[0].children.get_mut(0) {
            grandchild.mark_dirty();
        }

        // Check that summary bits propagated correctly:
        // - root should have dirty descendant (because child1 has dirty descendant)
        // - child1 should have dirty descendant (because grandchild is dirty)
        // - grandchild should be self dirty (but not have dirty descendant)
        assert!(!root.is_self_dirty);
        assert!(root.has_dirty_descendant);
        assert!(!root.children[0].is_self_dirty);
        assert!(root.children[0].has_dirty_descendant);
        assert!(root.children[0].children[0].is_self_dirty);
        assert!(!root.children[0].children[0].has_dirty_descendant);

        // Collect dirty nodes
        let dirty_nodes = root.collect_dirty_nodes();
        assert_eq!(dirty_nodes.len(), 1);

        // After collection, tree should be clean
        assert!(!root.has_dirty_nodes());
        assert!(!root.has_dirty_descendant);
        assert!(!root.children[0].has_dirty_descendant);
        assert!(!root.children[0].children[0].is_self_dirty);
    }

    #[test]
    fn test_multiple_dirty_nodes() {
        let mut root = HtmlNode::new("div");
        let child1 = HtmlNode::new("span");
        let child2 = HtmlNode::new("p");

        root = root.add_child(child1).add_child(child2);
        root.init_parent_pointers();

        // Clean initial dirty state
        let mut initial_dirty = Vec::new();
        root.find_all_dirty_nodes_recursive(&mut initial_dirty);

        // Mark root and second child as dirty
        root.mark_dirty();
        root.children[1].mark_dirty();

        // Check state after marking:
        // - root is self dirty and has dirty descendant
        // - child1 is not dirty
        // - child2 is self dirty but has no dirty descendant
        assert!(root.is_self_dirty);
        assert!(root.has_dirty_descendant);
        assert!(!root.children[0].is_self_dirty);
        assert!(!root.children[0].has_dirty_descendant);
        assert!(root.children[1].is_self_dirty);
        assert!(!root.children[1].has_dirty_descendant);

        // Collect dirty nodes
        let dirty_nodes = root.collect_dirty_nodes();
        assert_eq!(dirty_nodes.len(), 2);

        // Tree should be clean after collection
        assert!(!root.has_dirty_nodes());
    }

    #[test]
    fn test_ancestor_summary_propagation() {
        // Create deeper tree: root -> child -> grandchild -> great_grandchild
        let mut root = HtmlNode::new("div");
        let mut child = HtmlNode::new("span");
        let mut grandchild = HtmlNode::new("p");
        let great_grandchild = HtmlNode::new("a");

        grandchild = grandchild.add_child(great_grandchild);
        child = child.add_child(grandchild);
        root = root.add_child(child);
        root.init_parent_pointers();

        // Clean initial dirty state
        let mut initial_dirty = Vec::new();
        root.find_all_dirty_nodes_recursive(&mut initial_dirty);

        // Mark the deepest node as dirty
        root.children[0].children[0].children[0].mark_dirty();

        // Check that summary bits propagated all the way up
        assert!(!root.is_self_dirty);
        assert!(root.has_dirty_descendant);
        assert!(!root.children[0].is_self_dirty);
        assert!(root.children[0].has_dirty_descendant);
        assert!(!root.children[0].children[0].is_self_dirty);
        assert!(root.children[0].children[0].has_dirty_descendant);
        assert!(root.children[0].children[0].children[0].is_self_dirty);
        assert!(!root.children[0].children[0].children[0].has_dirty_descendant);

        // Collect dirty nodes
        let dirty_nodes = root.collect_dirty_nodes();
        assert_eq!(dirty_nodes.len(), 1);

        // All summary bits should be cleared
        assert!(!root.has_dirty_descendant);
        assert!(!root.children[0].has_dirty_descendant);
        assert!(!root.children[0].children[0].has_dirty_descendant);
    }

    #[test]
    fn test_process_dirty_nodes() {
        let mut root = HtmlNode::new("div");
        let child = HtmlNode::new("span");
        root = root.add_child(child);
        root.init_parent_pointers();

        // Clean initial dirty state
        let mut initial_dirty = Vec::new();
        root.find_all_dirty_nodes_recursive(&mut initial_dirty);

        // Mark child as dirty
        root.children[0].mark_dirty();

        let mut processed_count = 0;
        root.process_dirty_nodes(|_node_ptr| {
            processed_count += 1;
        });

        assert_eq!(processed_count, 1);
        assert!(!root.has_dirty_nodes());
    }
}
