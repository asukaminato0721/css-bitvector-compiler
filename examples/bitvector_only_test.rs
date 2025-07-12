use css_bitvector_compiler::*;
use generated_bitvector_functions::*;
use std::fs;

fn collect_all_bitvector_matches(
    node: &mut HtmlNode,
    parent_state: &BitVector,
    results: &mut Vec<(String, Vec<usize>)>,
) {
    // Process this node with BitVector-only approach
    let child_states = process_node_generated_bitvector_incremental(node, parent_state);

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
    let root = load_dom_from_file();
    let node_count = count_total_nodes(&root);
    println!("   ✓ Loaded DOM tree with {} nodes", node_count);

    // Load CSS rules
    println!("📜 Loading CSS rules...");
    let css_content = fs::read_to_string(format!(
        "css-gen-op/{}/{}.css",
        std::env::var("WEBSITE_NAME").unwrap(),
        std::env::var("WEBSITE_NAME").unwrap()
    ))
    .unwrap_or_else(|_| String::from("/* fallback CSS */"));
    let css_rules = parse_css(&css_content);
    println!("   ✓ Parsed {} CSS rules", css_rules.len());

    let mut compiler = CssCompiler::new();
    let program = compiler.compile_css_rules(&css_rules);
    println!(
        "   ✓ Generated {} NFA instructions",
        program.instructions.len()
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

    // Save results to file for comparison
    if let Err(e) = save_results_to_file(&bitvector_results, "bitvector_results.txt") {
        println!("❌ Failed to save BitVector results: {}", e);
        return Err(e.into());
    }

    println!("\nFull BitVector-only results saved to: bitvector_results.txt");

    Ok(())
}
