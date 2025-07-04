use css_bitvector_compiler::*;
use std::fs;

// Import the generated BitVector-only functions
mod generated_bitvector_functions {
    include!("../src/generated_bitvector_functions.rs");
}

use generated_bitvector_functions::*;

fn collect_all_bitvector_matches(
    node: &mut HtmlNode,
    parent_state: &BitVector,
    results: &mut Vec<(String, Vec<usize>)>,
) {
    // Process this node with BitVector-only approach
    let child_states = process_node_generated_bitvector_from_scratch(node, parent_state);

    // Collect matches for this node by checking its BitVector
    let mut matches = Vec::new();
    for i in 0..node.css_match_bitvector.capacity {
        if node.css_match_bitvector.is_bit_set(i) {
            matches.push(i);
        }
    }

    // Create node identifier using utility function
    let node_id = create_node_identifier(node);
    results.push((node_id, matches));

    // Process children
    for child in &mut node.children {
        collect_all_bitvector_matches(child, &child_states, results);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 CSS BitVector-Only Layout Engine Test");
    println!("==========================================");

    // Load DOM from Google trace file
    println!("📁 Loading DOM from trace file...");
    let mut root = load_dom_from_file();
    let node_count = count_total_nodes(&root);
    println!("   ✓ Loaded DOM tree with {} nodes", node_count);

    // Load CSS rules
    println!("📜 Loading CSS rules...");
    let css_content = fs::read_to_string("css-gen-op/https___www.google.com_.css")
        .unwrap_or_else(|_| String::from("/* fallback CSS */"));
    let css_rules = parse_basic_css(&css_content);
    println!("   ✓ Parsed {} CSS rules", css_rules.len());

    // Compile CSS to NFA program
    println!("⚙️  Compiling CSS rules...");
    let mut compiler = CssCompiler::new();
    let program = compiler.compile_css_rules(&css_rules);
    println!(
        "   ✓ Generated {} NFA instructions",
        program.instructions.len()
    );
    println!("   ✓ Total state bits: {}", program.total_bits);

    // Generate BitVector-only Rust code
    println!("🔨 Generating BitVector-only Rust code...");
    let bitvector_code = program.generate_bitvector_only_rust_code();
    fs::write("src/generated_bitvector_functions.rs", &bitvector_code)?;
    println!("   ✓ Generated BitVector-only code saved to src/generated_bitvector_functions.rs");

    // Also generate the original IState-based code for comparison
    println!("🔨 Generating IState-based Rust code for comparison...");
    let istate_code = program.generate_rust_code();
    fs::write("src/generated_istate_functions.rs", &istate_code)?;
    println!("   ✓ Generated IState-based code saved to src/generated_istate_functions.rs");

    println!("✅ Code generation complete!");
    println!();
    println!("📊 Code Statistics:");
    println!("   - BitVector-only code: {} bytes", bitvector_code.len());
    println!("   - IState-based code:   {} bytes", istate_code.len());
    println!(
        "   - Size difference:     {} bytes",
        if bitvector_code.len() > istate_code.len() {
            format!("+{}", bitvector_code.len() - istate_code.len())
        } else {
            format!("-{}", istate_code.len() - bitvector_code.len())
        }
    );

    // Now test the BitVector-only approach and collect results
    println!();
    println!("🧪 Testing BitVector-Only CSS Processing...");

    // Create a fresh copy of the DOM for testing
    let mut test_root = load_dom_from_file();
    let initial_state = BitVector::with_capacity(program.total_bits);
    let mut bitvector_results = Vec::new();

    // Collect all matching results using BitVector-only approach
    collect_all_bitvector_matches(&mut test_root, &initial_state, &mut bitvector_results);

    println!("📊 BitVector-Only Results Summary:");
    println!("   Total nodes processed: {}", bitvector_results.len());

    // Output first few nodes for verification
    println!("\n🔍 First 10 nodes with their CSS rule matches:");
    for (i, (node_id, matches)) in bitvector_results.iter().take(10).enumerate() {
        println!(
            "   Node {}: {} -> {} rules matched",
            i + 1,
            node_id,
            matches.len()
        );
        if !matches.is_empty() {
            let rule_list: Vec<String> = matches.iter().take(5).map(|&r| r.to_string()).collect();
            println!(
                "     Rules: {} {}",
                rule_list.join(", "),
                if matches.len() > 5 { "..." } else { "" }
            );
        }
    }

    // Save results to file for comparison
    if let Err(e) = save_results_to_file(&bitvector_results, "bitvector_results.txt") {
        println!("❌ Failed to save BitVector results: {}", e);
        return Err(e.into());
    }

    println!("\n💾 Full BitVector-only results saved to: bitvector_results.txt");

    // Compare with existing results if available
    println!("\n🔄 Comparing results with other approaches...");

    // Compare with naive results
    match compare_result_files("bitvector_results.txt", "naive_results.txt") {
        Ok(true) => println!("   ✅ BitVector-only results MATCH naive results!"),
        Ok(false) => println!("   ℹ️  Naive results not available for comparison"),
        Err(e) => println!("   ⚠️  Error comparing with naive results: {}", e),
    }

    // Compare with optimized results
    match compare_result_files("bitvector_results.txt", "optimized_results.txt") {
        Ok(true) => println!("   ✅ BitVector-only results MATCH optimized results!"),
        Ok(false) => println!("   ℹ️  Optimized results not available for comparison"),
        Err(e) => println!("   ⚠️  Error comparing with optimized results: {}", e),
    }

    println!();
    println!("🚀 Next steps:");
    println!("   1. Compile with `cargo build`");
    println!("   2. Compare performance of all three approaches");
    println!("   3. Run benchmarks to measure differences");
    println!("   4. Analyze memory usage patterns");

    println!();
    println!("💡 BitVector-only approach features:");
    println!("   - Uses pure BitVector operations for parent state tracking");
    println!("   - Avoids enum overhead of IState/OState");
    println!("   - Same optimization benefits as IState version");
    println!("   - Potentially better cache locality");

    Ok(())
}
