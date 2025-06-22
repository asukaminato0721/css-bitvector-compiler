use css_bitvector_compiler::{BitVector, HtmlNode, SimpleSelector};

// --- Incremental Processing Functions ---
pub fn process_node_generated_incremental(
    node: &mut HtmlNode,
    parent_state: BitVector,
) -> BitVector { // returns child_states
    if !node.needs_any_recomputation(parent_state) {
        return node.cached_child_states.unwrap_or_default();
    }

    if node.cached_node_intrinsic.is_none() || node.is_self_dirty {
        let mut intrinsic_matches = BitVector::new();

        // Instruction 0: CheckAndSetBit { selector: Id("gb"), bit_pos: 0 }
        if node_matches_selector_generated(node, &SimpleSelector::Id("gb".to_string())) {
            intrinsic_matches.set_bit(0); // match_Id("gb")
        }

        // Instruction 2: CheckAndSetBit { selector: Class("gbqfb,.gbqfba,.gbqfbb"), bit_pos: 2 }
        if node_matches_selector_generated(node, &SimpleSelector::Class("gbqfb,.gbqfba,.gbqfbb".to_string())) {
            intrinsic_matches.set_bit(2); // match_Class("gbqfb,.gbqfba,.gbqfbb")
        }

        // Instruction 4: CheckAndSetBit { selector: Class("gbprcd"), bit_pos: 4 }
        if node_matches_selector_generated(node, &SimpleSelector::Class("gbprcd".to_string())) {
            intrinsic_matches.set_bit(4); // match_Class("gbprcd")
        }

        // Instruction 6: CheckAndSetBit { selector: Id("gbmpas"), bit_pos: 6 }
        if node_matches_selector_generated(node, &SimpleSelector::Id("gbmpas".to_string())) {
            intrinsic_matches.set_bit(6); // match_Id("gbmpas")
        }

        // Instruction 8: CheckAndSetBit { selector: Id("gws-output-pages-elements-homepage_additional_languages__als"), bit_pos: 8 }
        if node_matches_selector_generated(node, &SimpleSelector::Id("gws-output-pages-elements-homepage_additional_languages__als".to_string())) {
            intrinsic_matches.set_bit(8); // match_Id("gws-output-pages-elements-homepage_additional_languages__als")
        }

        // Instruction 10: CheckAndSetBit { selector: Class("gsib_a"), bit_pos: 10 }
        if node_matches_selector_generated(node, &SimpleSelector::Class("gsib_a".to_string())) {
            intrinsic_matches.set_bit(10); // match_Class("gsib_a")
        }

        // Instruction 12: CheckAndSetBit { selector: Id("gb"), bit_pos: 0 }
        if node_matches_selector_generated(node, &SimpleSelector::Id("gb".to_string())) {
            intrinsic_matches.set_bit(0); // match_Id("gb")
        }

        // Instruction 14: CheckAndSetBit { selector: Class("gbqfb,.gbqfba,.gbqfbb"), bit_pos: 2 }
        if node_matches_selector_generated(node, &SimpleSelector::Class("gbqfb,.gbqfba,.gbqfbb".to_string())) {
            intrinsic_matches.set_bit(2); // match_Class("gbqfb,.gbqfba,.gbqfbb")
        }

        // Instruction 16: CheckAndSetBit { selector: Class("gbprcd"), bit_pos: 4 }
        if node_matches_selector_generated(node, &SimpleSelector::Class("gbprcd".to_string())) {
            intrinsic_matches.set_bit(4); // match_Class("gbprcd")
        }

        // Instruction 18: CheckAndSetBit { selector: Id("gbmpas"), bit_pos: 6 }
        if node_matches_selector_generated(node, &SimpleSelector::Id("gbmpas".to_string())) {
            intrinsic_matches.set_bit(6); // match_Id("gbmpas")
        }

        // Instruction 20: CheckAndSetBit { selector: Id("gws-output-pages-elements-homepage_additional_languages__als"), bit_pos: 8 }
        if node_matches_selector_generated(node, &SimpleSelector::Id("gws-output-pages-elements-homepage_additional_languages__als".to_string())) {
            intrinsic_matches.set_bit(8); // match_Id("gws-output-pages-elements-homepage_additional_languages__als")
        }

        // Instruction 22: CheckAndSetBit { selector: Class("gsib_a"), bit_pos: 10 }
        if node_matches_selector_generated(node, &SimpleSelector::Class("gsib_a".to_string())) {
            intrinsic_matches.set_bit(10); // match_Class("gsib_a")
        }

        // Instruction 24: CheckAndSetBit { selector: Id("gb"), bit_pos: 0 }
        if node_matches_selector_generated(node, &SimpleSelector::Id("gb".to_string())) {
            intrinsic_matches.set_bit(0); // match_Id("gb")
        }

        // Instruction 26: CheckAndSetBit { selector: Class("gbqfb,.gbqfba,.gbqfbb"), bit_pos: 2 }
        if node_matches_selector_generated(node, &SimpleSelector::Class("gbqfb,.gbqfba,.gbqfbb".to_string())) {
            intrinsic_matches.set_bit(2); // match_Class("gbqfb,.gbqfba,.gbqfbb")
        }

        // Instruction 28: CheckAndSetBit { selector: Class("gbprcd"), bit_pos: 4 }
        if node_matches_selector_generated(node, &SimpleSelector::Class("gbprcd".to_string())) {
            intrinsic_matches.set_bit(4); // match_Class("gbprcd")
        }

        // Instruction 30: CheckAndSetBit { selector: Id("gbmpas"), bit_pos: 6 }
        if node_matches_selector_generated(node, &SimpleSelector::Id("gbmpas".to_string())) {
            intrinsic_matches.set_bit(6); // match_Id("gbmpas")
        }

        // Instruction 32: CheckAndSetBit { selector: Id("gws-output-pages-elements-homepage_additional_languages__als"), bit_pos: 8 }
        if node_matches_selector_generated(node, &SimpleSelector::Id("gws-output-pages-elements-homepage_additional_languages__als".to_string())) {
            intrinsic_matches.set_bit(8); // match_Id("gws-output-pages-elements-homepage_additional_languages__als")
        }

        // Instruction 34: CheckAndSetBit { selector: Class("gsib_a"), bit_pos: 10 }
        if node_matches_selector_generated(node, &SimpleSelector::Class("gsib_a".to_string())) {
            intrinsic_matches.set_bit(10); // match_Class("gsib_a")
        }

        // Instruction 36: CheckAndSetBit { selector: Type("div"), bit_pos: 12 }
        if node_matches_selector_generated(node, &SimpleSelector::Type("div".to_string())) {
            intrinsic_matches.set_bit(12); // match_Type("div")
        }

        // Instruction 38: CheckAndSetBit { selector: Type("span"), bit_pos: 14 }
        if node_matches_selector_generated(node, &SimpleSelector::Type("span".to_string())) {
            intrinsic_matches.set_bit(14); // match_Type("span")
        }

        // Instruction 40: CheckAndSetBit { selector: Type("a"), bit_pos: 16 }
        if node_matches_selector_generated(node, &SimpleSelector::Type("a".to_string())) {
            intrinsic_matches.set_bit(16); // match_Type("a")
        }

        // Instruction 42: CheckAndSetBit { selector: Type("input"), bit_pos: 18 }
        if node_matches_selector_generated(node, &SimpleSelector::Type("input".to_string())) {
            intrinsic_matches.set_bit(18); // match_Type("input")
        }

        // Instruction 44: CheckAndSetBit { selector: Class("gbts"), bit_pos: 20 }
        if node_matches_selector_generated(node, &SimpleSelector::Class("gbts".to_string())) {
            intrinsic_matches.set_bit(20); // match_Class("gbts")
        }

        // Instruction 46: CheckAndSetBit { selector: Class("gbmt"), bit_pos: 22 }
        if node_matches_selector_generated(node, &SimpleSelector::Class("gbmt".to_string())) {
            intrinsic_matches.set_bit(22); // match_Class("gbmt")
        }

        // Instruction 48: CheckAndSetBit { selector: Class("lsb"), bit_pos: 24 }
        if node_matches_selector_generated(node, &SimpleSelector::Class("lsb".to_string())) {
            intrinsic_matches.set_bit(24); // match_Class("lsb")
        }

        // Instruction 50: CheckAndSetBit { selector: Id("gb"), bit_pos: 0 }
        if node_matches_selector_generated(node, &SimpleSelector::Id("gb".to_string())) {
            intrinsic_matches.set_bit(0); // match_Id("gb")
        }

        // Instruction 52: CheckAndSetBit { selector: Id("gbz"), bit_pos: 26 }
        if node_matches_selector_generated(node, &SimpleSelector::Id("gbz".to_string())) {
            intrinsic_matches.set_bit(26); // match_Id("gbz")
        }

        node.cached_node_intrinsic = Some(intrinsic_matches);
    }

    let mut current_matches = node.cached_node_intrinsic.unwrap();
    let mut child_states = BitVector::new();
    if current_matches.is_bit_set(0) {
        child_states.set_bit(1); // active_Id("gb")
    }
    if current_matches.is_bit_set(2) {
        child_states.set_bit(3); // active_Class("gbqfb,.gbqfba,.gbqfbb")
    }
    if current_matches.is_bit_set(4) {
        child_states.set_bit(5); // active_Class("gbprcd")
    }
    if current_matches.is_bit_set(6) {
        child_states.set_bit(7); // active_Id("gbmpas")
    }
    if current_matches.is_bit_set(8) {
        child_states.set_bit(9); // active_Id("gws-output-pages-elements-homepage_additional_languages__als")
    }
    if current_matches.is_bit_set(10) {
        child_states.set_bit(11); // active_Class("gsib_a")
    }
    if current_matches.is_bit_set(0) {
        child_states.set_bit(1); // active_Id("gb")
    }
    if current_matches.is_bit_set(2) {
        child_states.set_bit(3); // active_Class("gbqfb,.gbqfba,.gbqfbb")
    }
    if current_matches.is_bit_set(4) {
        child_states.set_bit(5); // active_Class("gbprcd")
    }
    if current_matches.is_bit_set(6) {
        child_states.set_bit(7); // active_Id("gbmpas")
    }
    if current_matches.is_bit_set(8) {
        child_states.set_bit(9); // active_Id("gws-output-pages-elements-homepage_additional_languages__als")
    }
    if current_matches.is_bit_set(10) {
        child_states.set_bit(11); // active_Class("gsib_a")
    }
    if current_matches.is_bit_set(0) {
        child_states.set_bit(1); // active_Id("gb")
    }
    if current_matches.is_bit_set(2) {
        child_states.set_bit(3); // active_Class("gbqfb,.gbqfba,.gbqfbb")
    }
    if current_matches.is_bit_set(4) {
        child_states.set_bit(5); // active_Class("gbprcd")
    }
    if current_matches.is_bit_set(6) {
        child_states.set_bit(7); // active_Id("gbmpas")
    }
    if current_matches.is_bit_set(8) {
        child_states.set_bit(9); // active_Id("gws-output-pages-elements-homepage_additional_languages__als")
    }
    if current_matches.is_bit_set(10) {
        child_states.set_bit(11); // active_Class("gsib_a")
    }
    if current_matches.is_bit_set(12) {
        child_states.set_bit(13); // active_Type("div")
    }
    if current_matches.is_bit_set(14) {
        child_states.set_bit(15); // active_Type("span")
    }
    if current_matches.is_bit_set(16) {
        child_states.set_bit(17); // active_Type("a")
    }
    if current_matches.is_bit_set(18) {
        child_states.set_bit(19); // active_Type("input")
    }
    if current_matches.is_bit_set(20) {
        child_states.set_bit(21); // active_Class("gbts")
    }
    if current_matches.is_bit_set(22) {
        child_states.set_bit(23); // active_Class("gbmt")
    }
    if current_matches.is_bit_set(24) {
        child_states.set_bit(25); // active_Class("lsb")
    }
    if current_matches.is_bit_set(0) {
        child_states.set_bit(1); // active_Id("gb")
    }
    if current_matches.is_bit_set(26) {
        child_states.set_bit(27); // active_Id("gbz")
    }
    node.css_match_bitvector = current_matches;
    node.cached_parent_state = Some(parent_state);
    node.cached_child_states = Some(child_states);
    node.mark_clean();

    child_states
}

