// This file contains types and logic shared between lib.rs and build.rs
// for CSS parsing, compilation, and code generation.

use std::collections::{HashMap, HashSet}; // Needed by various structs

// Definition of GoogleNode (copied from original lib.rs)
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
        if let Some(class_attr) = self.attributes.get("class") {
            node.classes = class_attr
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
        }
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

// Definition of BitVector (copied from original lib.rs)
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

// Definition of HtmlNode (copied from original lib.rs)
// Note: SelectorMatchingIndex is not directly used by build.rs, but HtmlNode methods might use it.
// For codegen, the structure of HtmlNode is important.
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
    pub parent: Option<*mut HtmlNode>, // This raw pointer might be tricky if instances are moved carelessly.
}

impl HtmlNode {
    pub fn new(tag_name: &str) -> Self {
        HtmlNode {
            tag_name: tag_name.to_lowercase(),
            id: None,
            classes: HashSet::new(),
            children: Vec::new(),
            css_match_bitvector: BitVector::new(),
            is_self_dirty: true, // Start dirty for initial computation
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

    // Methods related to dirty tracking and parent pointers are mostly for runtime use in the library,
    // but their presence defines the HtmlNode struct that generated code will interact with.
    // So, the struct definition needs to be complete.
    // Simplified for shared_codegen_logic.rs if full methods aren't strictly needed for build.rs to *define* types.
    // However, generate_rust_code output *uses* these field names.
    // And `to_html_node` (on GoogleNode) creates HtmlNodes.

    // Keeping a few essential methods for node manipulation if GoogleNode::to_html_node needs them.
    pub fn mark_dirty(&mut self) {
        self.is_self_dirty = true; /* Simplified, full dirty propagation not needed for basic structure */
    }

    // init_parent_pointers and other complex methods can be omitted if build.rs doesn't call them.
    // GoogleNode::to_html_node does not call init_parent_pointers. lib.rs calls it after conversion.
}

// Definition of SimpleSelector, CssRule, NFAInstruction (copied from original lib.rs)
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

// Definition of TreeNFAProgram (copied from original lib.rs)
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
        self.state_names.insert(bit_pos, name.clone()); // Use name.clone() here
        if bit_pos >= self.total_bits {
            self.total_bits = bit_pos + 1;
        }
    }

    // generate_rust_code and its helpers are fairly large.
    // This is the core logic needed by build.rs.
    pub fn generate_rust_code(&self) -> String {
        let mut code = String::new();
        code.push_str("use css_bitvector_compiler::{BitVector, HtmlNode, SimpleSelector};\n\n");
        let intrinsic_checks_code = self.generate_intrinsic_checks_code();
        let parent_dependent_rules_code = self.generate_parent_dependent_rules_code();
        let propagation_rules_code = self.generate_propagation_rules_code();

        code.push_str("// --- Incremental Processing Functions ---\n");
        code.push_str("pub fn process_node_generated_incremental(\n");
        code.push_str("    node: &mut HtmlNode,\n");
        code.push_str("    parent_state: BitVector,\n");
        code.push_str(") -> BitVector { // returns child_states\n");
        code.push_str("    if !node.needs_any_recomputation(parent_state) {\n");
        code.push_str("        return node.cached_child_states.unwrap_or_default();\n");
        code.push_str("    }\n\n");
        code.push_str("    if node.cached_node_intrinsic.is_none() || node.is_self_dirty {\n");
        code.push_str(&intrinsic_checks_code);
        code.push_str("        node.cached_node_intrinsic = Some(intrinsic_matches);\n");
        code.push_str("    }\n\n");
        code.push_str("    let mut current_matches = node.cached_node_intrinsic.unwrap();\n");
        code.push_str(&parent_dependent_rules_code);
        code.push_str("    let mut child_states = BitVector::new();\n");
        code.push_str(&propagation_rules_code);
        code.push_str("    node.css_match_bitvector = current_matches;\n");
        code.push_str("    node.cached_parent_state = Some(parent_state);\n");
        code.push_str("    node.cached_child_states = Some(child_states);\n");
        code.push_str("    node.mark_clean();\n\n");
        code.push_str("    child_states\n");
        code.push_str("}\n\n");

        code.push_str("// --- From-Scratch Processing Functions ---\n");
        code.push_str("pub fn process_node_generated_from_scratch(\n");
        code.push_str("    node: &mut HtmlNode,\n");
        code.push_str("    parent_state: BitVector,\n");
        code.push_str(") -> BitVector { // returns child_states\n");
        code.push_str(&intrinsic_checks_code);
        code.push_str("    let mut current_matches = intrinsic_matches;\n");
        code.push_str(&parent_dependent_rules_code);
        code.push_str("    let mut child_states = BitVector::new();\n");
        code.push_str(&propagation_rules_code);
        code.push_str("    node.css_match_bitvector = current_matches;\n");
        code.push_str("    child_states\n");
        code.push_str("}\n\n");

        code.push_str("pub fn node_matches_selector_generated(node: &HtmlNode, selector: &SimpleSelector) -> bool {\n");
        code.push_str("    match selector {\n");
        code.push_str("        SimpleSelector::Type(tag) => node.tag_name == *tag,\n");
        code.push_str("        SimpleSelector::Class(class) => node.classes.contains(class),\n");
        code.push_str("        SimpleSelector::Id(id) => node.id.as_deref() == Some(id),\n");
        code.push_str("    }\n");
        code.push_str("}\n\n");
        code.push_str(&self.generate_traversal_wrappers());
        code
    }

