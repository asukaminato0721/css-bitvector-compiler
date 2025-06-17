use std::collections::HashMap;
use crate::*;

#[derive(Debug, Clone)]
pub struct PerformanceDataPoint {
    pub nodes_count: usize,
    pub modification_type: String,
    pub incremental_cycles: u64,
    pub fromscratch_cycles: u64,
    pub cache_hit_rate: f64,
    pub speedup: f64,
}

pub struct PerformanceTester;

impl PerformanceTester {
    pub fn new() -> Self {
        PerformanceTester
    }

    pub fn run_comprehensive_benchmark(&self) -> Vec<PerformanceDataPoint> {
        let mut data_points = Vec::new();
        
        println!("🎯 Starting comprehensive performance benchmark...\n");

        // Test 1: Different tree sizes (using Google trace data)
        data_points.extend(self.test_different_tree_sizes());
        
        // Test 2: Different modification patterns
        data_points.extend(self.test_modification_patterns());
        
        // Test 3: Cache efficiency scenarios
        data_points.extend(self.test_cache_scenarios());

        println!("📊 Benchmark complete! Collected {} data points", data_points.len());
        
        data_points
    }

    fn test_different_tree_sizes(&self) -> Vec<PerformanceDataPoint> {
        println!("🌳 Testing different tree sizes...");
        let mut data_points = Vec::new();
        
        // Load Google trace data multiple times with different sizes
        for &tree_size_factor in &[0.25, 0.5, 0.75, 1.0, 1.25] {
            if let Ok(mut dom_tree) = self.create_test_tree_with_factor(tree_size_factor) {
                let nodes_count = self.count_total_nodes(&dom_tree);
                
                // Run multiple iterations for stable measurements
                let iterations = 10;
                let mut incremental_total = 0u64;
                let mut fromscratch_total = 0u64;
                let mut cache_hit_rates = Vec::new();

                for _ in 0..iterations {
                    // Test incremental processing
                    let start_inc = rdtsc();
                    let (_, hits, misses) = self.run_incremental_test(&mut dom_tree.clone());
                    let end_inc = rdtsc();
                    incremental_total += cycles_to_duration(start_inc, end_inc);

                    // Test from-scratch processing
                    let start_fs = rdtsc();
                    let _ = self.run_fromscratch_test(&mut dom_tree.clone());
                    let end_fs = rdtsc();
                    fromscratch_total += cycles_to_duration(start_fs, end_fs);

                    cache_hit_rates.push(hits as f64 / (hits + misses) as f64);
                }

                let avg_incremental = incremental_total / iterations as u64;
                let avg_fromscratch = fromscratch_total / iterations as u64;
                let avg_cache_hit_rate = cache_hit_rates.iter().sum::<f64>() / cache_hit_rates.len();
                let speedup = avg_fromscratch as f64 / avg_incremental.max(1) as f64;

                data_points.push(PerformanceDataPoint {
                    nodes_count,
                    modification_type: format!("tree_size_{:.2}", tree_size_factor),
                    incremental_cycles: avg_incremental,
                    fromscratch_cycles: avg_fromscratch,
                    cache_hit_rate: avg_cache_hit_rate,
                    speedup,
                });

                println!("  📏 Size factor {:.2}: {} nodes, speedup: {:.2}x", 
                    tree_size_factor, nodes_count, speedup);
            }
        }
        
        data_points
    }