// --- From-Scratch Processing Functions ---
pub fn process_node_generated_from_scratch(
    node: &mut HtmlNode,
    parent_state: BitVector,
) -> BitVector { // returns child_states
        let mut intrinsic_matches = BitVector::new();

        // Instruction 0: CheckAndSetBit { selector: Id("gb"), bit_pos: 0 }
        if node_matches_selector_generated(node, &SimpleSelector::Id("gb".to_string())) {
            intrinsic_matches.set_bit(0); // match_Id("gb")
        }

        // Instruction 2: CheckAndSetBit { selector: Class("gbqfb,.gbqfba,.gbqfbb"), bit_pos: 2 }
        if node_matches_selector_generated(node, &SimpleSelector::Class("gbqfb,.gbqfba,.gbqfbb".to_string())) {
            intrinsic_matches.set_bit(2); // match_Class("gbqfb,.gbqfba,.gbqfbb")
        }

        // Instruction 4: CheckAndSetBit { selector: Class("gbprcd"), bit_pos: 4 }
        if node_matches_selector_generated(node, &SimpleSelector::Class("gbprcd".to_string())) {
            intrinsic_matches.set_bit(4); // match_Class("gbprcd")
        }

        // Instruction 6: CheckAndSetBit { selector: Id("gbmpas"), bit_pos: 6 }
        if node_matches_selector_generated(node, &SimpleSelector::Id("gbmpas".to_string())) {
            intrinsic_matches.set_bit(6); // match_Id("gbmpas")
        }

        // Instruction 8: CheckAndSetBit { selector: Id("gws-output-pages-elements-homepage_additional_languages__als"), bit_pos: 8 }
        if node_matches_selector_generated(node, &SimpleSelector::Id("gws-output-pages-elements-homepage_additional_languages__als".to_string())) {
            intrinsic_matches.set_bit(8); // match_Id("gws-output-pages-elements-homepage_additional_languages__als")
        }

        // Instruction 10: CheckAndSetBit { selector: Class("gsib_a"), bit_pos: 10 }
        if node_matches_selector_generated(node, &SimpleSelector::Class("gsib_a".to_string())) {
            intrinsic_matches.set_bit(10); // match_Class("gsib_a")
        }

        // Instruction 12: CheckAndSetBit { selector: Id("gb"), bit_pos: 0 }
        if node_matches_selector_generated(node, &SimpleSelector::Id("gb".to_string())) {
            intrinsic_matches.set_bit(0); // match_Id("gb")
        }

        // Instruction 14: CheckAndSetBit { selector: Class("gbqfb,.gbqfba,.gbqfbb"), bit_pos: 2 }
        if node_matches_selector_generated(node, &SimpleSelector::Class("gbqfb,.gbqfba,.gbqfbb".to_string())) {
            intrinsic_matches.set_bit(2); // match_Class("gbqfb,.gbqfba,.gbqfbb")
        }

        // Instruction 16: CheckAndSetBit { selector: Class("gbprcd"), bit_pos: 4 }
        if node_matches_selector_generated(node, &SimpleSelector::Class("gbprcd".to_string())) {
            intrinsic_matches.set_bit(4); // match_Class("gbprcd")
        }

        // Instruction 18: CheckAndSetBit { selector: Id("gbmpas"), bit_pos: 6 }
        if node_matches_selector_generated(node, &SimpleSelector::Id("gbmpas".to_string())) {
            intrinsic_matches.set_bit(6); // match_Id("gbmpas")
        }

        // Instruction 20: CheckAndSetBit { selector: Id("gws-output-pages-elements-homepage_additional_languages__als"), bit_pos: 8 }
        if node_matches_selector_generated(node, &SimpleSelector::Id("gws-output-pages-elements-homepage_additional_languages__als".to_string())) {
            intrinsic_matches.set_bit(8); // match_Id("gws-output-pages-elements-homepage_additional_languages__als")
        }

        // Instruction 22: CheckAndSetBit { selector: Class("gsib_a"), bit_pos: 10 }
        if node_matches_selector_generated(node, &SimpleSelector::Class("gsib_a".to_string())) {
            intrinsic_matches.set_bit(10); // match_Class("gsib_a")
        }

        // Instruction 24: CheckAndSetBit { selector: Id("gb"), bit_pos: 0 }
        if node_matches_selector_generated(node, &SimpleSelector::Id("gb".to_string())) {
            intrinsic_matches.set_bit(0); // match_Id("gb")
        }

        // Instruction 26: CheckAndSetBit { selector: Class("gbqfb,.gbqfba,.gbqfbb"), bit_pos: 2 }
        if node_matches_selector_generated(node, &SimpleSelector::Class("gbqfb,.gbqfba,.gbqfbb".to_string())) {
            intrinsic_matches.set_bit(2); // match_Class("gbqfb,.gbqfba,.gbqfbb")
        }

        // Instruction 28: CheckAndSetBit { selector: Class("gbprcd"), bit_pos: 4 }
        if node_matches_selector_generated(node, &SimpleSelector::Class("gbprcd".to_string())) {
            intrinsic_matches.set_bit(4); // match_Class("gbprcd")
        }

        // Instruction 30: CheckAndSetBit { selector: Id("gbmpas"), bit_pos: 6 }
        if node_matches_selector_generated(node, &SimpleSelector::Id("gbmpas".to_string())) {
            intrinsic_matches.set_bit(6); // match_Id("gbmpas")
        }

        // Instruction 32: CheckAndSetBit { selector: Id("gws-output-pages-elements-homepage_additional_languages__als"), bit_pos: 8 }
        if node_matches_selector_generated(node, &SimpleSelector::Id("gws-output-pages-elements-homepage_additional_languages__als".to_string())) {
            intrinsic_matches.set_bit(8); // match_Id("gws-output-pages-elements-homepage_additional_languages__als")
        }

        // Instruction 34: CheckAndSetBit { selector: Class("gsib_a"), bit_pos: 10 }
        if node_matches_selector_generated(node, &SimpleSelector::Class("gsib_a".to_string())) {
            intrinsic_matches.set_bit(10); // match_Class("gsib_a")
        }

        // Instruction 36: CheckAndSetBit { selector: Type("div"), bit_pos: 12 }
        if node_matches_selector_generated(node, &SimpleSelector::Type("div".to_string())) {
            intrinsic_matches.set_bit(12); // match_Type("div")
        }

        // Instruction 38: CheckAndSetBit { selector: Type("span"), bit_pos: 14 }
        if node_matches_selector_generated(node, &SimpleSelector::Type("span".to_string())) {
            intrinsic_matches.set_bit(14); // match_Type("span")
        }

        // Instruction 40: CheckAndSetBit { selector: Type("a"), bit_pos: 16 }
        if node_matches_selector_generated(node, &SimpleSelector::Type("a".to_string())) {
            intrinsic_matches.set_bit(16); // match_Type("a")
        }

        // Instruction 42: CheckAndSetBit { selector: Type("input"), bit_pos: 18 }
        if node_matches_selector_generated(node, &SimpleSelector::Type("input".to_string())) {
            intrinsic_matches.set_bit(18); // match_Type("input")
        }

        // Instruction 44: CheckAndSetBit { selector: Class("gbts"), bit_pos: 20 }
        if node_matches_selector_generated(node, &SimpleSelector::Class("gbts".to_string())) {
            intrinsic_matches.set_bit(20); // match_Class("gbts")
        }

        // Instruction 46: CheckAndSetBit { selector: Class("gbmt"), bit_pos: 22 }
        if node_matches_selector_generated(node, &SimpleSelector::Class("gbmt".to_string())) {
            intrinsic_matches.set_bit(22); // match_Class("gbmt")
        }

        // Instruction 48: CheckAndSetBit { selector: Class("lsb"), bit_pos: 24 }
        if node_matches_selector_generated(node, &SimpleSelector::Class("lsb".to_string())) {
            intrinsic_matches.set_bit(24); // match_Class("lsb")
        }

        // Instruction 50: CheckAndSetBit { selector: Id("gb"), bit_pos: 0 }
        if node_matches_selector_generated(node, &SimpleSelector::Id("gb".to_string())) {
            intrinsic_matches.set_bit(0); // match_Id("gb")
        }

        // Instruction 52: CheckAndSetBit { selector: Id("gbz"), bit_pos: 26 }
        if node_matches_selector_generated(node, &SimpleSelector::Id("gbz".to_string())) {
            intrinsic_matches.set_bit(26); // match_Id("gbz")
        }

    let mut current_matches = intrinsic_matches;
    let mut child_states = BitVector::new();
    if current_matches.is_bit_set(0) {
        child_states.set_bit(1); // active_Id("gb")
    }
    if current_matches.is_bit_set(2) {
        child_states.set_bit(3); // active_Class("gbqfb,.gbqfba,.gbqfbb")
    }
    if current_matches.is_bit_set(4) {
        child_states.set_bit(5); // active_Class("gbprcd")
    }
    if current_matches.is_bit_set(6) {
        child_states.set_bit(7); // active_Id("gbmpas")
    }
    if current_matches.is_bit_set(8) {
        child_states.set_bit(9); // active_Id("gws-output-pages-elements-homepage_additional_languages__als")
    }
    if current_matches.is_bit_set(10) {
        child_states.set_bit(11); // active_Class("gsib_a")
    }
    if current_matches.is_bit_set(0) {
        child_states.set_bit(1); // active_Id("gb")
    }
    if current_matches.is_bit_set(2) {
        child_states.set_bit(3); // active_Class("gbqfb,.gbqfba,.gbqfbb")
    }
    if current_matches.is_bit_set(4) {
        child_states.set_bit(5); // active_Class("gbprcd")
    }
    if current_matches.is_bit_set(6) {
        child_states.set_bit(7); // active_Id("gbmpas")
    }
    if current_matches.is_bit_set(8) {
        child_states.set_bit(9); // active_Id("gws-output-pages-elements-homepage_additional_languages__als")
    }
    if current_matches.is_bit_set(10) {
        child_states.set_bit(11); // active_Class("gsib_a")
    }
    if current_matches.is_bit_set(0) {
        child_states.set_bit(1); // active_Id("gb")
    }
    if current_matches.is_bit_set(2) {
        child_states.set_bit(3); // active_Class("gbqfb,.gbqfba,.gbqfbb")
    }
    if current_matches.is_bit_set(4) {
        child_states.set_bit(5); // active_Class("gbprcd")
    }
    if current_matches.is_bit_set(6) {
        child_states.set_bit(7); // active_Id("gbmpas")
    }
    if current_matches.is_bit_set(8) {
        child_states.set_bit(9); // active_Id("gws-output-pages-elements-homepage_additional_languages__als")
    }
    if current_matches.is_bit_set(10) {
        child_states.set_bit(11); // active_Class("gsib_a")
    }
    if current_matches.is_bit_set(12) {
        child_states.set_bit(13); // active_Type("div")
    }
    if current_matches.is_bit_set(14) {
        child_states.set_bit(15); // active_Type("span")
    }
    if current_matches.is_bit_set(16) {
        child_states.set_bit(17); // active_Type("a")
    }
    if current_matches.is_bit_set(18) {
        child_states.set_bit(19); // active_Type("input")
    }
    if current_matches.is_bit_set(20) {
        child_states.set_bit(21); // active_Class("gbts")
    }
    if current_matches.is_bit_set(22) {
        child_states.set_bit(23); // active_Class("gbmt")
    }
    if current_matches.is_bit_set(24) {
        child_states.set_bit(25); // active_Class("lsb")
    }
    if current_matches.is_bit_set(0) {
        child_states.set_bit(1); // active_Id("gb")
    }
    if current_matches.is_bit_set(26) {
        child_states.set_bit(27); // active_Id("gbz")
    }
    node.css_match_bitvector = current_matches;
    child_states
}