    fn generate_intrinsic_checks_code(&self) -> String {
        let mut code = String::new();
        code.push_str("        let mut intrinsic_matches = BitVector::new();\n\n");
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
                    SimpleSelector::Id(id) => format!("SimpleSelector::Id(\"{}\".to_string())", id),
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
        code
    }

    fn generate_parent_dependent_rules_code(&self) -> String {
        let mut code = String::new();
        for instruction in &self.instructions {
            if let NFAInstruction::CheckParentAndSetBit {
                parent_state_bit,
                child_selector,
                result_bit,
            } = instruction
            {
                let child_selector_str = match child_selector {
                    SimpleSelector::Type(tag) => {
                        format!("SimpleSelector::Type(\"{}\".to_string())", tag)
                    }
                    SimpleSelector::Class(class) => {
                        format!("SimpleSelector::Class(\"{}\".to_string())", class)
                    }
                    SimpleSelector::Id(id) => format!("SimpleSelector::Id(\"{}\".to_string())", id),
                };
                code.push_str(&format!("    if parent_state.is_bit_set({}) && node_matches_selector_generated(node, &{}) {{\n", parent_state_bit, child_selector_str));
                code.push_str(&format!(
                    "        current_matches.set_bit({}); // {}\n",
                    result_bit,
                    self.state_names
                        .get(result_bit)
                        .unwrap_or(&format!("bit_{}", result_bit))
                ));
                code.push_str("    }\n");
            }
        }
        code
    }

    fn generate_propagation_rules_code(&self) -> String {
        let mut code = String::new();
        for instruction in &self.instructions {
            if let NFAInstruction::PropagateToChildren {
                match_bit,
                active_bit,
            } = instruction
            {
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
                code.push_str("    }\n");
            }
        }
        code
    }

