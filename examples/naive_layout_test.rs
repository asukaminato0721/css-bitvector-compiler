use css_bitvector_compiler::*;

// Generated naive CSS processing functions
// These functions calculate layout from scratch without any optimization

use css_bitvector_compiler::{HtmlNode, SimpleSelector};
use std::collections::HashMap;

// === NAIVE CSS MATCHING FUNCTIONS ===
// These functions calculate layout from scratch without any caching

// Rule 0: CheckAndSetBit { selector: Class("chunked"), bit_pos: 0 }
pub fn matches_rule_0(node: &HtmlNode) -> bool {
    node.classes.contains("chunked")
}

// Rule 2: CheckAndSetBit { selector: Class("external-icon"), bit_pos: 2 }
pub fn matches_rule_2(node: &HtmlNode) -> bool {
    node.classes.contains("external-icon")
}

// Rule 4: CheckAndSetBit { selector: Class("gbmt"), bit_pos: 4 }
pub fn matches_rule_4(node: &HtmlNode) -> bool {
    node.classes.contains("gbmt")
}

// Rule 6: CheckAndSetBit { selector: Class("gbts"), bit_pos: 6 }
pub fn matches_rule_6(node: &HtmlNode) -> bool {
    node.classes.contains("gbts")
}

// Rule 8: CheckAndSetBit { selector: Class("grecaptcha-badge"), bit_pos: 8 }
pub fn matches_rule_8(node: &HtmlNode) -> bool {
    node.classes.contains("grecaptcha-badge")
}

// Rule 10: CheckAndSetBit { selector: Class("hidden"), bit_pos: 10 }
pub fn matches_rule_10(node: &HtmlNode) -> bool {
    node.classes.contains("hidden")
}

// Rule 12: CheckAndSetBit { selector: Class("lsb"), bit_pos: 12 }
pub fn matches_rule_12(node: &HtmlNode) -> bool {
    node.classes.contains("lsb")
}

// Rule 14: CheckAndSetBit { selector: Class("masthead-skeleton-icon"), bit_pos: 14 }
pub fn matches_rule_14(node: &HtmlNode) -> bool {
    node.classes.contains("masthead-skeleton-icon")
}

// Rule 16: CheckAndSetBit { selector: Class("shell"), bit_pos: 16 }
pub fn matches_rule_16(node: &HtmlNode) -> bool {
    node.classes.contains("shell")
}

// Rule 18: CheckAndSetBit { selector: Class("yt-icons-ext"), bit_pos: 18 }
pub fn matches_rule_18(node: &HtmlNode) -> bool {
    node.classes.contains("yt-icons-ext")
}

// Rule 20: CheckAndSetBit { selector: Id("gb"), bit_pos: 20 }
pub fn matches_rule_20(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"gb".to_string())
}

// Rule 22: CheckAndSetBit { selector: Id("gbz"), bit_pos: 22 }
pub fn matches_rule_22(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"gbz".to_string())
}

// Rule 24: CheckAndSetBit { selector: Id("masthead-logo"), bit_pos: 24 }
pub fn matches_rule_24(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"masthead-logo".to_string())
}

// Rule 26: CheckAndSetBit { selector: Id("masthead-skeleton-icons"), bit_pos: 26 }
pub fn matches_rule_26(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"masthead-skeleton-icons".to_string())
}

// Rule 28: CheckAndSetBit { selector: Id("yt-logo-red-svg"), bit_pos: 28 }
pub fn matches_rule_28(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"yt-logo-red-svg".to_string())
}

// Rule 30: CheckAndSetBit { selector: Id("yt-logo-red-updated-svg"), bit_pos: 30 }
pub fn matches_rule_30(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"yt-logo-red-updated-svg".to_string())
}

// Rule 32: CheckAndSetBit { selector: Id("yt-logo-svg"), bit_pos: 32 }
pub fn matches_rule_32(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"yt-logo-svg".to_string())
}

// Rule 34: CheckAndSetBit { selector: Id("yt-logo-updated-svg"), bit_pos: 34 }
pub fn matches_rule_34(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"yt-logo-updated-svg".to_string())
}

// Rule 36: CheckAndSetBit { selector: Type("a"), bit_pos: 36 }
pub fn matches_rule_36(node: &HtmlNode) -> bool {
    node.tag_name == "a"
}

// Rule 38: CheckAndSetBit { selector: Type("body"), bit_pos: 38 }
pub fn matches_rule_38(node: &HtmlNode) -> bool {
    node.tag_name == "body"
}

// Rule 40: CheckAndSetBit { selector: Type("div"), bit_pos: 40 }
pub fn matches_rule_40(node: &HtmlNode) -> bool {
    node.tag_name == "div"
}

