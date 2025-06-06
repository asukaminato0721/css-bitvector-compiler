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

    // Convert JSON DOM to HtmlNode and initialize parent pointers
    let mut root = convert_json_dom_to_html_node(google_node_data);
    root.init_parent_pointers();
    root
}
// Real incremental processing with statistics tracking
fn process_tree_incremental_with_stats(root: &mut HtmlNode) -> (usize, usize, usize) {
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
fn process_tree_full_recompute(root: &mut HtmlNode) -> (usize, usize, usize) {
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
    let child_states = process_node_generated_inplace(node, parent_state);

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
fn main() {
    // 使用 RDTSC 指令测量程序启动时间
    let program_start_cycles = rdtsc();
    println!("🚀 Program started at CPU cycle: {}", program_start_cycles);

    // Create the Google DOM tree from file-based data
    let dom_load_start = rdtsc();
    let mut root = load_dom_from_file();
    let dom_load_end = rdtsc();
    let dom_load_cycles = cycles_to_duration(dom_load_start, dom_load_end);

    let total_nodes = count_total_nodes(&root);
    println!("🌳 DOM tree loaded: {} nodes", total_nodes);
    println!("📊 DOM loading took: {} CPU cycles", dom_load_cycles);

    // Test 1: First run (all cache misses expected)
    println!("\n📊 Test 1: Initial processing (cache population)");
    let start_test1 = rdtsc();
    let (total1, hits1, misses1) = process_tree_with_stats(&mut root);
    let end_test1 = rdtsc();
    let cycles_test1 = cycles_to_duration(start_test1, end_test1);

    println!("  Processed nodes: {}", total1);
    println!("  Cache hits: {}", hits1);
    println!("  Cache misses: {}", misses1);
    println!(
        "  Cache hit rate: {:.2}%",
        if total1 > 0 {
            hits1 as f64 / total1 as f64 * 100.0
        } else {
            0.0
        }
    );
    println!("  Processing cycles: {}", cycles_test1);
    println!("  Total CSS matches: {}", count_matches(&root));

    // Test 2: Second run (should have high cache hit rate)
    println!("\n📊 Test 2: Second run (cache optimization)");
    let start_test2 = rdtsc();
    let (total2, hits2, misses2) = process_tree_with_stats(&mut root);
    let end_test2 = rdtsc();
    let cycles_test2 = cycles_to_duration(start_test2, end_test2);

    println!("  Processed nodes: {}", total2);
    println!("  Cache hits: {}", hits2);
    println!("  Cache misses: {}", misses2);
    println!(
        "  Cache hit rate: {:.2}%",
        if total2 > 0 {
            hits2 as f64 / total2 as f64 * 100.0
        } else {
            0.0
        }
    );
    println!("  Processing cycles: {}", cycles_test2);

    // Performance comparison
    if cycles_test1 > 0 && cycles_test2 > 0 {
        let speedup = cycles_test1 as f64 / cycles_test2 as f64;
        println!("  🚀 Speedup from caching: {:.2}x", speedup);
    }

    // Test 3: Mark a deep node dirty and test incremental processing
    if let Some(deep_node) = find_deep_node(&mut root, 5) {
        deep_node.mark_self_dirty();
        println!("\n📝 Marked a deep node dirty...");

        println!("\n📊 Test 3: After deep node modification");
        let start_test3 = rdtsc();
        let (total3, hits3, misses3) = process_tree_with_stats(&mut root);
        let end_test3 = rdtsc();
        let cycles_test3 = cycles_to_duration(start_test3, end_test3);

        println!("  Processed nodes: {}", total3);
        println!("  Cache hits: {}", hits3);
        println!("  Cache misses: {}", misses3);
        println!(
            "  Cache hit rate: {:.2}%",
            if total3 > 0 {
                hits3 as f64 / total3 as f64 * 100.0
            } else {
                0.0
            }
        );
        println!("  Processing cycles: {}", cycles_test3);
        println!(
            "  💡 Optimization: Only {} nodes needed reprocessing!",
            misses3
        );

        // Compare with full reprocessing
        if cycles_test1 > 0 && cycles_test3 > 0 {
            let incremental_speedup = cycles_test1 as f64 / cycles_test3 as f64;
            println!(
                "  ⚡ Incremental speedup: {:.2}x vs full recomputation",
                incremental_speedup
            );
        }
    } else {
        println!("\n⚠️  Could not find deep node for Test 3");
    }

    // Performance benchmark with multiple iterations
    println!("\n🏁 Performance Benchmark (Real Generated Code)");
    let iterations = 100;

    // Benchmark 1: Full recomputation (no caching)
    let mut root_full = load_dom_from_file();
    root_full.init_parent_pointers();
    let full_time = time_processing(
        || {
            // Always reset cache state to force full recomputation
            reset_cache_state(&mut root_full);
            let _ = process_tree_full_recompute(&mut root_full);
        },
        iterations,
    );

    // Benchmark 2: Incremental processing (with caching)
    let cached_time = time_processing(
        || {
            let _ = process_tree_incremental_with_stats(&mut root);
        },
        iterations,
    );

    // Benchmark 3: Generated code direct call (minimal overhead)
    let mut root_direct = load_dom_from_file();
    root_direct.init_parent_pointers();
    let direct_time = time_processing(
        || {
            process_node_generated_incremental(&mut root_direct, BitVector::new());
        },
        iterations,
    );

    println!(
        "  Full recompute ({} iterations): {} cycles",
        iterations, full_time
    );
    println!(
        "  Incremental cached ({} iterations): {} cycles",
        iterations, cached_time
    );
    println!(
        "  Direct generated function ({} iterations): {} cycles",
        iterations, direct_time
    );

    if full_time > 0 && cached_time > 0 {
        let incremental_speedup = full_time as f64 / cached_time as f64;
        println!(
            "  📈 Incremental vs Full speedup: {:.2}x",
            incremental_speedup
        );
    }

    if cached_time > 0 && direct_time > 0 {
        let direct_speedup = cached_time as f64 / direct_time as f64;
        println!(
            "  ⚡ Generated code efficiency: {:.2}x vs tree traversal",
            direct_speedup
        );
    }

    let program_end_cycles = rdtsc();
    let total_program_cycles = cycles_to_duration(program_start_cycles, program_end_cycles);
    println!(
        "⏱️  Total program execution: {} CPU cycles",
        total_program_cycles
    );

    println!("\nSUCCESS: Generated CSS engine with comprehensive performance testing completed!");
}

// Helper function to reset cache state for benchmarking
fn reset_cache_state(node: &mut HtmlNode) {
    node.is_self_dirty = true;
    node.has_dirty_descendant = false;
    node.cached_parent_state = None;
    node.cached_node_intrinsic = None;
    node.cached_child_states = None;

    for child in node.children.iter_mut() {
        reset_cache_state(child);
    }
}