    fn generate_traversal_wrappers(&self) -> String {
        r#"
/// Incremental processing driver with statistics tracking
pub fn process_tree_incremental_with_stats(root: &mut HtmlNode) -> (usize, usize, usize) {
    let mut total_nodes = 0;
    let mut cache_hits = 0;
    let mut cache_misses = 0;
    process_tree_recursive_incremental(root, BitVector::new(), &mut total_nodes, &mut cache_hits, &mut cache_misses);
    (total_nodes, cache_hits, cache_misses)
}

fn process_tree_recursive_incremental(node: &mut HtmlNode, parent_state: BitVector,
                                    total: &mut usize, hits: &mut usize, misses: &mut usize) {
    *total += 1;
    if !node.needs_any_recomputation(parent_state) { // Assumes HtmlNode has needs_any_recomputation
        *hits += 1;
        return;
    }
    *misses += 1;
    let child_states = process_node_generated_incremental(node, parent_state);
    for child in node.children.iter_mut() {
        process_tree_recursive_incremental(child, child_states, total, hits, misses);
    }
}

/// From-scratch processing driver for comparison
pub fn process_tree_full_recompute(root: &mut HtmlNode) -> (usize, usize, usize) {
    let mut total_nodes = 0;
    process_tree_recursive_from_scratch(root, BitVector::new(), &mut total_nodes);
    (total_nodes, 0, total_nodes) // 0 hits, all misses
}

fn process_tree_recursive_from_scratch(node: &mut HtmlNode, parent_state: BitVector, total: &mut usize) {
    *total += 1;
    let child_states = process_node_generated_from_scratch(node, parent_state);
    for child in node.children.iter_mut() {
        process_tree_recursive_from_scratch(child, child_states, total);
    }
}
"#
        .to_string()
    }
    // print_program method is not strictly needed for codegen, can be omitted from shared.
}

