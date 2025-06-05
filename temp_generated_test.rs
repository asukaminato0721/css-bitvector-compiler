
// Generated file - do not format manually
use std::collections::HashSet;

// Copy necessary types and structs
#[derive(Debug, Clone)]
struct HtmlNode {
    tag_name: String,
    id: Option<String>,
    classes: HashSet<String>,
    children: Vec<HtmlNode>,
    css_match_bitvector: u64,
}

impl HtmlNode {
    fn new(tag_name: &str) -> Self {
        HtmlNode {
            tag_name: tag_name.to_lowercase(),
            id: None,
            classes: HashSet::new(),
            children: Vec::new(),
            css_match_bitvector: 0,
        }
    }

    fn with_id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    fn with_class(mut self, class: &str) -> Self {
        self.classes.insert(class.to_string());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum SimpleSelector {
    Type(String),
    Class(String),
    Id(String),
}

// Generated Tree NFA Program
// This program processes HTML nodes and computes CSS matches

fn process_node_generated(
    node: &HtmlNode,
    parent_state: u64,
) -> (u64, u64) { // (current_matches, child_states)
    let mut current_matches: u64 = 0;
    let mut child_states: u64 = 0;

    // Instruction 0: CheckAndSetBit { selector: Type("div"), bit_pos: 0 }
    if node_matches_selector(node, &SimpleSelector::Type("div".to_string())) {
        current_matches |= 1 << 0; // match_Type("div")
    }

    // Instruction 1: PropagateToChildren { match_bit: 0, active_bit: 1 }
    if (current_matches & (1 << 0)) != 0 {
        child_states |= 1 << 1; // active_Type("div")
    }

    // Instruction 2: CheckAndSetBit { selector: Class("item"), bit_pos: 2 }
    if node_matches_selector(node, &SimpleSelector::Class("item".to_string())) {
        current_matches |= 1 << 2; // match_Class("item")
    }

    // Instruction 3: PropagateToChildren { match_bit: 2, active_bit: 3 }
    if (current_matches & (1 << 2)) != 0 {
        child_states |= 1 << 3; // active_Class("item")
    }

    // Instruction 4: CheckAndSetBit { selector: Id("specific"), bit_pos: 4 }
    if node_matches_selector(node, &SimpleSelector::Id("specific".to_string())) {
        current_matches |= 1 << 4; // match_Id("specific")
    }

    // Instruction 5: PropagateToChildren { match_bit: 4, active_bit: 5 }
    if (current_matches & (1 << 4)) != 0 {
        child_states |= 1 << 5; // active_Id("specific")
    }

    // Instruction 6: CheckParentAndSetBit { parent_state_bit: 1, child_selector: Class("item"), result_bit: 6 }
    if (parent_state & (1 << 1)) != 0 && node_matches_selector(node, &SimpleSelector::Class("item".to_string())) {
        current_matches |= 1 << 6; // match_Type("div")_gt_Class("item")
    }

    (current_matches, child_states)
}

fn node_matches_selector(node: &HtmlNode, selector: &SimpleSelector) -> bool {
    match selector {
        SimpleSelector::Type(tag) => node.tag_name == *tag,
        SimpleSelector::Class(class) => node.classes.contains(class),
        SimpleSelector::Id(id) => node.id.as_deref() == Some(id),
    }
}


fn main() {
    // Test case 1: div node
    let div_node = HtmlNode::new("div").with_id("test").with_class("item");
    let (matches, child_states) = process_node_generated(&div_node, 0);
    println!("div.item#test - matches: {:016b}, child_states: {:016b}", matches, child_states);
    
    // Test case 2: span node with class
    let span_node = HtmlNode::new("span").with_class("item");
    let (matches2, child_states2) = process_node_generated(&span_node, child_states);
    println!("span.item (child of div) - matches: {:016b}, child_states: {:016b}", matches2, child_states2);
    
    // Test case 3: node with specific id
    let specific_node = HtmlNode::new("p").with_id("specific");
    let (matches3, child_states3) = process_node_generated(&specific_node, 0);
    println!("p#specific - matches: {:016b}, child_states: {:016b}", matches3, child_states3);
    
    // Test case 4: child selector test
    let item_node = HtmlNode::new("div").with_class("item");
    let (matches4, child_states4) = process_node_generated(&item_node, 0);
    let specific_child = HtmlNode::new("span").with_id("specific");
    let (matches5, _) = process_node_generated(&specific_child, child_states4);
    println!("div.item parent - matches: {:016b}, child_states: {:016b}", matches4, child_states4);
    println!("span#specific (under div.item) - matches: {:016b}", matches5);
    
    println!("SUCCESS: Generated Rust code executed successfully!");
}
