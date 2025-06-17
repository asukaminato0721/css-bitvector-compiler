use std::collections::HashMap;
use serde_json;
use crate::*;

#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub test_name: String,
    pub nodes_count: usize,
    pub incremental_cycles: u64,
    pub fromscratch_cycles: u64,
    pub cache_hit_rate: f64,
    pub speedup: f64,
}

pub fn run_comprehensive_benchmark() -> Vec<BenchmarkResult> {
    println!("🚀 Starting comprehensive benchmark: Incremental vs From-Scratch");
    println!("================================================================\n");
    
    let mut results = Vec::new();
    
    // Test 1: Different node counts using scaled Google trace data
    results.extend(benchmark_different_scales());
    
    // Test 2: Different modification patterns
    results.extend(benchmark_modification_patterns());
    
    // Test 3: Cache scenarios
    results.extend(benchmark_cache_scenarios());
    
    results
}

fn benchmark_different_scales() -> Vec<BenchmarkResult> {
    println!("📏 Benchmarking different tree scales...");
    let mut results = Vec::new();
    
    // Different scaling factors for the tree
    let scale_factors = vec![0.1, 0.25, 0.5, 0.75, 1.0, 1.25, 1.5];
    
    for &scale in &scale_factors {
        if let Ok(mut tree) = create_scaled_google_tree(scale) {
            let nodes_count = count_total_nodes(&tree);
            
            // Skip if tree is too small
            if nodes_count < 5 {
                continue;
            }
            
            println!("  🌳 Testing scale {:.2} ({} nodes)...", scale, nodes_count);
            
            let result = benchmark_tree_processing(&mut tree, 
                &format!("scale_{:.2}", scale));
            results.push(result);
        }
    }
    
    results
}

fn benchmark_modification_patterns() -> Vec<BenchmarkResult> {
    println!("🔧 Benchmarking modification patterns...");
    let mut results = Vec::new();
    
    if let Ok(base_tree) = create_scaled_google_tree(1.0) {
        let modification_patterns = vec![
            ("no_change", 0),
            ("single_node", 1),
            ("small_subtree", 3),
            ("medium_subtree", 8),
            ("large_subtree", 20),
        ];
        
        for (pattern_name, _affected_count) in modification_patterns {
            let mut test_tree = base_tree.clone();
            apply_modification_pattern(&mut test_tree, pattern_name);
            
            println!("  🔧 Testing pattern: {}...", pattern_name);
            let result = benchmark_tree_processing(&mut test_tree, pattern_name);
            results.push(result);
        }
    }
    
    results
}

fn benchmark_cache_scenarios() -> Vec<BenchmarkResult> {
    println!("⚡ Benchmarking cache scenarios...");
    let mut results = Vec::new();
    
    if let Ok(mut tree) = create_scaled_google_tree(1.0) {
        // Test cold cache (everything dirty)
        clear_all_cache(&mut tree);
        let result = benchmark_tree_processing(&mut tree, "cold_cache");
        results.push(result);
        
        // Test warm cache (pre-computed)
        let mut warm_tree = tree.clone();
        let _ = run_incremental_processing(&mut warm_tree); // Pre-warm
        let result = benchmark_tree_processing(&mut warm_tree, "warm_cache");
        results.push(result);
    }
    
    results
}

fn benchmark_tree_processing(tree: &mut HtmlNode, test_name: &str) -> BenchmarkResult {
    let nodes_count = count_total_nodes(tree);
    let iterations = if nodes_count > 100 { 10 } else { 50 };
    
    // Benchmark incremental processing
    let mut incremental_total = 0u64;
    let mut cache_hit_rates = Vec::new();
    
    for _ in 0..iterations {
        let mut inc_tree = tree.clone();
        
        let start = rdtsc();
        let (total, hits, misses) = run_incremental_processing(&mut inc_tree);
        let end = rdtsc();
        
        incremental_total += cycles_to_duration(start, end);
        
        if total > 0 {
            cache_hit_rates.push(hits as f64 / total as f64);
        }
    }
    
    // Benchmark from-scratch processing
    let mut fromscratch_total = 0u64;
    
    for _ in 0..iterations {
        let mut fs_tree = tree.clone();
        clear_all_cache(&mut fs_tree); // Ensure from-scratch processing
        
        let start = rdtsc();
        let _ = run_incremental_processing(&mut fs_tree);
        let end = rdtsc();
        
        fromscratch_total += cycles_to_duration(start, end);
    }
    
    let avg_incremental = incremental_total / iterations as u64;
    let avg_fromscratch = fromscratch_total / iterations as u64;
    let avg_cache_hit_rate = if cache_hit_rates.is_empty() { 
        0.0 
    } else { 
        cache_hit_rates.iter().sum::<f64>() / cache_hit_rates.len() 
    };
    let speedup = if avg_incremental > 0 {
        avg_fromscratch as f64 / avg_incremental as f64
    } else {
        1.0
    };
    
    println!("    📊 {} nodes: {:.2}x speedup, {:.1}% cache hits", 
        nodes_count, speedup, avg_cache_hit_rate * 100.0);
    
    BenchmarkResult {
        test_name: test_name.to_string(),
        nodes_count,
        incremental_cycles: avg_incremental,
        fromscratch_cycles: avg_fromscratch,
        cache_hit_rate: avg_cache_hit_rate,
        speedup,
    }
}

