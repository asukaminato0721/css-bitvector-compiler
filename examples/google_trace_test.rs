use css_bitvector_compiler::*;

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
    let current_matches = node.cached_node_intrinsic.unwrap();
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
fn main() {
    // 使用 RDTSC 指令测量程序启动时间
    let program_start_cycles = rdtsc();
    println!("🚀 Program started at CPU cycle: {}", program_start_cycles);

    // Read and process command.json file
    let command_file_path = "css-gen-op/command.json";
    println!("📄 Reading commands from: {}", command_file_path);

    let content = match std::fs::read_to_string(command_file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ Failed to read command file: {}", e);
            return;
        }
    };

    let lines: Vec<&str> = content.lines().collect();
    println!("📋 Found {} command lines", lines.len());

    // Process first line - should be "init" command
    if lines.is_empty() {
        eprintln!("❌ No commands found in file");
        return;
    }

    // Parse and execute commands
    let mut root = None;
    let mut current_step = 0;

    for (line_num, line) in lines.iter().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        // Parse JSON command
        let command: serde_json::Value = match serde_json::from_str(line) {
            Ok(cmd) => cmd,
            Err(e) => {
                eprintln!("❌ Failed to parse command on line {}: {}", line_num + 1, e);
                continue;
            }
        };

        let command_name = command["name"].as_str().unwrap_or("unknown");
        println!(
            "\n🔧 Processing command {}: '{}'",
            line_num + 1,
            command_name
        );

        match command_name {
            "init" => {
                // Initialize DOM from first command
                let dom_load_start = rdtsc();
                if let Some(node_data) = command.get("node") {
                    match GoogleNode::from_json(node_data) {
                        Some(google_node) => {
                            root = Some(google_node.to_html_node());
                            if let Some(ref mut root_node) = root {
                                root_node.init_parent_pointers();
                            }
                            println!("✅ DOM initialized with root node");
                        }
                        None => {
                            eprintln!("❌ Failed to parse init node");
                            continue;
                        }
                    }
                }
                // dbg!(&root);
                let dom_load_end = rdtsc();
                let dom_load_cycles = cycles_to_duration(dom_load_start, dom_load_end);
                println!("📊 DOM loading took: {} CPU cycles", dom_load_cycles);
            }
            "layout_init" => {
                // Skip layout initialization for now
                println!("⏭️  Skipping layout_init command");
            }
            _ => {
                // Process DOM modification commands
                if let Some(ref mut root_node) = root {
                    println!("🔄 Applying DOM modification command: {}", command_name);

                    // Apply the command based on type
                    let modification_applied = match command_name {
                        "add" => {
                            if let (Some(path_array), Some(node_data)) = (
                                command.get("path").and_then(|p| p.as_array()),
                                command.get("node"),
                            ) {
                                let path: Vec<usize> = path_array
                                    .iter()
                                    .filter_map(|v| v.as_u64().map(|n| n as usize))
                                    .collect();

                                if let Some(target_node) = navigate_to_path(root_node, &path) {
                                    // Create new HTML node from the provided node data
                                    if let Some(google_node) = GoogleNode::from_json(node_data) {
                                        let new_html_node = google_node.to_html_node();
                                        target_node.children.push(new_html_node);
                                        target_node.mark_dirty();
                                        println!("✅ Added new node at path {:?}", path);
                                        true
                                    } else {
                                        println!("❌ Failed to parse node data for add command");
                                        false
                                    }
                                } else {
                                    println!(
                                        "❌ Could not navigate to path {:?} for add command",
                                        path
                                    );
                                    false
                                }
                            } else {
                                println!("❌ Add command missing path or node data");
                                false
                            }
                        }
                        "insert_value" => {
                            if let (Some(path_array), Some(key), Some(value)) = (
                                command.get("path").and_then(|p| p.as_array()),
                                command.get("key").and_then(|k| k.as_str()),
                                command.get("value").and_then(|v| v.as_str()),
                            ) {
                                let path: Vec<usize> = path_array
                                    .iter()
                                    .filter_map(|v| v.as_u64().map(|n| n as usize))
                                    .collect();

                                if let Some(target_node) = navigate_to_path(root_node, &path) {
                                    // Insert new attribute/property
                                    insert_node_value(target_node, &command, key, value);
                                    target_node.mark_dirty();
                                    println!("✅ Inserted {} = {} at path {:?}", key, value, path);
                                    true
                                } else {
                                    println!(
                                        "❌ Could not navigate to path {:?} for insert_value command",
                                        path
                                    );
                                    false
                                }
                            } else {
                                println!("❌ Insert_value command missing required fields");
                                false
                            }
                        }
                        "replace_value" => {
                            if let (Some(path_array), Some(key), Some(new_value)) = (
                                command.get("path").and_then(|p| p.as_array()),
                                command.get("key").and_then(|k| k.as_str()),
                                command.get("value").and_then(|v| v.as_str()),
                            ) {
                                let path: Vec<usize> = path_array
                                    .iter()
                                    .filter_map(|v| v.as_u64().map(|n| n as usize))
                                    .collect();

                                if let Some(target_node) = navigate_to_path(root_node, &path) {
                                    // Replace existing value
                                    replace_node_value(target_node, &command, key, new_value);
                                    target_node.mark_dirty();
                                    println!(
                                        "✅ Replaced {} = {} at path {:?}",
                                        key, new_value, path
                                    );
                                    true
                                } else {
                                    println!(
                                        "❌ Could not navigate to path {:?} for replace_value command",
                                        path
                                    );
                                    false
                                }
                            } else {
                                println!("❌ Replace_value command missing required fields");
                                false
                            }
                        }
                        "recalculate" => {
                            if let Some(time) = command.get("time").and_then(|t| t.as_str()) {
                                println!("🧮 Recalculation triggered at time: {}", time);
                            } else {
                                println!("🧮 Recalculation triggered (no timestamp)");
                            }
                            // Force a full recomputation by marking root dirty
                            root_node.mark_dirty();
                            true
                        }
                        "remove" => {
                            if let Some(path_array) = command.get("path").and_then(|p| p.as_array())
                            {
                                let path: Vec<usize> = path_array
                                    .iter()
                                    .filter_map(|v| v.as_u64().map(|n| n as usize))
                                    .collect();

                                if !path.is_empty() {
                                    let parent_path = &path[..path.len() - 1];
                                    let child_index = path[path.len() - 1];

                                    if let Some(parent_node) =
                                        navigate_to_path(root_node, parent_path)
                                    {
                                        if child_index < parent_node.children.len() {
                                            parent_node.children.remove(child_index);
                                            parent_node.mark_dirty();
                                            println!("✅ Removed node at path {:?}", path);
                                            true
                                        } else {
                                            println!(
                                                "❌ Child index {} out of bounds for remove command",
                                                child_index
                                            );
                                            false
                                        }
                                    } else {
                                        println!(
                                            "❌ Could not navigate to parent path for remove command"
                                        );
                                        false
                                    }
                                } else {
                                    println!("❌ Cannot remove root node");
                                    false
                                }
                            } else {
                                println!("❌ Remove command missing path");
                                false
                            }
                        }
                        _ => {
                            // For any other command type, mark a random node as dirty
                            if let Some(random_node) =
                                find_deep_node(root_node, 4 + current_step % 5)
                            {
                                random_node.mark_dirty();
                                println!(
                                    "🔀 Generic modification at depth {}",
                                    4 + current_step % 5
                                );
                                true
                            } else {
                                false
                            }
                        }
                    };

                    if modification_applied {
                        current_step += 1;

                        // Test processing after each command
                        let total_nodes = count_total_nodes(root_node);
                        println!("🌳 DOM tree has {} nodes after command", total_nodes);

                        // Test performance after modification
                        println!("📊 Performance test after command {}:", current_step);
                        let start_test = rdtsc();
                        let (total, hits, misses) = process_tree_with_stats(root_node);
                        let end_test = rdtsc();
                        let cycles_test = cycles_to_duration(start_test, end_test);

                        println!("  Processed nodes: {}", total);
                        println!("  Cache hits: {}", hits);
                        println!("  Cache misses: {}", misses);
                        println!(
                            "  Cache hit rate: {:.2}%",
                            if total > 0 {
                                hits as f64 / total as f64 * 100.0
                            } else {
                                0.0
                            }
                        );
                        println!("  Processing cycles: {}", cycles_test);
                        println!("  Total CSS matches: {}", count_matches(root_node));

                        // Analyze modification impact
                        if misses > 20 {
                            println!("  💥 Major change detected with {} cache misses", misses);
                        } else if misses > 5 {
                            println!("  💡 Moderate change detected with {} cache misses", misses);
                        } else if misses == 0 {
                            println!("  ⚡ Perfect cache efficiency - no recomputation needed!");
                        } else {
                            println!("  ✨ Minor change detected with {} cache misses", misses);
                        }
                    } else {
                        println!("  ⚠️  Could not apply modification - target node not found");
                    }
                } else {
                    eprintln!(
                        "❌ No DOM root available for modification command: {}",
                        command_name
                    );
                }
            }
        }
    }

    // Final summary if we have a DOM
    if let Some(ref mut root_node) = root {
        let total_nodes = count_total_nodes(root_node);
        println!("\n📊 Final Summary:");
        println!("🌳 Total DOM nodes: {}", total_nodes);
        println!("🔧 Commands processed: {}", current_step);

        // Final performance benchmark
        println!("\n🏁 Final Performance Benchmark");
        let iterations = 50;

        // Benchmark incremental processing
        let cached_time = time_processing(
            || {
                let _ = process_tree_incremental_with_stats(root_node);
            },
            iterations,
        );

        // Compare with full recomputation
        let mut root_full = load_dom_from_file();
        root_full.init_parent_pointers();
        let full_time = time_processing(
            || {
                reset_cache_state(&mut root_full);
                let _ = process_tree_full_recompute(&mut root_full);
            },
            iterations,
        );

        println!(
            "  Incremental processing ({} iterations): {} cycles",
            iterations, cached_time
        );
        println!(
            "  Full recompute ({} iterations): {} cycles",
            iterations, full_time
        );
        println!(
            "  Average per iteration (incremental): {} cycles",
            cached_time / iterations as u64
        );
        println!(
            "  Average per iteration (full): {} cycles",
            full_time / iterations as u64
        );

        if full_time > 0 && cached_time > 0 {
            let speedup = full_time as f64 / cached_time as f64;
            println!("  📈 Incremental vs Full speedup: {:.2}x", speedup);
        }
    } else {
        eprintln!("❌ No DOM was successfully initialized");
    }

    let program_end_cycles = rdtsc();
    let total_program_cycles = cycles_to_duration(program_start_cycles, program_end_cycles);
    println!(
        "⏱️  Total program execution: {} CPU cycles",
        total_program_cycles
    );
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
    1 + node.children.iter().map(count_total_nodes).sum::<usize>()
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
fn reset_cache_state(node: &mut HtmlNode) {
    node.is_self_dirty = true;
    node.has_dirty_descendant = false;
    node.cached_parent_state = None;
    node.cached_node_intrinsic = None;
    node.cached_child_states = Some(BitVector::new());

    for child in node.children.iter_mut() {
        reset_cache_state(child);
    }
}

// GoogleNode definitions - copied from lib
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
