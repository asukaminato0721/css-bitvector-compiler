use crate::{BitVector, HtmlNode, IState, SimpleSelector};
use std::collections::HashMap;
use std::sync::OnceLock;

pub const BITVECTOR_CAPACITY: usize = 18;

// String interning for optimized selector matching
static STRING_TO_ID: OnceLock<HashMap<&'static str, u32>> = OnceLock::new();

fn get_string_to_id_map() -> &'static HashMap<&'static str, u32> {
    STRING_TO_ID.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert("a", 5);
        map.insert("div", 6);
        map.insert("gb", 3);
        map.insert("gbts", 1);
        map.insert("lsb", 2);
        map.insert("gbmt", 0);
        map.insert("gbz", 4);
        map.insert("input", 7);
        map.insert("span", 8);
        map
    })
}

// Fast selector matching using integer IDs and switch
#[inline]
fn get_node_tag_id(node: &HtmlNode) -> Option<u32> {
    get_string_to_id_map().get(node.tag_name.as_str()).copied()
}

#[inline]
fn get_node_id_id(node: &HtmlNode) -> Option<u32> {
    node.id
        .as_ref()
        .and_then(|id| get_string_to_id_map().get(id.as_str()).copied())
}

#[inline]
fn node_has_class_id(node: &HtmlNode, class_id: u32) -> bool {
    let string_map = get_string_to_id_map();
    for class in &node.classes {
        if let Some(id) = string_map.get(class.as_str()) {
            if *id == class_id {
                return true;
            }
        }
    }
    false
}

// Optimized selector matching with switch on integer IDs
#[inline]
fn matches_tag_id(node: &HtmlNode, tag_id: u32) -> bool {
    if let Some(node_tag_id) = get_node_tag_id(node) {
        node_tag_id == tag_id
    } else {
        false
    }
}

#[inline]
fn matches_id_id(node: &HtmlNode, id_id: u32) -> bool {
    if let Some(node_id_id) = get_node_id_id(node) {
        node_id_id == id_id
    } else {
        false
    }
}

#[inline]
fn matches_class_id(node: &HtmlNode, class_id: u32) -> bool {
    node_has_class_id(node, class_id)
}

// --- Incremental Processing Functions ---
pub fn process_node_generated_incremental(
    node: &mut HtmlNode,
    parent_state: &BitVector,
) -> BitVector {
    // returns child_states
    // Check if we need to recompute
    if !node.needs_any_recomputation(parent_state) {
        // Return cached result - entire subtree can be skipped
        return node.cached_child_states.clone().unwrap_or_default();
    }

    // Recompute node intrinsic matches if needed
    if node.cached_node_intrinsic.is_none() || node.is_self_dirty {
        let mut intrinsic_matches = BitVector::with_capacity(BITVECTOR_CAPACITY);

        // Instruction 0: CheckAndSetBit { selector: Class("gbmt"), bit_pos: 0 }
        if matches_class_id(node, 0) {
            intrinsic_matches.set_bit(0); // match_Class("gbmt")
        }

        // Instruction 2: CheckAndSetBit { selector: Class("gbts"), bit_pos: 2 }
        if matches_class_id(node, 1) {
            intrinsic_matches.set_bit(2); // match_Class("gbts")
        }

        // Instruction 4: CheckAndSetBit { selector: Class("lsb"), bit_pos: 4 }
        if matches_class_id(node, 2) {
            intrinsic_matches.set_bit(4); // match_Class("lsb")
        }

        // Instruction 6: CheckAndSetBit { selector: Id("gb"), bit_pos: 6 }
        if matches_id_id(node, 3) {
            intrinsic_matches.set_bit(6); // match_Id("gb")
        }

        // Instruction 8: CheckAndSetBit { selector: Id("gbz"), bit_pos: 8 }
        if matches_id_id(node, 4) {
            intrinsic_matches.set_bit(8); // match_Id("gbz")
        }

        // Instruction 10: CheckAndSetBit { selector: Type("a"), bit_pos: 10 }
        if matches_tag_id(node, 5) {
            intrinsic_matches.set_bit(10); // match_Type("a")
        }

        // Instruction 12: CheckAndSetBit { selector: Type("div"), bit_pos: 12 }
        if matches_tag_id(node, 6) {
            intrinsic_matches.set_bit(12); // match_Type("div")
        }

        // Instruction 14: CheckAndSetBit { selector: Type("input"), bit_pos: 14 }
        if matches_tag_id(node, 7) {
            intrinsic_matches.set_bit(14); // match_Type("input")
        }

        // Instruction 16: CheckAndSetBit { selector: Type("span"), bit_pos: 16 }
        if matches_tag_id(node, 8) {
            intrinsic_matches.set_bit(16); // match_Type("span")
        }

        node.cached_node_intrinsic = Some(intrinsic_matches);
    }

    let mut current_matches = node.cached_node_intrinsic.clone().unwrap();

    // Track which parent state bits we actually use
    let mut parent_usage_tracker = vec![IState::IUnused; parent_state.capacity];
    let mut child_states = BitVector::with_capacity(BITVECTOR_CAPACITY);
    if current_matches.is_bit_set(0) {
        child_states.set_bit(1); // active_Class("gbmt")
    }
    if current_matches.is_bit_set(2) {
        child_states.set_bit(3); // active_Class("gbts")
    }
    if current_matches.is_bit_set(4) {
        child_states.set_bit(5); // active_Class("lsb")
    }
    if current_matches.is_bit_set(6) {
        child_states.set_bit(7); // active_Id("gb")
    }
    if current_matches.is_bit_set(8) {
        child_states.set_bit(9); // active_Id("gbz")
    }
    if current_matches.is_bit_set(10) {
        child_states.set_bit(11); // active_Type("a")
    }
    if current_matches.is_bit_set(12) {
        child_states.set_bit(13); // active_Type("div")
    }
    if current_matches.is_bit_set(14) {
        child_states.set_bit(15); // active_Type("input")
    }
    if current_matches.is_bit_set(16) {
        child_states.set_bit(17); // active_Type("span")
    }
    node.css_match_bitvector = current_matches;
    node.cached_parent_state = Some(parent_usage_tracker);
    node.cached_child_states = Some(child_states.clone());
    node.mark_clean();

    child_states
}

