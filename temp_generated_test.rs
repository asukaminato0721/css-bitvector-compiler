
// Generated file - do not format manually
use std::collections::HashSet;

// Copy necessary types and structs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BitVector {
    bits: u64,
}

impl BitVector {
    fn new() -> Self {
        BitVector { bits: 0 }
    }

    fn from_u64(bits: u64) -> Self {
        BitVector { bits }
    }

    fn set_bit(&mut self, pos: usize) {
        self.bits |= 1 << pos;
    }

    fn is_bit_set(&self, pos: usize) -> bool {
        (self.bits & (1 << pos)) != 0
    }

    fn is_empty(&self) -> bool {
        self.bits == 0
    }

    fn as_u64(&self) -> u64 {
        self.bits
    }
}

impl std::fmt::Binary for BitVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:016b}", self.bits)
    }
}

#[derive(Debug, Clone)]
struct HtmlNode {
    tag_name: String,
    id: Option<String>,
    classes: HashSet<String>,
    children: Vec<HtmlNode>,
    css_match_bitvector: BitVector,
    
    // Incremental processing state
    cached_parent_state: Option<BitVector>,     // Input: parent state from last computation
    cached_node_intrinsic: Option<BitVector>,   // Input: node's own selector matches (computed once)
    cached_child_states: Option<BitVector>,     // Output: states to propagate to children
    is_dirty: bool,                             // Whether this node needs recomputation
}

impl HtmlNode {
    fn new(tag_name: &str) -> Self {
        HtmlNode {
            tag_name: tag_name.to_lowercase(),
            id: None,
            classes: HashSet::new(),
            children: Vec::new(),
            css_match_bitvector: BitVector::new(),
            cached_parent_state: None,
            cached_node_intrinsic: None,
            cached_child_states: None,
            is_dirty: true,
        }
    }

    fn with_id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self.mark_dirty(); // Changing attributes makes node dirty
        self
    }

    fn with_class(mut self, class: &str) -> Self {
        self.classes.insert(class.to_string());
        self.mark_dirty(); // Changing attributes makes node dirty
        self
    }

    fn add_child(mut self, child: HtmlNode) -> Self {
        self.children.push(child);
        self
    }
    
    // Mark this node and all descendants as needing recomputation
    fn mark_dirty(&mut self) {
        self.is_dirty = true;
        self.cached_parent_state = None;
        self.cached_node_intrinsic = None;
        self.cached_child_states = None;
        
        // Propagate dirtiness to children since parent state will change
        for child in &mut self.children {
            child.mark_dirty();
        }
    }
    
    // Check if inputs have changed and we need to recompute
    fn needs_recomputation(&self, new_parent_state: BitVector) -> bool {
        self.is_dirty || 
        self.cached_parent_state.is_none() ||
        self.cached_parent_state.unwrap() != new_parent_state
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

fn process_node_generated_inplace(
    node: &mut HtmlNode,
    parent_state: BitVector,
) -> BitVector { // returns child_states
    let mut current_matches = BitVector::new();
    let mut child_states = BitVector::new();

    // Instruction 0: CheckAndSetBit { selector: Type("div"), bit_pos: 0 }
    if node_matches_selector(node, &SimpleSelector::Type("div".to_string())) {
        current_matches.set_bit(0); // match_Type("div")
    }

    // Instruction 1: PropagateToChildren { match_bit: 0, active_bit: 1 }
    if current_matches.is_bit_set(0) {
        child_states.set_bit(1); // active_Type("div")
    }

    // Instruction 2: CheckAndSetBit { selector: Class("item"), bit_pos: 2 }
    if node_matches_selector(node, &SimpleSelector::Class("item".to_string())) {
        current_matches.set_bit(2); // match_Class("item")
    }

    // Instruction 3: PropagateToChildren { match_bit: 2, active_bit: 3 }
    if current_matches.is_bit_set(2) {
        child_states.set_bit(3); // active_Class("item")
    }

    // Instruction 4: CheckAndSetBit { selector: Id("specific"), bit_pos: 4 }
    if node_matches_selector(node, &SimpleSelector::Id("specific".to_string())) {
        current_matches.set_bit(4); // match_Id("specific")
    }

    // Instruction 5: PropagateToChildren { match_bit: 4, active_bit: 5 }
    if current_matches.is_bit_set(4) {
        child_states.set_bit(5); // active_Id("specific")
    }

    // Instruction 6: CheckParentAndSetBit { parent_state_bit: 1, child_selector: Class("item"), result_bit: 6 }
    if parent_state.is_bit_set(1) && node_matches_selector(node, &SimpleSelector::Class("item".to_string())) {
        current_matches.set_bit(6); // match_Type("div")_gt_Class("item")
    }

    // Store result in node (in-place)
    node.css_match_bitvector = current_matches;
    child_states
}

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

fn process_tree_generated(root: &mut HtmlNode) {
    process_tree_recursive_generated(root, BitVector::new());
}

fn process_tree_recursive_generated(node: &mut HtmlNode, parent_state: BitVector) {
    let child_states = process_node_generated_inplace(node, parent_state);
    
    // Recursively process children
    for child in node.children.iter_mut() {
        process_tree_recursive_generated(child, child_states);
    }
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
    let mut div_node = HtmlNode::new("div").with_id("test").with_class("item");
    let child_states = process_node_generated_inplace(&mut div_node, BitVector::new());
    println!("div.item#test - matches: {:b}, child_states: {:b}", div_node.css_match_bitvector, child_states);
    
    // Test case 2: span node with class
    let mut span_node = HtmlNode::new("span").with_class("item");
    let child_states2 = process_node_generated_inplace(&mut span_node, child_states);
    println!("span.item (child of div) - matches: {:b}, child_states: {:b}", span_node.css_match_bitvector, child_states2);
    
    // Test case 3: node with specific id
    let mut specific_node = HtmlNode::new("p").with_id("specific");
    let child_states3 = process_node_generated_inplace(&mut specific_node, BitVector::new());
    println!("p#specific - matches: {:b}, child_states: {:b}", specific_node.css_match_bitvector, child_states3);
    
    // Test case 4: driver function test
    let mut tree = HtmlNode::new("div")
        .with_class("item")
        .add_child(HtmlNode::new("span").with_id("specific"));
    
    println!("\nTesting tree processing...");
    process_tree_generated(&mut tree);
    
    fn print_tree_results(node: &HtmlNode, depth: usize) {
        let indent = "  ".repeat(depth);
        println!("{}{} (matches: {:b})", indent, node.tag_name, node.css_match_bitvector);
        for child in &node.children {
            print_tree_results(child, depth + 1);
        }
    }
    
    print_tree_results(&tree, 0);
    
    println!("SUCCESS: Generated Rust code executed successfully!");
}
