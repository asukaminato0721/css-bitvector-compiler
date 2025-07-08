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
