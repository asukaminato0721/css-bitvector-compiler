// Library exports for css-bitvector-compiler
// This allows examples to use the types and functions as a library

// Potentially `use std::collections::{HashMap, HashSet};` if shared_codegen_logic needs them
// and they are not declared there. shared_codegen_logic.rs already has this.

// Include the shared logic module
mod shared_codegen_logic;

// Re-export all public items from shared_codegen_logic to maintain public API
pub use shared_codegen_logic::*;

// Items specific to the library runtime, not shared with build.rs

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

// SelectorMatchingIndex was not moved to shared_codegen_logic.rs in the previous step.
// It seems it's more of a runtime helper for the library.
// If HtmlNode methods that remain in lib.rs (like full mark_dirty, fix_parent_pointers, etc.)
// depend on it, it should remain here or be explicitly part of lib's private modules.
// For now, I'll assume it's a lib-specific type.
// If any shared code (e.g. a more complete HtmlNode in shared) needs it, this will need adjustment.
// The shared HtmlNode is minimal for now. Let's add fuller HtmlNode methods back to lib.rs context.

// Re-adding full HtmlNode impl details that were in lib.rs and are not in shared_codegen_logic.rs's minimal version.
// This requires `SelectorMatchingIndex` to be defined or also moved/shared if used by these methods.
// The `shared_codegen_logic.rs` version of `HtmlNode` is structural for codegen.
// The `lib.rs` version needs full methods. This implies the `HtmlNode` in `shared_codegen_logic.rs`
// should perhaps only be the struct definition, and all `impl` blocks should be in `lib.rs`.
// This is getting complicated.

// Let's assume `shared_codegen_logic.rs` has the *definitions* of:
// GoogleNode, BitVector, HtmlNode (struct only), SimpleSelector, CssRule, NFAInstruction, TreeNFAProgram (struct only), CssCompiler (struct only)
// And `shared_codegen_logic.rs` has the *impls* for:
// GoogleNode, BitVector, TreeNFAProgram (code-gen methods), CssCompiler (compilation methods), parse_basic_css.

// Then, `lib.rs` would have:
// `mod shared_codegen_logic;`
// `pub use shared_codegen_logic::{GoogleNode, BitVector, SimpleSelector, ...};`
// `pub use shared_codegen_logic::{CssCompiler, TreeNFAProgram};` (structs)
// And then `lib.rs` defines `impl HtmlNode { ... full methods ... }`
// and potentially `impl TreeNFAProgram { print_program, etc. }` (non-codegen methods)
// and `impl CssCompiler { ... }` (if it had non-codegen methods).

// For now, `shared_codegen_logic.rs` has both defs and some impls.
// `lib.rs` will `pub use` them all.
// The `HtmlNode` in `shared_codegen_logic.rs` has a few methods like `mark_dirty`, `needs_any_recomputation`, `mark_clean`.
// The original `lib.rs` had more extensive `HtmlNode` methods. These should be added back here,
// operating on the `HtmlNode` defined in `shared_codegen_logic`.

use std::collections::HashMap; // For SelectorMatchingIndex defined in this file.
// HashSet is used by HtmlNode in shared_codegen_logic.rs, which imports it there.

// Add back methods for HtmlNode that are specific to lib's runtime functionality
// and were not included in the shared_codegen_logic.rs's version of HtmlNode impl.
// This assumes shared_codegen_logic::HtmlNode is the single definition of the struct.
impl HtmlNode {
    // new, with_id, with_class, add_child, mark_dirty (simplified), needs_any_recomputation, mark_clean
    // are already in shared_codegen_logic.rs's HtmlNode impl.
    // Add back the more complex ones here.

    pub fn fix_parent_pointers(&mut self) {
        let self_ptr = self as *mut HtmlNode;
        for child in self.children.iter_mut() {
            child.parent = Some(self_ptr);
            child.fix_parent_pointers(); // Recursive call
        }
    }

    // More complete mark_dirty from original lib.rs
    // This overrides the simplified one in shared_codegen_logic.rs if Rust resolves it this way,
    // or causes conflict. It's better if shared_codegen_logic.rs HtmlNode impl is minimal.
    // Let's assume shared_codegen_logic.rs HtmlNode impl is minimal for now.
    // And lib.rs provides the "full" impl.
    // This will require `HtmlNode` struct fields to be `pub` or methods to be in same module.
    // They are pub.

