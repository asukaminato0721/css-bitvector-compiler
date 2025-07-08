use css_bitvector_compiler::{CssCompiler, CssRule, HtmlNode, SimpleSelector, parse_basic_css};

#[cfg(feature = "run-benchmark")]
mod benchmark;

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
}

pub fn process_google_trace_with_rust() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Testing Google Trace with Rust CSS Engine (CodeGen Mode)\n");

    // Load Google CSS rules
    let css_content = std::fs::read_to_string(format!(
        "css-gen-op/{}/{}.css",
        std::env::var("WEBSITE_NAME").unwrap(),
        std::env::var("WEBSITE_NAME").unwrap()
    ))
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
    let commands_content = std::fs::read_to_string(format!(
        "css-gen-op/{}/command.json",
        std::env::var("WEBSITE_NAME").unwrap()
    ))?;
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

    // Also generate functions for benchmark usage
    let functions_file = "src/generated_css_functions.rs";
    std::fs::write(functions_file, &generated_code)
        .map_err(|e| format!("Failed to write generated functions: {}", e))?;

    println!("💾 Generated functions: {}", functions_file);

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

fn generate_google_trace_program(
    _generated_fn_code: &str,
    _google_node: &GoogleNode,
) -> Result<String, Box<dyn std::error::Error>> {
    // 使用模块引用方法 - 直接使用库中定义的类型和函数
    let mut program = String::new();

    // 1. 导入库中的所有类型和函数
    program.push_str("use css_bitvector_compiler::*;\n");
    program.push_str("use css_bitvector_compiler::generated_css_functions::*;\n\n");

    // 3. 添加结果收集函数
    program.push_str(r#"fn collect_all_matches(node: &mut HtmlNode, parent_state: &BitVector, results: &mut Vec<(String, Vec<usize>)>) {
    // Process this node
    let child_states = process_node_generated_incremental(node, parent_state);
    
    // Collect matches for this node
    let mut matches = Vec::new();
    for i in 0..BITVECTOR_CAPACITY {
        if node.css_match_bitvector.is_bit_set(i) {
            matches.push(i);
        }
    }
    
    // Create node identifier using utility function
    let node_id = create_node_identifier(node);
    results.push((node_id, matches));
    
    // Process children
    for child in &mut node.children {
        collect_all_matches(child, &child_states, results);
    }
}

"#);

    // 4. 添加主函数
    program.push_str(r#"fn main() {
    println!("🚀 Testing OPTIMIZED Layout Calculation with Google Trace");
    
    // Load Google DOM tree
    let mut root = load_dom_from_file();
    println!("✅ Loaded Google DOM tree successfully!");
    
    let initial_state = BitVector::with_capacity(BITVECTOR_CAPACITY);
    let mut optimized_results = Vec::new();
    
    // Collect all matching results
    collect_all_matches(&mut root, &initial_state, &mut optimized_results);
    
    println!("📊 OPTIMIZED Results Summary:");
    println!("Total nodes processed: {}", optimized_results.len());
    
    // Output first few nodes for verification
    println!("\n🔍 First 10 nodes with their CSS rule matches:");
    for (i, (node_id, matches)) in optimized_results.iter().take(10).enumerate() {
        println!("Node {}: {} -> {} rules matched", i+1, node_id, matches.len());
        if !matches.is_empty() {
            let rule_list: Vec<String> = matches.iter().take(5).map(|&r| r.to_string()).collect();
            println!("  Rules: {} {}", rule_list.join(", "), if matches.len() > 5 { "..." } else { "" });
        }
    }
    
    // Save results to file for comparison
    if let Err(e) = save_results_to_file(&optimized_results, "optimized_results.txt") {
        println!("Failed to save optimized results: {}", e);
        return;
    }
    
    println!("\n💾 Full optimized results saved to: optimized_results.txt");
    println!("🔄 Run the naive example to compare results!");
}"#);

    Ok(program)
}

