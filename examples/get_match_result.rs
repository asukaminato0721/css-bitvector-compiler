use css_bitvector_compiler::{
    generated_istate_functions::{BITVECTOR_CAPACITY, process_node_generated},
    generated_naive_functions::process_node_naive,
    *,
};
use generated_bitvector_functions::*;
use std::fs;

fn collect_all_naive_matches(
    node: &mut HtmlNode,
    parent_state: &BitVector,
    results: &mut Vec<(String, Vec<usize>)>,
) {
    let mut tmp_vec = vec![Default::default(); parent_state.capacity + 10];
    for i in 0..parent_state.capacity {
        tmp_vec[i] = parent_state.is_bit_set(i);
    }
    let child_states = process_node_naive(node, &tmp_vec);

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
    let mut tmp_child = BitVector::new();
    for (i, &bo) in child_states.iter().enumerate() {
        if bo {
            tmp_child.set_bit(i);
        }
    }
    // Process children
    for child in &mut node.children {
        collect_all_naive_matches(child, &tmp_child, results);
    }
}

fn collect_all_bitvector_matches(
    node: &mut HtmlNode,
    parent_state: &BitVector,
    results: &mut Vec<(String, Vec<usize>)>,
) {
    // Process this node with BitVector-only approach
    let child_states = process_node_generated_bitvector(node, parent_state);

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

fn collect_all_istate_matches(
    node: &mut HtmlNode,
    parent_state: &BitVector,
    results: &mut Vec<(String, Vec<usize>)>,
) {
    // Process this node with BitVector-only approach
    let child_states = process_node_generated(node, parent_state);
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
        collect_all_istate_matches(child, &child_states, results);
    }
}

fn save_results_to_file(
    results: &[(String, Vec<usize>)],
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut output = String::new();
    for (node_id, matches) in results {
        output.push_str(&format!("{}: {:?}\n", node_id, matches));
    }
    std::fs::write(filename, output)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 CSS BitVector-Only Layout Engine Test");
    println!("==========================================");

    // Load DOM from Google trace file
    println!("📁 Loading DOM from trace file...");
    let root = load_dom_from_file();
    let node_count = count_total_nodes(&root);
    println!("  Loaded DOM tree with {} nodes", node_count);

    // Load CSS rules
    println!("📜 Loading CSS rules...");
    let css_content = fs::read_to_string(format!(
        "css-gen-op/{}/{}.css",
        std::env::var("WEBSITE_NAME").unwrap(),
        std::env::var("WEBSITE_NAME").unwrap()
    ))?;
    let css_rules = parse_css(&css_content);
    println!("   Parsed {} CSS rules", css_rules.len());

    let mut compiler = CssCompiler::new();
    let program = compiler.compile_css_rules(&css_rules);
    println!(
        "   Generated {} NFA instructions",
        program.instructions.len()
    );

    // Create a fresh copy of the DOM for testing
    let mut test_root = load_dom_from_file();
    let initial_state = BitVector::with_capacity(program.total_bits);
    let mut bitvector_results = Vec::new();

    collect_all_bitvector_matches(&mut test_root, &initial_state, &mut bitvector_results);
    save_results_to_file(&bitvector_results, "bitvector_results.txt")?;

    let mut istate = vec![];

    collect_all_istate_matches(&mut test_root, &initial_state, &mut istate);
    save_results_to_file(&istate, "optimized_results.txt")?;
    let mut naive_result = vec![];
    collect_all_naive_matches(&mut test_root, &initial_state, &mut naive_result);
    save_results_to_file(&naive_result, "naive_results.txt")?;

    Ok(())
}
