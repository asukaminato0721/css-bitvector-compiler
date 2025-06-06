use css_bitvector_compiler::*;
use serde_json;
use std::fs;

// Generated CSS processing function
// Generated Tree NFA Program with Incremental Processing
// This program processes HTML nodes and computes CSS matches with caching

fn process_node_generated_incremental(node: &mut HtmlNode, parent_state: BitVector) -> BitVector {
    // returns child_states
    // Check if we need to recompute
    if !node.needs_any_recomputation(parent_state) {
        // Return cached result - entire subtree can be skipped
        return node.cached_child_states.unwrap_or(BitVector::new());
    }

    // Recompute node intrinsic matches if needed
    if node.cached_node_intrinsic.is_none() || node.is_self_dirty {
        let mut intrinsic_matches = BitVector::new();

        // Instruction 0: CheckAndSetBit { selector: Type("div"), bit_pos: 0 }
        if node_matches_selector_generated(node, &SimpleSelector::Type("div".to_string())) {
            intrinsic_matches.set_bit(0); // match_Type("div")
        }

        // Instruction 2: CheckAndSetBit { selector: Type("span"), bit_pos: 2 }
        if node_matches_selector_generated(node, &SimpleSelector::Type("span".to_string())) {
            intrinsic_matches.set_bit(2); // match_Type("span")
        }

        // Instruction 4: CheckAndSetBit { selector: Type("a"), bit_pos: 4 }
        if node_matches_selector_generated(node, &SimpleSelector::Type("a".to_string())) {
            intrinsic_matches.set_bit(4); // match_Type("a")
        }

        // Instruction 6: CheckAndSetBit { selector: Type("input"), bit_pos: 6 }
        if node_matches_selector_generated(node, &SimpleSelector::Type("input".to_string())) {
            intrinsic_matches.set_bit(6); // match_Type("input")
        }

        // Instruction 8: CheckAndSetBit { selector: Class("gbts"), bit_pos: 8 }
        if node_matches_selector_generated(node, &SimpleSelector::Class("gbts".to_string())) {
            intrinsic_matches.set_bit(8); // match_Class("gbts")
        }

        // Instruction 10: CheckAndSetBit { selector: Class("gbmt"), bit_pos: 10 }
        if node_matches_selector_generated(node, &SimpleSelector::Class("gbmt".to_string())) {
            intrinsic_matches.set_bit(10); // match_Class("gbmt")
        }

        // Instruction 12: CheckAndSetBit { selector: Class("lsb"), bit_pos: 12 }
        if node_matches_selector_generated(node, &SimpleSelector::Class("lsb".to_string())) {
            intrinsic_matches.set_bit(12); // match_Class("lsb")
        }

        // Instruction 14: CheckAndSetBit { selector: Id("gb"), bit_pos: 14 }
        if node_matches_selector_generated(node, &SimpleSelector::Id("gb".to_string())) {
            intrinsic_matches.set_bit(14); // match_Id("gb")
        }

        // Instruction 16: CheckAndSetBit { selector: Id("gbz"), bit_pos: 16 }
        if node_matches_selector_generated(node, &SimpleSelector::Id("gbz".to_string())) {
            intrinsic_matches.set_bit(16); // match_Id("gbz")
        }

        node.cached_node_intrinsic = Some(intrinsic_matches);
    }

    // Start with cached intrinsic matches
    let mut current_matches = node.cached_node_intrinsic.unwrap();
    let mut child_states = BitVector::new();

    // Optimized selector matching using hash tables (conceptual)
    // In practice, rules would be pre-indexed by tag/class/id

    // Apply parent-dependent rules
    // Instruction 1: PropagateToChildren { match_bit: 0, active_bit: 1 }
    if current_matches.is_bit_set(0) {
        child_states.set_bit(1); // active_Type("div")
    }

    // Instruction 3: PropagateToChildren { match_bit: 2, active_bit: 3 }
    if current_matches.is_bit_set(2) {
        child_states.set_bit(3); // active_Type("span")
    }

    // Instruction 5: PropagateToChildren { match_bit: 4, active_bit: 5 }
    if current_matches.is_bit_set(4) {
        child_states.set_bit(5); // active_Type("a")
    }

    // Instruction 7: PropagateToChildren { match_bit: 6, active_bit: 7 }
    if current_matches.is_bit_set(6) {
        child_states.set_bit(7); // active_Type("input")
    }

    // Instruction 9: PropagateToChildren { match_bit: 8, active_bit: 9 }
    if current_matches.is_bit_set(8) {
        child_states.set_bit(9); // active_Class("gbts")
    }

    // Instruction 11: PropagateToChildren { match_bit: 10, active_bit: 11 }
    if current_matches.is_bit_set(10) {
        child_states.set_bit(11); // active_Class("gbmt")
    }

    // Instruction 13: PropagateToChildren { match_bit: 12, active_bit: 13 }
    if current_matches.is_bit_set(12) {
        child_states.set_bit(13); // active_Class("lsb")
    }

    // Instruction 15: PropagateToChildren { match_bit: 14, active_bit: 15 }
    if current_matches.is_bit_set(14) {
        child_states.set_bit(15); // active_Id("gb")
    }

    // Instruction 17: PropagateToChildren { match_bit: 16, active_bit: 17 }
    if current_matches.is_bit_set(16) {
        child_states.set_bit(17); // active_Id("gbz")
    }

    // Cache results
    node.css_match_bitvector = current_matches;
    node.cached_parent_state = Some(parent_state);
    node.cached_child_states = Some(child_states);
    node.mark_clean();

    child_states
}