    fn test_modification_patterns(&self) -> Vec<PerformanceDataPoint> {
        println!("🔧 Testing different modification patterns...");
        let mut data_points = Vec::new();

        if let Ok(base_tree) = self.create_test_tree_with_factor(1.0) {
            let base_nodes = self.count_total_nodes(&base_tree);

            // Different types of modifications
            let modification_types = vec![
                ("single_leaf", 1),      // Modify 1 leaf node
                ("small_subtree", 3),    // Modify a small subtree
                ("medium_subtree", 8),   // Modify a medium subtree
                ("large_subtree", 20),   // Modify a large subtree
                ("root_change", 50),     // Major change from root
            ];

            for (mod_type, affected_nodes) in modification_types {
                let mut test_tree = base_tree.clone();
                
                // Apply modifications based on type
                self.apply_modification_pattern(&mut test_tree, mod_type, affected_nodes);

                // Measure performance
                let iterations = 20;
                let mut incremental_total = 0u64;
                let mut fromscratch_total = 0u64;
                let mut cache_hit_rates = Vec::new();

                for _ in 0..iterations {
                    let mut inc_tree = test_tree.clone();
                    let mut fs_tree = test_tree.clone();

                    // Incremental test
                    let start_inc = rdtsc();
                    let (_, hits, misses) = self.run_incremental_test(&mut inc_tree);
                    let end_inc = rdtsc();
                    incremental_total += cycles_to_duration(start_inc, end_inc);

                    // From-scratch test
                    let start_fs = rdtsc();
                    let _ = self.run_fromscratch_test(&mut fs_tree);
                    let end_fs = rdtsc();
                    fromscratch_total += cycles_to_duration(start_fs, end_fs);

                    cache_hit_rates.push(hits as f64 / (hits + misses).max(1) as f64);
                }

                let avg_incremental = incremental_total / iterations as u64;
                let avg_fromscratch = fromscratch_total / iterations as u64;
                let avg_cache_hit_rate = cache_hit_rates.iter().sum::<f64>() / cache_hit_rates.len();
                let speedup = avg_fromscratch as f64 / avg_incremental.max(1) as f64;

                data_points.push(PerformanceDataPoint {
                    nodes_count: base_nodes,
                    modification_type: mod_type.to_string(),
                    incremental_cycles: avg_incremental,
                    fromscratch_cycles: avg_fromscratch,
                    cache_hit_rate: avg_cache_hit_rate,
                    speedup,
                });

                println!("  🔧 {} (affects ~{} nodes): speedup {:.2}x, cache hit rate: {:.1}%", 
                    mod_type, affected_nodes, speedup, avg_cache_hit_rate * 100.0);
            }
        }

        data_points
    }

    fn test_cache_scenarios(&self) -> Vec<PerformanceDataPoint> {
        println!("⚡ Testing cache efficiency scenarios...");
        let mut data_points = Vec::new();

        if let Ok(base_tree) = self.create_test_tree_with_factor(1.0) {
            let base_nodes = self.count_total_nodes(&base_tree);

            // Different cache scenarios
            let scenarios = vec![
                ("cold_cache", true),      // Clear cache before each test
                ("warm_cache", false),     // Keep cache warm
                ("mixed_workload", false), // Mix of cache hits and misses
            ];

            for (scenario_name, clear_cache) in scenarios {
                let mut test_tree = base_tree.clone();
                
                // Pre-warm cache if needed
                if !clear_cache {
                    let _ = self.run_incremental_test(&mut test_tree);
                }

                let iterations = 30;
                let mut incremental_total = 0u64;
                let mut fromscratch_total = 0u64;
                let mut cache_hit_rates = Vec::new();

                for i in 0..iterations {
                    let mut inc_tree = test_tree.clone();
                    let mut fs_tree = test_tree.clone();

                    if clear_cache {
                        self.clear_cache_state(&mut inc_tree);
                    }

                    // For mixed workload, occasionally modify nodes
                    if scenario_name == "mixed_workload" && i % 5 == 0 {
                        self.apply_small_modification(&mut inc_tree);
                    }

                    // Incremental test
                    let start_inc = rdtsc();
                    let (_, hits, misses) = self.run_incremental_test(&mut inc_tree);
                    let end_inc = rdtsc();
                    incremental_total += cycles_to_duration(start_inc, end_inc);

                    // From-scratch test
                    let start_fs = rdtsc();
                    let _ = self.run_fromscratch_test(&mut fs_tree);
                    let end_fs = rdtsc();
                    fromscratch_total += cycles_to_duration(start_fs, end_fs);

                    cache_hit_rates.push(hits as f64 / (hits + misses).max(1) as f64);
                }

                let avg_incremental = incremental_total / iterations as u64;
                let avg_fromscratch = fromscratch_total / iterations as u64;
                let avg_cache_hit_rate = cache_hit_rates.iter().sum::<f64>() / cache_hit_rates.len();
                let speedup = avg_fromscratch as f64 / avg_incremental.max(1) as f64;

                data_points.push(PerformanceDataPoint {
                    nodes_count: base_nodes,
                    modification_type: scenario_name.to_string(),
                    incremental_cycles: avg_incremental,
                    fromscratch_cycles: avg_fromscratch,
                    cache_hit_rate: avg_cache_hit_rate,
                    speedup,
                });

                println!("  ⚡ {}: speedup {:.2}x, cache hit rate: {:.1}%", 
                    scenario_name, speedup, avg_cache_hit_rate * 100.0);
            }
        }

        data_points
    }