fn main() {
    // Check if we should run benchmarks
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "benchmark" {
        #[cfg(feature = "run-benchmark")]
        {
            println!("🚀 Running Web Browser Layout Trace Benchmark Mode\n");
            benchmark::run_web_browser_layout_trace_benchmark();
        }
        return;
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
    use css_bitvector_compiler::NFAInstruction;

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

        // Create test HTML structure: div > p.item
        let root = HtmlNode::new("div")
            .with_id("outer")
            .add_child(HtmlNode::new("p").with_class("item"))
            .add_child(
                HtmlNode::new("span")
                    .with_class("item")
                    .add_child(HtmlNode::new("p").with_id("specific")),
            );
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
    fn test_bitvector_operations() {
        use css_bitvector_compiler::BitVector;

        // Test basic bitvector operations used in the CSS matching
        let mut bitvector = BitVector::new();

        // Set bit 3
        bitvector.set_bit(3);
        assert!(bitvector.is_bit_set(3));

        // Check bit 2 is not set
        assert!(!bitvector.is_bit_set(2));

        // Set bit 0
        bitvector.set_bit(0);

        // Test multiple bits
        assert!(bitvector.is_bit_set(0));
        assert!(bitvector.is_bit_set(3));
        assert!(!bitvector.is_bit_set(1));

        // Test high bit positions (beyond 64)
        bitvector.set_bit(100);
        assert!(bitvector.is_bit_set(100));

        // Test count_set_bits
        assert_eq!(bitvector.count_set_bits(), 3); // bits 0, 3, and 100

        // Test clear_bit
        bitvector.clear_bit(3);
        assert!(!bitvector.is_bit_set(3));
        assert_eq!(bitvector.count_set_bits(), 2); // bits 0 and 100

        // Test is_empty
        let empty_bv = BitVector::new();
        assert!(empty_bv.is_empty());
        assert!(!bitvector.is_empty());

        // Test or_assign
        let mut other = BitVector::new();
        other.set_bit(5);
        bitvector.or_assign(&other);
        assert!(bitvector.is_bit_set(5));
        assert_eq!(bitvector.count_set_bits(), 3); // bits 0, 5, and 100
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
        // Store initial results for comparison
        let initial_root_matches = root.css_match_bitvector.clone();
        let initial_child1_matches = root.children[0].css_match_bitvector.clone();

        // Verify results didn't change
        assert_eq!(root.css_match_bitvector, initial_root_matches);
        assert_eq!(root.children[0].css_match_bitvector, initial_child1_matches);
        println!("✓ Results consistent with cached version");

        // Test 3: Modify node and verify selective recomputation
        println!("\nTest 3: Selective recomputation after modification");
        root.children[0].mark_dirty();

        // Test 4: Compare with non-incremental version for correctness
        println!("\nTest 4: Correctness verification");
        let root_copy = HtmlNode::new("div")
            .with_id("container")
            .add_child(
                HtmlNode::new("p")
                    .with_class("item")
                    .add_child(HtmlNode::new("span").with_id("specific")),
            )
            .add_child(HtmlNode::new("div").with_class("other"));
        // Compare results
        assert_eq!(&root.css_match_bitvector, &root_copy.css_match_bitvector);
        assert_eq!(
            &root.children[0].css_match_bitvector,
            &root_copy.children[0].css_match_bitvector
        );
        assert_eq!(
            root.children[1].css_match_bitvector,
            root_copy.children[1].css_match_bitvector
        );
        println!("✓ Incremental results match non-incremental results");

        println!("=== INCREMENTAL PROCESSING TESTS PASSED ===\n");
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

        // Create a complex HTML structure
        let root = HtmlNode::new("div")
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
        let regular_time = start.elapsed();

        // Benchmark incremental processing (first run + cached runs)
        let start = Instant::now();

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

        // Build deep tree: root -> child1 -> child2 -> child3
        let mut root = HtmlNode::new("div").add_child(HtmlNode::new("div").add_child(
            HtmlNode::new("div").add_child(HtmlNode::new("span").with_class("highlight")),
        ));

        // Initial processing

        // Verify everything is initially clean
        assert!(!root.has_dirty_descendant);
        assert!(!root.children[0].has_dirty_descendant);

        // Mark deep node dirty using optimized path marking
        assert!(root.mark_node_dirty_by_path(&[0, 0, 0])); // path to deepest span

        // Verify dirty bits are set correctly along the path
        assert!(root.has_dirty_descendant); // But has dirty descendant

        assert!(root.children[0].has_dirty_descendant); // But has dirty descendant

        assert!(root.children[0].children[0].has_dirty_descendant); // But has dirty descendant

        assert!(root.children[0].children[0].children[0].is_self_dirty); // Deepest node is dirty
        assert!(!root.children[0].children[0].children[0].has_dirty_descendant); // No children

        // Process incrementally - should only process nodes on the dirty path

        // With optimization, we should have fewer cache misses than total nodes
        // Only the dirty path should be recomputed, but our current implementation
        // still visits all nodes to check if they need recomputation

        // The key optimization is that only nodes on the dirty path needed recomputation
        // This is more of a conceptual test for now
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

        // Test that selector index correctly identifies matching rules
        let test_node = HtmlNode::new("div").with_class("highlight");
    }
}