pub fn node_matches_selector_generated(node: &HtmlNode, selector: &SimpleSelector) -> bool {
    match selector {
        SimpleSelector::Type(tag) => node.tag_name == *tag,
        SimpleSelector::Class(class) => node.classes.contains(class),
        SimpleSelector::Id(id) => node.id.as_deref() == Some(id),
    }
}


/// Incremental processing driver with statistics tracking
pub fn process_tree_incremental_with_stats(root: &mut HtmlNode) -> (usize, usize, usize) {
    let mut total_nodes = 0;
    let mut cache_hits = 0;
    let mut cache_misses = 0;
    process_tree_recursive_incremental(root, BitVector::new(), &mut total_nodes, &mut cache_hits, &mut cache_misses);
    (total_nodes, cache_hits, cache_misses)
}

fn process_tree_recursive_incremental(node: &mut HtmlNode, parent_state: BitVector,
                                    total: &mut usize, hits: &mut usize, misses: &mut usize) {
    *total += 1;
    if !node.needs_any_recomputation(parent_state) { // Assumes HtmlNode has needs_any_recomputation
        *hits += 1;
        return;
    }
    *misses += 1;
    let child_states = process_node_generated_incremental(node, parent_state);
    for child in node.children.iter_mut() {
        process_tree_recursive_incremental(child, child_states, total, hits, misses);
    }
}

/// From-scratch processing driver for comparison
pub fn process_tree_full_recompute(root: &mut HtmlNode) -> (usize, usize, usize) {
    let mut total_nodes = 0;
    process_tree_recursive_from_scratch(root, BitVector::new(), &mut total_nodes);
    (total_nodes, 0, total_nodes) // 0 hits, all misses
}

fn process_tree_recursive_from_scratch(node: &mut HtmlNode, parent_state: BitVector, total: &mut usize) {
    *total += 1;
    let child_states = process_node_generated_from_scratch(node, parent_state);
    for child in node.children.iter_mut() {
        process_tree_recursive_from_scratch(child, child_states, total);
    }
}