    // Helper methods
    fn create_test_tree_with_factor(&self, factor: f64) -> Result<HtmlNode, Box<dyn std::error::Error>> {
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
        
        // Scale tree based on factor
        if factor != 1.0 {
            self.scale_tree(&mut html_node, factor);
        }
        
        html_node.init_parent_pointers();
        Ok(html_node)
    }

    fn scale_tree(&self, node: &mut HtmlNode, factor: f64) {
        if factor < 1.0 {
            // Remove some children
            let keep_count = (node.children.len() as f64 * factor).ceil() as usize;
            node.children.truncate(keep_count);
        } else if factor > 1.0 {
            // Duplicate some children
            let original_children = node.children.clone();
            let extra_count = ((node.children.len() as f64 * (factor - 1.0)).ceil() as usize).min(10);
            for i in 0..extra_count {
                if i < original_children.len() {
                    node.children.push(original_children[i].clone());
                }
            }
        }
        
        // Recursively scale children
        for child in &mut node.children {
            self.scale_tree(child, factor);
        }
    }

    fn count_total_nodes(&self, node: &HtmlNode) -> usize {
        1 + node.children.iter().map(|child| self.count_total_nodes(child)).sum::<usize>()
    }

    fn run_incremental_test(&self, tree: &mut HtmlNode) -> (usize, usize, usize) {
        // Load CSS rules
        let css_content = std::fs::read_to_string("css-gen-op/https___www.google.com_.css")
            .unwrap_or_else(|_| "div { display: block; }".to_string());
        let css_rules = parse_basic_css(&css_content);
        
        let mut compiler = CssCompiler::new();
        let program = compiler.compile_css_rules(&css_rules);
        let vm = TreeNFAVM::new(program);
        
        vm.process_tree_incremental_with_stats(tree)
    }

    fn run_fromscratch_test(&self, tree: &mut HtmlNode) -> (usize, usize, usize) {
        // Load CSS rules
        let css_content = std::fs::read_to_string("css-gen-op/https___www.google.com_.css")
            .unwrap_or_else(|_| "div { display: block; }".to_string());
        let css_rules = parse_basic_css(&css_content);
        
        let mut compiler = CssCompiler::new();
        let program = compiler.compile_css_rules(&css_rules);
        let vm = TreeNFAVM::new(program);
        
        // Clear all cache state first
        self.clear_cache_state(tree);
        
        // Process with always-miss cache (simulating from-scratch)
        vm.process_tree_incremental_with_stats(tree)
    }

    fn clear_cache_state(&self, node: &mut HtmlNode) {
        node.is_self_dirty = true;
        node.has_dirty_descendant = false;
        node.cached_parent_state = None;
        node.cached_node_intrinsic = None;
        node.cached_child_states = None;
        
        for child in &mut node.children {
            self.clear_cache_state(child);
        }
    }

