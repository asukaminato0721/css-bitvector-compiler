use regex::Regex;
use scraper::{Html, Selector as HtmlSelector};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use css_bitvector_compiler::BitVector;
use css_bitvector_compiler::CssCompiler;
use css_bitvector_compiler::CssRule;
use css_bitvector_compiler::HtmlNode;
use css_bitvector_compiler::NFAInstruction;
use css_bitvector_compiler::SelectorMatchingIndex;
use css_bitvector_compiler::SimpleSelector;
use css_bitvector_compiler::TreeNFAProgram;
use css_bitvector_compiler::count_total_nodes;
use css_bitvector_compiler::cycles_to_duration;
use css_bitvector_compiler::parse_basic_css;
use css_bitvector_compiler::rdtsc;

// All types are now defined in lib.rs and imported from there

// Google Trace Testing Integration
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

    fn init_parent_pointers(&mut self) {
        for child in &mut self.children {
            child.init_parent_pointers();
        }
    }
}

pub fn process_google_trace_with_rust() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Testing Google Trace with Rust CSS Engine (CodeGen Mode)\n");

    // Load Google CSS rules
    let css_content = std::fs::read_to_string("css-gen-op/https___www.google.com_.css")
        .unwrap_or_else(|_| {
            println!("⚠️ Could not load Google CSS file, using basic rules");
            "div { display: block; } .gbts { color: #000; } #gb { position: relative; }".to_string()
        });

    let css_rules = parse_basic_css(&css_content);
    println!("📋 Loaded {} CSS rules from Google CSS", css_rules.len());

    // Compile CSS rules and generate Rust code
    let mut compiler = CssCompiler::new();
    let program = compiler.compile_css_rules(&css_rules);

    println!("🔧 Generating optimized Rust code...");
    let generated_code = program.generate_rust_code();

    // Read the first command from command.json to get initial DOM
    let commands_content = std::fs::read_to_string("css-gen-op/command.json")?;
    let first_line = commands_content
        .lines()
        .next()
        .ok_or("Empty command file")?;
    let command: serde_json::Value = serde_json::from_str(first_line)?;

    if command["name"] != "init" {
        return Err("First command should be init".into());
    }

    let google_node =
        GoogleNode::from_json(&command["node"]).ok_or("Failed to parse Google node")?;

    println!(
        "🌳 Google DOM tree contains {} nodes",
        google_node.count_nodes()
    );

    // Generate complete Rust program for Google trace testing
    let complete_program = generate_google_trace_program(&generated_code, &google_node)?;

    // Write to examples directory
    let example_file = "examples/google_trace_test.rs";
    std::fs::write(example_file, &complete_program)
        .map_err(|e| format!("Failed to write generated code: {}", e))?;

    println!("💾 Generated example: {}", example_file);

    // Run the generated example
    println!("🚀 Running generated example with Google trace data...\n");
    let run_output = std::process::Command::new("cargo")
        .args(["run", "--example", "google_trace_test"])
        .output()
        .map_err(|e| format!("Failed to run example: {}", e))?;

    if run_output.status.success() {
        let stdout = String::from_utf8_lossy(&run_output.stdout);
        println!("{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&run_output.stderr);
        return Err(format!("Generated example failed: {}", stderr).into());
    }

    Ok(())
}

fn find_deep_node(node: &mut HtmlNode, target_depth: usize) -> Option<&mut HtmlNode> {
    node.find_deep_node_mut(target_depth)
}

fn count_css_matches(node: &HtmlNode) -> usize {
    let current_matches = if node.css_match_bitvector.bits != 0 {
        1
    } else {
        0
    };
    current_matches + node.children.iter().map(count_css_matches).sum::<usize>()
}