fn create_scaled_google_tree(scale: f64) -> Result<HtmlNode, Box<dyn std::error::Error>> {
    // Load Google trace data
    let command_file_path = "css-gen-op/command.json";
    let content = std::fs::read_to_string(command_file_path)?;
    let first_line = content.lines().next().ok_or("Empty command file")?;
    let command: serde_json::Value = serde_json::from_str(first_line)?;
    
    if command["name"] != "init" {
        return Err("First command should be init".into());
    }
    
    let google_node = GoogleNode::from_json(&command["node"])
        .ok_or("Failed to parse Google node")?;
    
    let mut html_node = google_node.to_html_node();
    
    // Scale the tree
    if scale != 1.0 {
        scale_tree_recursive(&mut html_node, scale, 0, 3); // Limit recursion depth
    }
    
    html_node.init_parent_pointers();
    Ok(html_node)
}

fn scale_tree_recursive(node: &mut HtmlNode, scale: f64, depth: usize, max_depth: usize) {
    if depth >= max_depth {
        return;
    }
    
    if scale < 1.0 {
        // Remove some children
        let keep_count = (node.children.len() as f64 * scale).ceil() as usize;
        node.children.truncate(keep_count);
    } else if scale > 1.0 && node.children.len() > 0 {
        // Duplicate some children
        let original_children = node.children.clone();
        let extra_count = ((node.children.len() as f64 * (scale - 1.0)).ceil() as usize).min(5);
        
        for i in 0..extra_count {
            if i < original_children.len() {
                node.children.push(original_children[i].clone());
            }
        }
    }
    
    // Recursively scale children
    for child in &mut node.children {
        scale_tree_recursive(child, scale, depth + 1, max_depth);
    }
}

fn apply_modification_pattern(tree: &mut HtmlNode, pattern: &str) {
    match pattern {
        "no_change" => {
            // Do nothing - test pure cache performance
        }
        "single_node" => {
            // Mark one deep node as dirty
            if let Some(node) = find_node_at_depth(tree, 2) {
                node.mark_dirty();
            }
        }
        "small_subtree" => {
            // Mark first child subtree as dirty
            if !tree.children.is_empty() {
                tree.children[0].mark_dirty();
            }
        }
        "medium_subtree" => {
            // Mark multiple subtrees as dirty
            for i in 0..tree.children.len().min(3) {
                tree.children[i].mark_dirty();
            }
        }
        "large_subtree" => {
            // Mark most of the tree as dirty
            tree.mark_dirty();
        }
        _ => {}
    }
}

fn find_node_at_depth(node: &mut HtmlNode, depth: usize) -> Option<&mut HtmlNode> {
    if depth == 0 {
        return Some(node);
    }
    
    for child in &mut node.children {
        if let Some(found) = find_node_at_depth(child, depth - 1) {
            return Some(found);
        }
    }
    
    None
}

fn run_incremental_processing(tree: &mut HtmlNode) -> (usize, usize, usize) {
    // Load CSS rules
    let css_content = std::fs::read_to_string("css-gen-op/https___www.google.com_.css")
        .unwrap_or_else(|_| {
            // Fallback CSS rules
            r#"
            /* Rule 0 */ div {}
            /* Rule 1 */ .gbts {}
            /* Rule 2 */ #gb {}
            /* Rule 3 */ span {}
            /* Rule 4 */ .gac_m {}
            "#.to_string()
        });
    
    let css_rules = parse_basic_css(&css_content);
    let mut compiler = CssCompiler::new();
    let program = compiler.compile_css_rules(&css_rules);
    
    // Simulate incremental processing with stats
    let vm = TreeNFAVM::new(program);
    vm.process_tree_incremental_with_stats(tree)
}

fn clear_all_cache(node: &mut HtmlNode) {
    node.is_self_dirty = true;
    node.has_dirty_descendant = false;
    node.cached_parent_state = None;
    node.cached_node_intrinsic = None;
    node.cached_child_states = None;
    
    for child in &mut node.children {
        clear_all_cache(child);
    }
}

