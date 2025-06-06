
// Generated Google Trace Test Program
use std::collections::HashSet;

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
    
    // Double Dirty Bit Algorithm state
    is_self_dirty: bool,
    has_dirty_descendant: bool,
    
    // Incremental processing cache
    cached_parent_state: Option<BitVector>,
    cached_node_intrinsic: Option<BitVector>,
    cached_child_states: Option<BitVector>,
}

impl HtmlNode {
    fn new(tag_name: &str) -> Self {
        HtmlNode {
            tag_name: tag_name.to_string(),
            id: None,
            classes: HashSet::new(),
            children: Vec::new(),
            css_match_bitvector: BitVector::new(),
            is_self_dirty: true,
            has_dirty_descendant: false,
            cached_parent_state: None,
            cached_node_intrinsic: None,
            cached_child_states: None,
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

    fn add_child(mut self, child: HtmlNode) -> Self {
        self.children.push(child);
        self
    }

    fn needs_any_recomputation(&self, new_parent_state: BitVector) -> bool {
        self.is_self_dirty ||
        self.has_dirty_descendant ||
        self.cached_parent_state.is_none() ||
        self.cached_parent_state.unwrap().as_u64() != new_parent_state.as_u64()
    }

    fn mark_clean(&mut self) {
        self.is_self_dirty = false;
        self.has_dirty_descendant = false;
    }

    fn mark_self_dirty(&mut self) {
        self.is_self_dirty = true;
        // Also invalidate cached intrinsic matches
        self.cached_node_intrinsic = None;
    }
}

#[derive(Debug, Clone, PartialEq)]
enum SimpleSelector {
    Type(String),
    Class(String),
    Id(String),
}

// Generated Tree NFA Program with Incremental Processing
// This program processes HTML nodes and computes CSS matches with caching

fn process_node_generated_incremental(
    node: &mut HtmlNode,
    parent_state: BitVector,
) -> BitVector { // returns child_states
    // Check if we need to recompute
    if !node.needs_any_recomputation(parent_state) {
        // Return cached result - entire subtree can be skipped
        return node.cached_child_states.unwrap_or(BitVector::new());
    }

    // Recompute node intrinsic matches if needed
    if node.cached_node_intrinsic.is_none() || node.is_self_dirty {
        let mut intrinsic_matches = BitVector::new();

        // Instruction 0: CheckAndSetBit { selector: Type("div"), bit_pos: 0 }
        if node_matches_selector_generated(node, &SimpleSelector::Type("div".to_string())) {
            intrinsic_matches.set_bit(0); // match_Type("div")
        }

        // Instruction 2: CheckAndSetBit { selector: Type("span"), bit_pos: 2 }
        if node_matches_selector_generated(node, &SimpleSelector::Type("span".to_string())) {
            intrinsic_matches.set_bit(2); // match_Type("span")
        }

        // Instruction 4: CheckAndSetBit { selector: Type("a"), bit_pos: 4 }
        if node_matches_selector_generated(node, &SimpleSelector::Type("a".to_string())) {
            intrinsic_matches.set_bit(4); // match_Type("a")
        }

        // Instruction 6: CheckAndSetBit { selector: Type("input"), bit_pos: 6 }
        if node_matches_selector_generated(node, &SimpleSelector::Type("input".to_string())) {
            intrinsic_matches.set_bit(6); // match_Type("input")
        }

        // Instruction 8: CheckAndSetBit { selector: Class("gbts"), bit_pos: 8 }
        if node_matches_selector_generated(node, &SimpleSelector::Class("gbts".to_string())) {
            intrinsic_matches.set_bit(8); // match_Class("gbts")
        }

        // Instruction 10: CheckAndSetBit { selector: Class("gbmt"), bit_pos: 10 }
        if node_matches_selector_generated(node, &SimpleSelector::Class("gbmt".to_string())) {
            intrinsic_matches.set_bit(10); // match_Class("gbmt")
        }

        // Instruction 12: CheckAndSetBit { selector: Class("lsb"), bit_pos: 12 }
        if node_matches_selector_generated(node, &SimpleSelector::Class("lsb".to_string())) {
            intrinsic_matches.set_bit(12); // match_Class("lsb")
        }

        // Instruction 14: CheckAndSetBit { selector: Id("gb"), bit_pos: 14 }
        if node_matches_selector_generated(node, &SimpleSelector::Id("gb".to_string())) {
            intrinsic_matches.set_bit(14); // match_Id("gb")
        }

        // Instruction 16: CheckAndSetBit { selector: Id("gbz"), bit_pos: 16 }
        if node_matches_selector_generated(node, &SimpleSelector::Id("gbz".to_string())) {
            intrinsic_matches.set_bit(16); // match_Id("gbz")
        }

        node.cached_node_intrinsic = Some(intrinsic_matches);
    }

    // Start with cached intrinsic matches
    let mut current_matches = node.cached_node_intrinsic.unwrap();
    let mut child_states = BitVector::new();

    // Optimized selector matching using hash tables (conceptual)
    // In practice, rules would be pre-indexed by tag/class/id

    // Apply parent-dependent rules
    // Instruction 1: PropagateToChildren { match_bit: 0, active_bit: 1 }
    if current_matches.is_bit_set(0) {
        child_states.set_bit(1); // active_Type("div")
    }

    // Instruction 3: PropagateToChildren { match_bit: 2, active_bit: 3 }
    if current_matches.is_bit_set(2) {
        child_states.set_bit(3); // active_Type("span")
    }

    // Instruction 5: PropagateToChildren { match_bit: 4, active_bit: 5 }
    if current_matches.is_bit_set(4) {
        child_states.set_bit(5); // active_Type("a")
    }

    // Instruction 7: PropagateToChildren { match_bit: 6, active_bit: 7 }
    if current_matches.is_bit_set(6) {
        child_states.set_bit(7); // active_Type("input")
    }

    // Instruction 9: PropagateToChildren { match_bit: 8, active_bit: 9 }
    if current_matches.is_bit_set(8) {
        child_states.set_bit(9); // active_Class("gbts")
    }

    // Instruction 11: PropagateToChildren { match_bit: 10, active_bit: 11 }
    if current_matches.is_bit_set(10) {
        child_states.set_bit(11); // active_Class("gbmt")
    }

    // Instruction 13: PropagateToChildren { match_bit: 12, active_bit: 13 }
    if current_matches.is_bit_set(12) {
        child_states.set_bit(13); // active_Class("lsb")
    }

    // Instruction 15: PropagateToChildren { match_bit: 14, active_bit: 15 }
    if current_matches.is_bit_set(14) {
        child_states.set_bit(15); // active_Id("gb")
    }

    // Instruction 17: PropagateToChildren { match_bit: 16, active_bit: 17 }
    if current_matches.is_bit_set(16) {
        child_states.set_bit(17); // active_Id("gbz")
    }

    // Cache results
    node.css_match_bitvector = current_matches;
    node.cached_parent_state = Some(parent_state);
    node.cached_child_states = Some(child_states);
    node.mark_clean();

    child_states
}

fn process_node_generated_inplace(
    node: &mut HtmlNode,
    parent_state: BitVector,
) -> BitVector { // returns child_states
    let mut current_matches = BitVector::new();
    let mut child_states = BitVector::new();

    // Instruction 0: CheckAndSetBit { selector: Type("div"), bit_pos: 0 }
    if node_matches_selector_generated(node, &SimpleSelector::Type("div".to_string())) {
        current_matches.set_bit(0); // match_Type("div")
    }

    // Instruction 1: PropagateToChildren { match_bit: 0, active_bit: 1 }
    if current_matches.is_bit_set(0) {
        child_states.set_bit(1); // active_Type("div")
    }

    // Instruction 2: CheckAndSetBit { selector: Type("span"), bit_pos: 2 }
    if node_matches_selector_generated(node, &SimpleSelector::Type("span".to_string())) {
        current_matches.set_bit(2); // match_Type("span")
    }

    // Instruction 3: PropagateToChildren { match_bit: 2, active_bit: 3 }
    if current_matches.is_bit_set(2) {
        child_states.set_bit(3); // active_Type("span")
    }

    // Instruction 4: CheckAndSetBit { selector: Type("a"), bit_pos: 4 }
    if node_matches_selector_generated(node, &SimpleSelector::Type("a".to_string())) {
        current_matches.set_bit(4); // match_Type("a")
    }

    // Instruction 5: PropagateToChildren { match_bit: 4, active_bit: 5 }
    if current_matches.is_bit_set(4) {
        child_states.set_bit(5); // active_Type("a")
    }

    // Instruction 6: CheckAndSetBit { selector: Type("input"), bit_pos: 6 }
    if node_matches_selector_generated(node, &SimpleSelector::Type("input".to_string())) {
        current_matches.set_bit(6); // match_Type("input")
    }

    // Instruction 7: PropagateToChildren { match_bit: 6, active_bit: 7 }
    if current_matches.is_bit_set(6) {
        child_states.set_bit(7); // active_Type("input")
    }

    // Instruction 8: CheckAndSetBit { selector: Class("gbts"), bit_pos: 8 }
    if node_matches_selector_generated(node, &SimpleSelector::Class("gbts".to_string())) {
        current_matches.set_bit(8); // match_Class("gbts")
    }

    // Instruction 9: PropagateToChildren { match_bit: 8, active_bit: 9 }
    if current_matches.is_bit_set(8) {
        child_states.set_bit(9); // active_Class("gbts")
    }

    // Instruction 10: CheckAndSetBit { selector: Class("gbmt"), bit_pos: 10 }
    if node_matches_selector_generated(node, &SimpleSelector::Class("gbmt".to_string())) {
        current_matches.set_bit(10); // match_Class("gbmt")
    }

    // Instruction 11: PropagateToChildren { match_bit: 10, active_bit: 11 }
    if current_matches.is_bit_set(10) {
        child_states.set_bit(11); // active_Class("gbmt")
    }

    // Instruction 12: CheckAndSetBit { selector: Class("lsb"), bit_pos: 12 }
    if node_matches_selector_generated(node, &SimpleSelector::Class("lsb".to_string())) {
        current_matches.set_bit(12); // match_Class("lsb")
    }

    // Instruction 13: PropagateToChildren { match_bit: 12, active_bit: 13 }
    if current_matches.is_bit_set(12) {
        child_states.set_bit(13); // active_Class("lsb")
    }

    // Instruction 14: CheckAndSetBit { selector: Id("gb"), bit_pos: 14 }
    if node_matches_selector_generated(node, &SimpleSelector::Id("gb".to_string())) {
        current_matches.set_bit(14); // match_Id("gb")
    }

    // Instruction 15: PropagateToChildren { match_bit: 14, active_bit: 15 }
    if current_matches.is_bit_set(14) {
        child_states.set_bit(15); // active_Id("gb")
    }

    // Instruction 16: CheckAndSetBit { selector: Id("gbz"), bit_pos: 16 }
    if node_matches_selector_generated(node, &SimpleSelector::Id("gbz".to_string())) {
        current_matches.set_bit(16); // match_Id("gbz")
    }

    // Instruction 17: PropagateToChildren { match_bit: 16, active_bit: 17 }
    if current_matches.is_bit_set(16) {
        child_states.set_bit(17); // active_Id("gbz")
    }

    // Store result in node (in-place)
    node.css_match_bitvector = current_matches;
    child_states
}

fn needs_recomputation_generated(node: &HtmlNode, new_parent_state: BitVector) -> bool {
    node.is_self_dirty ||
    node.has_dirty_descendant ||
    node.cached_parent_state.is_none() ||
    node.cached_parent_state.unwrap() != new_parent_state
}

fn process_tree_generated_incremental(root: &mut HtmlNode) {
    process_tree_recursive_generated_incremental(root, BitVector::new());
}

fn process_tree_recursive_generated_incremental(node: &mut HtmlNode, parent_state: BitVector) {
    let child_states = process_node_generated_incremental(node, parent_state);
    
    // Recursively process children
    for child in node.children.iter_mut() {
        process_tree_recursive_generated_incremental(child, child_states);
    }
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

fn node_matches_selector_generated(node: &HtmlNode, selector: &SimpleSelector) -> bool {
    match selector {
        SimpleSelector::Type(tag) => node.tag_name == *tag,
        SimpleSelector::Class(class) => node.classes.contains(class),
        SimpleSelector::Id(id) => node.id.as_deref() == Some(id),
    }
}


fn count_matches(node: &HtmlNode) -> usize {
    let current = if node.css_match_bitvector.as_u64() != 0 { 1 } else { 0 };
    current + node.children.iter().map(|child| count_matches(child)).sum::<usize>()
}

fn count_total_nodes(node: &HtmlNode) -> usize {
    1 + node.children.iter().map(|child| count_total_nodes(child)).sum::<usize>()
}

fn process_tree_with_stats(root: &mut HtmlNode) -> (usize, usize, usize) {
    let mut total_nodes = 0;
    let mut cache_hits = 0;
    let mut cache_misses = 0;
    
    fn process_recursive(node: &mut HtmlNode, parent_state: BitVector, stats: &mut (usize, usize, usize)) {
        stats.0 += 1; // total_nodes
        
        let was_cached = !node.needs_any_recomputation(parent_state);
        if was_cached {
            stats.1 += 1; // cache_hits
            return; // Skip processing - use cached result
        } else {
            stats.2 += 1; // cache_misses
        }
        
        let child_states = process_node_generated_incremental(node, parent_state);
        
        // Process all children - they need to be processed at least once
        for child in node.children.iter_mut() {
            process_recursive(child, child_states, stats);
        }
    }
    
    let mut stats = (0, 0, 0);
    process_recursive(root, BitVector::new(), &mut stats);
    stats
}

fn find_deep_node(node: &mut HtmlNode, target_depth: usize) -> Option<&mut HtmlNode> {
    if target_depth == 0 {
        return Some(node);
    }
    
    for child in &mut node.children {
        if let Some(found) = find_deep_node(child, target_depth - 1) {
            return Some(found);
        }
    }
    
    None
}

fn main() {
    println!("🚀 CodeGen Google Trace Performance Test\n");
    
    // Create the Google DOM tree
    let mut root =     HtmlNode::new("#document").with_id("4")
    .add_child(
        HtmlNode::new("html").with_id("5")
    )
    .add_child(
        HtmlNode::new("HTML").with_id("6")
        .add_child(
            HtmlNode::new("HEAD").with_id("10")
            .add_child(
                HtmlNode::new("META").with_id("11")
            )
            .add_child(
                HtmlNode::new("META").with_id("14")
            )
            .add_child(
                HtmlNode::new("META").with_id("17")
            )
            .add_child(
                HtmlNode::new("META").with_id("20")
            )
            .add_child(
                HtmlNode::new("TITLE").with_id("23")
                .add_child(
                    HtmlNode::new("#text").with_id("24")
                )
            )
            .add_child(
                HtmlNode::new("SCRIPT").with_id("25")
                .add_child(
                    HtmlNode::new("#text").with_id("27")
                )
            )
            .add_child(
                HtmlNode::new("STYLE").with_id("28")
                .add_child(
                    HtmlNode::new("#text").with_id("29")
                )
            )
            .add_child(
                HtmlNode::new("STYLE").with_id("30")
                .add_child(
                    HtmlNode::new("#text").with_id("31")
                )
            )
            .add_child(
                HtmlNode::new("SCRIPT").with_id("32")
                .add_child(
                    HtmlNode::new("#text").with_id("34")
                )
            )
        )
        .add_child(
            HtmlNode::new("BODY").with_id("35")
            .add_child(
                HtmlNode::new("SCRIPT").with_id("37")
                .add_child(
                    HtmlNode::new("#text").with_id("39")
                )
            )
            .add_child(
                HtmlNode::new("DIV").with_id("40")
                .add_child(
                    HtmlNode::new("DIV").with_id("42")
                    .add_child(
                        HtmlNode::new("SCRIPT").with_id("44")
                        .add_child(
                            HtmlNode::new("#text").with_id("46")
                        )
                    )
                    .add_child(
                        HtmlNode::new("DIV").with_id("47")
                        .add_child(
                            HtmlNode::new("DIV").with_id("49")
                            .add_child(
                                HtmlNode::new("SPAN").with_id("51").with_class("gbtcb")
                            )
                            .add_child(
                                HtmlNode::new("OL").with_id("53").with_class("gbtc")
                                .add_child(
                                    HtmlNode::new("LI").with_id("56").with_class("gbt")
                                    .add_child(
                                        HtmlNode::new("A").with_id("58").with_class("gbzt").with_class("gbz0l").with_class("gbp1")
                                        .add_child(
                                            HtmlNode::new("SPAN").with_id("62").with_class("gbtb2")
                                        )
                                        .add_child(
                                            HtmlNode::new("SPAN").with_id("64").with_class("gbts")
                                            .add_child(
                                                HtmlNode::new("#text").with_id("66")
                                            )
                                        )
                                    )
                                )
                                .add_child(
                                    HtmlNode::new("LI").with_id("67").with_class("gbt")
                                    .add_child(
                                        HtmlNode::new("A").with_id("69").with_class("gbzt")
                                        .add_child(
                                            HtmlNode::new("SPAN").with_id("73").with_class("gbtb2")
                                        )
                                        .add_child(
                                            HtmlNode::new("SPAN").with_id("75").with_class("gbts")
                                            .add_child(
                                                HtmlNode::new("#text").with_id("77")
                                            )
                                        )
                                    )
                                )
                                .add_child(
                                    HtmlNode::new("LI").with_id("78").with_class("gbt")
                                    .add_child(
                                        HtmlNode::new("A").with_id("80").with_class("gbzt")
                                        .add_child(
                                            HtmlNode::new("SPAN").with_id("84").with_class("gbtb2")
                                        )
                                        .add_child(
                                            HtmlNode::new("SPAN").with_id("86").with_class("gbts")
                                            .add_child(
                                                HtmlNode::new("#text").with_id("88")
                                            )
                                        )
                                    )
                                )
                                .add_child(
                                    HtmlNode::new("LI").with_id("89").with_class("gbt")
                                    .add_child(
                                        HtmlNode::new("A").with_id("91").with_class("gbzt")
                                        .add_child(
                                            HtmlNode::new("SPAN").with_id("95").with_class("gbtb2")
                                        )
                                        .add_child(
                                            HtmlNode::new("SPAN").with_id("97").with_class("gbts")
                                            .add_child(
                                                HtmlNode::new("#text").with_id("99")
                                            )
                                        )
                                    )
                                )
                                .add_child(
                                    HtmlNode::new("LI").with_id("100").with_class("gbt")
                                    .add_child(
                                        HtmlNode::new("A").with_id("102").with_class("gbzt")
                                        .add_child(
                                            HtmlNode::new("SPAN").with_id("106").with_class("gbtb2")
                                        )
                                        .add_child(
                                            HtmlNode::new("SPAN").with_id("108").with_class("gbts")
                                            .add_child(
                                                HtmlNode::new("#text").with_id("110")
                                            )
                                        )
                                    )
                                )
                                .add_child(
                                    HtmlNode::new("LI").with_id("111").with_class("gbt")
                                    .add_child(
                                        HtmlNode::new("A").with_id("113").with_class("gbzt")
                                        .add_child(
                                            HtmlNode::new("SPAN").with_id("117").with_class("gbtb2")
                                        )
                                        .add_child(
                                            HtmlNode::new("SPAN").with_id("119").with_class("gbts")
                                            .add_child(
                                                HtmlNode::new("#text").with_id("121")
                                            )
                                        )
                                    )
                                )
                                .add_child(
                                    HtmlNode::new("LI").with_id("122").with_class("gbt")
                                    .add_child(
                                        HtmlNode::new("A").with_id("124").with_class("gbzt")
                                        .add_child(
                                            HtmlNode::new("SPAN").with_id("128").with_class("gbtb2")
                                        )
                                        .add_child(
                                            HtmlNode::new("SPAN").with_id("130").with_class("gbts")
                                            .add_child(
                                                HtmlNode::new("#text").with_id("132")
                                            )
                                        )
                                    )
                                )
                                .add_child(
                                    HtmlNode::new("LI").with_id("133").with_class("gbt")
                                    .add_child(
                                        HtmlNode::new("A").with_id("135").with_class("gbzt")
                                        .add_child(
                                            HtmlNode::new("SPAN").with_id("139").with_class("gbtb2")
                                        )
                                        .add_child(
                                            HtmlNode::new("SPAN").with_id("141").with_class("gbts")
                                            .add_child(
                                                HtmlNode::new("#text").with_id("143")
                                            )
                                        )
                                    )
                                )
                                .add_child(
                                    HtmlNode::new("LI").with_id("144").with_class("gbt")
                                    .add_child(
                                        HtmlNode::new("A").with_id("146").with_class("gbgt")
                                        .add_child(
                                            HtmlNode::new("SPAN").with_id("152").with_class("gbtb2")
                                        )
                                        .add_child(
                                            HtmlNode::new("SPAN").with_id("154").with_class("gbts").with_class("gbtsa")
                                            .add_child(
                                                HtmlNode::new("SPAN").with_id("157")
                                                .add_child(
                                                    HtmlNode::new("#text").with_id("159")
                                                )
                                            )
                                            .add_child(
                                                HtmlNode::new("SPAN").with_id("160").with_class("gbma")
                                            )
                                        )
                                    )
                                    .add_child(
                                        HtmlNode::new("SCRIPT").with_id("162")
                                        .add_child(
                                            HtmlNode::new("#text").with_id("164")
                                        )
                                    )
                                    .add_child(
                                        HtmlNode::new("DIV").with_id("165").with_class("gbm")
                                        .add_child(
                                            HtmlNode::new("DIV").with_id("169").with_class("gbmc").with_class("gbsb").with_class("gbsbis")
                                            .add_child(
                                                HtmlNode::new("OL").with_id("172").with_class("gbmcc").with_class("gbsbic")
                                                .add_child(
                                                    HtmlNode::new("LI").with_id("175").with_class("gbmtc")
                                                    .add_child(
                                                        HtmlNode::new("A").with_id("177").with_class("gbmt")
                                                        .add_child(
                                                            HtmlNode::new("#text").with_id("181")
                                                        )
                                                    )
                                                )
                                                .add_child(
                                                    HtmlNode::new("LI").with_id("182").with_class("gbmtc")
                                                    .add_child(
                                                        HtmlNode::new("A").with_id("184").with_class("gbmt")
                                                        .add_child(
                                                            HtmlNode::new("#text").with_id("188")
                                                        )
                                                    )
                                                )
                                                .add_child(
                                                    HtmlNode::new("LI").with_id("189").with_class("gbmtc")
                                                    .add_child(
                                                        HtmlNode::new("A").with_id("191").with_class("gbmt")
                                                        .add_child(
                                                            HtmlNode::new("#text").with_id("195")
                                                        )
                                                    )
                                                )
                                                .add_child(
                                                    HtmlNode::new("LI").with_id("196").with_class("gbmtc")
                                                    .add_child(
                                                        HtmlNode::new("A").with_id("198").with_class("gbmt")
                                                        .add_child(
                                                            HtmlNode::new("#text").with_id("202")
                                                        )
                                                    )
                                                )
                                                .add_child(
                                                    HtmlNode::new("LI").with_id("203").with_class("gbmtc")
                                                    .add_child(
                                                        HtmlNode::new("A").with_id("205").with_class("gbmt")
                                                        .add_child(
                                                            HtmlNode::new("#text").with_id("209")
                                                        )
                                                    )
                                                )
                                                .add_child(
                                                    HtmlNode::new("LI").with_id("210").with_class("gbmtc")
                                                    .add_child(
                                                        HtmlNode::new("A").with_id("212").with_class("gbmt")
                                                        .add_child(
                                                            HtmlNode::new("#text").with_id("216")
                                                        )
                                                    )
                                                )
                                                .add_child(
                                                    HtmlNode::new("LI").with_id("217").with_class("gbmtc")
                                                    .add_child(
                                                        HtmlNode::new("A").with_id("219").with_class("gbmt")
                                                        .add_child(
                                                            HtmlNode::new("#text").with_id("223")
                                                        )
                                                    )
                                                )
                                                .add_child(
                                                    HtmlNode::new("LI").with_id("224").with_class("gbmtc")
                                                    .add_child(
                                                        HtmlNode::new("A").with_id("226").with_class("gbmt")
                                                        .add_child(
                                                            HtmlNode::new("#text").with_id("230")
                                                        )
                                                    )
                                                )
                                                .add_child(
                                                    HtmlNode::new("LI").with_id("231").with_class("gbmtc")
                                                    .add_child(
                                                        HtmlNode::new("DIV").with_id("233").with_class("gbmt").with_class("gbmh")
                                                    )
                                                )
                                                .add_child(
                                                    HtmlNode::new("LI").with_id("235").with_class("gbmtc")
                                                    .add_child(
                                                        HtmlNode::new("A").with_id("237").with_class("gbmt")
                                                        .add_child(
                                                            HtmlNode::new("#text").with_id("240")
                                                        )
                                                    )
                                                    .add_child(
                                                        HtmlNode::new("SCRIPT").with_id("241")
                                                        .add_child(
                                                            HtmlNode::new("#text").with_id("243")
                                                        )
                                                    )
                                                )
                                            )
                                            .add_child(
                                                HtmlNode::new("DIV").with_id("244").with_class("gbsbt")
                                            )
                                            .add_child(
                                                HtmlNode::new("DIV").with_id("246").with_class("gbsbb")
                                            )
                                        )
                                    )
                                )
                            )
                        )
                        .add_child(
                            HtmlNode::new("DIV").with_id("248")
                            .add_child(
                                HtmlNode::new("H2").with_id("250").with_class("gbxx")
                                .add_child(
                                    HtmlNode::new("#text").with_id("252")
                                )
                            )
                            .add_child(
                                HtmlNode::new("SPAN").with_id("253").with_class("gbtcb")
                            )
                            .add_child(
                                HtmlNode::new("OL").with_id("255").with_class("gbtc")
                                .add_child(
                                    HtmlNode::new("LI").with_id("257").with_class("gbt")
                                    .add_child(
                                        HtmlNode::new("A").with_id("259").with_class("gbgt")
                                        .add_child(
                                            HtmlNode::new("SPAN").with_id("265").with_class("gbtb2")
                                        )
                                        .add_child(
                                            HtmlNode::new("SPAN").with_id("267").with_class("gbts")
                                            .add_child(
                                                HtmlNode::new("SPAN").with_id("270")
                                                .add_child(
                                                    HtmlNode::new("#text").with_id("272")
                                                )
                                            )
                                        )
                                    )
                                )
                                .add_child(
                                    HtmlNode::new("LI").with_id("273").with_class("gbt").with_class("gbtb")
                                    .add_child(
                                        HtmlNode::new("SPAN").with_id("275").with_class("gbts")
                                    )
                                )
                                .add_child(
                                    HtmlNode::new("LI").with_id("277").with_class("gbt")
                                    .add_child(
                                        HtmlNode::new("A").with_id("279").with_class("gbgt")
                                        .add_child(
                                            HtmlNode::new("SPAN").with_id("286").with_class("gbtb2")
                                        )
                                        .add_child(
                                            HtmlNode::new("SPAN").with_id("288").with_class("gbts")
                                            .add_child(
                                                HtmlNode::new("SPAN").with_id("291")
                                            )
                                        )
                                    )
                                    .add_child(
                                        HtmlNode::new("SCRIPT").with_id("293")
                                        .add_child(
                                            HtmlNode::new("#text").with_id("295")
                                        )
                                    )
                                    .add_child(
                                        HtmlNode::new("DIV").with_id("296").with_class("gbm")
                                        .add_child(
                                            HtmlNode::new("DIV").with_id("300").with_class("gbmc")
                                            .add_child(
                                                HtmlNode::new("OL").with_id("302").with_class("gbmcc")
                                                .add_child(
                                                    HtmlNode::new("LI").with_id("305").with_class("gbkc").with_class("gbmtc")
                                                    .add_child(
                                                        HtmlNode::new("A").with_id("307").with_class("gbmt")
                                                        .add_child(
                                                            HtmlNode::new("#text").with_id("310")
                                                        )
                                                    )
                                                )
                                                .add_child(
                                                    HtmlNode::new("LI").with_id("311").with_class("gbmtc")
                                                    .add_child(
                                                        HtmlNode::new("DIV").with_id("313").with_class("gbmt").with_class("gbmh")
                                                    )
                                                )
                                                .add_child(
                                                    HtmlNode::new("LI").with_id("315").with_class("gbkp").with_class("gbmtc")
                                                    .add_child(
                                                        HtmlNode::new("A").with_id("317").with_class("gbmt")
                                                        .add_child(
                                                            HtmlNode::new("#text").with_id("320")
                                                        )
                                                    )
                                                )
                                            )
                                        )
                                    )
                                )
                            )
                        )
                    )
                    .add_child(
                        HtmlNode::new("DIV").with_id("321")
                    )
                    .add_child(
                        HtmlNode::new("DIV").with_id("323")
                    )
                    .add_child(
                        HtmlNode::new("SCRIPT").with_id("325")
                        .add_child(
                            HtmlNode::new("#text").with_id("327")
                        )
                    )
                )
            )
            .add_child(
                HtmlNode::new("CENTER").with_id("328")
                .add_child(
                    HtmlNode::new("BR").with_id("329")
                )
                .add_child(
                    HtmlNode::new("DIV").with_id("332")
                    .add_child(
                        HtmlNode::new("IMG").with_id("334")
                    )
                    .add_child(
                        HtmlNode::new("BR").with_id("341")
                    )
                    .add_child(
                        HtmlNode::new("BR").with_id("342")
                    )
                )
                .add_child(
                    HtmlNode::new("FORM").with_id("343")
                    .add_child(
                        HtmlNode::new("TABLE").with_id("346")
                        .add_child(
                            HtmlNode::new("TBODY").with_id("349")
                            .add_child(
                                HtmlNode::new("TR").with_id("350")
                                .add_child(
                                    HtmlNode::new("TD").with_id("352")
                                    .add_child(
                                        HtmlNode::new("#text").with_id("354")
                                    )
                                )
                                .add_child(
                                    HtmlNode::new("TD").with_id("355")
                                    .add_child(
                                        HtmlNode::new("INPUT").with_id("358")
                                    )
                                    .add_child(
                                        HtmlNode::new("INPUT").with_id("362")
                                    )
                                    .add_child(
                                        HtmlNode::new("INPUT").with_id("366")
                                    )
                                    .add_child(
                                        HtmlNode::new("INPUT").with_id("369")
                                    )
                                    .add_child(
                                        HtmlNode::new("DIV").with_id("372").with_class("ds")
                                        .add_child(
                                            HtmlNode::new("INPUT").with_id("375").with_class("lst")
                                            .add_child(
                                                HtmlNode::new("#shadow-root").with_id("384")
                                                .add_child(
                                                    HtmlNode::new("DIV").with_id("385")
                                                    .add_child(
                                                        HtmlNode::new("DIV").with_id("386")
                                                    )
                                                    .add_child(
                                                        HtmlNode::new("DIV").with_id("388")
                                                        .add_child(
                                                            HtmlNode::new("#text").with_id("389")
                                                        )
                                                    )
                                                )
                                            )
                                        )
                                    )
                                    .add_child(
                                        HtmlNode::new("BR").with_id("390")
                                    )
                                    .add_child(
                                        HtmlNode::new("SPAN").with_id("392").with_class("ds")
                                        .add_child(
                                            HtmlNode::new("SPAN").with_id("394").with_class("lsbb")
                                            .add_child(
                                                HtmlNode::new("INPUT").with_id("396").with_class("lsb")
                                                .add_child(
                                                    HtmlNode::new("#shadow-root").with_id("401")
                                                    .add_child(
                                                        HtmlNode::new("SPAN").with_id("402")
                                                        .add_child(
                                                            HtmlNode::new("#text").with_id("404")
                                                        )
                                                    )
                                                )
                                            )
                                        )
                                    )
                                    .add_child(
                                        HtmlNode::new("SPAN").with_id("405").with_class("ds")
                                        .add_child(
                                            HtmlNode::new("SPAN").with_id("407").with_class("lsbb")
                                            .add_child(
                                                HtmlNode::new("INPUT").with_id("409").with_class("lsb")
                                                .add_child(
                                                    HtmlNode::new("#shadow-root").with_id("415")
                                                    .add_child(
                                                        HtmlNode::new("SPAN").with_id("416")
                                                        .add_child(
                                                            HtmlNode::new("#text").with_id("418")
                                                        )
                                                    )
                                                )
                                            )
                                            .add_child(
                                                HtmlNode::new("SCRIPT").with_id("419")
                                                .add_child(
                                                    HtmlNode::new("#text").with_id("421")
                                                )
                                            )
                                            .add_child(
                                                HtmlNode::new("INPUT").with_id("422")
                                            )
                                        )
                                    )
                                )
                                .add_child(
                                    HtmlNode::new("TD").with_id("426").with_class("fl").with_class("sblc")
                                    .add_child(
                                        HtmlNode::new("A").with_id("431")
                                        .add_child(
                                            HtmlNode::new("#text").with_id("433")
                                        )
                                    )
                                )
                            )
                        )
                    )
                    .add_child(
                        HtmlNode::new("INPUT").with_id("434")
                    )
                    .add_child(
                        HtmlNode::new("SCRIPT").with_id("439")
                        .add_child(
                            HtmlNode::new("#text").with_id("441")
                        )
                    )
                )
                .add_child(
                    HtmlNode::new("DIV").with_id("442")
                    .add_child(
                        HtmlNode::new("BR").with_id("444")
                    )
                    .add_child(
                        HtmlNode::new("DIV").with_id("445")
                        .add_child(
                            HtmlNode::new("STYLE").with_id("447")
                            .add_child(
                                HtmlNode::new("#text").with_id("448")
                            )
                        )
                        .add_child(
                            HtmlNode::new("DIV").with_id("449")
                            .add_child(
                                HtmlNode::new("#text").with_id("451")
                            )
                            .add_child(
                                HtmlNode::new("A").with_id("452")
                                .add_child(
                                    HtmlNode::new("#text").with_id("454")
                                )
                            )
                            .add_child(
                                HtmlNode::new("#text").with_id("455")
                            )
                        )
                    )
                )
                .add_child(
                    HtmlNode::new("SPAN").with_id("456")
                    .add_child(
                        HtmlNode::new("DIV").with_id("458")
                        .add_child(
                            HtmlNode::new("DIV").with_id("460")
                            .add_child(
                                HtmlNode::new("A").with_id("463")
                                .add_child(
                                    HtmlNode::new("#text").with_id("465")
                                )
                            )
                            .add_child(
                                HtmlNode::new("A").with_id("466")
                                .add_child(
                                    HtmlNode::new("#text").with_id("468")
                                )
                            )
                            .add_child(
                                HtmlNode::new("A").with_id("469")
                                .add_child(
                                    HtmlNode::new("#text").with_id("471")
                                )
                            )
                            .add_child(
                                HtmlNode::new("A").with_id("472")
                                .add_child(
                                    HtmlNode::new("#text").with_id("474")
                                )
                            )
                        )
                    )
                    .add_child(
                        HtmlNode::new("P").with_id("475")
                        .add_child(
                            HtmlNode::new("#text").with_id("477")
                        )
                        .add_child(
                            HtmlNode::new("A").with_id("478")
                            .add_child(
                                HtmlNode::new("#text").with_id("480")
                            )
                        )
                        .add_child(
                            HtmlNode::new("#text").with_id("481")
                        )
                        .add_child(
                            HtmlNode::new("A").with_id("482")
                            .add_child(
                                HtmlNode::new("#text").with_id("484")
                            )
                        )
                    )
                )
            )
            .add_child(
                HtmlNode::new("SCRIPT").with_id("485")
                .add_child(
                    HtmlNode::new("#text").with_id("487")
                )
            )
        )
    );
    
    let total_nodes = count_total_nodes(&root);
    println!("🌳 DOM tree loaded: {} nodes", total_nodes);
    
    // Test 1: Initial processing (all cache misses expected)
    println!("\n📊 Test 1: Initial processing");
    let (total1, hits1, misses1) = process_tree_with_stats(&mut root);
    println!("  Processed nodes: {}", total1);
    println!("  Cache hits: {}", hits1);
    println!("  Cache misses: {}", misses1);
    println!("  Cache hit rate: {:.2}%", if total1 > 0 { hits1 as f64 / total1 as f64 * 100.0 } else { 0.0 });
    
    // Test 2: Second run (should have high cache hit rate)
    println!("\n📊 Test 2: Second run (cache optimization)");
    let (total2, hits2, misses2) = process_tree_with_stats(&mut root);
    println!("  Processed nodes: {}", total2);
    println!("  Cache hits: {}", hits2);
    println!("  Cache misses: {}", misses2);
    println!("  Cache hit rate: {:.2}%", if total2 > 0 { hits2 as f64 / total2 as f64 * 100.0 } else { 0.0 });
    
    // Test 3: Mark a deep node dirty and test incremental processing
    if let Some(deep_node) = find_deep_node(&mut root, 5) {
        deep_node.mark_self_dirty();
        println!("\n📝 Marked a deep node dirty...");
        
        println!("\n📊 Test 3: After deep node modification");
        let (total3, hits3, misses3) = process_tree_with_stats(&mut root);
        println!("  Processed nodes: {}", total3);
        println!("  Cache hits: {}", hits3);
        println!("  Cache misses: {}", misses3);
        println!("  Cache hit rate: {:.2}%", if total3 > 0 { hits3 as f64 / total3 as f64 * 100.0 } else { 0.0 });
        println!("  💡 Optimization: Only {} nodes needed reprocessing!", total3);
    }
    
    // Show matching results
    let matches = count_matches(&root);
    println!("\n🎯 CSS Matching Results:");
    println!("  Total nodes with matches: {} / {}", matches, total_nodes);
    println!("  Match percentage: {:.1}%", matches as f64 / total_nodes as f64 * 100.0);
    
    println!("\n✅ SUCCESS: CodeGen Google trace test completed!");
}
