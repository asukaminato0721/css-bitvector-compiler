use crate::{BitVector, HtmlNode, SimpleSelector};
use std::collections::HashMap;
use std::sync::OnceLock;

const BITVECTOR_CAPACITY: usize = 48;

// String interning for optimized selector matching
static STRING_TO_ID: OnceLock<HashMap<&'static str, u32>> = OnceLock::new();

fn get_string_to_id_map() -> &'static HashMap<&'static str, u32> {
    STRING_TO_ID.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert("yt-logo-red-svg", 14);
        map.insert("input", 22);
        map.insert("external-icon", 1);
        map.insert("gb", 10);
        map.insert("div", 20);
        map.insert("html", 21);
        map.insert("masthead-skeleton-icons", 13);
        map.insert("gbts", 3);
        map.insert("masthead-logo", 12);
        map.insert("yt-logo-svg", 16);
        map.insert("yt-logo-red-updated-svg", 15);
        map.insert("lsb", 6);
        map.insert("grecaptcha-badge", 4);
        map.insert("span", 23);
        map.insert("chunked", 0);
        map.insert("shell", 8);
        map.insert("gbmt", 2);
        map.insert("yt-icons-ext", 9);
        map.insert("gbz", 11);
        map.insert("body", 19);
        map.insert("a", 18);
        map.insert("hidden", 5);
        map.insert("masthead-skeleton-icon", 7);
        map.insert("yt-logo-updated-svg", 17);
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

// --- BitVector-only Incremental Processing Functions ---
pub fn process_node_generated_bitvector_incremental(
    node: &mut HtmlNode,
    parent_state: &BitVector,
) -> BitVector {
    // returns child_states
    // Check if we need to recompute using BitVector-only tracking
    if !node.needs_any_recomputation_bitvector(parent_state) {
        // Return cached result - entire subtree can be skipped
        return node.cached_child_states.clone().unwrap_or_default();
    }

    // Recompute node intrinsic matches if needed
    if node.cached_node_intrinsic.is_none() || node.is_self_dirty {
        let mut intrinsic_matches = BitVector::with_capacity(BITVECTOR_CAPACITY);

        // Instruction 0: CheckAndSetBit { selector: Class("chunked"), bit_pos: 0 }
        if matches_class_id(node, 0) {
            intrinsic_matches.set_bit(0); // match_Class("chunked")
        }

        // Instruction 2: CheckAndSetBit { selector: Class("external-icon"), bit_pos: 2 }
        if matches_class_id(node, 1) {
            intrinsic_matches.set_bit(2); // match_Class("external-icon")
        }

        // Instruction 4: CheckAndSetBit { selector: Class("gbmt"), bit_pos: 4 }
        if matches_class_id(node, 2) {
            intrinsic_matches.set_bit(4); // match_Class("gbmt")
        }

        // Instruction 6: CheckAndSetBit { selector: Class("gbts"), bit_pos: 6 }
        if matches_class_id(node, 3) {
            intrinsic_matches.set_bit(6); // match_Class("gbts")
        }

        // Instruction 8: CheckAndSetBit { selector: Class("grecaptcha-badge"), bit_pos: 8 }
        if matches_class_id(node, 4) {
            intrinsic_matches.set_bit(8); // match_Class("grecaptcha-badge")
        }

        // Instruction 10: CheckAndSetBit { selector: Class("hidden"), bit_pos: 10 }
        if matches_class_id(node, 5) {
            intrinsic_matches.set_bit(10); // match_Class("hidden")
        }

        // Instruction 12: CheckAndSetBit { selector: Class("lsb"), bit_pos: 12 }
        if matches_class_id(node, 6) {
            intrinsic_matches.set_bit(12); // match_Class("lsb")
        }

        // Instruction 14: CheckAndSetBit { selector: Class("masthead-skeleton-icon"), bit_pos: 14 }
        if matches_class_id(node, 7) {
            intrinsic_matches.set_bit(14); // match_Class("masthead-skeleton-icon")
        }

        // Instruction 16: CheckAndSetBit { selector: Class("shell"), bit_pos: 16 }
        if matches_class_id(node, 8) {
            intrinsic_matches.set_bit(16); // match_Class("shell")
        }

        // Instruction 18: CheckAndSetBit { selector: Class("yt-icons-ext"), bit_pos: 18 }
        if matches_class_id(node, 9) {
            intrinsic_matches.set_bit(18); // match_Class("yt-icons-ext")
        }

        // Instruction 20: CheckAndSetBit { selector: Id("gb"), bit_pos: 20 }
        if matches_id_id(node, 10) {
            intrinsic_matches.set_bit(20); // match_Id("gb")
        }

        // Instruction 22: CheckAndSetBit { selector: Id("gbz"), bit_pos: 22 }
        if matches_id_id(node, 11) {
            intrinsic_matches.set_bit(22); // match_Id("gbz")
        }

        // Instruction 24: CheckAndSetBit { selector: Id("masthead-logo"), bit_pos: 24 }
        if matches_id_id(node, 12) {
            intrinsic_matches.set_bit(24); // match_Id("masthead-logo")
        }

        // Instruction 26: CheckAndSetBit { selector: Id("masthead-skeleton-icons"), bit_pos: 26 }
        if matches_id_id(node, 13) {
            intrinsic_matches.set_bit(26); // match_Id("masthead-skeleton-icons")
        }

        // Instruction 28: CheckAndSetBit { selector: Id("yt-logo-red-svg"), bit_pos: 28 }
        if matches_id_id(node, 14) {
            intrinsic_matches.set_bit(28); // match_Id("yt-logo-red-svg")
        }

        // Instruction 30: CheckAndSetBit { selector: Id("yt-logo-red-updated-svg"), bit_pos: 30 }
        if matches_id_id(node, 15) {
            intrinsic_matches.set_bit(30); // match_Id("yt-logo-red-updated-svg")
        }

        // Instruction 32: CheckAndSetBit { selector: Id("yt-logo-svg"), bit_pos: 32 }
        if matches_id_id(node, 16) {
            intrinsic_matches.set_bit(32); // match_Id("yt-logo-svg")
        }

        // Instruction 34: CheckAndSetBit { selector: Id("yt-logo-updated-svg"), bit_pos: 34 }
        if matches_id_id(node, 17) {
            intrinsic_matches.set_bit(34); // match_Id("yt-logo-updated-svg")
        }

        // Instruction 36: CheckAndSetBit { selector: Type("a"), bit_pos: 36 }
        if matches_tag_id(node, 18) {
            intrinsic_matches.set_bit(36); // match_Type("a")
        }

        // Instruction 38: CheckAndSetBit { selector: Type("body"), bit_pos: 38 }
        if matches_tag_id(node, 19) {
            intrinsic_matches.set_bit(38); // match_Type("body")
        }

        // Instruction 40: CheckAndSetBit { selector: Type("div"), bit_pos: 40 }
        if matches_tag_id(node, 20) {
            intrinsic_matches.set_bit(40); // match_Type("div")
        }

        // Instruction 42: CheckAndSetBit { selector: Type("html"), bit_pos: 42 }
        if matches_tag_id(node, 21) {
            intrinsic_matches.set_bit(42); // match_Type("html")
        }

        // Instruction 44: CheckAndSetBit { selector: Type("input"), bit_pos: 44 }
        if matches_tag_id(node, 22) {
            intrinsic_matches.set_bit(44); // match_Type("input")
        }

        // Instruction 46: CheckAndSetBit { selector: Type("span"), bit_pos: 46 }
        if matches_tag_id(node, 23) {
            intrinsic_matches.set_bit(46); // match_Type("span")
        }

        node.cached_node_intrinsic = Some(intrinsic_matches);
    }

    let mut current_matches = node.cached_node_intrinsic.clone().unwrap();

    // BitVector-only parent state tracking
    let mut parent_bits_read = BitVector::with_capacity(parent_state.capacity);
    let mut parent_values_read = BitVector::with_capacity(parent_state.capacity);
    let mut child_states = BitVector::with_capacity(BITVECTOR_CAPACITY);
    if current_matches.is_bit_set(0) {
        child_states.set_bit(1); // active_Class("chunked")
    }
    if current_matches.is_bit_set(2) {
        child_states.set_bit(3); // active_Class("external-icon")
    }
    if current_matches.is_bit_set(4) {
        child_states.set_bit(5); // active_Class("gbmt")
    }
    if current_matches.is_bit_set(6) {
        child_states.set_bit(7); // active_Class("gbts")
    }
    if current_matches.is_bit_set(8) {
        child_states.set_bit(9); // active_Class("grecaptcha-badge")
    }
    if current_matches.is_bit_set(10) {
        child_states.set_bit(11); // active_Class("hidden")
    }
    if current_matches.is_bit_set(12) {
        child_states.set_bit(13); // active_Class("lsb")
    }
    if current_matches.is_bit_set(14) {
        child_states.set_bit(15); // active_Class("masthead-skeleton-icon")
    }
    if current_matches.is_bit_set(16) {
        child_states.set_bit(17); // active_Class("shell")
    }
    if current_matches.is_bit_set(18) {
        child_states.set_bit(19); // active_Class("yt-icons-ext")
    }
    if current_matches.is_bit_set(20) {
        child_states.set_bit(21); // active_Id("gb")
    }
    if current_matches.is_bit_set(22) {
        child_states.set_bit(23); // active_Id("gbz")
    }
    if current_matches.is_bit_set(24) {
        child_states.set_bit(25); // active_Id("masthead-logo")
    }
    if current_matches.is_bit_set(26) {
        child_states.set_bit(27); // active_Id("masthead-skeleton-icons")
    }
    if current_matches.is_bit_set(28) {
        child_states.set_bit(29); // active_Id("yt-logo-red-svg")
    }
    if current_matches.is_bit_set(30) {
        child_states.set_bit(31); // active_Id("yt-logo-red-updated-svg")
    }
    if current_matches.is_bit_set(32) {
        child_states.set_bit(33); // active_Id("yt-logo-svg")
    }
    if current_matches.is_bit_set(34) {
        child_states.set_bit(35); // active_Id("yt-logo-updated-svg")
    }
    if current_matches.is_bit_set(36) {
        child_states.set_bit(37); // active_Type("a")
    }
    if current_matches.is_bit_set(38) {
        child_states.set_bit(39); // active_Type("body")
    }
    if current_matches.is_bit_set(40) {
        child_states.set_bit(41); // active_Type("div")
    }
    if current_matches.is_bit_set(42) {
        child_states.set_bit(43); // active_Type("html")
    }
    if current_matches.is_bit_set(44) {
        child_states.set_bit(45); // active_Type("input")
    }
    if current_matches.is_bit_set(46) {
        child_states.set_bit(47); // active_Type("span")
    }
    node.css_match_bitvector = current_matches;
    node.set_parent_state_cache_bitvector(parent_bits_read, parent_values_read);
    node.cached_child_states = Some(child_states.clone());
    node.mark_clean();

    child_states
}

// --- BitVector-only From-Scratch Processing Functions ---
pub fn process_node_generated_bitvector_from_scratch(
    node: &mut HtmlNode,
    parent_state: &BitVector,
) -> BitVector {
    // returns child_states
    let mut intrinsic_matches = BitVector::with_capacity(BITVECTOR_CAPACITY);

    // Instruction 0: CheckAndSetBit { selector: Class("chunked"), bit_pos: 0 }
    if matches_class_id(node, 0) {
        intrinsic_matches.set_bit(0); // match_Class("chunked")
    }

    // Instruction 2: CheckAndSetBit { selector: Class("external-icon"), bit_pos: 2 }
    if matches_class_id(node, 1) {
        intrinsic_matches.set_bit(2); // match_Class("external-icon")
    }

    // Instruction 4: CheckAndSetBit { selector: Class("gbmt"), bit_pos: 4 }
    if matches_class_id(node, 2) {
        intrinsic_matches.set_bit(4); // match_Class("gbmt")
    }

    // Instruction 6: CheckAndSetBit { selector: Class("gbts"), bit_pos: 6 }
    if matches_class_id(node, 3) {
        intrinsic_matches.set_bit(6); // match_Class("gbts")
    }

    // Instruction 8: CheckAndSetBit { selector: Class("grecaptcha-badge"), bit_pos: 8 }
    if matches_class_id(node, 4) {
        intrinsic_matches.set_bit(8); // match_Class("grecaptcha-badge")
    }

    // Instruction 10: CheckAndSetBit { selector: Class("hidden"), bit_pos: 10 }
    if matches_class_id(node, 5) {
        intrinsic_matches.set_bit(10); // match_Class("hidden")
    }

    // Instruction 12: CheckAndSetBit { selector: Class("lsb"), bit_pos: 12 }
    if matches_class_id(node, 6) {
        intrinsic_matches.set_bit(12); // match_Class("lsb")
    }

    // Instruction 14: CheckAndSetBit { selector: Class("masthead-skeleton-icon"), bit_pos: 14 }
    if matches_class_id(node, 7) {
        intrinsic_matches.set_bit(14); // match_Class("masthead-skeleton-icon")
    }

    // Instruction 16: CheckAndSetBit { selector: Class("shell"), bit_pos: 16 }
    if matches_class_id(node, 8) {
        intrinsic_matches.set_bit(16); // match_Class("shell")
    }

    // Instruction 18: CheckAndSetBit { selector: Class("yt-icons-ext"), bit_pos: 18 }
    if matches_class_id(node, 9) {
        intrinsic_matches.set_bit(18); // match_Class("yt-icons-ext")
    }

    // Instruction 20: CheckAndSetBit { selector: Id("gb"), bit_pos: 20 }
    if matches_id_id(node, 10) {
        intrinsic_matches.set_bit(20); // match_Id("gb")
    }

    // Instruction 22: CheckAndSetBit { selector: Id("gbz"), bit_pos: 22 }
    if matches_id_id(node, 11) {
        intrinsic_matches.set_bit(22); // match_Id("gbz")
    }

    // Instruction 24: CheckAndSetBit { selector: Id("masthead-logo"), bit_pos: 24 }
    if matches_id_id(node, 12) {
        intrinsic_matches.set_bit(24); // match_Id("masthead-logo")
    }

    // Instruction 26: CheckAndSetBit { selector: Id("masthead-skeleton-icons"), bit_pos: 26 }
    if matches_id_id(node, 13) {
        intrinsic_matches.set_bit(26); // match_Id("masthead-skeleton-icons")
    }

    // Instruction 28: CheckAndSetBit { selector: Id("yt-logo-red-svg"), bit_pos: 28 }
    if matches_id_id(node, 14) {
        intrinsic_matches.set_bit(28); // match_Id("yt-logo-red-svg")
    }

    // Instruction 30: CheckAndSetBit { selector: Id("yt-logo-red-updated-svg"), bit_pos: 30 }
    if matches_id_id(node, 15) {
        intrinsic_matches.set_bit(30); // match_Id("yt-logo-red-updated-svg")
    }

    // Instruction 32: CheckAndSetBit { selector: Id("yt-logo-svg"), bit_pos: 32 }
    if matches_id_id(node, 16) {
        intrinsic_matches.set_bit(32); // match_Id("yt-logo-svg")
    }

    // Instruction 34: CheckAndSetBit { selector: Id("yt-logo-updated-svg"), bit_pos: 34 }
    if matches_id_id(node, 17) {
        intrinsic_matches.set_bit(34); // match_Id("yt-logo-updated-svg")
    }

    // Instruction 36: CheckAndSetBit { selector: Type("a"), bit_pos: 36 }
    if matches_tag_id(node, 18) {
        intrinsic_matches.set_bit(36); // match_Type("a")
    }

    // Instruction 38: CheckAndSetBit { selector: Type("body"), bit_pos: 38 }
    if matches_tag_id(node, 19) {
        intrinsic_matches.set_bit(38); // match_Type("body")
    }

    // Instruction 40: CheckAndSetBit { selector: Type("div"), bit_pos: 40 }
    if matches_tag_id(node, 20) {
        intrinsic_matches.set_bit(40); // match_Type("div")
    }

    // Instruction 42: CheckAndSetBit { selector: Type("html"), bit_pos: 42 }
    if matches_tag_id(node, 21) {
        intrinsic_matches.set_bit(42); // match_Type("html")
    }

    // Instruction 44: CheckAndSetBit { selector: Type("input"), bit_pos: 44 }
    if matches_tag_id(node, 22) {
        intrinsic_matches.set_bit(44); // match_Type("input")
    }

    // Instruction 46: CheckAndSetBit { selector: Type("span"), bit_pos: 46 }
    if matches_tag_id(node, 23) {
        intrinsic_matches.set_bit(46); // match_Type("span")
    }

    let mut current_matches = intrinsic_matches;
    let mut _parent_bits_read = BitVector::with_capacity(parent_state.capacity);
    let mut _parent_values_read = BitVector::with_capacity(parent_state.capacity);
    let mut child_states = BitVector::with_capacity(BITVECTOR_CAPACITY);
    if current_matches.is_bit_set(0) {
        child_states.set_bit(1); // active_Class("chunked")
    }
    if current_matches.is_bit_set(2) {
        child_states.set_bit(3); // active_Class("external-icon")
    }
    if current_matches.is_bit_set(4) {
        child_states.set_bit(5); // active_Class("gbmt")
    }
    if current_matches.is_bit_set(6) {
        child_states.set_bit(7); // active_Class("gbts")
    }
    if current_matches.is_bit_set(8) {
        child_states.set_bit(9); // active_Class("grecaptcha-badge")
    }
    if current_matches.is_bit_set(10) {
        child_states.set_bit(11); // active_Class("hidden")
    }
    if current_matches.is_bit_set(12) {
        child_states.set_bit(13); // active_Class("lsb")
    }
    if current_matches.is_bit_set(14) {
        child_states.set_bit(15); // active_Class("masthead-skeleton-icon")
    }
    if current_matches.is_bit_set(16) {
        child_states.set_bit(17); // active_Class("shell")
    }
    if current_matches.is_bit_set(18) {
        child_states.set_bit(19); // active_Class("yt-icons-ext")
    }
    if current_matches.is_bit_set(20) {
        child_states.set_bit(21); // active_Id("gb")
    }
    if current_matches.is_bit_set(22) {
        child_states.set_bit(23); // active_Id("gbz")
    }
    if current_matches.is_bit_set(24) {
        child_states.set_bit(25); // active_Id("masthead-logo")
    }
    if current_matches.is_bit_set(26) {
        child_states.set_bit(27); // active_Id("masthead-skeleton-icons")
    }
    if current_matches.is_bit_set(28) {
        child_states.set_bit(29); // active_Id("yt-logo-red-svg")
    }
    if current_matches.is_bit_set(30) {
        child_states.set_bit(31); // active_Id("yt-logo-red-updated-svg")
    }
    if current_matches.is_bit_set(32) {
        child_states.set_bit(33); // active_Id("yt-logo-svg")
    }
    if current_matches.is_bit_set(34) {
        child_states.set_bit(35); // active_Id("yt-logo-updated-svg")
    }
    if current_matches.is_bit_set(36) {
        child_states.set_bit(37); // active_Type("a")
    }
    if current_matches.is_bit_set(38) {
        child_states.set_bit(39); // active_Type("body")
    }
    if current_matches.is_bit_set(40) {
        child_states.set_bit(41); // active_Type("div")
    }
    if current_matches.is_bit_set(42) {
        child_states.set_bit(43); // active_Type("html")
    }
    if current_matches.is_bit_set(44) {
        child_states.set_bit(45); // active_Type("input")
    }
    if current_matches.is_bit_set(46) {
        child_states.set_bit(47); // active_Type("span")
    }
    node.css_match_bitvector = current_matches;
    child_states
}

/// BitVector-only incremental processing driver with statistics tracking
pub fn process_tree_bitvector_incremental_with_stats(root: &mut HtmlNode) -> (usize, usize, usize) {
    let mut total_nodes = 0;
    let mut cache_hits = 0;
    let mut cache_misses = 0;
    let initial_state = BitVector::with_capacity(BITVECTOR_CAPACITY);
    process_tree_recursive_bitvector_incremental(
        root,
        &initial_state,
        &mut total_nodes,
        &mut cache_hits,
        &mut cache_misses,
    );
    (total_nodes, cache_hits, cache_misses)
}

fn process_tree_recursive_bitvector_incremental(
    node: &mut HtmlNode,
    parent_state: &BitVector,
    total: &mut usize,
    hits: &mut usize,
    misses: &mut usize,
) {
    *total += 1;

    // Logic 1: Check if node itself needs recomputation using BitVector-only tracking
    let child_states = if node.needs_self_recomputation_bitvector(parent_state) {
        *misses += 1;
        // Recompute node and get fresh child_states
        process_node_generated_bitvector_incremental(node, parent_state)
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
            process_tree_recursive_bitvector_incremental(child, &child_states, total, hits, misses);
        }
    }
    // If no dirty descendants, skip entire subtree recursion - major optimization!
}

/// BitVector-only from-scratch processing driver for comparison
pub fn process_tree_bitvector_full_recompute(root: &mut HtmlNode) -> (usize, usize, usize) {
    let mut total_nodes = 0;
    let initial_state = BitVector::with_capacity(BITVECTOR_CAPACITY);
    process_tree_recursive_bitvector_from_scratch(root, &initial_state, &mut total_nodes);
    (total_nodes, 0, total_nodes) // 0 hits, all misses
}

fn process_tree_recursive_bitvector_from_scratch(
    node: &mut HtmlNode,
    parent_state: &BitVector,
    total: &mut usize,
) {
    *total += 1;
    let child_states = process_node_generated_bitvector_from_scratch(node, parent_state);
    for child in node.children.iter_mut() {
        process_tree_recursive_bitvector_from_scratch(child, &child_states, total);
    }
}
