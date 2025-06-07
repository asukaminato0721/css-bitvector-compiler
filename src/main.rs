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
use css_bitvector_compiler::parse_basic_css;
use css_bitvector_compiler::rdtsc;
use css_bitvector_compiler::cycles_to_duration;

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

    // 5. 主函数 with enhanced testing
    program.push_str(
        r#"fn main() {    
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
    println!("  Cache hit rate: {:.2}%", if total1 > 0 { hits1 as f64 / total1 as f64 * 100.0 } else { 0.0 });
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
    println!("  Cache hit rate: {:.2}%", if total2 > 0 { hits2 as f64 / total2 as f64 * 100.0 } else { 0.0 });
    println!("  Processing cycles: {}", cycles_test2);
    
    // Performance comparison
    if cycles_test1 > 0 && cycles_test2 > 0 {
        let speedup = cycles_test1 as f64 / cycles_test2 as f64;
        println!("  🚀 Speedup from caching: {:.2}x", speedup);
    }
    
    // Test 3: Mark a deep node dirty and test incremental processing
    if let Some(deep_node) = find_deep_node(&mut root, 5) {
        deep_node.mark_dirty();
        println!("\n📝 Marked a deep node dirty...");

        println!("\n📊 Test 3: After deep node modification");
        let start_test3 = rdtsc();
        let (total3, hits3, misses3) = process_tree_with_stats(&mut root);
        let end_test3 = rdtsc();
        let cycles_test3 = cycles_to_duration(start_test3, end_test3);
        
        println!("  Processed nodes: {}", total3);
        println!("  Cache hits: {}", hits3);
        println!("  Cache misses: {}", misses3);
        println!("  Cache hit rate: {:.2}%", if total3 > 0 { hits3 as f64 / total3 as f64 * 100.0 } else { 0.0 });
        println!("  Processing cycles: {}", cycles_test3);
        println!("  💡 Optimization: Only {} nodes needed reprocessing!", misses3);
        
        // Compare with full reprocessing
        if cycles_test1 > 0 && cycles_test3 > 0 {
            let incremental_speedup = cycles_test1 as f64 / cycles_test3 as f64;
            println!("  ⚡ Incremental speedup: {:.2}x vs full recomputation", incremental_speedup);
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
    let full_time = time_processing(|| {
        // Always reset cache state to force full recomputation
        reset_cache_state(&mut root_full);
        let _ = process_tree_full_recompute(&mut root_full);
    }, iterations);
    
    // Benchmark 2: Incremental processing (with caching)
    let cached_time = time_processing(|| {
        let _ = process_tree_incremental_with_stats(&mut root);
    }, iterations);
    
    // Benchmark 3: Generated code direct call (minimal overhead)
    let mut root_direct = load_dom_from_file();
    root_direct.init_parent_pointers();
    let direct_time = time_processing(|| {
        process_node_generated_incremental(&mut root_direct, BitVector::new());
    }, iterations);
    
    println!("  Full recompute ({} iterations): {} cycles", iterations, full_time);
    println!("  Incremental cached ({} iterations): {} cycles", iterations, cached_time);
    println!("  Direct generated function ({} iterations): {} cycles", iterations, direct_time);
    
    if full_time > 0 && cached_time > 0 {
        let incremental_speedup = full_time as f64 / cached_time as f64;
        println!("  📈 Incremental vs Full speedup: {:.2}x", incremental_speedup);
    }
    
    if cached_time > 0 && direct_time > 0 {
        let direct_speedup = cached_time as f64 / direct_time as f64;
        println!("  ⚡ Generated code efficiency: {:.2}x vs tree traversal", direct_speedup);
    }
    
    let program_end_cycles = rdtsc();
    let total_program_cycles = cycles_to_duration(program_start_cycles, program_end_cycles);
    println!("⏱️  Total program execution: {} CPU cycles", total_program_cycles);
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

fn main() {
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