// --- From-Scratch Processing Functions ---
pub fn process_node_generated_from_scratch(
    node: &mut HtmlNode,
    parent_state: &BitVector,
) -> BitVector {
    // returns child_states
    let mut intrinsic_matches = BitVector::with_capacity(BITVECTOR_CAPACITY);

    // Instruction 0: CheckAndSetBit { selector: Class("gbmt"), bit_pos: 0 }
    if matches_class_id(node, 0) {
        intrinsic_matches.set_bit(0); // match_Class("gbmt")
    }

    // Instruction 2: CheckAndSetBit { selector: Class("gbts"), bit_pos: 2 }
    if matches_class_id(node, 1) {
        intrinsic_matches.set_bit(2); // match_Class("gbts")
    }

    // Instruction 4: CheckAndSetBit { selector: Class("lsb"), bit_pos: 4 }
    if matches_class_id(node, 2) {
        intrinsic_matches.set_bit(4); // match_Class("lsb")
    }

    // Instruction 6: CheckAndSetBit { selector: Id("gb"), bit_pos: 6 }
    if matches_id_id(node, 3) {
        intrinsic_matches.set_bit(6); // match_Id("gb")
    }

    // Instruction 8: CheckAndSetBit { selector: Id("gbz"), bit_pos: 8 }
    if matches_id_id(node, 4) {
        intrinsic_matches.set_bit(8); // match_Id("gbz")
    }

    // Instruction 10: CheckAndSetBit { selector: Type("a"), bit_pos: 10 }
    if matches_tag_id(node, 5) {
        intrinsic_matches.set_bit(10); // match_Type("a")
    }

    // Instruction 12: CheckAndSetBit { selector: Type("div"), bit_pos: 12 }
    if matches_tag_id(node, 6) {
        intrinsic_matches.set_bit(12); // match_Type("div")
    }

    // Instruction 14: CheckAndSetBit { selector: Type("input"), bit_pos: 14 }
    if matches_tag_id(node, 7) {
        intrinsic_matches.set_bit(14); // match_Type("input")
    }

    // Instruction 16: CheckAndSetBit { selector: Type("span"), bit_pos: 16 }
    if matches_tag_id(node, 8) {
        intrinsic_matches.set_bit(16); // match_Type("span")
    }

    let mut current_matches = intrinsic_matches;
    let mut child_states = BitVector::with_capacity(BITVECTOR_CAPACITY);
    if current_matches.is_bit_set(0) {
        child_states.set_bit(1); // active_Class("gbmt")
    }
    if current_matches.is_bit_set(2) {
        child_states.set_bit(3); // active_Class("gbts")
    }
    if current_matches.is_bit_set(4) {
        child_states.set_bit(5); // active_Class("lsb")
    }
    if current_matches.is_bit_set(6) {
        child_states.set_bit(7); // active_Id("gb")
    }
    if current_matches.is_bit_set(8) {
        child_states.set_bit(9); // active_Id("gbz")
    }
    if current_matches.is_bit_set(10) {
        child_states.set_bit(11); // active_Type("a")
    }
    if current_matches.is_bit_set(12) {
        child_states.set_bit(13); // active_Type("div")
    }
    if current_matches.is_bit_set(14) {
        child_states.set_bit(15); // active_Type("input")
    }
    if current_matches.is_bit_set(16) {
        child_states.set_bit(17); // active_Type("span")
    }
    node.css_match_bitvector = current_matches;
    child_states
}