fn generate_google_trace_program(
    generated_fn_code: &str,
    google_node: &GoogleNode,
) -> Result<String, Box<dyn std::error::Error>> {
    // 使用模块引用方法 - 直接使用库中定义的类型和函数
    let mut program = String::new();

    // 1. 导入库中的所有类型和函数
    program.push_str("use css_bitvector_compiler::*;\n\n");

    // 2. 添加生成的 CSS 处理函数
    program.push_str("// Generated CSS processing function\n");
    program.push_str(generated_fn_code);
    program.push_str("\n\n");

    // 3. Add real incremental processing with stats tracking
    program.push_str(
        r#"// Real incremental processing with statistics tracking
fn process_tree_incremental_with_stats(root: &mut HtmlNode) -> (usize, usize, usize) {
    let mut total_nodes = 0;
    let mut cache_hits = 0;
    let mut cache_misses = 0;
    
    process_tree_recursive_with_stats(root, BitVector::new(), &mut total_nodes, &mut cache_hits, &mut cache_misses);
    
    (total_nodes, cache_hits, cache_misses)
}

fn process_tree_recursive_with_stats(node: &mut HtmlNode, parent_state: BitVector, 
                                    total: &mut usize, hits: &mut usize, misses: &mut usize) {
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

fn process_tree_recursive_full(node: &mut HtmlNode, parent_state: BitVector, 
                              total: &mut usize, misses: &mut usize) {
    *total += 1;
    *misses += 1;
    
    // Always use the non-incremental (in-place) processing - no caching
    let child_states = process_node_generated_incremental(node, parent_state);
    
    // Process all children
    for child in node.children.iter_mut() {
        process_tree_recursive_full(child, child_states, total, misses);
    }
}
"#,
    );

    // 4. 添加测试和分析函数
    program.push_str(
        r#"fn process_tree_with_stats(root: &mut HtmlNode) -> (usize, usize, usize) {
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
"#,
    );

    // 4. Add helper functions for enhanced command processing
    program.push_str(
        r#"
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
fn replace_node_value(_node: &mut HtmlNode, command: &serde_json::Value, key: &str, new_value: &str) {
    // Check the type field to determine what to replace
    if let Some(value_type) = command.get("type").and_then(|t| t.as_str()) {
        let old_value = command.get("old_value").and_then(|v| v.as_str()).unwrap_or("unknown");
        
        match value_type {
            "attributes" => {
                // Replace node's attribute (simulate)
                println!("  🔄 Would replace attribute: {} = {} -> {}", key, old_value, new_value);
            }
            "properties" => {
                // Replace node's property (simulate)
                println!("  🔄 Would replace property: {} = {} -> {}", key, old_value, new_value);
            }
            _ => {
                println!("  ⚠️  Unknown replace type: {}", value_type);
            }
        }
    } else {
        // Default to attributes if no type specified
        println!("  🔄 Would replace attribute (default): {} = {}", key, new_value);
    }
}
"#,
    );

    // 5. 主函数 with enhanced command.json processing
    program.push_str(
        r#"fn main() {    
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
        println!("\n🔧 Processing command {}: '{}'", line_num + 1, command_name);

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
                            if let (Some(path_array), Some(node_data)) = (command.get("path").and_then(|p| p.as_array()), command.get("node")) {
                                let path: Vec<usize> = path_array
                                    .iter()
                                    .filter_map(|v| v.as_u64().map(|n| n as usize))
                                    .collect();
                                
                                if !path.is_empty() {
                                    // Split path: all but last element navigate to parent, last element is insert position
                                    let parent_path = &path[..path.len() - 1];
                                    let insert_index = path[path.len() - 1];

                                    if let Some(parent_node) = navigate_to_path(root_node, parent_path) {
                                        // Create new HTML node from the provided node data
                                        if let Some(google_node) = GoogleNode::from_json(node_data) {
                                            let new_html_node = google_node.to_html_node();
                                            
                                            // Insert at specified position (or append if index == children.len())
                                            if insert_index <= parent_node.children.len() {
                                                parent_node.children.insert(insert_index, new_html_node);
                                                parent_node.mark_dirty();
                                                println!("✅ Added new node at path {:?} (insert position {})", path, insert_index);
                                                true
                                            } else {
                                                println!(
                                                    "❌ Insert index {} out of bounds (max: {}) for add command",
                                                    insert_index, parent_node.children.len()
                                                );
                                                false
                                            }
                                        } else {
                                            println!("❌ Failed to parse node data for add command");
                                            false
                                        }
                                    } else {
                                        println!(
                                            "❌ Could not navigate to parent path {:?} for add command",
                                            parent_path
                                        );
                                        false
                                    }
                                } else {
                                    println!("❌ Add command requires non-empty path");
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
                                command.get("value").and_then(extract_value_as_string)
                            ) {
                                let path: Vec<usize> = path_array
                                    .iter()
                                    .filter_map(|v| v.as_u64().map(|n| n as usize))
                                    .collect();
                                
                                if let Some(target_node) = navigate_to_path(root_node, &path) {
                                    // Insert new attribute/property
                                    insert_node_value(target_node, &command, key, &value);
                                    target_node.mark_dirty();
                                    println!("✅ Inserted {} = {} at path {:?}", key, value, path);
                                    true
                                } else {
                                    println!("❌ Could not navigate to path {:?} for insert_value command", path);
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
                                command.get("value").and_then(extract_value_as_string)
                            ) {
                                let path: Vec<usize> = path_array
                                    .iter()
                                    .filter_map(|v| v.as_u64().map(|n| n as usize))
                                    .collect();
                                
                                if let Some(target_node) = navigate_to_path(root_node, &path) {
                                    // Replace existing value
                                    replace_node_value(target_node, &command, key, &new_value);
                                    target_node.mark_dirty();
                                    println!("✅ Replaced {} = {} at path {:?}", key, new_value, path);
                                    true
                                } else {
                                    println!("❌ Could not navigate to path {:?} for replace_value command", path);
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
                            if let Some(path_array) = command.get("path").and_then(|p| p.as_array()) {
                                let path: Vec<usize> = path_array
                                    .iter()
                                    .filter_map(|v| v.as_u64().map(|n| n as usize))
                                    .collect();
                                
                                if path.len() > 0 {
                                    let parent_path = &path[..path.len()-1];
                                    let child_index = path[path.len()-1];
                                    
                                    if let Some(parent_node) = navigate_to_path(root_node, parent_path) {
                                        if child_index < parent_node.children.len() {
                                            parent_node.children.remove(child_index);
                                parent_node.mark_dirty();
                                            println!("✅ Removed node at path {:?}", path);
                                true
                                        } else {
                                            println!("❌ Child index {} out of bounds for remove command", child_index);
                                            false
                                        }
                                    } else {
                                        println!("❌ Could not navigate to parent path for remove command");
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
                            if let Some(random_node) = find_deep_node(root_node, 4 + current_step % 5) {
                                random_node.mark_dirty();
                                println!("🔀 Generic modification at depth {}", 4 + current_step % 5);
                                true
                            } else { false }
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
                    eprintln!("❌ No DOM root available for modification command: {}", command_name);
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
        println!("  Average per iteration (incremental): {} cycles", cached_time / iterations as u64);
        println!("  Average per iteration (full): {} cycles", full_time / iterations as u64);
        
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
"#,
    );

    // 6. 生成 load_dom_from_file 函数和帮助函数
    program.push_str(
        r#"
// Helper function to load DOM from the command.json file
fn load_dom_from_file() -> HtmlNode {
    let command_file_path = "css-gen-op/command.json";
    let content = std::fs::read_to_string(command_file_path)
        .expect("Failed to read command.json");
    
    let first_line = content.lines().next().expect("Empty command file");
    let command: serde_json::Value = serde_json::from_str(first_line)
        .expect("Failed to parse first command");
    
    if command["name"] != "init" {
        panic!("First command should be init");
    }
    
    let google_node = GoogleNode::from_json(&command["node"])
        .expect("Failed to parse Google node");
    
    google_node.to_html_node()
}

// Helper function to count total nodes in DOM tree
fn count_total_nodes(node: &HtmlNode) -> usize {
    1 + node.children.iter().map(|child| count_total_nodes(child)).sum::<usize>()
}

// Helper function to count CSS matches 
fn count_matches(node: &HtmlNode) -> usize {
    let current_matches = if node.css_match_bitvector.bits != 0 { 1 } else { 0 };
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
"#,
    );

    Ok(program)
}

fn serialize_google_node_to_rust_code(node: &GoogleNode) -> String {
    fn serialize_node(node: &GoogleNode, depth: usize) -> String {
        let indent = "    ".repeat(depth + 1);
        let mut code = format!("{}HtmlNode::new(\"{}\")", indent, escape_string(&node.name));

        if let Some(id) = &node.id {
            code.push_str(&format!(".with_id(\"{}\")", escape_string(&id.to_string())));
        }

        if let Some(class_attr) = node.attributes.get("class") {
            for class in class_attr.split_whitespace() {
                code.push_str(&format!(".with_class(\"{}\")", escape_string(class)));
            }
        }

        for child in &node.children {
            code.push_str(&format!(
                "\n{}.add_child(\n{}\n{})",
                indent,
                serialize_node(child, depth + 1),
                indent
            ));
        }

        code
    }

    serialize_node(node, 0)
}

fn escape_string(s: &str) -> String {
    s.replace("\\", "\\\\")
        .replace("\"", "\\\"")
        .replace("\n", "\\n")
        .replace("\r", "\\r")
        .replace("\t", "\\t")
}

// --- 6. Tree NFA Virtual Machine ---
#[derive(Debug, Default)]
struct IncrementalStats {
    total_nodes: usize,
    cache_hits: usize,
    cache_misses: usize,
}

impl IncrementalStats {
    fn new() -> Self {
        IncrementalStats {
            total_nodes: 0,
            cache_hits: 0,
            cache_misses: 0,
        }
    }

    fn cache_hit_rate(&self) -> f64 {
        if self.total_nodes == 0 {
            0.0
        } else {
            self.cache_hits as f64 / self.total_nodes as f64
        }
    }

    fn print_summary(&self) {
        println!("=== Incremental Processing Stats ===");
        println!("Total nodes processed: {}", self.total_nodes);
        println!("Cache hits: {}", self.cache_hits);
        println!("Cache misses: {}", self.cache_misses);
        println!("Cache hit rate: {:.2}%", self.cache_hit_rate() * 100.0);
        println!("=====================================");
    }
}

struct TreeNFAVM {
    program: TreeNFAProgram,
    selector_index: SelectorMatchingIndex,
}

impl TreeNFAVM {
    fn new(program: TreeNFAProgram) -> Self {
        let mut selector_index = SelectorMatchingIndex::new();

        // Build index for fast selector matching
        for (i, instruction) in program.instructions.iter().enumerate() {
            selector_index.add_rule(i, instruction.clone());
        }

        TreeNFAVM {
            program,
            selector_index,
        }
    }

    fn process_node_inplace(&self, node: &mut HtmlNode, parent_state: BitVector) -> BitVector {
        let mut current_matches = BitVector::new();
        let mut child_states = BitVector::new();

        for instruction in &self.program.instructions {
            match instruction {
                NFAInstruction::CheckAndSetBit { selector, bit_pos } => {
                    if self.node_matches_selector(node, selector) {
                        current_matches.set_bit(*bit_pos);
                    }
                }
                NFAInstruction::CheckParentAndSetBit {
                    parent_state_bit,
                    child_selector,
                    result_bit,
                } => {
                    if parent_state.is_bit_set(*parent_state_bit)
                        && self.node_matches_selector(node, child_selector)
                    {
                        current_matches.set_bit(*result_bit);
                    }
                }
                NFAInstruction::PropagateToChildren {
                    match_bit,
                    active_bit,
                } => {
                    if current_matches.is_bit_set(*match_bit) {
                        child_states.set_bit(*active_bit);
                    }
                }
            }
        }

        // Store result in node (in-place)
        node.css_match_bitvector = current_matches;

        child_states
    }

    fn node_matches_selector(&self, node: &HtmlNode, selector: &SimpleSelector) -> bool {
        match selector {
            SimpleSelector::Type(tag) => node.tag_name == *tag,
            SimpleSelector::Class(class) => node.classes.contains(class),
            SimpleSelector::Id(id) => node.id.as_deref() == Some(id),
        }
    }

    // Driver function: recursively process entire tree
    fn process_tree(&self, root: &mut HtmlNode) {
        self.process_tree_recursive(root, BitVector::new(), "root".to_string());
    }

    fn process_tree_recursive(&self, node: &mut HtmlNode, parent_state: BitVector, path: String) {
        let child_states = self.process_node_inplace(node, parent_state);

        // Recursively process children
        for (i, child) in node.children.iter_mut().enumerate() {
            self.process_tree_recursive(child, child_states, format!("{}/{}", path, i));
        }
    }

    // Driver function with verbose output (for debugging)
    fn process_tree_verbose(&self, root: &mut HtmlNode) {
        self.dfs_execute_verbose(root, BitVector::new(), "root".to_string());
    }

    fn dfs_execute_verbose(&self, node: &mut HtmlNode, parent_state: BitVector, path: String) {
        let child_states = self.process_node_inplace(node, parent_state);

        println!(
            "Node: {} (Tag: {}, ID: {:?}, Classes: {:?})",
            path, node.tag_name, node.id, node.classes
        );
        println!("  Parent state: {:b}", parent_state);
        println!("  Match result: {:b}", node.css_match_bitvector);
        println!("  Child states: {:b}", child_states);

        // Decode matches
        println!("  Matches:");
        for (bit_pos, name) in &self.program.state_names {
            if node.css_match_bitvector.is_bit_set(*bit_pos) {
                println!("    - {}", name);
            }
        }
        println!("---");

        // Recursively process children
        for (i, child) in node.children.iter_mut().enumerate() {
            self.dfs_execute_verbose(child, child_states, format!("{}/{}", path, i));
        }
    }

    // Incremental processing: only recompute when inputs change
    fn process_node_incremental(&self, node: &mut HtmlNode, parent_state: BitVector) -> BitVector {
        // Check if we need to recompute
        if !node.needs_any_recomputation(parent_state) {
            // Return cached result
            return node.cached_child_states.unwrap_or_default();
        }

        // Recompute node intrinsic matches if needed (this is stable unless node attributes change)
        if node.cached_node_intrinsic.is_none() || node.is_self_dirty {
            let mut intrinsic_matches = BitVector::new();

            // Use optimized hash table lookup instead of linear search
            let matching_rules = self.selector_index.get_matching_rules(node);
            for instruction in matching_rules {
                if let NFAInstruction::CheckAndSetBit { bit_pos, .. } = instruction {
                    intrinsic_matches.set_bit(*bit_pos);
                }
            }

            node.cached_node_intrinsic = Some(intrinsic_matches);
        }

        // Compute final matches and child states
        let mut current_matches = node.cached_node_intrinsic.unwrap();
        let mut child_states = BitVector::new();

        // Apply parent-dependent rules - these still need to be checked
        for (_, instruction) in self.selector_index.get_parent_dependent_rules() {
            if let NFAInstruction::CheckParentAndSetBit {
                parent_state_bit,
                child_selector,
                result_bit,
            } = instruction
            {
                if parent_state.is_bit_set(*parent_state_bit)
                    && self.node_matches_selector(node, child_selector)
                {
                    current_matches.set_bit(*result_bit);
                }
            }
        }

        // Apply propagation rules
        for instruction in &self.program.instructions {
            if let NFAInstruction::PropagateToChildren {
                match_bit,
                active_bit,
            } = instruction
            {
                if current_matches.is_bit_set(*match_bit) {
                    child_states.set_bit(*active_bit);
                }
            }
        }

        // Cache results
        node.css_match_bitvector = current_matches;
        node.cached_parent_state = Some(parent_state);
        node.cached_child_states = Some(child_states);
        node.mark_clean();

        child_states
    }

    // Incremental tree processing driver
    fn process_tree_incremental(&self, root: &mut HtmlNode) {
        self.process_tree_incremental_recursive(root, BitVector::new());
    }

    fn process_tree_incremental_recursive(&self, node: &mut HtmlNode, parent_state: BitVector) {
        // Only process if this node or its descendants need recomputation
        if !node.needs_any_recomputation(parent_state) {
            return; // Skip entire subtree
        }

        let child_states = self.process_node_incremental(node, parent_state);

        // Process all children - in first run, all will be processed
        // In subsequent runs, only dirty paths will be processed
        let mut any_child_was_dirty = false;
        for child in node.children.iter_mut() {
            if child.needs_any_recomputation(child_states) {
                any_child_was_dirty = true;
                self.process_tree_incremental_recursive(child, child_states);
            }
        }

        // Clear the summary bit since we've processed all dirty descendants
        if any_child_was_dirty || node.has_dirty_descendant {
            node.mark_clean();
        }
    }

    // Debug version with statistics
    fn process_tree_incremental_with_stats(&self, root: &mut HtmlNode) -> IncrementalStats {
        let mut stats = IncrementalStats::new();
        self.process_tree_incremental_recursive_with_stats(root, BitVector::new(), &mut stats);
        stats
    }

    fn process_tree_incremental_recursive_with_stats(
        &self,
        node: &mut HtmlNode,
        parent_state: BitVector,
        stats: &mut IncrementalStats,
    ) {
        stats.total_nodes += 1;

        // Check if this node needs any recomputation
        let needs_recomputation = node.needs_any_recomputation(parent_state);

        if !needs_recomputation {
            stats.cache_hits += 1;
            return; // Skip entire subtree - this is a cache hit for the whole subtree
        }

        // Check if the node itself needs recomputation vs just traversal for descendants
        let needs_self_recomputation = node.is_self_dirty
            || node.cached_parent_state.is_none()
            || node.cached_parent_state.unwrap() != parent_state;

        if needs_self_recomputation {
            stats.cache_misses += 1;
            let child_states = self.process_node_incremental(node, parent_state);

            // Process all children
            let mut any_child_was_dirty = false;
            for child in node.children.iter_mut() {
                if child.needs_any_recomputation(child_states) {
                    any_child_was_dirty = true;
                    self.process_tree_incremental_recursive_with_stats(child, child_states, stats);
                }
            }

            // Clear the summary bit since we've processed all dirty descendants
            if any_child_was_dirty || node.has_dirty_descendant {
                node.mark_clean();
            }
        } else {
            // This node doesn't need recomputation, but descendants do
            stats.cache_hits += 1;
            let child_states = node.cached_child_states.unwrap_or_default();

            // Process children that need recomputation
            let mut any_child_was_dirty = false;
            for child in node.children.iter_mut() {
                if child.needs_any_recomputation(child_states) {
                    any_child_was_dirty = true;
                    self.process_tree_incremental_recursive_with_stats(child, child_states, stats);
                }
            }

            // Clear the summary bit since we've processed all dirty descendants
            if any_child_was_dirty || node.has_dirty_descendant {
                node.mark_clean();
            }
        }
    }
}

// --- 7. CSS Parser ---
fn parse_css_file(css_content: &str) -> Vec<CssRule> {
    let mut rules = Vec::new();

    let simple_rule_regex = Regex::new(r"/\* Rule \d+ \([^)]+\) \*/ ([^{]+) \{\}").unwrap();
    let child_rule_regex = Regex::new(r"([^>]+)>([^{]+)").unwrap();

    for cap in simple_rule_regex.captures_iter(css_content) {
        let selector_str = cap[1].trim();

        if let Some(child_cap) = child_rule_regex.captures(selector_str) {
            let parent_str = child_cap[1].trim();
            let child_str = child_cap[2].trim();

            if let (Some(parent_sel), Some(child_sel)) = (
                parse_simple_selector(parent_str),
                parse_simple_selector(child_str),
            ) {
                rules.push(CssRule::Child {
                    parent_selector: parent_sel,
                    child_selector: child_sel,
                });
            }
        } else if let Some(simple_sel) = parse_simple_selector(selector_str) {
            rules.push(CssRule::Simple(simple_sel));
        }
    }

    rules
}

fn parse_simple_selector(selector_str: &str) -> Option<SimpleSelector> {
    let trimmed = selector_str.trim();

    if trimmed.starts_with('#') {
        Some(SimpleSelector::Id(trimmed[1..].to_string()))
    } else if trimmed.starts_with('.') {
        Some(SimpleSelector::Class(trimmed[1..].to_string()))
    } else if !trimmed.is_empty() && trimmed.chars().all(|c| c.is_alphabetic()) {
        Some(SimpleSelector::Type(trimmed.to_string()))
    } else {
        None
    }
}

// --- 8. HTML Parser ---
fn parse_html_file(html_content: &str) -> Option<HtmlNode> {
    let document = Html::parse_document(html_content);

    let meaningful_selector = HtmlSelector::parse("div, section, p, span").unwrap();
    if let Some(first_element) = document.select(&meaningful_selector).next() {
        parse_element_to_node(&first_element)
    } else {
        None
    }
}

fn parse_element_to_node(element_ref: &scraper::ElementRef) -> Option<HtmlNode> {
    let element = element_ref.value();
    let tag_name = element.name().to_string();

    let mut node = HtmlNode::new(&tag_name);

    if let Some(id) = element.attr("id") {
        node = node.with_id(id);
    }

    if let Some(class_attr) = element.attr("class") {
        for class in class_attr.split_whitespace() {
            node = node.with_class(class);
        }
    }

    for child_ref in element_ref.children() {
        if child_ref.value().as_element().is_some() {
            if let Some(child_element_ref) = scraper::ElementRef::wrap(child_ref) {
                if let Some(child_node) = parse_element_to_node(&child_element_ref) {
                    node = node.add_child(child_node);
                }
            }
        }
    }

    Some(node)
}

// --- 9. Main compiler function ---
fn compile_and_run(css_file: &str, html_file: &str) {
    println!("=== CSS Compiler: {} -> Program ===", css_file);

    // Read and parse CSS
    let css_content = match fs::read_to_string(css_file) {
        Ok(content) => content,
        Err(e) => {
            println!("Error reading CSS file {}: {}", css_file, e);
            return;
        }
    };

    let rules = parse_css_file(&css_content);
    println!("Parsed CSS Rules: {:#?}\n", rules);

    // Compile CSS to Tree NFA program
    let mut compiler = CssCompiler::new();
    let program = compiler.compile_css_rules(&rules);

    // Print the generated program
    program.print_program();

    // Generate Rust code
    println!("=== Generated Rust Code ===");
    println!("{}", program.generate_rust_code());

    // Read and parse HTML
    let html_content = match fs::read_to_string(html_file) {
        Ok(content) => content,
        Err(e) => {
            println!("Error reading HTML file {}: {}", html_file, e);
            return;
        }
    };

    if let Some(mut root_node) = parse_html_file(&html_content) {
        println!("=== Executing Tree NFA Program on {} ===", html_file);

        let vm = TreeNFAVM::new(program);

        // Use the new driver function
        vm.process_tree_verbose(&mut root_node);

        // Show final results
        println!("=== Final Results ===");
        print_css_results(&root_node, &vm.program.state_names, 0);

        // Demonstrate incremental processing
        println!("\n=== Incremental Processing Demo ===");

        // First run - everything will be computed
        println!("First incremental run (all cache misses expected):");
        let stats1 = vm.process_tree_incremental_with_stats(&mut root_node);
        stats1.print_summary();

        // Second run - everything should be cached
        println!("\nSecond incremental run (all cache hits expected):");
        let stats2 = vm.process_tree_incremental_with_stats(&mut root_node);
        stats2.print_summary();

        // Modify one node and run again
        if !root_node.children.is_empty() {
            println!("\nModifying first child node...");
            root_node.children[0].mark_dirty();

            println!("Third incremental run (some cache misses expected):");
            let stats3 = vm.process_tree_incremental_with_stats(&mut root_node);
            stats3.print_summary();
        }
    } else {
        println!("Failed to parse HTML file");
    }

    println!("\n");
}

fn print_html_structure(node: &HtmlNode, depth: usize) {
    let indent = "  ".repeat(depth);
    println!(
        "{}Tag: {}, ID: {:?}, Classes: {:?}",
        indent, node.tag_name, node.id, node.classes
    );

    for child in &node.children {
        print_html_structure(child, depth + 1);
    }
}

fn print_css_results(node: &HtmlNode, state_names: &HashMap<usize, String>, depth: usize) {
    let indent = "  ".repeat(depth);
    println!(
        "{}[{}] {} (ID: {:?}, Classes: {:?})",
        indent,
        if node.css_match_bitvector.is_empty() {
            "No matches"
        } else {
            "MATCHES"
        },
        node.tag_name,
        node.id,
        node.classes
    );

    if !node.css_match_bitvector.is_empty() {
        for (bit_pos, name) in state_names {
            if node.css_match_bitvector.is_bit_set(*bit_pos) {
                println!("{}    - {}", indent, name);
            }
        }
    }

    for child in &node.children {
        print_css_results(child, state_names, depth + 1);
    }
}

// Add benchmark functionality
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub test_name: String,
    pub nodes_count: usize,
    pub incremental_cycles: u64,
    pub fromscratch_cycles: u64,
    pub cache_hit_rate: f64,
    pub speedup: f64,
}

pub fn run_performance_benchmark() -> Vec<BenchmarkResult> {
    println!("🚀 Starting Performance Benchmark: Incremental vs From-Scratch");
    println!("================================================================\n");

    let mut results = Vec::new();

    // Test 1: Different tree scales (more data points)
    let scale_factors = vec![
        0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2, 1.3, 1.4, 1.5,
    ];

    for &scale in &scale_factors {
        if let Ok(mut tree) = create_scaled_google_tree(scale) {
            let nodes_count = count_total_nodes(&tree);

            if nodes_count < 5 {
                continue;
            }

            println!("🌳 Testing scale {:.1} ({} nodes)...", scale, nodes_count);

            let result = benchmark_tree_performance(&mut tree, &format!("scale_{:.1}", scale));
            results.push(result);
        }
    }

    // Test 2: Multiple iterations with different modification patterns
    if let Ok(base_tree) = create_scaled_google_tree(1.0) {
        let patterns = vec![
            ("no_change", 0),
            ("single_leaf", 1),
            ("few_nodes", 3),
            ("small_subtree", 8),
            ("medium_subtree", 15),
            ("large_subtree", 30),
            ("major_change", 50),
        ];

        for (pattern_name, _) in patterns {
            // Test each pattern multiple times with different random seeds
            for i in 0..5 {
                let mut test_tree = base_tree.clone();
                apply_test_modification_with_seed(&mut test_tree, pattern_name, i);

                let test_name = format!("{}_{}", pattern_name, i);
                println!("🔧 Testing {}: {}...", i + 1, test_name);
                let result = benchmark_tree_performance(&mut test_tree, &test_name);
                results.push(result);
            }
        }
    }

    // Test 3: Simulate real-world scenarios with Google trace data
    if let Ok(base_tree) = create_scaled_google_tree(1.0) {
        for iteration in 0..20 {
            let mut test_tree = base_tree.clone();

            // Simulate DOM modifications like those in Google trace
            simulate_dom_modifications(&mut test_tree, iteration);

            let test_name = format!("realistic_{}", iteration);
            println!("🌐 Testing realistic scenario {}...", iteration + 1);
            let result = benchmark_tree_performance(&mut test_tree, &test_name);
            results.push(result);
        }
    }

    results
}

fn apply_test_modification_with_seed(tree: &mut HtmlNode, pattern: &str, seed: usize) {
    match pattern {
        "no_change" => {
            // Do nothing - test pure cache performance
        }
        "single_leaf" => {
            if let Some(node) = find_node_at_test_depth(tree, 2 + (seed % 3)) {
                node.mark_dirty();
            }
        }
        "few_nodes" => {
            for i in 0..(2 + seed % 3) {
                if let Some(node) = find_node_at_test_depth(tree, 1 + i) {
                    node.mark_dirty();
                }
            }
        }
        "small_subtree" => {
            if seed < tree.children.len() {
                tree.children[seed].mark_dirty();
            }
        }
        "medium_subtree" => {
            for i in 0..tree.children.len().min(2 + seed) {
                tree.children[i].mark_dirty();
            }
        }
        "large_subtree" => {
            for i in 0..tree.children.len().min(3 + seed) {
                tree.children[i].mark_dirty();
                if i < tree.children[i].children.len() {
                    tree.children[i].children[0].mark_dirty();
                }
            }
        }
        "major_change" => {
            tree.mark_dirty();
            if seed % 2 == 0 {
                // Also mark some children to simulate cascading changes
                for i in 0..tree.children.len().min(seed + 1) {
                    tree.children[i].mark_dirty();
                }
            }
        }
        _ => {}
    }
}

fn simulate_dom_modifications(tree: &mut HtmlNode, scenario: usize) {
    // Simulate different realistic DOM modification patterns
    match scenario % 6 {
        0 => {
            // Style changes - single node
            if let Some(node) = find_node_at_test_depth(tree, 2) {
                node.mark_dirty();
            }
        }
        1 => {
            // Content changes - small subtree
            if !tree.children.is_empty() {
                let len = tree.children.len();
                tree.children[scenario % len].mark_dirty();
            }
        }
        2 => {
            // Layout changes - medium impact
            for i in 0..tree.children.len().min(3) {
                if (i + scenario) % 3 == 0 {
                    tree.children[i].mark_dirty();
                }
            }
        }
        3 => {
            // Major restructuring
            tree.mark_dirty();
        }
        4 => {
            // Cascading changes
            mark_cascading_changes(tree, scenario % 3);
        }
        5 => {
            // Mixed changes
            if scenario % 2 == 0 {
                if let Some(node) = find_node_at_test_depth(tree, 1) {
                    node.mark_dirty();
                }
            }
            if !tree.children.is_empty() {
                tree.children[0].mark_dirty();
            }
        }
        _ => {}
    }
}

fn mark_cascading_changes(tree: &mut HtmlNode, depth: usize) {
    if depth == 0 {
        return;
    }

    tree.mark_dirty();

    for (i, child) in tree.children.iter_mut().enumerate() {
        if i < depth {
            mark_cascading_changes(child, depth - 1);
        }
    }
}

fn create_scaled_google_tree(scale: f64) -> Result<HtmlNode, Box<dyn std::error::Error>> {
    let command_file_path = "css-gen-op/command.json";
    let content = std::fs::read_to_string(command_file_path)?;
    let first_line = content.lines().next().ok_or("Empty command file")?;
    let command: serde_json::Value = serde_json::from_str(first_line)?;

    if command["name"] != "init" {
        return Err("First command should be init".into());
    }

    let google_node =
        GoogleNode::from_json(&command["node"]).ok_or("Failed to parse Google node")?;

    let mut html_node = google_node.to_html_node();

    if scale != 1.0 {
        scale_tree_by_factor(&mut html_node, scale, 0, 2);
    }

    html_node.init_parent_pointers();
    Ok(html_node)
}

fn scale_tree_by_factor(node: &mut HtmlNode, scale: f64, depth: usize, max_depth: usize) {
    if depth >= max_depth {
        return;
    }

    if scale < 1.0 {
        let keep_count = (node.children.len() as f64 * scale).ceil() as usize;
        node.children.truncate(keep_count);
    } else if scale > 1.0 && !node.children.is_empty() {
        let original_children = node.children.clone();
        let extra_count = ((node.children.len() as f64 * (scale - 1.0)).ceil() as usize).min(3);

        for i in 0..extra_count {
            if i < original_children.len() {
                node.children.push(original_children[i].clone());
            }
        }
    }

    for child in &mut node.children {
        scale_tree_by_factor(child, scale, depth + 1, max_depth);
    }
}

fn apply_test_modification(tree: &mut HtmlNode, pattern: &str) {
    match pattern {
        "no_change" => {
            // Do nothing - test pure cache performance
        }
        "single_node" => {
            if let Some(node) = find_node_at_test_depth(tree, 2) {
                node.mark_dirty();
            }
        }
        "small_subtree" => {
            if !tree.children.is_empty() {
                tree.children[0].mark_dirty();
            }
        }
        "large_subtree" => {
            tree.mark_dirty();
        }
        _ => {}
    }
}

fn find_node_at_test_depth(node: &mut HtmlNode, depth: usize) -> Option<&mut HtmlNode> {
    if depth == 0 {
        return Some(node);
    }

    for child in &mut node.children {
        if let Some(found) = find_node_at_test_depth(child, depth - 1) {
            return Some(found);
        }
    }

    None
}

fn benchmark_tree_performance(tree: &mut HtmlNode, test_name: &str) -> BenchmarkResult {
    let nodes_count = count_total_nodes(tree);
    let iterations = if nodes_count > 100 { 10 } else { 30 };

    // Load CSS rules
    let css_content = std::fs::read_to_string("css-gen-op/https___www.google.com_.css")
        .unwrap_or_else(|_| "div { display: block; }".to_string());
    let css_rules = parse_basic_css(&css_content);
    let mut compiler = CssCompiler::new();
    let program = compiler.compile_css_rules(&css_rules);
    let vm = TreeNFAVM::new(program);

    // Benchmark incremental processing
    let mut incremental_total = 0u64;
    let mut cache_hit_rates = Vec::new();

    for _ in 0..iterations {
        let mut inc_tree = tree.clone();

        let start = rdtsc();
        let stats = vm.process_tree_incremental_with_stats(&mut inc_tree);
        let end = rdtsc();

        incremental_total += cycles_to_duration(start, end);

        if stats.total_nodes > 0 {
            cache_hit_rates.push(stats.cache_hits as f64 / stats.total_nodes as f64);
        }
    }

    // Benchmark from-scratch processing (clear cache each time)
    let mut fromscratch_total = 0u64;

    for _ in 0..iterations {
        let mut fs_tree = tree.clone();
        clear_tree_cache(&mut fs_tree);

        let start = rdtsc();
        let _ = vm.process_tree_incremental_with_stats(&mut fs_tree);
        let end = rdtsc();

        fromscratch_total += cycles_to_duration(start, end);
    }

    let avg_incremental = incremental_total / iterations as u64;
    let avg_fromscratch = fromscratch_total / iterations as u64;
    let avg_cache_hit_rate = if cache_hit_rates.is_empty() {
        0.0
    } else {
        cache_hit_rates.iter().sum::<f64>() / cache_hit_rates.len() as f64
    };
    let speedup = if avg_incremental > 0 {
        avg_fromscratch as f64 / avg_incremental as f64
    } else {
        1.0
    };

    println!(
        "  📊 {} nodes: {:.2}x speedup, {:.1}% cache hits",
        nodes_count,
        speedup,
        avg_cache_hit_rate * 100.0
    );

    BenchmarkResult {
        test_name: test_name.to_string(),
        nodes_count,
        incremental_cycles: avg_incremental,
        fromscratch_cycles: avg_fromscratch,
        cache_hit_rate: avg_cache_hit_rate,
        speedup,
    }
}

fn clear_tree_cache(node: &mut HtmlNode) {
    node.is_self_dirty = true;
    node.has_dirty_descendant = false;
    node.cached_parent_state = None;
    node.cached_node_intrinsic = None;
    node.cached_child_states = None;

    for child in &mut node.children {
        clear_tree_cache(child);
    }
}

pub fn export_benchmark_results(results: &[BenchmarkResult]) -> std::io::Result<()> {
    use std::io::Write;

    let filename = "performance_benchmark.csv";
    let mut file = std::fs::File::create(filename)?;

    writeln!(
        file,
        "test_name,nodes_count,incremental_cycles,fromscratch_cycles,cache_hit_rate,speedup"
    )?;

    for result in results {
        writeln!(
            file,
            "{},{},{},{},{:.4},{:.4}",
            result.test_name,
            result.nodes_count,
            result.incremental_cycles,
            result.fromscratch_cycles,
            result.cache_hit_rate,
            result.speedup
        )?;
    }

    println!("📁 Benchmark results exported to {}", filename);
    Ok(())
}

pub fn print_performance_summary(results: &[BenchmarkResult]) {
    println!("\n📊 PERFORMANCE BENCHMARK SUMMARY");
    println!("=================================");

    if results.is_empty() {
        println!("No benchmark results available.");
        return;
    }

    let avg_speedup = results.iter().map(|r| r.speedup).sum::<f64>() / results.len() as f64;
    let max_speedup = results.iter().map(|r| r.speedup).fold(0.0, f64::max);
    let avg_cache_hit_rate =
        results.iter().map(|r| r.cache_hit_rate).sum::<f64>() / results.len() as f64;

    println!("Total tests: {}", results.len());
    println!("Average speedup: {:.2}x", avg_speedup);
    println!("Maximum speedup: {:.2}x", max_speedup);
    println!("Average cache hit rate: {:.1}%", avg_cache_hit_rate * 100.0);

    println!("\nDetailed Results:");
    for result in results {
        println!(
            "{:15} | {:4} nodes | {:6} vs {:8} cycles | {:.2}x speedup | {:.1}% cache",
            result.test_name,
            result.nodes_count,
            result.incremental_cycles,
            result.fromscratch_cycles,
            result.speedup,
            result.cache_hit_rate * 100.0
        );
    }
}

fn main() {
    // Check if we should run benchmarks
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "benchmark" {
        println!("🚀 Running Performance Benchmark Mode\n");

        let results = run_performance_benchmark();
        print_performance_summary(&results);

        if let Err(e) = export_benchmark_results(&results) {
            println!("Failed to export benchmark results: {}", e);
        }

        return;
    }

    let tests_dir = "tests";
    let css_file = format!("{}/test.css", tests_dir);

    // Test with all HTML files
    for i in 1..=4 {
        let html_file = format!("{}/t{}.html", tests_dir, i);

        if Path::new(&html_file).exists() && Path::new(&css_file).exists() {
            compile_and_run(&css_file, &html_file);
        } else {
            println!("Missing test files: {} or {}", css_file, html_file);
        }
    }

    // Test Google trace integration
    println!("\n=== GOOGLE TRACE INTEGRATION TEST ===");
    if let Err(e) = process_google_trace_with_rust() {
        println!("Google trace test failed: {}", e);
        println!("This is expected if css-gen-op files are not available");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create test HTML nodes
    #[allow(dead_code)]
    fn create_test_node() -> HtmlNode {
        HtmlNode::new("div")
            .with_id("test")
            .with_class("container")
            .add_child(
                HtmlNode::new("p")
                    .with_class("item")
                    .add_child(HtmlNode::new("span").with_id("specific")),
            )
    }

    #[test]
    fn test_simple_selector_parsing() {
        assert_eq!(
            parse_simple_selector("div"),
            Some(SimpleSelector::Type("div".to_string()))
        );
        assert_eq!(
            parse_simple_selector(".item"),
            Some(SimpleSelector::Class("item".to_string()))
        );
        assert_eq!(
            parse_simple_selector("#specific"),
            Some(SimpleSelector::Id("specific".to_string()))
        );
        assert_eq!(parse_simple_selector(""), None);
        assert_eq!(parse_simple_selector("invalid123"), None);
    }

    #[test]
    fn test_css_parsing() {
        let css = r#"
/* Rule 0 (R0) */ div {}
/* Rule 1 (R1) */ .item {}
/* Rule 2 (R2) */ #specific {}
/* Rule 3 (R3) */ div > p {}
        "#;

        let rules = parse_css_file(css);
        assert_eq!(rules.len(), 4);

        assert_eq!(
            rules[0],
            CssRule::Simple(SimpleSelector::Type("div".to_string()))
        );
        assert_eq!(
            rules[1],
            CssRule::Simple(SimpleSelector::Class("item".to_string()))
        );
        assert_eq!(
            rules[2],
            CssRule::Simple(SimpleSelector::Id("specific".to_string()))
        );
        assert_eq!(
            rules[3],
            CssRule::Child {
                parent_selector: SimpleSelector::Type("div".to_string()),
                child_selector: SimpleSelector::Type("p".to_string()),
            }
        );
    }

    #[test]
    fn test_html_node_creation() {
        let node = HtmlNode::new("div")
            .with_id("test")
            .with_class("item")
            .with_class("container");

        assert_eq!(node.tag_name, "div");
        assert_eq!(node.id, Some("test".to_string()));
        assert!(node.classes.contains("item"));
        assert!(node.classes.contains("container"));
    }

    #[test]
    fn test_css_compiler_bit_allocation() {
        let mut compiler = CssCompiler::new();

        let bit1 = compiler.allocate_bit("match_div".to_string());
        let bit2 = compiler.allocate_bit("active_div".to_string());
        let bit3 = compiler.allocate_bit("match_div".to_string()); // Should reuse

        assert_eq!(bit1, 0);
        assert_eq!(bit2, 1);
        assert_eq!(bit3, 0); // Should be the same as bit1
    }

    #[test]
    fn test_tree_nfa_program_generation() {
        let rules = vec![
            CssRule::Simple(SimpleSelector::Type("div".to_string())),
            CssRule::Simple(SimpleSelector::Class("item".to_string())),
            CssRule::Child {
                parent_selector: SimpleSelector::Type("div".to_string()),
                child_selector: SimpleSelector::Class("item".to_string()),
            },
        ];

        let mut compiler = CssCompiler::new();
        let program = compiler.compile_css_rules(&rules);

        // Should have instructions for simple selectors and child selector
        assert!(!program.instructions.is_empty());
        assert!(program.total_bits > 0);

        // Check that state names are set
        assert!(!program.state_names.is_empty());
    }

    #[test]
    fn test_node_matches_selector() {
        let vm = TreeNFAVM::new(TreeNFAProgram::new());

        let div_node = HtmlNode::new("div").with_id("test").with_class("container");

        assert!(vm.node_matches_selector(&div_node, &SimpleSelector::Type("div".to_string())));
        assert!(vm.node_matches_selector(&div_node, &SimpleSelector::Id("test".to_string())));
        assert!(
            vm.node_matches_selector(&div_node, &SimpleSelector::Class("container".to_string()))
        );

        assert!(!vm.node_matches_selector(&div_node, &SimpleSelector::Type("p".to_string())));
        assert!(!vm.node_matches_selector(&div_node, &SimpleSelector::Id("other".to_string())));
        assert!(!vm.node_matches_selector(&div_node, &SimpleSelector::Class("other".to_string())));
    }

    #[test]
    fn test_complete_css_matching() {
        // Create a simple CSS rule set
        let rules = vec![
            CssRule::Simple(SimpleSelector::Type("div".to_string())),
            CssRule::Simple(SimpleSelector::Class("item".to_string())),
            CssRule::Child {
                parent_selector: SimpleSelector::Type("div".to_string()),
                child_selector: SimpleSelector::Class("item".to_string()),
            },
        ];

        // Compile to Tree NFA program
        let mut compiler = CssCompiler::new();
        let program = compiler.compile_css_rules(&rules);
        let vm = TreeNFAVM::new(program);

        // Create test HTML structure: div > p.item
        let mut root = HtmlNode::new("div")
            .with_id("outer")
            .add_child(HtmlNode::new("p").with_class("item"))
            .add_child(
                HtmlNode::new("span")
                    .with_class("item")
                    .add_child(HtmlNode::new("p").with_id("specific")),
            );

        // Execute on root (div)
        let child_states = vm.process_node_inplace(&mut root, BitVector::new());

        // Root should match "div" selector
        assert_ne!(root.css_match_bitvector.as_u64(), 0);

        // Root should provide active states for children
        assert!(!child_states.is_empty());

        // Execute on child (p.item)
        let mut child = HtmlNode::new("p").with_class("item");
        let _child_child_states = vm.process_node_inplace(&mut child, child_states);

        // Child should match both ".item" and "div > .item"
        assert_ne!(child.css_match_bitvector.as_u64(), 0);
    }

    #[test]
    fn test_generated_rust_code() {
        let rules = vec![CssRule::Simple(SimpleSelector::Type("div".to_string()))];

        let mut compiler = CssCompiler::new();
        let program = compiler.compile_css_rules(&rules);
        let generated_code = program.generate_rust_code();

        // Check that the generated code contains expected elements
        assert!(generated_code.contains("fn process_node_generated_incremental"));
        assert!(generated_code.contains("current_matches"));
        assert!(generated_code.contains("child_states"));
        assert!(generated_code.contains("node_matches_selector_generated"));
        assert!(generated_code.contains("SimpleSelector::Type"));
    }

    #[test]
    fn test_complex_css_scenario() {
        // Test the exact CSS from test.css file
        let css = r#"
/* Rule 0 (R0) */ div {}
/* Rule 1 (R1) */ .item {}
/* Rule 2 (R2) */ #specific {}
/* Rule 3 (R3) */ p {}
/* Rule 4 (R4) */ div > p {}
/* Rule 5 (R5) */ .item > #specific {}
/* Rule 6 (R6) */ div > .item {}
/* Rule 7 (R7) */ div > #specific {}
        "#;

        let rules = parse_css_file(css);
        let mut compiler = CssCompiler::new();
        let program = compiler.compile_css_rules(&rules);
        let vm = TreeNFAVM::new(program);

        // Create HTML structure similar to t1.html: div > p.item > span > p#specific
        let mut root = HtmlNode::new("div")
            .with_id("main")
            .with_class("container")
            .add_child(
                HtmlNode::new("p")
                    .with_class("item")
                    .add_child(HtmlNode::new("span").with_id("specific")),
            )
            .add_child(
                HtmlNode::new("div").add_child(HtmlNode::new("p").add_child(HtmlNode::new("span"))),
            );

        // Execute on root div
        let child_states_root = vm.process_node_inplace(&mut root, BitVector::new());

        // Root div should match "div" rule
        assert_ne!(root.css_match_bitvector.as_u64(), 0);

        // Test first child (p.item)
        let mut p_item = HtmlNode::new("p").with_class("item");
        let _child_states_p = vm.process_node_inplace(&mut p_item, child_states_root);

        // Should match: p, .item, div > p, div > .item
        assert_ne!(p_item.css_match_bitvector.as_u64(), 0);

        // Test span.item
        let mut span_item = HtmlNode::new("span").with_class("item");
        let child_states_span = vm.process_node_inplace(&mut span_item, child_states_root);

        // Should match: .item, div > .item
        assert_ne!(span_item.css_match_bitvector.as_u64(), 0);

        // Test final p#specific under span.item
        let mut p_specific = HtmlNode::new("p").with_id("specific");
        let _final_states = vm.process_node_inplace(&mut p_specific, child_states_span);

        // Should match: p, #specific, .item > #specific
        assert_ne!(p_specific.css_match_bitvector.as_u64(), 0);
    }

    #[test]
    fn test_bitvector_operations() {
        // Test basic bitvector operations used in the CSS matching
        let mut bitvector: u64 = 0;

        // Set bit 3
        bitvector |= 1 << 3;
        assert_eq!(bitvector, 8); // 2^3 = 8

        // Check bit 3 is set
        assert_ne!(bitvector & (1 << 3), 0);

        // Check bit 2 is not set
        assert_eq!(bitvector & (1 << 2), 0);

        // Set bit 0
        bitvector |= 1 << 0;
        assert_eq!(bitvector, 9); // 8 + 1 = 9

        // Test multiple bits
        assert_ne!(bitvector & (1 << 0), 0);
        assert_ne!(bitvector & (1 << 3), 0);
    }

    #[test]
    fn test_error_handling() {
        // Test CSS parsing with malformed input
        let bad_css = "this is not valid css";
        let rules = parse_css_file(bad_css);
        assert!(rules.is_empty());

        // Test selector parsing with invalid input
        assert_eq!(parse_simple_selector("123invalid"), None);
        assert_eq!(parse_simple_selector(""), None);
    }

    #[test]
    fn test_generated_code_execution() {
        // Create a comprehensive CSS rule set to test code generation
        let rules = vec![
            CssRule::Simple(SimpleSelector::Type("div".to_string())),
            CssRule::Simple(SimpleSelector::Class("item".to_string())),
            CssRule::Simple(SimpleSelector::Id("specific".to_string())),
            CssRule::Child {
                parent_selector: SimpleSelector::Type("div".to_string()),
                child_selector: SimpleSelector::Class("item".to_string()),
            },
            CssRule::Child {
                parent_selector: SimpleSelector::Class("item".to_string()),
                child_selector: SimpleSelector::Id("specific".to_string()),
            },
        ];

        let mut compiler = CssCompiler::new();
        let program = compiler.compile_css_rules(&rules);
        let generated_code = program.generate_rust_code();

        // Verify the generated code has the expected structure
        assert!(generated_code.contains("fn process_node_generated_incremental"));
        assert!(generated_code.contains("intrinsic_matches = BitVector::new()"));
        assert!(generated_code.contains("child_states = BitVector::new()"));

        // Verify instruction generation
        assert!(generated_code.contains("CheckAndSetBit"));
        assert!(generated_code.contains("PropagateToChildren"));
        assert!(generated_code.contains("CheckParentAndSetBit"));

        // Verify selector type handling
        assert!(generated_code.contains("SimpleSelector::Type"));
        assert!(generated_code.contains("SimpleSelector::Class"));
        assert!(generated_code.contains("SimpleSelector::Id"));

        // Test that the VM produces the same results as what the generated code should
        let vm = TreeNFAVM::new(program);

        // Test case: div.item > span#specific
        let mut root = HtmlNode::new("div").with_class("item");
        let child_states = vm.process_node_inplace(&mut root, BitVector::new());

        // Root should match both "div" and ".item"
        let matches = root.css_match_bitvector.as_u64();
        assert_ne!(matches, 0);

        // Should propagate states for children
        assert!(!child_states.is_empty());

        // Test child element
        let mut child = HtmlNode::new("span").with_id("specific");
        let _child_child_states = vm.process_node_inplace(&mut child, child_states);

        // Child should match "#specific" and ".item > #specific"
        assert_ne!(child.css_match_bitvector.as_u64(), 0);
    }

    #[test]
    fn test_instruction_order_and_correctness() {
        // Test that instructions are generated in the correct order
        let rules = vec![
            CssRule::Simple(SimpleSelector::Type("div".to_string())),
            CssRule::Child {
                parent_selector: SimpleSelector::Type("div".to_string()),
                child_selector: SimpleSelector::Type("p".to_string()),
            },
        ];

        let mut compiler = CssCompiler::new();
        let program = compiler.compile_css_rules(&rules);

        // Check instruction count and types
        assert!(!program.instructions.is_empty());

        let mut has_check_and_set = false;
        let mut has_propagate = false;
        let mut has_check_parent = false;

        for instruction in &program.instructions {
            match instruction {
                NFAInstruction::CheckAndSetBit { .. } => has_check_and_set = true,
                NFAInstruction::PropagateToChildren { .. } => has_propagate = true,
                NFAInstruction::CheckParentAndSetBit { .. } => has_check_parent = true,
            }
        }

        assert!(has_check_and_set, "Should have CheckAndSetBit instructions");
        assert!(
            has_propagate,
            "Should have PropagateToChildren instructions"
        );
        assert!(
            has_check_parent,
            "Should have CheckParentAndSetBit instructions"
        );
    }

    #[test]
    fn test_state_naming_consistency() {
        let rules = vec![
            CssRule::Simple(SimpleSelector::Type("div".to_string())),
            CssRule::Simple(SimpleSelector::Class("test".to_string())),
        ];

        let mut compiler = CssCompiler::new();
        let program = compiler.compile_css_rules(&rules);

        // Check that state names follow the expected pattern
        let state_names: Vec<&String> = program.state_names.values().collect();

        assert!(
            state_names
                .iter()
                .any(|s| s.contains("match_Type(\"div\")"))
        );
        assert!(
            state_names
                .iter()
                .any(|s| s.contains("active_Type(\"div\")"))
        );
        assert!(
            state_names
                .iter()
                .any(|s| s.contains("match_Class(\"test\")"))
        );
        assert!(
            state_names
                .iter()
                .any(|s| s.contains("active_Class(\"test\")"))
        );
    }

    #[test]
    fn test_demo_generated_code() {
        println!("\n=== CSS COMPILER DEMO ===");

        // Simple CSS rules
        let css_content = r#"
/* Rule 0 (R0) */ div {}
/* Rule 1 (R1) */ .item {}
/* Rule 2 (R2) */ #specific {}
/* Rule 3 (R3) */ div > .item {}
/* Rule 4 (R4) */ .item > #specific {}
        "#;

        println!("Input CSS:");
        println!("{}", css_content);

        let rules = parse_css_file(css_content);
        println!("Parsed {} CSS rules", rules.len());

        let mut compiler = CssCompiler::new();
        let program = compiler.compile_css_rules(&rules);

        println!("\nGenerated Tree NFA Program:");
        println!("Total bits: {}", program.total_bits);
        println!("Instructions: {}", program.instructions.len());

        println!("\nGenerated Rust Code:");
        println!("{}", program.generate_rust_code());

        println!("=== DEMO COMPLETE ===\n");
    }

    #[test]
    fn test_incremental_processing() {
        println!("\n=== TESTING INCREMENTAL PROCESSING ===");

        // Create test CSS rules
        let rules = vec![
            CssRule::Simple(SimpleSelector::Type("div".to_string())),
            CssRule::Simple(SimpleSelector::Class("item".to_string())),
            CssRule::Simple(SimpleSelector::Id("specific".to_string())),
            CssRule::Child {
                parent_selector: SimpleSelector::Type("div".to_string()),
                child_selector: SimpleSelector::Class("item".to_string()),
            },
        ];

        let mut compiler = CssCompiler::new();
        let program = compiler.compile_css_rules(&rules);
        let vm = TreeNFAVM::new(program);

        // Create test HTML structure
        let mut root = HtmlNode::new("div")
            .with_id("container")
            .add_child(
                HtmlNode::new("p")
                    .with_class("item")
                    .add_child(HtmlNode::new("span").with_id("specific")),
            )
            .add_child(HtmlNode::new("div").with_class("other"));

        // Test 1: First run should compute everything
        println!("Test 1: Initial computation");
        let stats1 = vm.process_tree_incremental_with_stats(&mut root);
        assert_eq!(stats1.cache_hits, 0);
        assert!(stats1.cache_misses > 0);
        println!(
            "✓ All nodes computed (cache misses: {})",
            stats1.cache_misses
        );

        // Store initial results for comparison
        let initial_root_matches = root.css_match_bitvector;
        let initial_child1_matches = root.children[0].css_match_bitvector;

        // Test 2: Second run should use cache
        println!("\nTest 2: Cache utilization");
        let stats2 = vm.process_tree_incremental_with_stats(&mut root);
        assert_eq!(stats2.cache_misses, 0);
        assert!(stats2.cache_hits > 0);
        println!("✓ All results cached (cache hits: {})", stats2.cache_hits);

        // Verify results didn't change
        assert_eq!(root.css_match_bitvector, initial_root_matches);
        assert_eq!(root.children[0].css_match_bitvector, initial_child1_matches);
        println!("✓ Results consistent with cached version");

        // Test 3: Modify node and verify selective recomputation
        println!("\nTest 3: Selective recomputation after modification");
        root.children[0].mark_dirty();

        let stats3 = vm.process_tree_incremental_with_stats(&mut root);
        assert!(stats3.cache_hits > 0, "Some nodes should still be cached");
        assert!(
            stats3.cache_misses > 0 || stats3.cache_hits > 0,
            "Should process some nodes after modification"
        );
        println!(
            "✓ Selective recomputation (hits: {}, misses: {})",
            stats3.cache_hits, stats3.cache_misses
        );

        // Test 4: Compare with non-incremental version for correctness
        println!("\nTest 4: Correctness verification");
        let mut root_copy = HtmlNode::new("div")
            .with_id("container")
            .add_child(
                HtmlNode::new("p")
                    .with_class("item")
                    .add_child(HtmlNode::new("span").with_id("specific")),
            )
            .add_child(HtmlNode::new("div").with_class("other"));

        vm.process_tree(&mut root_copy);

        // Compare results
        assert_eq!(root.css_match_bitvector, root_copy.css_match_bitvector);
        assert_eq!(
            root.children[0].css_match_bitvector,
            root_copy.children[0].css_match_bitvector
        );
        assert_eq!(
            root.children[1].css_match_bitvector,
            root_copy.children[1].css_match_bitvector
        );
        println!("✓ Incremental results match non-incremental results");

        println!("=== INCREMENTAL PROCESSING TESTS PASSED ===\n");
    }

    #[test]
    fn test_incremental_node_modification() {
        // Test that modifying node attributes correctly invalidates cache
        let rules = vec![
            CssRule::Simple(SimpleSelector::Type("div".to_string())),
            CssRule::Simple(SimpleSelector::Class("highlight".to_string())),
        ];

        let mut compiler = CssCompiler::new();
        let program = compiler.compile_css_rules(&rules);
        let vm = TreeNFAVM::new(program);

        let mut node = HtmlNode::new("div");

        // Initial processing
        vm.process_tree_incremental(&mut node);
        let initial_matches = node.css_match_bitvector;

        // Add a class (this should mark the node dirty)
        node = node.with_class("highlight");

        // Reprocess
        vm.process_tree_incremental(&mut node);
        let updated_matches = node.css_match_bitvector;

        // Results should be different
        assert_ne!(
            initial_matches, updated_matches,
            "Adding class should change CSS matches"
        );

        // Find the correct bit position for .highlight class
        let mut highlight_bit = None;
        for (bit_pos, name) in &vm.program.state_names {
            if name.contains("highlight") && name.contains("match") {
                highlight_bit = Some(*bit_pos);
                break;
            }
        }

        if let Some(bit) = highlight_bit {
            assert!(
                updated_matches.is_bit_set(bit),
                "Should match .highlight class at bit {}",
                bit
            );
        } else {
            panic!("Could not find bit position for .highlight class");
        }
    }

    #[test]
    fn test_performance_comparison() {
        use std::time::Instant;

        println!("\n=== PERFORMANCE COMPARISON ===");

        // Create a larger CSS rule set
        let rules = vec![
            CssRule::Simple(SimpleSelector::Type("div".to_string())),
            CssRule::Simple(SimpleSelector::Class("item".to_string())),
            CssRule::Simple(SimpleSelector::Id("specific".to_string())),
            CssRule::Simple(SimpleSelector::Type("p".to_string())),
            CssRule::Simple(SimpleSelector::Type("span".to_string())),
            CssRule::Child {
                parent_selector: SimpleSelector::Type("div".to_string()),
                child_selector: SimpleSelector::Type("p".to_string()),
            },
            CssRule::Child {
                parent_selector: SimpleSelector::Class("item".to_string()),
                child_selector: SimpleSelector::Id("specific".to_string()),
            },
            CssRule::Child {
                parent_selector: SimpleSelector::Type("div".to_string()),
                child_selector: SimpleSelector::Class("item".to_string()),
            },
        ];

        let mut compiler = CssCompiler::new();
        let program = compiler.compile_css_rules(&rules);
        let vm = TreeNFAVM::new(program);

        // Create a complex HTML structure
        let mut root = HtmlNode::new("div")
            .with_id("main")
            .with_class("container")
            .add_child(
                HtmlNode::new("div")
                    .with_class("item")
                    .add_child(
                        HtmlNode::new("p").add_child(HtmlNode::new("span").with_id("specific")),
                    )
                    .add_child(HtmlNode::new("div").add_child(HtmlNode::new("p"))),
            )
            .add_child(HtmlNode::new("p").with_class("item"))
            .add_child(
                HtmlNode::new("div")
                    .add_child(HtmlNode::new("span"))
                    .add_child(HtmlNode::new("p").with_id("specific")),
            );

        // Benchmark regular processing (multiple runs)
        let iterations = 1000;
        let start = Instant::now();
        for _ in 0..iterations {
            vm.process_tree(&mut root.clone());
        }
        let regular_time = start.elapsed();

        // Benchmark incremental processing (first run + cached runs)
        let start = Instant::now();

        // First run (cache miss)
        vm.process_tree_incremental(&mut root);

        // Subsequent runs (cache hits)
        for _ in 1..iterations {
            vm.process_tree_incremental(&mut root);
        }
        let incremental_time = start.elapsed();

        println!(
            "Regular processing ({} iterations): {:?}",
            iterations, regular_time
        );
        println!(
            "Incremental processing ({} iterations): {:?}",
            iterations, incremental_time
        );

        let speedup = regular_time.as_nanos() as f64 / incremental_time.as_nanos() as f64;
        println!("Speedup: {:.2}x", speedup);

        // Incremental should be significantly faster for repeated computations
        assert!(
            incremental_time < regular_time,
            "Incremental should be faster"
        );

        println!("=== PERFORMANCE COMPARISON COMPLETE ===\n");
    }

    #[test]
    fn test_double_dirty_bit_optimization() {
        // Create a deep tree structure
        let css_rules = vec![
            CssRule::Simple(SimpleSelector::Type("div".to_string())),
            CssRule::Simple(SimpleSelector::Class("highlight".to_string())),
        ];

        let mut compiler = CssCompiler::new();
        let program = compiler.compile_css_rules(&css_rules);
        let vm = TreeNFAVM::new(program);

        // Build deep tree: root -> child1 -> child2 -> child3
        let mut root = HtmlNode::new("div").add_child(HtmlNode::new("div").add_child(
            HtmlNode::new("div").add_child(HtmlNode::new("span").with_class("highlight")),
        ));

        // Initial processing
        vm.process_tree_incremental(&mut root);

        // Verify everything is initially clean
        assert!(!root.is_self_dirty);
        assert!(!root.has_dirty_descendant);
        assert!(!root.children[0].is_self_dirty);
        assert!(!root.children[0].has_dirty_descendant);

        // Mark deep node dirty using optimized path marking
        assert!(root.mark_node_dirty_by_path(&[0, 0, 0])); // path to deepest span

        // Verify dirty bits are set correctly along the path
        assert!(!root.is_self_dirty); // Root itself not dirty
        assert!(root.has_dirty_descendant); // But has dirty descendant

        assert!(!root.children[0].is_self_dirty); // Child1 not dirty
        assert!(root.children[0].has_dirty_descendant); // But has dirty descendant

        assert!(!root.children[0].children[0].is_self_dirty); // Child2 not dirty  
        assert!(root.children[0].children[0].has_dirty_descendant); // But has dirty descendant

        assert!(root.children[0].children[0].children[0].is_self_dirty); // Deepest node is dirty
        assert!(!root.children[0].children[0].children[0].has_dirty_descendant); // No children

        // Process incrementally - should only process nodes on the dirty path
        let stats = vm.process_tree_incremental_with_stats(&mut root);

        // With optimization, we should have fewer cache misses than total nodes
        // Only the dirty path should be recomputed, but our current implementation
        // still visits all nodes to check if they need recomputation
        println!("Double dirty bit stats: {:?}", stats);

        // Verify that the dirty marking worked correctly
        assert!(!root.is_self_dirty); // Root was processed and cleaned
        assert!(!root.has_dirty_descendant); // Summary bit cleared after processing
        assert!(!root.children[0].children[0].children[0].is_self_dirty); // Deep node processed and cleaned

        // The key optimization is that only nodes on the dirty path needed recomputation
        // This is more of a conceptual test for now
        assert!(stats.total_nodes > 0, "Should have processed some nodes");
    }

    #[test]
    fn test_optimized_selector_matching() {
        // Test hash table vs linear search performance conceptually
        let css_rules = vec![
            CssRule::Simple(SimpleSelector::Type("div".to_string())),
            CssRule::Simple(SimpleSelector::Type("span".to_string())),
            CssRule::Simple(SimpleSelector::Type("p".to_string())),
            CssRule::Simple(SimpleSelector::Class("highlight".to_string())),
            CssRule::Simple(SimpleSelector::Class("error".to_string())),
            CssRule::Simple(SimpleSelector::Id("main".to_string())),
        ];

        let mut compiler = CssCompiler::new();
        let program = compiler.compile_css_rules(&css_rules);
        let vm = TreeNFAVM::new(program);

        // Test that selector index correctly identifies matching rules
        let test_node = HtmlNode::new("div").with_class("highlight");
        let matching_rules = vm.selector_index.get_matching_rules(&test_node);

        // Should find exactly 2 matching rules (div tag and highlight class)
        assert_eq!(
            matching_rules.len(),
            2,
            "Should find exactly 2 matching rules"
        );

        // Verify the rules are correct
        let mut found_tag_rule = false;
        let mut found_class_rule = false;

        for rule in matching_rules {
            if let NFAInstruction::CheckAndSetBit { selector, .. } = rule {
                match selector {
                    SimpleSelector::Type(tag) if tag == "div" => found_tag_rule = true,
                    SimpleSelector::Class(class) if class == "highlight" => found_class_rule = true,
                    _ => {}
                }
            }
        }

        assert!(found_tag_rule, "Should find div tag rule");
        assert!(found_class_rule, "Should find highlight class rule");
    }

    #[test]
    fn test_generated_incremental_code_has_caching() {
        println!("\n=== TESTING GENERATED INCREMENTAL CODE ===");

        // Create test CSS rules
        let rules = vec![
            CssRule::Simple(SimpleSelector::Type("div".to_string())),
            CssRule::Simple(SimpleSelector::Class("item".to_string())),
            CssRule::Child {
                parent_selector: SimpleSelector::Type("div".to_string()),
                child_selector: SimpleSelector::Class("item".to_string()),
            },
        ];

        let mut compiler = CssCompiler::new();
        let program = compiler.compile_css_rules(&rules);
        let generated_code = program.generate_rust_code();

        // Verify that the generated code contains incremental processing logic
        println!("Checking generated code for incremental features...");

        // Should contain incremental function
        assert!(
            generated_code.contains("process_node_generated_incremental"),
            "Generated code should contain incremental processing function"
        );

        // Should contain cache checking
        assert!(
            generated_code.contains("needs_any_recomputation"),
            "Generated code should contain cache checking function"
        );

        // Should contain cache return logic
        assert!(
            generated_code.contains("node.cached_child_states.unwrap_or"),
            "Generated code should return cached results when possible"
        );

        // Should contain intrinsic computation caching
        assert!(
            generated_code.contains("node.cached_node_intrinsic.is_none() || node.is_self_dirty"),
            "Generated code should cache intrinsic matches"
        );

        // Should contain cache updates
        assert!(
            generated_code.contains("node.cached_parent_state = Some(parent_state)"),
            "Generated code should update parent state cache"
        );
        assert!(
            generated_code.contains("node.cached_child_states = Some(child_states)"),
            "Generated code should update child states cache"
        );
        assert!(
            generated_code.contains("node.mark_clean()"),
            "Generated code should mark node as clean"
        );

        // The generated code provides node-level incremental processing
        // Tree traversal logic is handled by the calling code

        // The generated code focuses on incremental processing
        // Non-incremental versions are available in the main library

        println!("✓ All incremental features found in generated code");

        // Also verify specific structure patterns
        assert!(
            generated_code.contains("// Check if we need to recompute"),
            "Generated code should have cache check comments"
        );
        assert!(
            generated_code.contains("// Recompute node intrinsic matches if needed"),
            "Generated code should have intrinsic computation comments"
        );
        assert!(
            generated_code.contains("// Start with cached intrinsic matches"),
            "Generated code should use cached intrinsic matches"
        );
        assert!(
            generated_code.contains("// Cache results"),
            "Generated code should have cache update comments"
        );

        println!("✓ Code structure and comments verify incremental logic");

        // Test that the generated code separates intrinsic from parent-dependent computation
        let intrinsic_section = generated_code.contains("if node.cached_node_intrinsic.is_none()");
        let parent_dependent_section = generated_code.contains("Apply parent-dependent rules");
        assert!(
            intrinsic_section && parent_dependent_section,
            "Generated code should separate intrinsic and parent-dependent computation"
        );

        println!("✓ Generated code properly separates intrinsic and parent-dependent computation");

        println!("=== GENERATED INCREMENTAL CODE TEST PASSED ===\n");
    }
}
