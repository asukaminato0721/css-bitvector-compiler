// These code are generated, dont edit by hand
        use crate::{BitVector, HtmlNode, IState, SimpleSelector};
        use std::collections::HashMap;
        use std::sync::OnceLock;

pub const BITVECTOR_CAPACITY: usize = 32;

/// generate_string_interning_code
// String interning for optimized selector matching
static STRING_TO_ID: OnceLock<HashMap<&'static str, u32>> = OnceLock::new();

fn get_string_to_id_map() -> &'static HashMap<&'static str, u32> {
    STRING_TO_ID.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert("chunked", 0);
        map.insert("external-icon", 1);
        map.insert("yt-logo-red-svg", 9);
        map.insert("masthead-skeleton-icon", 4);
        map.insert("yt-icons-ext", 6);
        map.insert("yt-logo-updated-svg", 12);
        map.insert("html", 14);
        map.insert("masthead-logo", 7);
        map.insert("grecaptcha-badge", 2);
        map.insert("yt-logo-svg", 11);
        map.insert("shell", 5);
        map.insert("hidden", 3);
        map.insert("input", 15);
        map.insert("yt-logo-red-updated-svg", 10);
        map.insert("body", 13);
        map.insert("masthead-skeleton-icons", 8);
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
            node.id.as_ref().and_then(|id| get_string_to_id_map().get(id.as_str()).copied())
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

        pub fn process_node_generated(
            node: &mut HtmlNode,
            parent_state: &BitVector,
        ) -> BitVector {
            if !node.needs_any_recomputation(parent_state) {
                return node.child_states.clone().unwrap();
            }
            if node.node_intrinsic.is_none() || node.self_dirty {
let mut intrinsic_matches = BitVector::with_capacity(BITVECTOR_CAPACITY);
match get_node_tag_id(node) {
// Instruction 26: CheckAndSetBit { selector: Type("body"), bit_pos: 26 }
 Some(13)  => {
            intrinsic_matches.set_bit(26); // match_Type("body")
        }

// Instruction 28: CheckAndSetBit { selector: Type("html"), bit_pos: 28 }
 Some(14)  => {
            intrinsic_matches.set_bit(28); // match_Type("html")
        }

// Instruction 30: CheckAndSetBit { selector: Type("input"), bit_pos: 30 }
 Some(15)  => {
            intrinsic_matches.set_bit(30); // match_Type("input")
        }

_ => {}}
        // Instruction 0: CheckAndSetBit { selector: Class("chunked"), bit_pos: 0 }
        if node_has_class_id(node, 0) {
            intrinsic_matches.set_bit(0); // match_Class("chunked")
        }

        // Instruction 2: CheckAndSetBit { selector: Class("external-icon"), bit_pos: 2 }
        if node_has_class_id(node, 1) {
            intrinsic_matches.set_bit(2); // match_Class("external-icon")
        }

        // Instruction 4: CheckAndSetBit { selector: Class("grecaptcha-badge"), bit_pos: 4 }
        if node_has_class_id(node, 2) {
            intrinsic_matches.set_bit(4); // match_Class("grecaptcha-badge")
        }

        // Instruction 6: CheckAndSetBit { selector: Class("hidden"), bit_pos: 6 }
        if node_has_class_id(node, 3) {
            intrinsic_matches.set_bit(6); // match_Class("hidden")
        }

        // Instruction 8: CheckAndSetBit { selector: Class("masthead-skeleton-icon"), bit_pos: 8 }
        if node_has_class_id(node, 4) {
            intrinsic_matches.set_bit(8); // match_Class("masthead-skeleton-icon")
        }

        // Instruction 10: CheckAndSetBit { selector: Class("shell"), bit_pos: 10 }
        if node_has_class_id(node, 5) {
            intrinsic_matches.set_bit(10); // match_Class("shell")
        }

        // Instruction 12: CheckAndSetBit { selector: Class("yt-icons-ext"), bit_pos: 12 }
        if node_has_class_id(node, 6) {
            intrinsic_matches.set_bit(12); // match_Class("yt-icons-ext")
        }

match get_node_id_id(node) {
        // Instruction 14: CheckAndSetBit { selector: Id("masthead-logo"), bit_pos: 14 }
        Some(7) => {
            intrinsic_matches.set_bit(14); // match_Id("masthead-logo")
        }
        // Instruction 16: CheckAndSetBit { selector: Id("masthead-skeleton-icons"), bit_pos: 16 }
        Some(8) => {
            intrinsic_matches.set_bit(16); // match_Id("masthead-skeleton-icons")
        }
        // Instruction 18: CheckAndSetBit { selector: Id("yt-logo-red-svg"), bit_pos: 18 }
        Some(9) => {
            intrinsic_matches.set_bit(18); // match_Id("yt-logo-red-svg")
        }
        // Instruction 20: CheckAndSetBit { selector: Id("yt-logo-red-updated-svg"), bit_pos: 20 }
        Some(10) => {
            intrinsic_matches.set_bit(20); // match_Id("yt-logo-red-updated-svg")
        }
        // Instruction 22: CheckAndSetBit { selector: Id("yt-logo-svg"), bit_pos: 22 }
        Some(11) => {
            intrinsic_matches.set_bit(22); // match_Id("yt-logo-svg")
        }
        // Instruction 24: CheckAndSetBit { selector: Id("yt-logo-updated-svg"), bit_pos: 24 }
        Some(12) => {
            intrinsic_matches.set_bit(24); // match_Id("yt-logo-updated-svg")
        }
_ => {}}
        node.node_intrinsic = Some(intrinsic_matches);
            }
            let mut current_matches = node.node_intrinsic.clone().unwrap();
            // Track which parent state bits we actually use
            let mut parent_usage_tracker = vec![IState::IUnused; parent_state.capacity];
    let mut child_states = BitVector::with_capacity(BITVECTOR_CAPACITY);
/// generate_propagation_rules_code
    if current_matches.is_bit_set(0) {
        child_states.set_bit(1); // active_Class("chunked")
    }
    if current_matches.is_bit_set(2) {
        child_states.set_bit(3); // active_Class("external-icon")
    }
    if current_matches.is_bit_set(4) {
        child_states.set_bit(5); // active_Class("grecaptcha-badge")
    }
    if current_matches.is_bit_set(6) {
        child_states.set_bit(7); // active_Class("hidden")
    }
    if current_matches.is_bit_set(8) {
        child_states.set_bit(9); // active_Class("masthead-skeleton-icon")
    }
    if current_matches.is_bit_set(10) {
        child_states.set_bit(11); // active_Class("shell")
    }
    if current_matches.is_bit_set(12) {
        child_states.set_bit(13); // active_Class("yt-icons-ext")
    }
    if current_matches.is_bit_set(14) {
        child_states.set_bit(15); // active_Id("masthead-logo")
    }
    if current_matches.is_bit_set(16) {
        child_states.set_bit(17); // active_Id("masthead-skeleton-icons")
    }
    if current_matches.is_bit_set(18) {
        child_states.set_bit(19); // active_Id("yt-logo-red-svg")
    }
    if current_matches.is_bit_set(20) {
        child_states.set_bit(21); // active_Id("yt-logo-red-updated-svg")
    }
    if current_matches.is_bit_set(22) {
        child_states.set_bit(23); // active_Id("yt-logo-svg")
    }
    if current_matches.is_bit_set(24) {
        child_states.set_bit(25); // active_Id("yt-logo-updated-svg")
    }
    if current_matches.is_bit_set(26) {
        child_states.set_bit(27); // active_Type("body")
    }
    if current_matches.is_bit_set(28) {
        child_states.set_bit(29); // active_Type("html")
    }
    if current_matches.is_bit_set(30) {
        child_states.set_bit(31); // active_Type("input")
    }
    node.css_match_bitvector = current_matches;
            node.parent_state = Some(parent_usage_tracker);
            node.child_states = Some(child_states.clone());
            node.mark_clean();
            child_states
        }
pub fn process_tree_trivector(root: &mut HtmlNode) -> (usize, usize, usize) {
    let mut total_nodes = 0;
    let mut cache_hits = 0;
    let mut cache_misses = 0;
    let initial_state = BitVector::with_capacity(BITVECTOR_CAPACITY);
    process_tree_recursive(root, &initial_state, &mut total_nodes, &mut cache_hits, &mut cache_misses);
    (total_nodes, cache_hits, cache_misses)
}

fn process_tree_recursive(node: &mut HtmlNode, parent_state: &BitVector,
                                    total: &mut usize, hits: &mut usize, misses: &mut usize) {
    *total += 1;
    
    let child_states = if node.needs_self_recomputation(parent_state) {
        *misses += 1;
        process_node_generated(node, parent_state)
    } else {
        *hits += 1;
        node.child_states.clone().unwrap_or_else(|| BitVector::with_capacity(BITVECTOR_CAPACITY))
    };
    
    if node.has_dirty_descendant {
        for child in node.children.iter_mut() {
            process_tree_recursive(child, &child_states, total, hits, misses);
        }
    }
}