// Definition of CssCompiler (copied from original lib.rs)
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
                    let parent_active_state = format!("active_{:?}", parent_selector);
                    let parent_active_bit = self.allocate_bit(parent_active_state.clone());
                    program.set_state_name(parent_active_bit, parent_active_state);
                    let child_match_state =
                        format!("match_{:?}_gt_{:?}", parent_selector, child_selector);
                    let child_match_bit = self.allocate_bit(child_match_state.clone());
                    program.set_state_name(child_match_bit, child_match_state);
                }
            }
        }
        for rule in rules {
            match rule {
                CssRule::Simple(selector) => {
                    let match_state = format!("match_{:?}", selector);
                    let active_state = format!("active_{:?}", selector);
                    let match_bit = self.state_mapping[&match_state];
                    let active_bit = self.state_mapping[&active_state];
                    program.add_instruction(NFAInstruction::CheckAndSetBit {
                        selector: selector.clone(),
                        bit_pos: match_bit,
                    });
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

// Definition of parse_basic_css (copied from original lib.rs)
pub fn parse_basic_css(css_content: &str) -> Vec<CssRule> {
    let mut rules = Vec::new();
    let lines: Vec<&str> = css_content.lines().collect();
    let mut current_selector_text = String::new();

    for line_str in lines {
        let line_trimmed = line_str.trim();
        if line_trimmed.is_empty() || line_trimmed.starts_with("/*") {
            continue;
        }

        let mut selector_to_parse: Option<String> = None;

        if line_trimmed.contains('{') && line_trimmed.contains('}') {
            // Case: Full rule on one line, e.g., "div {}" or ".item {}"
            current_selector_text = line_trimmed
                .split('{')
                .next()
                .unwrap_or("")
                .trim()
                .to_string();
            if !current_selector_text.is_empty() {
                selector_to_parse = Some(current_selector_text.clone());
            }
            current_selector_text.clear(); // Parsed or not, selector is consumed
        } else if line_trimmed.contains('{') {
            // Case: Selector and opening brace, e.g., "div {"
            current_selector_text = line_trimmed
                .split('{')
                .next()
                .unwrap_or("")
                .trim()
                .to_string();
            // Selector will be parsed when '}' is found on a subsequent line
        } else if line_trimmed.contains('}') {
            // Case: Closing brace '}'
            if !current_selector_text.is_empty() {
                selector_to_parse = Some(current_selector_text.clone());
                current_selector_text.clear();
            }
        } else if current_selector_text.is_empty()
            && !line_trimmed.contains('{')
            && !line_trimmed.contains('}')
        {
            // Case: Selector on its own line, without braces yet. e.g. "div"
            // This case is tricky if not followed by "{" on next line.
            // parse_basic_css seems to expect selector immediately before "{" or "{}".
            // For simplicity, we assume this case is not standard for this basic parser,
            // or it's handled if the next line brings a "{".
        }

        if let Some(selector_str) = selector_to_parse {
            if selector_str.starts_with('.') {
                let class_name = selector_str[1..].to_string();
                if !class_name.contains(' ') && !class_name.contains(':') && !class_name.is_empty()
                {
                    rules.push(CssRule::Simple(SimpleSelector::Class(class_name)));
                }
            } else if selector_str.starts_with('#') {
                let id_name = selector_str[1..].to_string();
                if !id_name.contains(' ') && !id_name.contains(':') && !id_name.is_empty() {
                    rules.push(CssRule::Simple(SimpleSelector::Id(id_name)));
                }
            } else if !selector_str.contains(' ')
                && !selector_str.contains(':')
                && !selector_str.contains('.')
                && !selector_str.contains('#')
                && !selector_str.is_empty()
            {
                rules.push(CssRule::Simple(SimpleSelector::Type(
                    selector_str.to_lowercase(),
                )));
            }
        }
    }
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

// The `SelectorMatchingIndex` is not directly used by the build script's logic,
// but `HtmlNode`'s full definition in lib.rs might use it.
// For `shared_codegen_logic.rs`, we only need `HtmlNode`'s structural definition
// as understood by `generate_rust_code`.
// If `generate_rust_code` or other shared functions need `SelectorMatchingIndex`, it should be here.
// Looking at `TreeNFAProgram::generate_rust_code`, it does not seem to directly use `SelectorMatchingIndex`.
// It generates code that *operates on* `HtmlNode` but doesn't need to know about `SelectorMatchingIndex` itself.

// Ensure all necessary `use` statements for types like `HashMap`, `HashSet` are at the top.
// `serde_json::Value` is used in `GoogleNode::from_json` -> add `use serde_json;` if not already global in build.rs
// It's a build dependency, so `build.rs` can use it directly.
// For this shared file, it's better to assume it might be compiled in contexts where `serde_json` isn't globally available.
// So, `use serde_json;` might be needed if `GoogleNode::from_json` remains here.
// However, `GoogleNode::from_json` is only used in `build.rs` to parse `command.json`.
// `lib.rs` uses it too (e.g. `load_dom_from_file`). So it's truly shared.

// Add `use serde_json;` for `GoogleNode::from_json`
use serde_json;

// HtmlNode needs some of its methods for the generated code to make sense,
// e.g. `needs_any_recomputation`, `mark_clean`.
// Let's add these to the HtmlNode impl here.
impl HtmlNode {
    // ... (new, with_id, with_class, add_child already there) ...

    // Methods needed by generated code from TreeNFAProgram::generate_rust_code
    pub fn needs_any_recomputation(&self, new_parent_state: BitVector) -> bool {
        // This is a simplified stub. The actual logic might be more complex.
        // The generated code calls this, so it must exist.
        self.is_self_dirty
            || self.has_dirty_descendant
            || self.cached_parent_state.is_none()
            || self.cached_parent_state.unwrap() != new_parent_state
    }

    pub fn mark_clean(&mut self) {
        // This is a simplified stub.
        self.is_self_dirty = false;
        self.has_dirty_descendant = false;
    }
}

// The `TreeNFAProgram::generate_rust_code` refers to `css_bitvector_compiler::{...}`.
// When this code is moved into `shared_codegen_logic.rs` and included in `lib.rs`,
// this path will resolve to items within `lib.rs` (potentially from `shared_codegen_logic` itself via `pub use`).
// When included in `build.rs`, this path will be problematic if `css_bitvector_compiler` refers to the crate being built.
// The generated code itself should use `crate::` or `super::` if it's part of the same crate,
// or `css_bitvector_compiler::` if it's meant to be used by external examples.
// The current line `code.push_str("use css_bitvector_compiler::{BitVector, HtmlNode, SimpleSelector};\n\n");`
// is for the *generated* code. This is fine, as the generated code will be part of `src/generated_css_functions.rs`
// or `examples/google_trace_test.rs`, both of which will depend on the `css_bitvector_compiler` crate.
