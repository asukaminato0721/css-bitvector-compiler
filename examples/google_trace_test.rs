use css_bitvector_compiler::*;
use css_bitvector_compiler::generated_css_functions::*;

fn collect_all_matches(node: &mut HtmlNode, parent_state: &BitVector, results: &mut Vec<(String, Vec<usize>)>) {
    // Process this node
    let child_states = process_node_generated_from_scratch(node, parent_state);
    
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

fn main() {
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
}