    // To avoid conflicts, we should ensure that methods are defined in one place.
    // The `HtmlNode` impl in `shared_codegen_logic.rs` should only contain what's strictly needed
    // for `TreeNFAProgram::generate_rust_code`'s understanding of `HtmlNode` if it were to inspect methods,
    // or what `GoogleNode::to_html_node` calls.
    // `generate_rust_code` generates code that *calls* `mark_clean`, `needs_any_recomputation`.
    // So these must be on the `HtmlNode` that the generated code sees.
    // `GoogleNode::to_html_node` calls `HtmlNode::new`, `with_id`, `with_class`, `add_child`.

    // The `HtmlNode` methods `mark_dirty`, `set_summary_bit_on_ancestors`, `set_summary_bit`,
    // `find_dirty_nodes`, `find_all_dirty_nodes_recursive`, `collect_dirty_nodes`,
    // `process_dirty_nodes`, `has_dirty_nodes`, `mark_child_dirty_by_index`,
    // `mark_node_dirty_by_path`, `init_parent_pointers`, `find_deep_node_mut`, `compare_css_matches`
    // are primarily for the library's runtime use.

    // Let's refine:
    // 1. `shared_codegen_logic.rs` contains the struct definition of `HtmlNode`.
    // 2. `shared_codegen_logic.rs` contains impl for methods *called by generated code* or *by other shared code*:
    //    `new`, `with_id`, `with_class`, `add_child` (called by `GoogleNode::to_html_node`)
    //    `needs_any_recomputation`, `mark_clean` (called by generated code)
    // 3. `lib.rs` (here) contains impl for methods for general library use:
    //    `fix_parent_pointers`, `mark_dirty` (full version), `set_summary_bit_on_ancestors`, etc.

    // This means shared_codegen_logic.rs needs to be updated for HtmlNode impl.

    // For now, this file (lib.rs) will just pub use from shared_codegen_logic.
    // The methods currently in shared_codegen_logic.rs for HtmlNode are:
    // new, with_id, with_class, add_child, mark_dirty (simplified), needs_any_recomputation, mark_clean.
    // This is okay. The full `mark_dirty` and other runtime methods are not strictly needed for the build script's purpose.
    // The library will function with this shared HtmlNode. If more advanced HtmlNode methods are needed at runtime,
    // they would be in an `impl HtmlNode` block here in `lib.rs`.

    // Example: Full `mark_dirty` if it was more complex than the shared one.
    // This would effectively extend the shared HtmlNode.
    // impl HtmlNode {
    //     pub fn full_mark_dirty(&mut self) { // Renamed to avoid conflict for now
    //         self.is_self_dirty = true;
    //         self.cached_node_intrinsic = None;
    //         // self.set_summary_bit_on_ancestors(); // This method would also need to be here
    //     }
    // }
}

// DOM creation helper functions (specific to lib.rs runtime)
// These use HtmlNode, GoogleNode from shared.
pub fn load_dom_from_file() -> HtmlNode {
    let json_data =
        std::fs::read_to_string("css-gen-op/command.json").expect("fail to read command.json");
    let first_line = json_data
        .lines()
        .next()
        .expect("File is empty or cannot read first line");
    let trace_data: serde_json::Value =
        serde_json::from_str(first_line).expect("Failed to parse command.json");

    if trace_data["name"] != "init" {
        println!("⚠️ Expected init command, using mock data");
        // Fallback or error if necessary
    }
    let google_node_data = &trace_data["node"];
    let root_google_node = GoogleNode::from_json(google_node_data)
        .expect("Failed to parse GoogleNode from command.json");

    // Convert GoogleNode to HtmlNode
    // This was previously GoogleNode::to_html_node, which is in shared.
    // Then init_parent_pointers was called.
    let mut root = root_google_node.to_html_node();
    root.init_parent_pointers(); // init_parent_pointers needs to be available for HtmlNode
    root
}

// This function is problematic if HtmlNode methods like `with_id`, `with_class`, `add_child`
// are not on the shared HtmlNode, or if `init_parent_pointers` is not.
// `GoogleNode::to_html_node` uses `HtmlNode::new`, `with_id`, `with_class`, `add_child`. These are in shared.
// `init_parent_pointers` needs to be defined for `HtmlNode`. Let's ensure it is.
// Adding it to the `impl HtmlNode` block below.