pub fn node_matches_selector_generated(node: &HtmlNode, selector: &SimpleSelector) -> bool {
    let string_map = get_string_to_id_map();
    match selector {
        SimpleSelector::Type(tag) => {
            if let Some(tag_id) = string_map.get(tag.as_str()) {
                matches_tag_id(node, *tag_id)
            } else {
                false
            }
        }
        SimpleSelector::Class(class) => {
            if let Some(class_id) = string_map.get(class.as_str()) {
                matches_class_id(node, *class_id)
            } else {
                false
            }
        }
        SimpleSelector::Id(id) => {
            if let Some(id_id) = string_map.get(id.as_str()) {
                matches_id_id(node, *id_id)
            } else {
                false
            }
        }
    }
}

/// Incremental processing driver with statistics tracking
pub fn process_tree_incremental_with_stats(root: &mut HtmlNode) -> (usize, usize, usize) {
    let mut total_nodes = 0;
    let mut cache_hits = 0;
    let mut cache_misses = 0;
    let initial_state = BitVector::with_capacity(BITVECTOR_CAPACITY);
    process_tree_recursive_incremental(
        root,
        &initial_state,
        &mut total_nodes,
        &mut cache_hits,
        &mut cache_misses,
    );
    (total_nodes, cache_hits, cache_misses)
}

fn process_tree_recursive_incremental(
    node: &mut HtmlNode,
    parent_state: &BitVector,
    total: &mut usize,
    hits: &mut usize,
    misses: &mut usize,
) {
    *total += 1;

    // Logic 1: Check if node itself needs recomputation
    let child_states = if node.needs_self_recomputation(parent_state) {
        *misses += 1;
        // Recompute node and get fresh child_states
        process_node_generated_incremental(node, parent_state)
    } else {
        *hits += 1;
        // Use cached child_states - major optimization for internal nodes!
        node.cached_child_states
            .clone()
            .unwrap_or_else(|| BitVector::with_capacity(BITVECTOR_CAPACITY))
    };

    // Logic 2: Check if we need to recurse (only if there are dirty descendants)
    if node.has_dirty_descendant {
        // Recurse into children only if there are dirty descendants
        for child in node.children.iter_mut() {
            process_tree_recursive_incremental(child, &child_states, total, hits, misses);
        }
    }
    // If no dirty descendants, skip entire subtree recursion - major optimization!
}

/// From-scratch processing driver for comparison
pub fn process_tree_full_recompute(root: &mut HtmlNode) -> (usize, usize, usize) {
    let mut total_nodes = 0;
    let initial_state = BitVector::with_capacity(BITVECTOR_CAPACITY);
    process_tree_recursive_from_scratch(root, &initial_state, &mut total_nodes);
    (total_nodes, 0, total_nodes) // 0 hits, all misses
}

fn process_tree_recursive_from_scratch(
    node: &mut HtmlNode,
    parent_state: &BitVector,
    total: &mut usize,
) {
    *total += 1;
    let child_states = process_node_generated_from_scratch(node, parent_state);
    for child in node.children.iter_mut() {
        process_tree_recursive_from_scratch(child, &child_states, total);
    }
}
