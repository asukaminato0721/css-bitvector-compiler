use css_bitvector_compiler::*;

// Generated naive CSS processing functions
// These functions calculate layout from scratch without any optimization

use css_bitvector_compiler::{HtmlNode, SimpleSelector};
use std::collections::HashMap;

// === NAIVE CSS MATCHING FUNCTIONS ===
// These functions calculate layout from scratch without any caching

// Rule 0: CheckAndSetBit { selector: Class("gbmt"), bit_pos: 0 }
pub fn matches_rule_0(node: &HtmlNode) -> bool {
    node.classes.contains("gbmt")
}

// Rule 2: CheckAndSetBit { selector: Class("gbts"), bit_pos: 2 }
pub fn matches_rule_2(node: &HtmlNode) -> bool {
    node.classes.contains("gbts")
}

// Rule 4: CheckAndSetBit { selector: Class("lsb"), bit_pos: 4 }
pub fn matches_rule_4(node: &HtmlNode) -> bool {
    node.classes.contains("lsb")
}

// Rule 6: CheckAndSetBit { selector: Id("gb"), bit_pos: 6 }
pub fn matches_rule_6(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"gb".to_string())
}

// Rule 8: CheckAndSetBit { selector: Id("gbz"), bit_pos: 8 }
pub fn matches_rule_8(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"gbz".to_string())
}

// Rule 10: CheckAndSetBit { selector: Type("a"), bit_pos: 10 }
pub fn matches_rule_10(node: &HtmlNode) -> bool {
    node.tag_name == "a"
}

// Rule 12: CheckAndSetBit { selector: Type("div"), bit_pos: 12 }
pub fn matches_rule_12(node: &HtmlNode) -> bool {
    node.tag_name == "div"
}

// Rule 14: CheckAndSetBit { selector: Type("input"), bit_pos: 14 }
pub fn matches_rule_14(node: &HtmlNode) -> bool {
    node.tag_name == "input"
}

// Rule 16: CheckAndSetBit { selector: Type("span"), bit_pos: 16 }
pub fn matches_rule_16(node: &HtmlNode) -> bool {
    node.tag_name == "span"
}

// === MAIN NAIVE PROCESSING FUNCTION ===
pub fn process_node_naive(node: &mut HtmlNode, parent_matches: &[bool]) -> Vec<bool> {
    let mut matches = vec![false; 18];

    // Check all simple selectors
    if matches_rule_0(node) {
        matches[0] = true;
    }
    if matches_rule_2(node) {
        matches[2] = true;
    }
    if matches_rule_4(node) {
        matches[4] = true;
    }
    if matches_rule_6(node) {
        matches[6] = true;
    }
    if matches_rule_8(node) {
        matches[8] = true;
    }
    if matches_rule_10(node) {
        matches[10] = true;
    }
    if matches_rule_12(node) {
        matches[12] = true;
    }
    if matches_rule_14(node) {
        matches[14] = true;
    }
    if matches_rule_16(node) {
        matches[16] = true;
    }

    // Check all parent-child rules
    // No parent-child rules to check
    let _ = parent_matches; // Suppress unused parameter warning

    matches
}

// === NAIVE TREE TRAVERSAL ===
pub fn process_tree_naive(root: &mut HtmlNode) -> usize {
    let mut total_nodes = 0;
    let empty_parent = vec![false; 18];
    process_tree_recursive_naive(root, &empty_parent, &mut total_nodes);
    total_nodes
}

fn process_tree_recursive_naive(node: &mut HtmlNode, parent_matches: &[bool], total: &mut usize) {
    *total += 1;

    // Calculate matches for this node from scratch
    let node_matches = process_node_naive(node, parent_matches);

    // Process all children with this node's matches as their parent context
    for child in node.children.iter_mut() {
        process_tree_recursive_naive(child, &node_matches, total);
    }
}

// === HELPER FUNCTIONS ===
pub fn get_rule_name(rule_index: usize) -> String {
    format!("rule_{}", rule_index)
}

// Rule mapping:
// Rule 16: match_Type("span")
// Rule 17: active_Type("span")
// Rule 13: active_Type("div")
// Rule 11: active_Type("a")
// Rule 3: active_Class("gbts")
// Rule 1: active_Class("gbmt")
// Rule 14: match_Type("input")
// Rule 9: active_Id("gbz")
// Rule 0: match_Class("gbmt")
// Rule 12: match_Type("div")
// Rule 15: active_Type("input")
// Rule 2: match_Class("gbts")
// Rule 10: match_Type("a")
// Rule 4: match_Class("lsb")
// Rule 6: match_Id("gb")
// Rule 5: active_Class("lsb")
// Rule 7: active_Id("gb")
// Rule 8: match_Id("gbz")

pub fn print_node_matches(node: &HtmlNode, matches: &[bool]) {
    println!("Node '{}' matches:", node.tag_name);
    for (i, &matched) in matches.iter().enumerate() {
        if matched {
            println!("  Rule {}: {}", i, get_rule_name(i));
        }
    }
}

pub fn get_total_rules() -> usize {
    18 // Total number of CSS rules
}

fn collect_all_naive_matches(
    node: &mut HtmlNode,
    parent_matches: &[bool],
    results: &mut Vec<(String, Vec<usize>)>,
) {
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

fn main() {
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
        println!(
            "Node {}: {} -> {} rules matched",
            i + 1,
            node_id,
            matches.len()
        );
        if !matches.is_empty() {
            let rule_list: Vec<String> = matches.iter().take(5).map(|&r| r.to_string()).collect();
            println!(
                "  Rules: {} {}",
                rule_list.join(", "),
                if matches.len() > 5 { "..." } else { "" }
            );
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
    println!(
        "🔍 Each node was checked against all {} CSS rules without any caching.",
        total_rules
    );
}