pub fn export_benchmark_csv(results: &[BenchmarkResult]) -> std::io::Result<()> {
    use std::io::Write;
    
    let filename = "benchmark_results.csv";
    let mut file = std::fs::File::create(filename)?;
    
    // Write CSV header
    writeln!(file, "test_name,nodes_count,incremental_cycles,fromscratch_cycles,cache_hit_rate,speedup")?;
    
    // Write data
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

pub fn print_benchmark_summary(results: &[BenchmarkResult]) {
    println!("\n📊 BENCHMARK SUMMARY");
    println!("====================");
    
    if results.is_empty() {
        println!("No benchmark results to display.");
        return;
    }
    
    let total_tests = results.len();
    let avg_speedup = results.iter().map(|r| r.speedup).sum::<f64>() / total_tests as f64;
    let max_speedup = results.iter().map(|r| r.speedup).fold(0.0, f64::max);
    let min_speedup = results.iter().map(|r| r.speedup).fold(f64::INFINITY, f64::min);
    let avg_cache_hit_rate = results.iter().map(|r| r.cache_hit_rate).sum::<f64>() / total_tests as f64;
    
    println!("Total tests: {}", total_tests);
    println!("Average speedup: {:.2}x", avg_speedup);
    println!("Maximum speedup: {:.2}x", max_speedup);
    println!("Minimum speedup: {:.2}x", min_speedup);
    println!("Average cache hit rate: {:.1}%", avg_cache_hit_rate * 100.0);
    
    println!("\nDetailed Results:");
    println!("================");
    for result in results {
        println!(
            "{:15} | {:4} nodes | {:8} vs {:8} cycles | {:.2}x speedup | {:.1}% cache hits",
            result.test_name,
            result.nodes_count,
            result.incremental_cycles,
            result.fromscratch_cycles,
            result.speedup,
            result.cache_hit_rate * 100.0
        );
    }
}

// We need these types from the main file - let's include the minimal required definitions
pub struct TreeNFAVM {
    program: TreeNFAProgram,
    selector_index: SelectorMatchingIndex,
}

impl TreeNFAVM {
    pub fn new(program: TreeNFAProgram) -> Self {
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
    
    pub fn process_tree_incremental_with_stats(&self, root: &mut HtmlNode) -> (usize, usize, usize) {
        let mut total_nodes = 0;
        let mut cache_hits = 0;
        let mut cache_misses = 0;
        
        self.process_node_recursive_with_stats(
            root, 
            BitVector::new(), 
            &mut total_nodes, 
            &mut cache_hits, 
            &mut cache_misses
        );
        
        (total_nodes, cache_hits, cache_misses)
    }
    
    fn process_node_recursive_with_stats(
        &self,
        node: &mut HtmlNode,
        parent_state: BitVector,
        total: &mut usize,
        hits: &mut usize,
        misses: &mut usize,
    ) {
        *total += 1;
        
        // Check if this node needs recomputation
        if node.needs_any_recomputation(parent_state) {
            *misses += 1;
            
            // Process this node
            let child_states = self.process_node_incremental(node, parent_state);
            
            // Process children
            for child in &mut node.children {
                self.process_node_recursive_with_stats(child, child_states, total, hits, misses);
            }
        } else {
            *hits += 1;
            // Skip entire subtree when cached
        }
    }
    
    fn process_node_incremental(&self, node: &mut HtmlNode, parent_state: BitVector) -> BitVector {
        // Simplified incremental processing
        if !node.needs_any_recomputation(parent_state) {
            return node.cached_child_states.unwrap_or_default();
        }
        
        // Compute matches
        let mut current_matches = BitVector::new();
        let mut child_states = BitVector::new();
        
        // Simple matching logic
        for instruction in &self.program.instructions {
            match instruction {
                NFAInstruction::CheckAndSetBit { selector, bit_pos } => {
                    if self.node_matches_selector(node, selector) {
                        current_matches.set_bit(*bit_pos);
                    }
                }
                NFAInstruction::PropagateToChildren { match_bit, active_bit } => {
                    if current_matches.is_bit_set(*match_bit) {
                        child_states.set_bit(*active_bit);
                    }
                }
                NFAInstruction::CheckParentAndSetBit { parent_state_bit, child_selector, result_bit } => {
                    if parent_state.is_bit_set(*parent_state_bit) 
                        && self.node_matches_selector(node, child_selector) {
                        current_matches.set_bit(*result_bit);
                    }
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
    
    fn node_matches_selector(&self, node: &HtmlNode, selector: &SimpleSelector) -> bool {
        match selector {
            SimpleSelector::Type(tag) => node.tag_name == *tag,
            SimpleSelector::Class(class) => node.classes.contains(class),
            SimpleSelector::Id(id) => node.id.as_deref() == Some(id),
        }
    }
} 