// Rule 42: CheckAndSetBit { selector: Type("html"), bit_pos: 42 }
pub fn matches_rule_42(node: &HtmlNode) -> bool {
    node.tag_name == "html"
}

// Rule 44: CheckAndSetBit { selector: Type("input"), bit_pos: 44 }
pub fn matches_rule_44(node: &HtmlNode) -> bool {
    node.tag_name == "input"
}

// Rule 46: CheckAndSetBit { selector: Type("span"), bit_pos: 46 }
pub fn matches_rule_46(node: &HtmlNode) -> bool {
    node.tag_name == "span"
}

// === MAIN NAIVE PROCESSING FUNCTION ===
pub fn process_node_naive(node: &mut HtmlNode, parent_matches: &[bool]) -> Vec<bool> {
    let mut matches = vec![false; 48];

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
    if matches_rule_18(node) {
        matches[18] = true;
    }
    if matches_rule_20(node) {
        matches[20] = true;
    }
    if matches_rule_22(node) {
        matches[22] = true;
    }
    if matches_rule_24(node) {
        matches[24] = true;
    }
    if matches_rule_26(node) {
        matches[26] = true;
    }
    if matches_rule_28(node) {
        matches[28] = true;
    }
    if matches_rule_30(node) {
        matches[30] = true;
    }
    if matches_rule_32(node) {
        matches[32] = true;
    }
    if matches_rule_34(node) {
        matches[34] = true;
    }
    if matches_rule_36(node) {
        matches[36] = true;
    }
    if matches_rule_38(node) {
        matches[38] = true;
    }
    if matches_rule_40(node) {
        matches[40] = true;
    }
    if matches_rule_42(node) {
        matches[42] = true;
    }
    if matches_rule_44(node) {
        matches[44] = true;
    }
    if matches_rule_46(node) {
        matches[46] = true;
    }

    // Check all parent-child rules
    // No parent-child rules to check
    let _ = parent_matches; // Suppress unused parameter warning

    matches
}

// === NAIVE TREE TRAVERSAL ===
pub fn process_tree_naive(root: &mut HtmlNode) -> usize {
    let mut total_nodes = 0;
    let empty_parent = vec![false; 48];
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
// Rule 11: active_Class("hidden")
// Rule 44: match_Type("input")
// Rule 10: match_Class("hidden")
// Rule 5: active_Class("gbmt")
// Rule 16: match_Class("shell")
// Rule 45: active_Type("input")
// Rule 24: match_Id("masthead-logo")
// Rule 14: match_Class("masthead-skeleton-icon")
// Rule 41: active_Type("div")
// Rule 46: match_Type("span")
// Rule 47: active_Type("span")
// Rule 8: match_Class("grecaptcha-badge")
// Rule 38: match_Type("body")
// Rule 23: active_Id("gbz")
// Rule 29: active_Id("yt-logo-red-svg")
// Rule 0: match_Class("chunked")
// Rule 2: match_Class("external-icon")
// Rule 37: active_Type("a")
// Rule 15: active_Class("masthead-skeleton-icon")
// Rule 43: active_Type("html")
// Rule 18: match_Class("yt-icons-ext")
// Rule 3: active_Class("external-icon")
// Rule 1: active_Class("chunked")
// Rule 6: match_Class("gbts")
// Rule 27: active_Id("masthead-skeleton-icons")
// Rule 32: match_Id("yt-logo-svg")
// Rule 39: active_Type("body")
// Rule 40: match_Type("div")
// Rule 28: match_Id("yt-logo-red-svg")
// Rule 4: match_Class("gbmt")
// Rule 7: active_Class("gbts")
// Rule 35: active_Id("yt-logo-updated-svg")
// Rule 17: active_Class("shell")
// Rule 30: match_Id("yt-logo-red-updated-svg")
// Rule 34: match_Id("yt-logo-updated-svg")
// Rule 9: active_Class("grecaptcha-badge")
// Rule 25: active_Id("masthead-logo")
// Rule 20: match_Id("gb")
// Rule 33: active_Id("yt-logo-svg")
// Rule 36: match_Type("a")
// Rule 42: match_Type("html")
// Rule 13: active_Class("lsb")
// Rule 31: active_Id("yt-logo-red-updated-svg")
// Rule 26: match_Id("masthead-skeleton-icons")
// Rule 12: match_Class("lsb")
// Rule 22: match_Id("gbz")
// Rule 21: active_Id("gb")
// Rule 19: active_Class("yt-icons-ext")

pub fn print_node_matches(node: &HtmlNode, matches: &[bool]) {
    println!("Node '{}' matches:", node.tag_name);
    for (i, &matched) in matches.iter().enumerate() {
        if matched {
            println!("  Rule {}: {}", i, get_rule_name(i));
        }
    }
}

pub fn get_total_rules() -> usize {
    48 // Total number of CSS rules
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
