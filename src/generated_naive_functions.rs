use crate::{HtmlNode, SimpleSelector};
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

// Rule 4: CheckAndSetBit { selector: Class("grecaptcha-badge"), bit_pos: 4 }
pub fn matches_rule_4(node: &HtmlNode) -> bool {
    node.classes.contains("grecaptcha-badge")
}

// Rule 6: CheckAndSetBit { selector: Class("hidden"), bit_pos: 6 }
pub fn matches_rule_6(node: &HtmlNode) -> bool {
    node.classes.contains("hidden")
}

// Rule 8: CheckAndSetBit { selector: Class("masthead-skeleton-icon"), bit_pos: 8 }
pub fn matches_rule_8(node: &HtmlNode) -> bool {
    node.classes.contains("masthead-skeleton-icon")
}

// Rule 10: CheckAndSetBit { selector: Class("shell"), bit_pos: 10 }
pub fn matches_rule_10(node: &HtmlNode) -> bool {
    node.classes.contains("shell")
}

// Rule 12: CheckAndSetBit { selector: Class("yt-icons-ext"), bit_pos: 12 }
pub fn matches_rule_12(node: &HtmlNode) -> bool {
    node.classes.contains("yt-icons-ext")
}

// Rule 14: CheckAndSetBit { selector: Id("masthead-logo"), bit_pos: 14 }
pub fn matches_rule_14(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"masthead-logo".to_string())
}

// Rule 16: CheckAndSetBit { selector: Id("masthead-skeleton-icons"), bit_pos: 16 }
pub fn matches_rule_16(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"masthead-skeleton-icons".to_string())
}

// Rule 18: CheckAndSetBit { selector: Id("yt-logo-red-svg"), bit_pos: 18 }
pub fn matches_rule_18(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"yt-logo-red-svg".to_string())
}

// Rule 20: CheckAndSetBit { selector: Id("yt-logo-red-updated-svg"), bit_pos: 20 }
pub fn matches_rule_20(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"yt-logo-red-updated-svg".to_string())
}

// Rule 22: CheckAndSetBit { selector: Id("yt-logo-svg"), bit_pos: 22 }
pub fn matches_rule_22(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"yt-logo-svg".to_string())
}

// Rule 24: CheckAndSetBit { selector: Id("yt-logo-updated-svg"), bit_pos: 24 }
pub fn matches_rule_24(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"yt-logo-updated-svg".to_string())
}

// Rule 26: CheckAndSetBit { selector: Type("body"), bit_pos: 26 }
pub fn matches_rule_26(node: &HtmlNode) -> bool {
    node.tag_name == "body"
}

// Rule 28: CheckAndSetBit { selector: Type("html"), bit_pos: 28 }
pub fn matches_rule_28(node: &HtmlNode) -> bool {
    node.tag_name == "html"
}

// Rule 30: CheckAndSetBit { selector: Type("input"), bit_pos: 30 }
pub fn matches_rule_30(node: &HtmlNode) -> bool {
    node.tag_name == "input"
}

// === MAIN NAIVE PROCESSING FUNCTION ===
pub fn process_node_naive(node: &mut HtmlNode, parent_matches: &[bool]) -> Vec<bool> {
    let mut matches = vec![false; 32];

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

    // Check all parent-child rules
    // No parent-child rules to check
    let _ = parent_matches; // Suppress unused parameter warning

    matches
}

// === NAIVE TREE TRAVERSAL ===
pub fn process_tree_naive(root: &mut HtmlNode) -> usize {
    let mut total_nodes = 0;
    let empty_parent = vec![false; 32];
    process_tree_recursive_naive(root, &empty_parent, &mut total_nodes);
    total_nodes
}
fn process_tree_recursive_naive(node: &mut HtmlNode, parent_matches: &[bool], total: &mut usize) {
    *total += 1;

    // Calculate matches for this node from scratch
    let node_matches = process_node_naive(node, parent_matches);
    // Process all children with this node's matches as their parent context,
    for child in node.children.iter_mut() {
        process_tree_recursive_naive(child, &node_matches, total);
    }
}

pub fn get_rule_name(rule_index: usize) -> String {
    format!("rule_{}", rule_index)
} // Rule mapping:
// Rule 27: active_Type("body")
// Rule 2: match_Class("external-icon")
// Rule 20: match_Id("yt-logo-red-updated-svg")
// Rule 7: active_Class("hidden")
// Rule 18: match_Id("yt-logo-red-svg")
// Rule 22: match_Id("yt-logo-svg")
// Rule 24: match_Id("yt-logo-updated-svg")
// Rule 29: active_Type("html")
// Rule 3: active_Class("external-icon")
// Rule 9: active_Class("masthead-skeleton-icon")
// Rule 1: active_Class("chunked")
// Rule 8: match_Class("masthead-skeleton-icon")
// Rule 21: active_Id("yt-logo-red-updated-svg")
// Rule 12: match_Class("yt-icons-ext")
// Rule 23: active_Id("yt-logo-svg")
// Rule 0: match_Class("chunked")
// Rule 16: match_Id("masthead-skeleton-icons")
// Rule 14: match_Id("masthead-logo")
// Rule 10: match_Class("shell")
// Rule 15: active_Id("masthead-logo")
// Rule 19: active_Id("yt-logo-red-svg")
// Rule 28: match_Type("html")
// Rule 30: match_Type("input")
// Rule 25: active_Id("yt-logo-updated-svg")
// Rule 4: match_Class("grecaptcha-badge")
// Rule 26: match_Type("body")
// Rule 5: active_Class("grecaptcha-badge")
// Rule 6: match_Class("hidden")
// Rule 31: active_Type("input")
// Rule 11: active_Class("shell")
// Rule 13: active_Class("yt-icons-ext")
// Rule 17: active_Id("masthead-skeleton-icons")

pub fn print_node_matches(node: &HtmlNode, matches: &[bool]) {
    println!("Node '{}' matches:", node.tag_name);
    for (i, &matched) in matches.iter().enumerate() {
        if matched {
            println!("  Rule {}: {}", i, get_rule_name(i));
        }
    }
}
pub fn get_total_rules() -> usize {
    32 // Total number of CSS rules
}
