use css_bitvector_compiler::*;

// Generated CSS processing function
// Generated Tree NFA Program with Incremental Processing
// This program processes HTML nodes and computes CSS matches with caching

pub fn process_node_generated_incremental(
    node: &mut HtmlNode,
    parent_state: BitVector,
) -> BitVector { // returns child_states
    // Check if we need to recompute
    if !node.needs_any_recomputation(parent_state) {
        // Return cached result - entire subtree can be skipped
        return node.cached_child_states.unwrap_or_default();
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

pub fn node_matches_selector_generated(node: &HtmlNode, selector: &SimpleSelector) -> bool {
    match selector {
        SimpleSelector::Type(tag) => node.tag_name == *tag,
        SimpleSelector::Class(class) => node.classes.contains(class),
        SimpleSelector::Id(id) => node.id.as_deref() == Some(id),
    }
}