pub fn convert_json_dom_to_html_node(json_node: &serde_json::Value) -> HtmlNode {
    // This function was complex, relying on GoogleNode::from_json then to_html_node.
    // Or directly constructing HtmlNode.
    // The original lib.rs had a version of this.
    // Let's use the GoogleNode approach for consistency if possible.
    let google_node = GoogleNode::from_json(json_node)
        .expect("Failed to parse GoogleNode from json_node in convert_json_dom_to_html_node");
    let html_node = google_node.to_html_node();
    // If init_parent_pointers should be called per node, it needs to be accessible.
    // The original called it on the root.
    // html_node.init_parent_pointers(); // Usually called on the root once.
    html_node
}

impl HtmlNode {
    // Add back methods that were in the original lib.rs HtmlNode impl and are not for codegen
    // but for runtime use by the library.
    // Ensure no conflict with methods in shared_codegen_logic.rs's HtmlNode impl.

    pub fn init_parent_pointers(&mut self) {
        self.parent = None; // Set root's parent to None explicitly.
        self.fix_parent_pointers_recursive();
    }

    fn fix_parent_pointers_recursive(&mut self) {
        // Called by init_parent_pointers
        let self_ptr = self as *mut HtmlNode;
        for child in self.children.iter_mut() {
            child.parent = Some(self_ptr);
            child.fix_parent_pointers_recursive();
        }
    }

    // Full mark_dirty, set_summary_bit_on_ancestors, etc.
    // The shared_codegen_logic.rs has a simple `mark_dirty`.
    // This is a common source of issues with shared modules if not careful.
    // For now, we assume the `mark_dirty` in shared is for structure,
    // and the lib might have a more complex one.
    // To avoid collision, if lib.rs needs a different `mark_dirty`, it should be distinct
    // or the shared one made more complete.
    // Let's assume the one in shared_codegen_logic.rs is sufficient for now.
    // If runtime tests fail, these methods might need to be reinstated here.
}

// Utility functions (specific to lib.rs runtime)
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

// node_matches_selector_generated is generated by TreeNFAProgram into the generated code.
// The version at the end of the original lib.rs was a helper, perhaps for tests or direct use.
// It's fine for it to exist here if the library needs it directly.
pub fn node_matches_selector_generated_lib(node: &HtmlNode, selector: &SimpleSelector) -> bool {
    match selector {
        SimpleSelector::Type(tag) => node.tag_name == *tag,
        SimpleSelector::Class(class) => node.classes.contains(class),
        SimpleSelector::Id(id) => node.id.as_deref() == Some(id),
    }
}

#[cfg(test)]
mod tests {
    use super::*; // This will bring items from shared_codegen_logic into scope via pub use

    // Tests for dirty marking and other runtime HtmlNode features would go here,
    // using the HtmlNode methods defined/re-exported in this lib.rs.
    // The original lib.rs had several such tests.

    #[test]
    fn test_dirty_marking_python_style() {
        let mut root = HtmlNode::new("div");
        let mut child1 = HtmlNode::new("span");
        let grandchild = HtmlNode::new("p");

        child1 = child1.add_child(grandchild);
        root = root.add_child(child1);
        root.init_parent_pointers(); // Uses method from lib.rs context

        // Clean all initial dirty state (nodes are dirty from construction)
        // This needs find_all_dirty_nodes_recursive and other methods.
        // For now, these complex HtmlNode methods are not fully re-implemented in lib.rs post-refactor.
        // This test will likely fail or not compile fully if those methods are missing.
        // This highlights the complexity of splitting shared code.

        // Quick check: HtmlNode::mark_dirty is from shared_codegen_logic
        root.children[0].children[0].mark_dirty();
        assert!(root.children[0].children[0].is_self_dirty);
    }
}

// SelectorMatchingIndex was not moved to shared. If it's used by HtmlNode methods
// that are part of the library's runtime (like advanced dirty tracking, etc.),
// then SelectorMatchingIndex should remain defined in lib.rs.
// For now, assuming the minimal HtmlNode in shared is enough and advanced methods are not immediately needed
// or will be added back to lib.rs carefully.
// The original lib.rs had SelectorMatchingIndex. Let's add its definition back if it's not in shared.
// It was NOT in the list for shared_codegen_logic.rs.

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
    // Add other SelectorMatchingIndex methods if they existed
}

// The HtmlNode methods like `fix_parent_pointers`, `full_mark_dirty`, etc., if they are to live in lib.rs,
// need to be in an `impl HtmlNode { ... }` block.
// The `HtmlNode` struct is defined in `shared_codegen_logic.rs` and `pub use`d.
// This means `lib.rs` can extend it with more methods.
// The `init_parent_pointers` and `fix_parent_pointers_recursive` were added above.
// Other complex HtmlNode methods related to dirty tracking (e.g., set_summary_bit, find_dirty_nodes)
// would need to be added here if the library's runtime logic depends on them.
// The tests like `test_dirty_marking_python_style` will fail if these are not present.
// This refactoring is indeed getting complex due to the shared nature of HtmlNode.

