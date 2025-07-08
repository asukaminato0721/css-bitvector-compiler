use css_bitvector_compiler::{CssCompiler, GoogleNode, parse_basic_css};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 CSS Naive Layout Code Generator");
    println!("📋 Generating layout calculation code without cache or optimization\n");

    // Load Google CSS rules (same as main binary for consistency)
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

    // Compile CSS rules
    let mut compiler = CssCompiler::new();
    let program = compiler.compile_css_rules(&css_rules);

    println!("🔧 Generating naive Rust code (no caching, no optimization)...");
    let naive_code = program.generate_naive_rust_code();

    // Read the first command from command.json to get initial DOM (for consistency)
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

    // Generate complete naive Rust program
    let complete_program = generate_naive_program(&naive_code, &google_node)?;

    // Write to examples directory
    let example_file = "examples/naive_layout_test.rs";
    std::fs::write(example_file, &complete_program)
        .map_err(|e| format!("Failed to write generated code: {}", e))?;

    println!("💾 Generated naive example: {}", example_file);

    // Also generate naive functions for direct usage
    let functions_file = "src/generated_naive_functions.rs";
    std::fs::write(functions_file, &naive_code)
        .map_err(|e| format!("Failed to write generated functions: {}", e))?;

    println!("💾 Generated naive functions: {}", functions_file);

    // Show comparison between optimized and naive approaches
    println!("\n📊 Code Generation Comparison:");
    println!("  Optimized code: uses BitVectors, caching, dirty bits");
    println!("  Naive code: uses Vec<bool>, no caching, calculates from scratch");
    println!("  Both approaches should produce identical results!");

    // Run the generated naive example
    println!("\n🚀 Running generated naive example...\n");
    let run_output = std::process::Command::new("cargo")
        .args(["run", "--example", "naive_layout_test"])
        .output()
        .map_err(|e| format!("Failed to run naive example: {}", e))?;

    if run_output.status.success() {
        let stdout = String::from_utf8_lossy(&run_output.stdout);
        println!("{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&run_output.stderr);
        return Err(format!("Generated naive example failed: {}", stderr).into());
    }

    Ok(())
}

fn generate_naive_program(
    generated_fn_code: &str,
    _google_node: &GoogleNode,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut program = String::new();

    // 1. Import library types and functions
    program.push_str("use css_bitvector_compiler::*;\n\n");

    // 2. Add the generated naive CSS processing functions
    program.push_str("// Generated naive CSS processing functions\n");
    program
        .push_str("// These functions calculate layout from scratch without any optimization\n\n");
    program.push_str(generated_fn_code);
    program.push_str("\n\n");

    // 3. Add result collection function for naive approach
    program.push_str(r#"fn collect_all_naive_matches(node: &mut HtmlNode, parent_matches: &[bool], results: &mut Vec<(String, Vec<usize>)>) {
    // Process this node with naive approach
    let matches = process_node_naive(node, parent_matches);
    
    // Collect rule indices that matched
    let mut matched_rules = Vec::new();
    for (i, &matched) in matches.iter().enumerate() {
        if matched {
            matched_rules.push(i);
        }
    }
    
    // Create node identifier using utility function
    let node_id = create_node_identifier(node);
    results.push((node_id, matched_rules));
    
    // Process children
    for child in &mut node.children {
        collect_all_naive_matches(child, &matches, results);
    }
}

"#);

    // 4. Add main function that uses Google trace and compares results
    program.push_str(r#"fn main() {
    println!("🐌 Testing NAIVE Layout Calculation with Google Trace");
    
    // Load Google DOM tree from css-gen-op/command.json
    let mut root = load_dom_from_file();
    println!("✅ Loaded Google DOM tree successfully!");
    
    let mut naive_results = Vec::new();
    let total_rules = get_total_rules();
    let initial_matches = vec![false; total_rules];
    
    // Collect all matching results
    collect_all_naive_matches(&mut root, &initial_matches, &mut naive_results);
    
    println!("📊 NAIVE Results Summary:");
    println!("Total nodes processed: {}", naive_results.len());
    
    // Output first few nodes for verification
    println!("\n🔍 First 10 nodes with their CSS rule matches:");
    for (i, (node_id, matches)) in naive_results.iter().take(10).enumerate() {
        println!("Node {}: {} -> {} rules matched", i+1, node_id, matches.len());
        if !matches.is_empty() {
            let rule_list: Vec<String> = matches.iter().take(5).map(|&r| r.to_string()).collect();
            println!("  Rules: {} {}", rule_list.join(", "), if matches.len() > 5 { "..." } else { "" });
        }
    }
    
    // Save results to file for comparison
    if let Err(e) = save_results_to_file(&naive_results, "naive_results.txt") {
        println!("Failed to save naive results: {}", e);
        return;
    }
    
    println!("\n💾 Full naive results saved to: naive_results.txt");
    
    // Compare with optimized results if available
    match compare_result_files("optimized_results.txt", "naive_results.txt") {
        Ok(true) => println!("🎉 SUCCESS: Results comparison completed!"),
        Ok(false) => println!("ℹ️  Comparison skipped - run optimized example first"),
        Err(e) => println!("⚠️  Error during comparison: {}", e),
    }
    
    println!("💡 This naive approach recalculates everything from scratch every time.");
    println!("🔍 Each node was checked against all {} CSS rules without any caching.", total_rules);
}"#);

    Ok(program)
}