    fn apply_modification_pattern(&self, tree: &mut HtmlNode, pattern: &str, _affected_nodes: usize) {
        match pattern {
            "single_leaf" => {
                // Mark a single deep leaf as dirty
                if let Some(leaf) = self.find_deep_leaf(tree, 5) {
                    leaf.mark_dirty();
                }
            }
            "small_subtree" => {
                // Mark a small subtree as dirty
                if tree.children.len() > 0 {
                    tree.children[0].mark_dirty();
                }
            }
            "medium_subtree" => {
                // Mark multiple subtrees as dirty
                for i in 0..tree.children.len().min(2) {
                    tree.children[i].mark_dirty();
                }
            }
            "large_subtree" => {
                // Mark a large portion of the tree as dirty
                for i in 0..tree.children.len().min(4) {
                    tree.children[i].mark_dirty();
                    if tree.children[i].children.len() > 0 {
                        tree.children[i].children[0].mark_dirty();
                    }
                }
            }
            "root_change" => {
                // Mark root and major branches as dirty
                tree.mark_dirty();
            }
            _ => {}
        }
    }

    fn apply_small_modification(&self, tree: &mut HtmlNode) {
        if let Some(node) = self.find_deep_leaf(tree, 3) {
            node.mark_dirty();
        }
    }

    fn find_deep_leaf(&self, node: &mut HtmlNode, depth: usize) -> Option<&mut HtmlNode> {
        if depth == 0 || node.children.is_empty() {
            return Some(node);
        }
        
        for child in &mut node.children {
            if let Some(found) = self.find_deep_leaf(child, depth - 1) {
                return Some(found);
            }
        }
        
        None
    }

    pub fn export_to_csv(&self, data_points: &[PerformanceDataPoint], filename: &str) -> std::io::Result<()> {
        use std::io::Write;
        
        let mut file = std::fs::File::create(filename)?;
        
        // CSV header
        writeln!(file, "nodes_count,modification_type,incremental_cycles,fromscratch_cycles,cache_hit_rate,speedup")?;
        
        // Data rows
        for point in data_points {
            writeln!(file, "{},{},{},{},{:.4},{:.4}", 
                point.nodes_count,
                point.modification_type,
                point.incremental_cycles,
                point.fromscratch_cycles,
                point.cache_hit_rate,
                point.speedup
            )?;
        }
        
        println!("📄 Performance data exported to {}", filename);
        Ok(())
    }

    pub fn print_summary(&self, data_points: &[PerformanceDataPoint]) {
        println!("\n📊 PERFORMANCE BENCHMARK SUMMARY");
        println!("================================");
        
        let avg_speedup = data_points.iter()
            .map(|p| p.speedup)
            .sum::<f64>() / data_points.len() as f64;
            
        let max_speedup = data_points.iter()
            .map(|p| p.speedup)
            .fold(0.0, f64::max);
            
        let avg_cache_hit_rate = data_points.iter()
            .map(|p| p.cache_hit_rate)
            .sum::<f64>() / data_points.len() as f64;

        println!("Total data points: {}", data_points.len());
        println!("Average speedup: {:.2}x", avg_speedup);
        println!("Maximum speedup: {:.2}x", max_speedup);
        println!("Average cache hit rate: {:.1}%", avg_cache_hit_rate * 100.0);
        
        println!("\nTop 5 best performing scenarios:");
        let mut sorted_points = data_points.to_vec();
        sorted_points.sort_by(|a, b| b.speedup.partial_cmp(&a.speedup).unwrap());
        
        for (i, point) in sorted_points.iter().take(5).enumerate() {
            println!("  {}. {} ({} nodes): {:.2}x speedup, {:.1}% cache hits",
                i + 1, point.modification_type, point.nodes_count, 
                point.speedup, point.cache_hit_rate * 100.0);
        }
    }
}

// Additional helper functions needed
pub fn rdtsc() -> u64 {
    // Simple CPU cycle counter using system time as fallback
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}

pub fn cycles_to_duration(start: u64, end: u64) -> u64 {
    end.saturating_sub(start)
} 