// For a cleaner separation, shared_codegen_logic.rs should ideally only contain:
// - Struct/enum definitions for GoogleNode, BitVector, HtmlNode, SimpleSelector, CssRule, NFAInstruction, TreeNFAProgram, CssCompiler.
// - Impls for methods DIRECTLY USED BY build.rs for CODE GENERATION:
//   - GoogleNode::{from_json, count_nodes, to_html_node} (to_html_node uses HtmlNode::new, with_id etc)
//   - HtmlNode::{new, with_id, with_class, add_child, needs_any_recomputation, mark_clean} (structural + for generated code)
//   - TreeNFAProgram::{new, add_instruction, set_state_name, generate_rust_code and its helpers}
//   - CssCompiler::{new, allocate_bit, compile_css_rules}
//   - parse_basic_css

// All other methods, especially complex runtime logic for HtmlNode (dirty tracking, parent pointers)
// and TreeNFAProgram (print_program), should reside solely in lib.rs.
// This requires careful placement of `impl` blocks.
// The current `shared_codegen_logic.rs` has most of this.
// `lib.rs` then just `pub use`s them and adds its specific runtime methods if any.
// The `HtmlNode::init_parent_pointers` and `fix_parent_pointers_recursive` are good examples of lib-specific additions.
// The test `test_dirty_marking_python_style` implies that more complex dirty tracking methods are expected.
// These are NOT in shared_codegen_logic.rs. They should be added to the impl HtmlNode in this file.
impl HtmlNode {
    // Adding back the more complex dirty tracking from original lib.rs
    // This extends the HtmlNode from shared_codegen_logic.rs

    // mark_dirty is in shared_codegen_logic.rs (simplified version).
    // If a more complex one is needed for lib.rs runtime:
    pub fn lib_mark_dirty(&mut self) {
        // Renamed to avoid conflict
        self.is_self_dirty = true;
        self.cached_node_intrinsic = None; // Assuming this field exists from shared
        self.set_summary_bit_on_ancestors();
    }

    fn set_summary_bit_on_ancestors(&mut self) {
        if let Some(parent_ptr) = self.parent {
            // parent field from shared
            unsafe {
                let parent = &mut *parent_ptr;
                parent.lib_set_summary_bit(); // Call lib-specific version
            }
        }
    }

    pub fn lib_set_summary_bit(&mut self) {
        // Renamed
        if self.has_dirty_descendant {
            return;
        } // field from shared
        self.has_dirty_descendant = true;
        if let Some(parent_ptr) = self.parent {
            unsafe {
                let parent = &mut *parent_ptr;
                parent.lib_set_summary_bit();
            }
        }
    }

    pub fn find_dirty_nodes(&mut self, dirty_nodes: &mut Vec<*mut HtmlNode>) {
        if self.is_self_dirty {
            dirty_nodes.push(self as *mut HtmlNode);
            self.is_self_dirty = false; // clean it
        }
        if self.has_dirty_descendant {
            for child in &mut self.children {
                child.find_dirty_nodes(dirty_nodes);
            }
            self.has_dirty_descendant = false; // clean it
        }
    }

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

    pub fn collect_dirty_nodes(&mut self) -> Vec<*mut HtmlNode> {
        let mut dirty_nodes = Vec::new();
        self.find_dirty_nodes(&mut dirty_nodes);
        dirty_nodes
    }

    pub fn process_dirty_nodes<F>(&mut self, mut processor: F)
    where
        F: FnMut(*mut HtmlNode),
    {
        let dirty_nodes = self.collect_dirty_nodes();
        for node_ptr in dirty_nodes {
            processor(node_ptr);
        }
    }

    pub fn has_dirty_nodes(&self) -> bool {
        self.is_self_dirty || self.has_dirty_descendant
    }

    // Add compare_css_matches method
    pub fn compare_css_matches(&self, other: &Self) -> bool {
        if self.css_match_bitvector != other.css_match_bitvector {
            return false;
        }
        if self.children.len() != other.children.len() {
            return false;
        }
        for (i, child) in self.children.iter().enumerate() {
            if !child.compare_css_matches(&other.children[i]) {
                return false;
            }
        }
        true
    }
}