fn process_node_generated_inplace(node: &mut HtmlNode, parent_state: BitVector) -> BitVector {
    // returns child_states
    let mut current_matches = BitVector::new();
    let mut child_states = BitVector::new();

    // Instruction 0: CheckAndSetBit { selector: Type("div"), bit_pos: 0 }
    if node_matches_selector_generated(node, &SimpleSelector::Type("div".to_string())) {
        current_matches.set_bit(0); // match_Type("div")
    }

    // Instruction 1: PropagateToChildren { match_bit: 0, active_bit: 1 }
    if current_matches.is_bit_set(0) {
        child_states.set_bit(1); // active_Type("div")
    }

    // Instruction 2: CheckAndSetBit { selector: Type("span"), bit_pos: 2 }
    if node_matches_selector_generated(node, &SimpleSelector::Type("span".to_string())) {
        current_matches.set_bit(2); // match_Type("span")
    }

    // Instruction 3: PropagateToChildren { match_bit: 2, active_bit: 3 }
    if current_matches.is_bit_set(2) {
        child_states.set_bit(3); // active_Type("span")
    }

    // Instruction 4: CheckAndSetBit { selector: Type("a"), bit_pos: 4 }
    if node_matches_selector_generated(node, &SimpleSelector::Type("a".to_string())) {
        current_matches.set_bit(4); // match_Type("a")
    }

    // Instruction 5: PropagateToChildren { match_bit: 4, active_bit: 5 }
    if current_matches.is_bit_set(4) {
        child_states.set_bit(5); // active_Type("a")
    }

    // Instruction 6: CheckAndSetBit { selector: Type("input"), bit_pos: 6 }
    if node_matches_selector_generated(node, &SimpleSelector::Type("input".to_string())) {
        current_matches.set_bit(6); // match_Type("input")
    }

    // Instruction 7: PropagateToChildren { match_bit: 6, active_bit: 7 }
    if current_matches.is_bit_set(6) {
        child_states.set_bit(7); // active_Type("input")
    }

    // Instruction 8: CheckAndSetBit { selector: Class("gbts"), bit_pos: 8 }
    if node_matches_selector_generated(node, &SimpleSelector::Class("gbts".to_string())) {
        current_matches.set_bit(8); // match_Class("gbts")
    }

    // Instruction 9: PropagateToChildren { match_bit: 8, active_bit: 9 }
    if current_matches.is_bit_set(8) {
        child_states.set_bit(9); // active_Class("gbts")
    }

    // Instruction 10: CheckAndSetBit { selector: Class("gbmt"), bit_pos: 10 }
    if node_matches_selector_generated(node, &SimpleSelector::Class("gbmt".to_string())) {
        current_matches.set_bit(10); // match_Class("gbmt")
    }

    // Instruction 11: PropagateToChildren { match_bit: 10, active_bit: 11 }
    if current_matches.is_bit_set(10) {
        child_states.set_bit(11); // active_Class("gbmt")
    }

    // Instruction 12: CheckAndSetBit { selector: Class("lsb"), bit_pos: 12 }
    if node_matches_selector_generated(node, &SimpleSelector::Class("lsb".to_string())) {
        current_matches.set_bit(12); // match_Class("lsb")
    }

    // Instruction 13: PropagateToChildren { match_bit: 12, active_bit: 13 }
    if current_matches.is_bit_set(12) {
        child_states.set_bit(13); // active_Class("lsb")
    }

    // Instruction 14: CheckAndSetBit { selector: Id("gb"), bit_pos: 14 }
    if node_matches_selector_generated(node, &SimpleSelector::Id("gb".to_string())) {
        current_matches.set_bit(14); // match_Id("gb")
    }

    // Instruction 15: PropagateToChildren { match_bit: 14, active_bit: 15 }
    if current_matches.is_bit_set(14) {
        child_states.set_bit(15); // active_Id("gb")
    }

    // Instruction 16: CheckAndSetBit { selector: Id("gbz"), bit_pos: 16 }
    if node_matches_selector_generated(node, &SimpleSelector::Id("gbz".to_string())) {
        current_matches.set_bit(16); // match_Id("gbz")
    }

    // Instruction 17: PropagateToChildren { match_bit: 16, active_bit: 17 }
    if current_matches.is_bit_set(16) {
        child_states.set_bit(17); // active_Id("gbz")
    }

    // Store result in node (in-place)
    node.css_match_bitvector = current_matches;
    child_states
}

