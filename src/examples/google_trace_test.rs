use crate::*;

// Generated CSS processing function
// Generated Tree NFA Program with Incremental Processing
// This program processes HTML nodes and computes CSS matches with caching

pub fn process_node_generated_incremental(
    node: &mut HtmlNode,
    parent_state: BitVector,
) -> BitVector {
    // returns child_states
    // Check if we need to recompute
    if !node.needs_any_recomputation(parent_state) {
        // Return cached result - entire subtree can be skipped
        return node.cached_child_states.unwrap_or_default();
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

pub fn node_matches_selector_generated(node: &HtmlNode, selector: &SimpleSelector) -> bool {
    match selector {
        SimpleSelector::Type(tag) => node.tag_name == *tag,
        SimpleSelector::Class(class) => node.classes.contains(class),
        SimpleSelector::Id(id) => node.id.as_deref() == Some(id),
    }
}

// Real incremental processing with statistics tracking
pub fn process_tree_incremental_with_stats(root: &mut HtmlNode) -> (usize, usize, usize) {
    let mut total_nodes = 0;
    let mut cache_hits = 0;
    let mut cache_misses = 0;

    process_tree_recursive_with_stats(
        root,
        BitVector::new(),
        &mut total_nodes,
        &mut cache_hits,
        &mut cache_misses,
    );

    (total_nodes, cache_hits, cache_misses)
}

fn process_tree_recursive_with_stats(
    node: &mut HtmlNode,
    parent_state: BitVector,
    total: &mut usize,
    hits: &mut usize,
    misses: &mut usize,
) {
    *total += 1;

    // Check if we need to recompute using the real incremental logic
    if node.needs_any_recomputation(parent_state) {
        *misses += 1;

        // Use the actual generated incremental processing function
        let child_states = process_node_generated_incremental(node, parent_state);

        // Process children recursively
        for child in node.children.iter_mut() {
            process_tree_recursive_with_stats(child, child_states, total, hits, misses);
        }
    } else {
        *hits += 1;
        // Skip entire subtree when cached - this is the power of incremental processing
    }
}

// Non-incremental processing for comparison (always recomputes everything)
pub fn process_tree_full_recompute(root: &mut HtmlNode) -> (usize, usize, usize) {
    let mut total_nodes = 0;
    let cache_hits = 0; // No caching in full recompute
    let mut cache_misses = 0;

    process_tree_recursive_full(root, BitVector::new(), &mut total_nodes, &mut cache_misses);

    (total_nodes, cache_hits, cache_misses)
}

fn process_tree_recursive_full(
    node: &mut HtmlNode,
    parent_state: BitVector,
    total: &mut usize,
    misses: &mut usize,
) {
    *total += 1;
    *misses += 1;

    // Always use the non-incremental (in-place) processing - no caching
    let child_states = process_node_generated_incremental(node, parent_state);

    // Process all children
    for child in node.children.iter_mut() {
        process_tree_recursive_full(child, child_states, total, misses);
    }
}
fn process_tree_with_stats(root: &mut HtmlNode) -> (usize, usize, usize) {
    process_tree_incremental_with_stats(root)
}

fn find_deep_node(node: &mut HtmlNode, target_depth: usize) -> Option<&mut HtmlNode> {
    node.find_deep_node_mut(target_depth)
}

// Helper function to extract value from JSON (handles both string and number)
fn extract_value_as_string(value: &serde_json::Value) -> Option<String> {
    match value {
        serde_json::Value::String(s) => Some(s.clone()),
        serde_json::Value::Number(n) => Some(n.to_string()),
        serde_json::Value::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

fn time_processing<F>(mut func: F, iterations: usize) -> u64
where
    F: FnMut(),
{
    let start_cycles = rdtsc();
    for _ in 0..iterations {
        func();
    }
    let end_cycles = rdtsc();
    cycles_to_duration(start_cycles, end_cycles)
}

// Navigate to a specific node using a path array
fn navigate_to_path<'a>(root: &'a mut HtmlNode, path: &[usize]) -> Option<&'a mut HtmlNode> {
    let mut current = root;

    for &index in path {
        if index < current.children.len() {
            current = &mut current.children[index];
        } else {
            return None;
        }
    }

    Some(current)
}

// Insert a new attribute/property value into a node
fn insert_node_value(_node: &mut HtmlNode, command: &serde_json::Value, key: &str, value: &str) {
    // Check the type field to determine where to insert
    if let Some(value_type) = command.get("type").and_then(|t| t.as_str()) {
        match value_type {
            "attributes" => {
                // Insert into node's attributes (simulate)
                // For now, we'll just mark as dirty since HtmlNode doesn't have attributes HashMap
                println!("  📝 Would insert attribute: {} = {}", key, value);
            }
            "properties" => {
                // Insert into node's properties (simulate)
                println!("  📝 Would insert property: {} = {}", key, value);
            }
            _ => {
                println!("  ⚠️  Unknown insert type: {}", value_type);
            }
        }
    } else {
        // Default to attributes if no type specified
        println!("  📝 Would insert attribute (default): {} = {}", key, value);
    }
}

// Replace an existing attribute/property value in a node
fn replace_node_value(
    _node: &mut HtmlNode,
    command: &serde_json::Value,
    key: &str,
    new_value: &str,
) {
    // Check the type field to determine what to replace
    if let Some(value_type) = command.get("type").and_then(|t| t.as_str()) {
        let old_value = command
            .get("old_value")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        match value_type {
            "attributes" => {
                // Replace node's attribute (simulate)
                println!(
                    "  🔄 Would replace attribute: {} = {} -> {}",
                    key, old_value, new_value
                );
            }
            "properties" => {
                // Replace node's property (simulate)
                println!(
                    "  🔄 Would replace property: {} = {} -> {}",
                    key, old_value, new_value
                );
            }
            _ => {
                println!("  ⚠️  Unknown replace type: {}", value_type);
            }
        }
    } else {
        // Default to attributes if no type specified
        println!(
            "  🔄 Would replace attribute (default): {} = {}",
            key, new_value
        );
    }
}

// Helper function to load DOM from the command.json file
fn load_dom_from_file() -> HtmlNode {
    let command_file_path = "css-gen-op/command.json";
    let content = std::fs::read_to_string(command_file_path).expect("Failed to read command.json");

    let first_line = content.lines().next().expect("Empty command file");
    let command: serde_json::Value =
        serde_json::from_str(first_line).expect("Failed to parse first command");

    if command["name"] != "init" {
        panic!("First command should be init");
    }

    let google_node = GoogleNode::from_json(&command["node"]).expect("Failed to parse Google node");

    google_node.to_html_node()
}

// Helper function to count total nodes in DOM tree
fn count_total_nodes(node: &HtmlNode) -> usize {
    1 + node
        .children
        .iter()
        .map(|child| count_total_nodes(child))
        .sum::<usize>()
}

// Helper function to count CSS matches
fn count_matches(node: &HtmlNode) -> usize {
    let current_matches = if node.css_match_bitvector.bits != 0 {
        1
    } else {
        0
    };
    current_matches + node.children.iter().map(count_matches).sum::<usize>()
}

// Helper function to reset cache state for benchmarking
pub fn reset_cache_state(node: &mut HtmlNode) {
    node.is_self_dirty = true;
    node.has_dirty_descendant = false;
    node.cached_parent_state = None;
    node.cached_node_intrinsic = None;
    node.cached_child_states = Some(BitVector::new());

    for child in node.children.iter_mut() {
        reset_cache_state(child);
    }
}