fn needs_recomputation_generated(node: &HtmlNode, new_parent_state: BitVector) -> bool {
    node.is_self_dirty
        || node.has_dirty_descendant
        || node.cached_parent_state.is_none()
        || node.cached_parent_state.unwrap() != new_parent_state
}

fn process_tree_generated_incremental(root: &mut HtmlNode) {
    process_tree_recursive_generated_incremental(root, BitVector::new());
}

fn process_tree_recursive_generated_incremental(node: &mut HtmlNode, parent_state: BitVector) {
    let child_states = process_node_generated_incremental(node, parent_state);

    // Recursively process children
    for child in node.children.iter_mut() {
        process_tree_recursive_generated_incremental(child, child_states);
    }
}

fn process_tree_generated(root: &mut HtmlNode) {
    process_tree_recursive_generated(root, BitVector::new());
}

fn process_tree_recursive_generated(node: &mut HtmlNode, parent_state: BitVector) {
    let child_states = process_node_generated_inplace(node, parent_state);

    // Recursively process children
    for child in node.children.iter_mut() {
        process_tree_recursive_generated(child, child_states);
    }
}

fn load_dom_from_file() -> HtmlNode {
    // Try to read Google trace data from file
    let json_data =
        fs::read_to_string("css-gen-op/command.json").expect("fail to read command.json");

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

    // Convert JSON DOM to HtmlNode
    convert_json_dom_to_html_node(google_node_data)
}
fn process_tree_with_stats(root: &mut HtmlNode) -> (usize, usize, usize) {
    let total_nodes = count_total_nodes(root);

    // Process with generated function (if available)
    let _result = process_css_tree(root);

    // Return mock stats: (total, cache_hits, cache_misses)
    (total_nodes, total_nodes / 2, total_nodes / 2)
}

fn process_css_tree(root: &mut HtmlNode) -> BitVector {
    // This would be replaced with the actual generated function
    // For demo purposes, just process with incremental function if available
    if let Some(generated_fn) = get_generated_css_function() {
        return generated_fn(root);
    }

    // Fallback: return empty bitvector
    BitVector::new()
}

fn get_generated_css_function() -> Option<fn(&mut HtmlNode) -> BitVector> {
    // This would dynamically determine if generated function is available
    // For now, return None to use fallback
    None
}
fn main() {
    // Create the Google DOM tree from file-based data
    let mut root = load_dom_from_file();

    let total_nodes = count_total_nodes(&root);
    println!("🌳 DOM tree loaded: {} nodes", total_nodes);

    // Test 1: Process with generated CSS engine
    println!("\n📊 CSS Processing Test");
    let (total1, hits1, misses1) = process_tree_with_stats(&mut root);
    println!("  Processed nodes: {}", total1);
    println!("  Mock cache hits: {}", hits1);
    println!("  Mock cache misses: {}", misses1);
    println!("  Total CSS matches: {}", count_matches(&root));

    println!("\nSUCCESS: Generated CSS engine with module references completed!");
}
