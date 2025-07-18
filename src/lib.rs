use cssparser::{Parser, ParserInput, Token};
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::_rdtsc;
use std::collections::{HashMap, HashSet};

// RDTSC 时间测量工具
#[inline(always)]
pub fn rdtsc() -> u64 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        _rdtsc()
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        // 对于非 x86_64 架构，回退到 nanos
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
    }
}

pub fn cycles_to_duration(start_cycles: u64, end_cycles: u64) -> u64 {
    end_cycles.saturating_sub(start_cycles)
}

/// whether a part of input is: 1, 0, or unused
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IState {
    IOne,
    IZero,
    IUnused,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OState {
    OOne,
    OZero,
    OFromParent,
}

// Export BitVector
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitVector {
    pub bits: Vec<u8>,
    pub capacity: usize, // Total number of bits this vector can hold
}

impl Default for BitVector {
    fn default() -> Self {
        Self::new()
    }
}

impl BitVector {
    pub fn new() -> Self {
        BitVector {
            bits: vec![0; 32], // Start with 256 bits (32 * 8)
            capacity: 256,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let num_bytes = (capacity + 7) / 8; // Round up to nearest byte
        BitVector {
            bits: vec![0; num_bytes],
            capacity,
        }
    }

    pub fn from_u64(bits: u64) -> Self {
        let mut bv = Self::with_capacity(64);
        // Convert u64 to bytes (little-endian)
        for i in 0..8 {
            bv.bits[i] = ((bits >> (i * 8)) & 0xFF) as u8;
        }
        bv
    }

    fn ensure_capacity(&mut self, pos: usize) {
        if pos >= self.capacity {
            let new_capacity = ((pos + 8) / 8) * 8; // Round up to nearest 8 bits
            let new_len = (new_capacity + 7) / 8;

            self.bits.resize(new_len, 0);
            self.capacity = new_capacity;
        }
    }

    pub fn set_bit(&mut self, pos: usize) {
        self.ensure_capacity(pos);
        let byte_index = pos / 8;
        let bit_index = pos % 8;
        self.bits[byte_index] |= 1u8 << bit_index;
    }

    pub fn clear_bit(&mut self, pos: usize) {
        if pos >= self.capacity {
            return; // Bit is already 0 if out of capacity
        }
        let byte_index = pos / 8;
        let bit_index = pos % 8;
        if byte_index < self.bits.len() {
            self.bits[byte_index] &= !(1u8 << bit_index);
        }
    }

    pub fn is_bit_set(&self, pos: usize) -> bool {
        if pos >= self.capacity {
            return false;
        }
        let byte_index = pos / 8;
        let bit_index = pos % 8;
        if byte_index < self.bits.len() {
            (self.bits[byte_index] & (1u8 << bit_index)) != 0
        } else {
            false
        }
    }

    pub fn or_assign(&mut self, other: &BitVector) {
        // Ensure we have capacity for all bits in other
        if !other.bits.is_empty() {
            let max_bit = (other.bits.len() * 8) - 1;
            self.ensure_capacity(max_bit);
        }

        let min_len = std::cmp::min(self.bits.len(), other.bits.len());
        for i in 0..min_len {
            self.bits[i] |= other.bits[i];
        }
    }

    pub fn and(&self, other: &BitVector) -> BitVector {
        let mut result = BitVector::new();
        let min_len = std::cmp::min(self.bits.len(), other.bits.len());

        if min_len > 0 {
            result.bits.resize(min_len, 0);
            for i in 0..min_len {
                result.bits[i] = self.bits[i] & other.bits[i];
            }
        }

        result
    }

    pub fn is_empty(&self) -> bool {
        self.bits.iter().all(|&byte| byte == 0)
    }

    pub fn as_u64(&self) -> u64 {
        let mut result = 0u64;
        for (i, &byte) in self.bits.iter().take(8).enumerate() {
            result |= (byte as u64) << (i * 8);
        }
        result
    }

    pub fn has_any_bits(&self, mask: &BitVector) -> bool {
        let min_len = std::cmp::min(self.bits.len(), mask.bits.len());
        for i in 0..min_len {
            if (self.bits[i] & mask.bits[i]) != 0 {
                return true;
            }
        }
        false
    }

    pub fn count_set_bits(&self) -> usize {
        self.bits
            .iter()
            .map(|&byte| byte.count_ones() as usize)
            .sum()
    }

    pub fn first_set_bit(&self) -> Option<usize> {
        for (byte_idx, &byte) in self.bits.iter().enumerate() {
            if byte != 0 {
                return Some(byte_idx * 8 + byte.trailing_zeros() as usize);
            }
        }
        None
    }
}

impl std::fmt::Binary for BitVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Display first 64 bits for compatibility, but indicate if there are more
        if self.bits.is_empty() {
            write!(f, "0")
        } else {
            // Show first 8 bytes (64 bits) in binary
            let bits_to_show = std::cmp::min(8, self.bits.len());
            for i in (0..bits_to_show).rev() {
                write!(f, "{:08b}", self.bits[i])?;
            }
            if self.bits.len() > 8 {
                write!(f, "...")?;
            }
            Ok(())
        }
    }
}

// Export selector matching index
#[derive(Debug, Clone)]
pub struct SelectorMatchingIndex {
    pub tag_rules: HashMap<String, Vec<(usize, NFAInstruction)>>,
    pub class_rules: HashMap<String, Vec<(usize, NFAInstruction)>>,
    pub id_rules: HashMap<String, Vec<(usize, NFAInstruction)>>,
    pub parent_dependent_rules: Vec<(usize, NFAInstruction)>,
}

impl Default for SelectorMatchingIndex {
    fn default() -> Self {
        Self::new()
    }
}

impl SelectorMatchingIndex {
    pub fn new() -> Self {
        Self {
            tag_rules: HashMap::new(),
            class_rules: HashMap::new(),
            id_rules: HashMap::new(),
            parent_dependent_rules: Vec::new(),
        }
    }

    pub fn add_rule(&mut self, rule_id: usize, instruction: NFAInstruction) {
        match &instruction {
            NFAInstruction::CheckAndSetBit { selector, .. } => match selector {
                SimpleSelector::Type(tag) => {
                    self.tag_rules
                        .entry(tag.clone())
                        .or_default()
                        .push((rule_id, instruction));
                }
                SimpleSelector::Class(class) => {
                    self.class_rules
                        .entry(class.clone())
                        .or_default()
                        .push((rule_id, instruction));
                }
                SimpleSelector::Id(id) => {
                    self.id_rules
                        .entry(id.clone())
                        .or_default()
                        .push((rule_id, instruction));
                }
            },
            NFAInstruction::CheckParentAndSetBit { .. } => {
                self.parent_dependent_rules.push((rule_id, instruction));
            }
            NFAInstruction::PropagateToChildren { .. } => {
                // These are processed separately after matching
            }
        }
    }

    pub fn get_matching_rules(&self, node: &HtmlNode) -> Vec<&NFAInstruction> {
        let mut matching_rules = Vec::new();

        // Check tag rules
        if let Some(tag_rules) = self.tag_rules.get(&node.tag_name) {
            for (_, instruction) in tag_rules {
                matching_rules.push(instruction);
            }
        }

        // Check class rules
        for class in &node.classes {
            if let Some(class_rules) = self.class_rules.get(class) {
                for (_, instruction) in class_rules {
                    matching_rules.push(instruction);
                }
            }
        }

        // Check id rules
        if let Some(id) = &node.id {
            if let Some(id_rules) = self.id_rules.get(id) {
                for (_, instruction) in id_rules {
                    matching_rules.push(instruction);
                }
            }
        }

        matching_rules
    }

    pub fn get_parent_dependent_rules(&self) -> &[(usize, NFAInstruction)] {
        &self.parent_dependent_rules
    }
}

// lets generalize IState first - this is two separate but very similar optimization
// (also, you should tag our old commit before today's work, we want the old version to compare in benchmark)  // it's already in the commit.
// note that not all input state is used, some state are downright ignored.
// as an example, imagine we have a query A B, saying we should match a node satisfying predicate B,
// where parent satsify predicate A
// the code will look something like this:
// if (B(self)) {
//   if (parent_bitvector.A) {
//     self.out[AB] = 1;
//   }
// }
// in such case, you can see that we are not actually reading A, if branch is not entered
// so, suppose the parent A changed, we should do 0 work recomputing
// todo this, we have to update co/pute/ let me explain how this work with an example
//
// Export HtmlNode structure
#[derive(Debug, Clone)]
pub struct HtmlNode {
    pub tag_name: String,
    pub id: Option<String>,
    pub classes: HashSet<String>,
    pub children: Vec<HtmlNode>,
    pub css_match_bitvector: BitVector,
    pub is_self_dirty: bool,
    pub has_dirty_descendant: bool,
    pub cached_parent_state: Option<Vec<IState>>,
    pub cached_node_intrinsic: Option<BitVector>,
    pub cached_child_states: Option<BitVector>,
    pub parent: Option<*mut HtmlNode>,
    // BitVector-only version of parent state tracking (alternative to IState)
    pub cached_parent_bits_read: Option<BitVector>, // which parent bits were actually read
    pub cached_parent_values_read: Option<BitVector>, // what values those bits had when read
}
// the idea is that check the child state first. so we dont need check parent if child not meet.
//
impl HtmlNode {
    pub fn new(tag_name: &str) -> Self {
        HtmlNode {
            tag_name: tag_name.to_lowercase(),
            id: None,
            classes: HashSet::new(),
            children: Vec::new(),
            css_match_bitvector: BitVector::new(),
            is_self_dirty: true,
            has_dirty_descendant: false,
            cached_parent_state: None,
            cached_node_intrinsic: None,
            cached_child_states: None,
            parent: None,
            cached_parent_bits_read: None,
            cached_parent_values_read: None,
        }
    }

    pub fn with_id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self.mark_dirty();
        self
    }

    pub fn with_class(mut self, class: &str) -> Self {
        self.classes.insert(class.to_string());
        self.mark_dirty();
        self
    }

    pub fn add_child(mut self, child: HtmlNode) -> Self {
        self.children.push(child);
        self
    }

    pub fn remove_child(&mut self, child_index: usize) {
        self.children.remove(child_index);
    }

    fn fix_parent_pointers(&mut self) {
        let self_ptr = self as *mut HtmlNode;
        for child in self.children.iter_mut() {
            child.parent = Some(self_ptr);
        }
        // Fix children's parent pointers recursively in a separate loop
        for child in self.children.iter_mut() {
            child.fix_parent_pointers();
        }
    }

    /// Mark this node as dirty and notify ancestors
    pub fn mark_dirty(&mut self) {
        self.is_self_dirty = true;
        self.cached_node_intrinsic = None;
        self.cached_parent_bits_read = None;
        self.cached_parent_values_read = None;
        self.set_summary_bit_on_ancestors();
    }

    /// Notify ancestors that they have a dirty descendant
    fn set_summary_bit_on_ancestors(&mut self) {
        if let Some(parent_ptr) = self.parent {
            unsafe {
                let parent = &mut *parent_ptr;
                parent.set_summary_bit();
            }
        }
    }

    /// Set summary bit and propagate upward
    pub fn set_summary_bit(&mut self) {
        if self.has_dirty_descendant {
            return;
        }

        self.has_dirty_descendant = true;

        if let Some(parent_ptr) = self.parent {
            unsafe {
                let parent = &mut *parent_ptr;
                parent.set_summary_bit();
            }
        }
    }

    /// Find all dirty nodes in subtree and clean them up
    pub fn find_dirty_nodes(&mut self, dirty_nodes: &mut Vec<*mut HtmlNode>) {
        if self.is_self_dirty {
            dirty_nodes.push(self as *mut HtmlNode);
            self.is_self_dirty = false;
        }

        if self.has_dirty_descendant {
            for child in &mut self.children {
                child.find_dirty_nodes(dirty_nodes);
            }
            self.has_dirty_descendant = false;
        }
    }

    /// Recursively find all dirty nodes regardless of summary bits
    pub fn find_all_dirty_nodes_recursive(&mut self, dirty_nodes: &mut Vec<*mut HtmlNode>) {
        if self.is_self_dirty {
            dirty_nodes.push(self as *mut HtmlNode);
            self.is_self_dirty = false;
        }

        for child in &mut self.children {
            child.find_all_dirty_nodes_recursive(dirty_nodes);
        }

        self.has_dirty_descendant = false;
    }

    /// Collect all dirty nodes
    pub fn collect_dirty_nodes(&mut self) -> Vec<*mut HtmlNode> {
        let mut dirty_nodes = Vec::new();
        self.find_dirty_nodes(&mut dirty_nodes);
        dirty_nodes
    }

    /// Process all dirty nodes with a closure
    pub fn process_dirty_nodes<F>(&mut self, mut processor: F)
    where
        F: FnMut(*mut HtmlNode),
    {
        let dirty_nodes = self.collect_dirty_nodes();
        for node_ptr in dirty_nodes {
            processor(node_ptr);
        }
    }

    /// Check if subtree has dirty nodes
    pub fn has_dirty_nodes(&self) -> bool {
        self.is_self_dirty || self.has_dirty_descendant
    }

    pub fn needs_any_recomputation(&self, new_parent_state: &BitVector) -> bool {
        self.has_relevant_parent_state_changed(new_parent_state)
            || self.is_self_dirty
            || self.has_dirty_descendant
            || self.cached_parent_state.is_none()
    }

    /// Check if only the node itself needs recomputation (not including dirty descendants)
    pub fn needs_self_recomputation(&self, new_parent_state: &BitVector) -> bool {
        self.has_relevant_parent_state_changed(new_parent_state)
            || self.is_self_dirty
            || self.cached_parent_state.is_none()
    }

    /// BitVector-only version: Check if subtree needs recomputation
    pub fn needs_any_recomputation_bitvector(&self, new_parent_state: &BitVector) -> bool {
        self.is_self_dirty
            || self.has_relevant_parent_state_changed_bitvector(new_parent_state)
            || self.has_dirty_descendant
            || self.cached_parent_bits_read.is_none()
    }

    /// BitVector-only version: Check if only the node itself needs recomputation (not including dirty descendants)
    pub fn needs_self_recomputation_bitvector(&self, new_parent_state: &BitVector) -> bool {
        self.is_self_dirty
            || self.has_relevant_parent_state_changed_bitvector(new_parent_state)
            || self.cached_parent_bits_read.is_none()
    }

    /// Check if any relevant part of the parent state has changed
    pub fn has_relevant_parent_state_changed(&self, new_parent_state: &BitVector) -> bool {
        if let Some(cached_states) = &self.cached_parent_state {
            // Check each tracked bit position
            for (bit_pos, &cached_state) in cached_states.iter().enumerate() {
                match cached_state {
                    IState::IOne => {
                        // We cached that this bit was 1, check if it's still 1
                        if !new_parent_state.is_bit_set(bit_pos) {
                            return true; // Changed from 1 to 0
                        }
                    }
                    IState::IZero => {
                        // We cached that this bit was 0, check if it's still 0
                        if new_parent_state.is_bit_set(bit_pos) {
                            return true; // Changed from 0 to 1
                        }
                    }
                    IState::IUnused => {
                        // We didn't use this bit, so changes don't matter
                        // No need to check - optimization!
                    }
                }
            }
            false // No relevant changes detected
        } else {
            true // No cached state, need to recompute
        }
    }

    /// BitVector-only version: Check if any relevant part of the parent state has changed
    pub fn has_relevant_parent_state_changed_bitvector(
        &self,
        new_parent_state: &BitVector,
    ) -> bool {
        if let (Some(cached_bits_read), Some(cached_values_read)) = (
            &self.cached_parent_bits_read,
            &self.cached_parent_values_read,
        ) {
            // Only check bits that were actually read (optimization: skip unused bits)
            for bit_pos in 0..cached_bits_read.capacity {
                if cached_bits_read.is_bit_set(bit_pos) {
                    // This bit was read, check if its value changed
                    let cached_value = cached_values_read.is_bit_set(bit_pos);
                    let current_value = new_parent_state.is_bit_set(bit_pos);
                    if cached_value != current_value {
                        return true; // Value changed for a bit we care about
                    }
                }
                // If bit was not read (not set in cached_bits_read), we ignore changes - optimization!
            }
            false // No relevant changes detected
        } else {
            true // No cached state, need to recompute
        }
    }

    pub fn mark_clean(&mut self) {
        self.is_self_dirty = false;
        self.has_dirty_descendant = false;
    }

    /// Record that a parent state bit was read with a specific value (BitVector-only version)
    pub fn record_parent_bit_read(&mut self, bit_pos: usize, value: bool) {
        // Initialize BitVectors if not present
        if self.cached_parent_bits_read.is_none() {
            self.cached_parent_bits_read = Some(BitVector::new());
        }
        if self.cached_parent_values_read.is_none() {
            self.cached_parent_values_read = Some(BitVector::new());
        }

        // Record that this bit was read
        if let Some(ref mut bits_read) = self.cached_parent_bits_read {
            bits_read.set_bit(bit_pos);
        }

        // Record the value that was read
        if let Some(ref mut values_read) = self.cached_parent_values_read {
            if value {
                values_read.set_bit(bit_pos);
            } else {
                values_read.clear_bit(bit_pos);
            }
        }
    }

    /// Set the complete BitVector-based parent state cache
    pub fn set_parent_state_cache_bitvector(
        &mut self,
        bits_read: BitVector,
        values_read: BitVector,
    ) {
        self.cached_parent_bits_read = Some(bits_read);
        self.cached_parent_values_read = Some(values_read);
    }

    pub fn mark_child_dirty_by_index(&mut self, child_index: usize) -> bool {
        if child_index >= self.children.len() {
            return false;
        }

        self.children[child_index].mark_dirty();
        true
    }

    pub fn mark_node_dirty_by_path(&mut self, path: &[usize]) -> bool {
        if path.is_empty() {
            self.mark_dirty();
            return true;
        }

        let first_index = path[0];
        if first_index >= self.children.len() {
            return false;
        }

        let success = self.children[first_index].mark_node_dirty_by_path(&path[1..]);
        if success {
            // Mark this node as having dirty descendant
            self.has_dirty_descendant = true;
        }
        success
    }

    pub fn init_parent_pointers(&mut self) {
        self.parent = None;
        self.fix_parent_pointers();
    }

    pub fn find_deep_node_mut(&mut self, target_depth: usize) -> Option<&mut HtmlNode> {
        if target_depth == 0 {
            return Some(self);
        }

        for child in &mut self.children {
            if let Some(found) = child.find_deep_node_mut(target_depth - 1) {
                return Some(found);
            }
        }

        None
    }
    pub fn compare_css_matches(&self, other: &Self) -> bool {
        if self.css_match_bitvector != other.css_match_bitvector {
            return false;
        }
        if self.children.len() != other.children.len() {
            return false;
        }
        for (i, child) in self.children.iter().enumerate() {
            if !child.compare_css_matches(&other.children[i]) {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SimpleSelector {
    Type(String),
    Class(String),
    Id(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CssRule {
    Simple(SimpleSelector),
    Child {
        parent_selector: SimpleSelector,
        child_selector: SimpleSelector,
    },
}

#[derive(Debug, Clone)]
pub enum NFAInstruction {
    CheckAndSetBit {
        selector: SimpleSelector,
        bit_pos: usize,
    },
    CheckParentAndSetBit {
        parent_state_bit: usize,
        child_selector: SimpleSelector,
        result_bit: usize,
    },
    PropagateToChildren {
        match_bit: usize,
        active_bit: usize,
    },
}

#[derive(Debug, Default)]
pub struct TreeNFAProgram {
    pub instructions: Vec<NFAInstruction>,
    pub state_names: HashMap<usize, String>,
    pub total_bits: usize,
    // String interning for optimized selector matching
    pub string_to_id: HashMap<String, u32>,
    pub id_to_string: HashMap<u32, String>,
    pub next_string_id: u32,
}

impl TreeNFAProgram {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_instruction(&mut self, instruction: NFAInstruction) {
        // Intern strings from this instruction
        match &instruction {
            NFAInstruction::CheckAndSetBit { selector, .. } => {
                self.intern_selector_strings(selector);
            }
            NFAInstruction::CheckParentAndSetBit { child_selector, .. } => {
                self.intern_selector_strings(child_selector);
            }
            NFAInstruction::PropagateToChildren { .. } => {
                // No strings to intern
            }
        }

        self.instructions.push(instruction);
    }

    fn intern_selector_strings(&mut self, selector: &SimpleSelector) {
        let string_to_intern = match selector {
            SimpleSelector::Type(tag) => tag,
            SimpleSelector::Class(class) => class,
            SimpleSelector::Id(id) => id,
        };

        if !self.string_to_id.contains_key(string_to_intern) {
            let id = self.next_string_id;
            self.string_to_id.insert(string_to_intern.clone(), id);
            self.id_to_string.insert(id, string_to_intern.clone());
            self.next_string_id += 1;
        }
    }

    pub fn set_state_name(&mut self, bit_pos: usize, name: String) {
        self.state_names.insert(bit_pos, name);
        if bit_pos >= self.total_bits {
            self.total_bits = bit_pos + 1;
        }
    }

    pub fn generate_istate_code(&self) -> String {
        fn generate_parent_dependent_rules_code(s: &TreeNFAProgram) -> String {
            let mut code = String::new();
            code.push_str(
                "// match get_node_tag_id(node) {
            ",
            );
            for instruction in &s.instructions {
                if let NFAInstruction::CheckParentAndSetBit {
                    parent_state_bit,
                    child_selector: SimpleSelector::Type(tag),
                    result_bit,
                } = instruction
                {
                    // Use optimized matching with integer IDs
                    let match_condition = {
                        let tag_id = s.string_to_id[tag];
                        format!("get_node_tag_id(node) == Some({})", tag_id)
                    };

                    code.push_str(&format!("
                    if {match_condition} {{
                        // Track that we're using parent state bit {parent_state_bit}
                        if {parent_state_bit} < parent_usage_tracker.len() {{            parent_usage_tracker[{parent_state_bit}] = if parent_state.is_bit_set({parent_state_bit}) {{ IState::IOne }} else {{ IState::IZero }};\n" ));
                    code.push_str("        }\n");
                    code.push_str(&format!(
                        "        if parent_state.is_bit_set({parent_state_bit}) {{\n",
                    ));
                    code.push_str(&format!(
                        "            current_matches.set_bit({result_bit}); // {}\n",
                        s.state_names
                            .get(result_bit)
                            .unwrap_or(&format!("bit_{}", result_bit))
                    ));
                    code.push_str("        }\n");
                    code.push_str("    }\n");
                }
            }
            for instruction in &s.instructions {
                if let NFAInstruction::CheckParentAndSetBit {
                    parent_state_bit,
                    child_selector: SimpleSelector::Class(class),
                    result_bit,
                } = instruction
                {
                    // Use optimized matching with integer IDs
                    let match_condition = {
                        let class_id = s.string_to_id[class];
                        format!("node_has_class_id(node, {})", class_id)
                    };

                    code.push_str(&format!("
                    if {match_condition} {{
                        // Track that we're using parent state bit {parent_state_bit}
                        if {parent_state_bit} < parent_usage_tracker.len() {{            parent_usage_tracker[{parent_state_bit}] = if parent_state.is_bit_set({parent_state_bit}) {{ IState::IOne }} else {{ IState::IZero }};\n" ));
                    code.push_str("        }\n");
                    code.push_str(&format!(
                        "        if parent_state.is_bit_set({parent_state_bit}) {{\n",
                    ));
                    code.push_str(&format!(
                        "            current_matches.set_bit({result_bit}); // {}\n",
                        s.state_names
                            .get(result_bit)
                            .unwrap_or(&format!("bit_{}", result_bit))
                    ));
                    code.push_str("        }\n");
                    code.push_str("    }\n");
                }
            }
            code.push_str(
                "// match get_node_id_id(node) {
            ",
            );
            for instruction in &s.instructions {
                if let NFAInstruction::CheckParentAndSetBit {
                    parent_state_bit,
                    child_selector: SimpleSelector::Id(id),
                    result_bit,
                } = instruction
                {
                    // Use optimized matching with integer IDs
                    let match_condition = {
                        let id_id = s.string_to_id[id];
                        format!("get_node_id_id(node) == Some({id_id})")
                    };

                    code.push_str(&format!("
                    if {match_condition} {{
                        // Track that we're using parent state bit {parent_state_bit}
                        if {parent_state_bit} < parent_usage_tracker.len() {{            parent_usage_tracker[{parent_state_bit}] = if parent_state.is_bit_set({parent_state_bit}) {{ IState::IOne }} else {{ IState::IZero }};\n" ));
                    code.push_str("        }\n");
                    code.push_str(&format!(
                        "        if parent_state.is_bit_set({parent_state_bit}) {{\n",
                    ));
                    code.push_str(&format!(
                        "            current_matches.set_bit({result_bit}); // {}\n",
                        s.state_names
                            .get(result_bit)
                            .unwrap_or(&format!("bit_{}", result_bit))
                    ));
                    code.push_str("        }\n");
                    code.push_str("    }\n");
                }
            }
            code
        }
        let mut code = String::new();

        code.push_str(
            "// These code are generated, dont edit by hand
        use crate::{BitVector, HtmlNode, IState, SimpleSelector};
        use std::collections::HashMap;
        use std::sync::OnceLock;\n\n",
        );

        // Add capacity constant for the generated BitVectors
        code.push_str(&format!(
            "pub const BITVECTOR_CAPACITY: usize = {};\n\n",
            self.total_bits
        ));

        // Generate string interning tables and optimized matcher
        code.push_str("/// generate_string_interning_code\n");
        code.push_str(&self.generate_string_interning_code());

        // --- Common parts ---
        let intrinsic_checks_code = self.generate_intrinsic_checks_code();
        let parent_dependent_rules_code = generate_parent_dependent_rules_code(self);
        let propagation_rules_code = self.generate_propagation_rules_code();

        // --- Generate Incremental Processing Function ---
        code.push_str(
            "// --- Incremental Processing Functions ---
        pub fn process_node_generated_incremental(
            node: &mut HtmlNode,
            parent_state: &BitVector,
        ) -> BitVector { // returns child_states
            // Check if we need to recompute
            if !node.needs_any_recomputation(parent_state) {
                // Return cached result - entire subtree can be skipped
                return node.cached_child_states.clone().unwrap();
            }
            // Recompute node intrinsic matches if needed
            if node.cached_node_intrinsic.is_none() || node.is_self_dirty {
        /// generate_intrinsic_checks_code\n",
        );
        code.push_str(&intrinsic_checks_code);
        code.push_str("        node.cached_node_intrinsic = Some(intrinsic_matches);\n");
        code.push_str("    }\n\n");
        code.push_str(
            "    let mut current_matches = node.cached_node_intrinsic.clone().unwrap();\n",
        );
        code.push_str("    \n");
        code.push_str("    // Track which parent state bits we actually use\n");
        code.push_str(
            "    let mut parent_usage_tracker = vec![IState::IUnused; parent_state.capacity];\n",
        );
        code.push_str("/// generate_parent_dependent_rules_code\n");
        code.push_str(&parent_dependent_rules_code);
        code.push_str("    let mut child_states = BitVector::with_capacity(BITVECTOR_CAPACITY);\n");
        code.push_str("/// generate_propagation_rules_code\n");
        code.push_str(&propagation_rules_code);
        code.push_str(
            "    node.css_match_bitvector = current_matches;
            node.cached_parent_state = Some(parent_usage_tracker);
            node.cached_child_states = Some(child_states.clone());
            node.mark_clean();
            child_states
        }",
        );

        // --- Generate Tree Traversal Wrappers ---
        code.push_str(&self.generate_traversal_wrappers());

        code
    }

    fn generate_intrinsic_checks_code(&self) -> String {
        let mut code = String::new();
        code.push_str(
            "let mut intrinsic_matches = BitVector::with_capacity(BITVECTOR_CAPACITY);
",
        );
        code.push_str(
            "match get_node_tag_id(node) {
",
        );
        for (i, instruction) in self.instructions.iter().enumerate() {
            if let NFAInstruction::CheckAndSetBit {
                selector: SimpleSelector::Type(tag),
                bit_pos,
            } = instruction
            {
                code.push_str(&format!("// Instruction {i}: {instruction:?}\n",));

                // Use optimized matching with integer IDs
                let match_condition = {
                    let tag_id = self.string_to_id[tag];
                    format!("Some({})", tag_id)
                };

                code.push_str(&format!(" {match_condition}  => {{\n",));
                code.push_str(&format!(
                    "            intrinsic_matches.set_bit({}); // {}\n",
                    bit_pos,
                    self.state_names
                        .get(bit_pos)
                        .unwrap_or(&format!("bit_{}", bit_pos))
                ));
                code.push_str("        }\n\n");
            }
        }
        code.push_str("_ => {}}\n");

        for (i, instruction) in self.instructions.iter().enumerate() {
            if let NFAInstruction::CheckAndSetBit {
                selector: SimpleSelector::Class(class),
                bit_pos,
            } = instruction
            {
                code.push_str(&format!(
                    "        // Instruction {}: {:?}\n",
                    i, instruction
                ));

                // Use optimized matching with integer IDs
                let match_condition = {
                    let class_id = self.string_to_id[class];
                    format!("node_has_class_id(node, {})", class_id)
                };

                code.push_str(&format!("        if {} {{\n", match_condition));
                code.push_str(&format!(
                    "            intrinsic_matches.set_bit({}); // {}\n",
                    bit_pos,
                    self.state_names
                        .get(bit_pos)
                        .unwrap_or(&format!("bit_{}", bit_pos))
                ));
                code.push_str("        }\n\n");
            }
        }
        code.push_str(
            "match get_node_id_id(node) {
",
        );
        for (i, instruction) in self.instructions.iter().enumerate() {
            if let NFAInstruction::CheckAndSetBit {
                selector: SimpleSelector::Id(id),
                bit_pos,
            } = instruction
            {
                code.push_str(&format!("        // Instruction {i}: {instruction:?}\n",));

                // Use optimized matching with integer IDs
                let match_condition = {
                    let id_id = self.string_to_id[id];
                    format!("Some({})", id_id)
                };

                code.push_str(&format!("        {match_condition} => {{\n",));
                code.push_str(&format!(
                    "            intrinsic_matches.set_bit({bit_pos}); // {}\n",
                    self.state_names
                        .get(bit_pos)
                        .unwrap_or(&format!("bit_{}", bit_pos))
                ));
                code.push_str("        }\n");
            }
        }
        code.push_str("_ => {}}\n");

        code
    }

    fn generate_propagation_rules_code(&self) -> String {
        let mut code = String::new();
        for instruction in &self.instructions {
            if let NFAInstruction::PropagateToChildren {
                match_bit,
                active_bit,
            } = instruction
            {
                code.push_str(&format!(
                    "    if current_matches.is_bit_set({}) {{\n",
                    match_bit
                ));
                code.push_str(&format!(
                    "        child_states.set_bit({}); // {}\n",
                    active_bit,
                    self.state_names
                        .get(active_bit)
                        .unwrap_or(&format!("bit_{}", active_bit))
                ));
                code.push_str("    }\n");
            }
        }
        code
    }

    /// Generate BitVector-only Rust code (alternative to IState-based version)
    pub fn generate_bitvector_code(&self) -> String {
        fn generate_parent_dependent_rules_bitvector_code(s: &TreeNFAProgram) -> String {
            let mut code = String::new();
            for instruction in &s.instructions {
                if let NFAInstruction::CheckParentAndSetBit {
                    parent_state_bit,
                    child_selector,
                    result_bit,
                } = instruction
                {
                    // Use optimized matching with integer IDs
                    let match_condition = match child_selector {
                        SimpleSelector::Type(tag) => {
                            let tag_id = s.string_to_id[tag];
                            format!("get_node_tag_id(node) == Some({})", tag_id)
                        }
                        SimpleSelector::Class(class) => {
                            let class_id = s.string_to_id[class];
                            format!("node_has_class_id(node, {})", class_id)
                        }
                        SimpleSelector::Id(id) => {
                            let id_id = s.string_to_id[id];
                            format!("get_node_id_id(node) == Some({})", id_id)
                        }
                    };

                    // First check if child matches (optimization: check child condition first)
                    code.push_str(&format!("    if {} {{\n", match_condition));
                    code.push_str(&format!(
                    "        // Record parent state bit {} was read (BitVector-only tracking)\n",
                    parent_state_bit
                ));
                    code.push_str(&format!(
                        "        parent_bits_read.set_bit({});\n",
                        parent_state_bit
                    ));
                    code.push_str(&format!(
                        "        let parent_bit_value = parent_state.is_bit_set({});\n",
                        parent_state_bit
                    ));
                    code.push_str(&format!("        if parent_bit_value {{\n"));
                    code.push_str(&format!(
                        "            parent_values_read.set_bit({});\n",
                        parent_state_bit
                    ));
                    code.push_str(&format!(
                        "            current_matches.set_bit({}); // {}\n",
                        result_bit,
                        s.state_names
                            .get(result_bit)
                            .unwrap_or(&format!("bit_{}", result_bit))
                    ));
                    code.push_str("        } else {\n");
                    code.push_str(&format!(
                        "            parent_values_read.clear_bit({});\n",
                        parent_state_bit
                    ));
                    code.push_str("        }\n");
                    code.push_str("    }\n");
                }
            }
            code
        }
        let mut code = String::new();

        // Add necessary imports for the generated code to be self-contained
        code.push_str("use crate::{BitVector, HtmlNode, SimpleSelector};\n");
        code.push_str("use std::collections::HashMap;\n");
        code.push_str("use std::sync::OnceLock;\n\n");

        // Add capacity constant for the generated BitVectors
        code.push_str(&format!(
            "const BITVECTOR_CAPACITY: usize = {};\n\n",
            self.total_bits
        ));

        // Generate string interning tables and optimized matcher
        code.push_str(&self.generate_string_interning_code());

        // --- Common parts ---
        let intrinsic_checks_code = self.generate_intrinsic_checks_code();
        let parent_dependent_rules_code = generate_parent_dependent_rules_bitvector_code(self);
        let propagation_rules_code = self.generate_propagation_rules_code();

        // --- Generate BitVector-only Incremental Processing Function ---
        code.push_str(
            "// --- BitVector-only Incremental Processing Functions ---
        pub fn process_node_generated_bitvector_incremental(
            node: &mut HtmlNode,
            parent_state: &BitVector,
        ) -> BitVector { // returns child_states
            // Check if we need to recompute using BitVector-only tracking
            if !node.needs_any_recomputation_bitvector(parent_state) {
                // Return cached result - entire subtree can be skipped
                return node.cached_child_states.clone().unwrap();
            }
            // Recompute node intrinsic matches if needed
            if node.cached_node_intrinsic.is_none() || node.is_self_dirty {
        ",
        );
        code.push_str(&intrinsic_checks_code);
        code.push_str("        node.cached_node_intrinsic = Some(intrinsic_matches);\n");
        code.push_str("    }\n\n");
        code.push_str(
            "    let mut current_matches = node.cached_node_intrinsic.clone().unwrap();\n",
        );
        code.push_str("    \n");
        code.push_str("    // BitVector-only parent state tracking\n");
        code.push_str(
            "    let mut parent_bits_read = BitVector::with_capacity(parent_state.capacity);\n",
        );
        code.push_str(
            "    let mut parent_values_read = BitVector::with_capacity(parent_state.capacity);\n",
        );
        code.push_str(&parent_dependent_rules_code);
        code.push_str("    let mut child_states = BitVector::with_capacity(BITVECTOR_CAPACITY);\n");
        code.push_str(&propagation_rules_code);
        code.push_str("    node.css_match_bitvector = current_matches;\n");
        code.push_str(
            "    node.set_parent_state_cache_bitvector(parent_bits_read, parent_values_read);\n",
        );
        code.push_str("    node.cached_child_states = Some(child_states.clone());\n");
        code.push_str("    node.mark_clean();\n\n");
        code.push_str("    child_states\n");
        code.push_str("}\n\n");

        // --- Generate Tree Traversal Wrappers for BitVector-only version ---
        code.push_str(&self.generate_bitvector_traversal_wrappers());

        code
    }

    fn generate_bitvector_traversal_wrappers(&self) -> String {
        r#"
/// BitVector-only incremental processing driver with statistics tracking
pub fn process_tree_bitvector_incremental_with_stats(root: &mut HtmlNode) -> (usize, usize, usize) {
    let mut total_nodes = 0;
    let mut cache_hits = 0;
    let mut cache_misses = 0;
    let initial_state = BitVector::with_capacity(BITVECTOR_CAPACITY);
    process_tree_recursive_bitvector_incremental(root, &initial_state, &mut total_nodes, &mut cache_hits, &mut cache_misses);
    (total_nodes, cache_hits, cache_misses)
}

fn process_tree_recursive_bitvector_incremental(node: &mut HtmlNode, parent_state: &BitVector,
                                               total: &mut usize, hits: &mut usize, misses: &mut usize) {
    *total += 1;
    
    // Logic 1: Check if node itself needs recomputation using BitVector-only tracking
    let child_states = if node.needs_self_recomputation_bitvector(parent_state) {
        *misses += 1;
        // Recompute node and get fresh child_states
        process_node_generated_bitvector_incremental(node, parent_state)
    } else {
        *hits += 1;
        // Use cached child_states - major optimization for internal nodes!
        node.cached_child_states.clone().unwrap_or_else(|| BitVector::with_capacity(BITVECTOR_CAPACITY))
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
"#.to_string()
    }

    pub fn generate_traversal_wrappers(&self) -> String {
        r#"
/// Incremental processing driver with statistics tracking
pub fn process_tree_incremental_with_stats(root: &mut HtmlNode) -> (usize, usize, usize) {
    let mut total_nodes = 0;
    let mut cache_hits = 0;
    let mut cache_misses = 0;
    let initial_state = BitVector::with_capacity(BITVECTOR_CAPACITY);
    process_tree_recursive_incremental(root, &initial_state, &mut total_nodes, &mut cache_hits, &mut cache_misses);
    (total_nodes, cache_hits, cache_misses)
}

fn process_tree_recursive_incremental(node: &mut HtmlNode, parent_state: &BitVector,
                                    total: &mut usize, hits: &mut usize, misses: &mut usize) {
    *total += 1;
    
    // Logic 1: Check if node itself needs recomputation
    let child_states = if node.needs_self_recomputation(parent_state) {
        *misses += 1;
        // Recompute node and get fresh child_states
        process_node_generated_incremental(node, parent_state)
    } else {
        *hits += 1;
        // Use cached child_states - major optimization for internal nodes!
        node.cached_child_states.clone().unwrap_or_else(|| BitVector::with_capacity(BITVECTOR_CAPACITY))
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
"#.to_string()
    }

    /// Generate completely naive layout calculation code without any optimization
    pub fn generate_naive_rust_code(&self) -> String {
        let mut code = String::new();

        code.push_str("use crate::{HtmlNode, SimpleSelector};\n");

        code.push_str("// === NAIVE CSS MATCHING FUNCTIONS ===\n");
        code.push_str("// These functions calculate layout from scratch without any caching\n\n");

        // Generate individual matching functions for each CSS rule
        for (i, instruction) in self.instructions.iter().enumerate() {
            match instruction {
                NFAInstruction::CheckAndSetBit { selector, bit_pos } => {
                    code.push_str(&format!("// Rule {}: {:?}\n", i, instruction));
                    code.push_str(&format!(
                        "pub fn matches_rule_{}(node: &HtmlNode) -> bool {{\n",
                        bit_pos
                    ));

                    let condition = match selector {
                        SimpleSelector::Type(tag) => format!("node.tag_name == \"{}\"", tag),
                        SimpleSelector::Class(class) => {
                            format!("node.classes.contains(\"{}\")", class)
                        }
                        SimpleSelector::Id(id) => {
                            format!("node.id.as_ref() == Some(&\"{}\".to_string())", id)
                        }
                    };

                    code.push_str(&format!("    {}\n", condition));
                    code.push_str("}\n\n");
                }
                NFAInstruction::CheckParentAndSetBit {
                    parent_state_bit,
                    child_selector,
                    result_bit,
                } => {
                    code.push_str(&format!("// Rule {}: Parent-child rule\n", i));
                    code.push_str(&format!("pub fn matches_parent_child_rule_{}(node: &HtmlNode, parent_matches: &[bool]) -> bool {{\n", result_bit));

                    let child_condition = match child_selector {
                        SimpleSelector::Type(tag) => format!("node.tag_name == \"{}\"", tag),
                        SimpleSelector::Class(class) => {
                            format!("node.classes.contains(\"{}\")", class)
                        }
                        SimpleSelector::Id(id) => {
                            format!("node.id.as_ref() == Some(&\"{}\".to_string())", id)
                        }
                    };

                    code.push_str(&format!("    if {} {{\n", child_condition));
                    code.push_str(&format!(
                        "        parent_matches.get({}).copied().unwrap_or(false)\n",
                        parent_state_bit
                    ));
                    code.push_str(
                        "    } else {
                            false
                        }
                    }\n\n",
                    );
                }
                _ => {} // Skip propagation rules in naive implementation
            }
        }

        // Generate main naive processing function
        code.push_str("// === MAIN NAIVE PROCESSING FUNCTION ===\n");
        code.push_str("pub fn process_node_naive(node: &mut HtmlNode, parent_matches: &[bool]) -> Vec<bool> {\n");
        code.push_str(&format!(
            "    let mut matches = vec![false; {}];\n",
            self.total_bits
        ));
        code.push_str("    \n");
        code.push_str("    // Check all simple selectors\n");

        for instruction in &self.instructions {
            if let NFAInstruction::CheckAndSetBit { bit_pos, .. } = instruction {
                code.push_str(&format!("    if matches_rule_{}(node) {{\n", bit_pos));
                code.push_str(&format!("        matches[{}] = true;\n", bit_pos));
                code.push_str("    }\n");
            }
        }

        code.push_str("    \n");
        code.push_str("    // Check all parent-child rules\n");

        let has_parent_child_rules = self
            .instructions
            .iter()
            .any(|inst| matches!(inst, NFAInstruction::CheckParentAndSetBit { .. }));

        if has_parent_child_rules {
            for instruction in &self.instructions {
                if let NFAInstruction::CheckParentAndSetBit { result_bit, .. } = instruction {
                    code.push_str(&format!(
                        "    if matches_parent_child_rule_{}(node, parent_matches) {{\n",
                        result_bit
                    ));
                    code.push_str(&format!("        matches[{}] = true;\n", result_bit));
                    code.push_str("    }\n");
                }
            }
        } else {
            code.push_str("    // No parent-child rules to check\n");
            code.push_str("    let _ = parent_matches; // Suppress unused parameter warning\n");
        }

        code.push_str("    \n");
        code.push_str("    matches\n");
        code.push_str("}\n\n");

        // Generate tree traversal function
        code.push_str("// === NAIVE TREE TRAVERSAL ===\n");
        code.push_str("pub fn process_tree_naive(root: &mut HtmlNode) -> usize {\n");
        code.push_str("    let mut total_nodes = 0;\n");
        code.push_str(&format!(
            "    let empty_parent = vec![false; {}];\n",
            self.total_bits
        ));
        code.push_str("    process_tree_recursive_naive(root, &empty_parent, &mut total_nodes);
            total_nodes
        }
        fn process_tree_recursive_naive(node: &mut HtmlNode, parent_matches: &[bool], total: &mut usize) {
            *total += 1;
            
            // Calculate matches for this node from scratch
            let node_matches = process_node_naive(node, parent_matches);
            ");
        code.push_str(
            "    // Process all children with this node's matches as their parent context,
            for child in node.children.iter_mut() {
                process_tree_recursive_naive(child, &node_matches, total);
            }
        }

        pub fn get_rule_name(rule_index: usize) -> String {
            format!(\"rule_{}\", rule_index)
        }",
        );

        // Add rule documentation as comments
        code.push_str("// Rule mapping:\n");
        for (bit_pos, name) in &self.state_names {
            code.push_str(&format!("// Rule {}: {}\n", bit_pos, name));
        }
        code.push_str("\n");

        // Generate function to print all matches for debugging
        code.push_str(
            "pub fn print_node_matches(node: &HtmlNode, matches: &[bool]) {
            println!(\"Node '{}' matches:\", node.tag_name);
            for (i, &matched) in matches.iter().enumerate() {
                if matched {
                    println!(\"  Rule {}: {}\", i, get_rule_name(i));
                }
            }
        }",
        );

        // Generate function to get total number of rules
        code.push_str("pub fn get_total_rules() -> usize {\n");
        code.push_str(&format!(
            "    {} // Total number of CSS rules\n",
            self.total_bits
        ));
        code.push_str("}\n");

        code
    }

    fn generate_string_interning_code(&self) -> String {
        let mut code = String::new();

        // Generate static hash map for string-to-id lookup using OnceLock
        code.push_str("// String interning for optimized selector matching\n");
        code.push_str(
            "static STRING_TO_ID: OnceLock<HashMap<&'static str, u32>> = OnceLock::new();\n\n",
        );
        code.push_str("fn get_string_to_id_map() -> &'static HashMap<&'static str, u32> {\n");
        code.push_str("    STRING_TO_ID.get_or_init(|| {\n");
        code.push_str("        let mut map = HashMap::new();\n");

        for (string, id) in &self.string_to_id {
            code.push_str(&format!("        map.insert(\"{}\", {});\n", string, id));
        }

        code.push_str("        map\n");
        code.push_str("    })\n");
        code.push_str("}\n\n");

        // Generate optimized node matching function using switch on integer IDs
        code.push_str(
            "// Fast selector matching using integer IDs and switch
        #[inline]
        fn get_node_tag_id(node: &HtmlNode) -> Option<u32> {
            use std::cell::RefCell;
            thread_local! {
                static CACHE: RefCell<Option<(String, Option<u32>)>> = RefCell::new(None);
            }

            CACHE.with(|cache| {
                let mut cache = cache.borrow_mut();
                
                if let Some((cached_tag, cached_id)) = &*cache {
                    if cached_tag == &node.tag_name {
                        return *cached_id;
                    }
                }
        
            let result = get_string_to_id_map().get(node.tag_name.as_str()).copied();
            *cache = Some((node.tag_name.clone(), result));
            result
        })
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
",
        );

        code
    }
}

// Export CSS Compiler
pub struct CssCompiler {
    pub bit_counter: usize,
    pub state_mapping: HashMap<String, usize>,
}

impl Default for CssCompiler {
    fn default() -> Self {
        Self::new()
    }
}

impl CssCompiler {
    pub fn new() -> Self {
        CssCompiler {
            bit_counter: 0,
            state_mapping: HashMap::new(),
        }
    }

    pub fn allocate_bit(&mut self, state_name: String) -> usize {
        if let Some(&existing_bit) = self.state_mapping.get(&state_name) {
            existing_bit
        } else {
            let bit_pos = self.bit_counter;
            self.state_mapping.insert(state_name, bit_pos);
            self.bit_counter += 1;
            bit_pos
        }
    }

    pub fn compile_css_rules(&mut self, rules: &[CssRule]) -> TreeNFAProgram {
        let mut program = TreeNFAProgram::new();

        // First pass: allocate bits for all selectors
        for rule in rules {
            match rule {
                CssRule::Simple(selector) => {
                    let match_state = format!("match_{:?}", selector);
                    let active_state = format!("active_{:?}", selector);

                    let match_bit = self.allocate_bit(match_state.clone());
                    let active_bit = self.allocate_bit(active_state.clone());

                    program.set_state_name(match_bit, match_state);
                    program.set_state_name(active_bit, active_state);
                }
                CssRule::Child {
                    parent_selector,
                    child_selector,
                } => {
                    // Ensure parent has active state
                    let parent_active_state = format!("active_{:?}", parent_selector);
                    let parent_active_bit = self.allocate_bit(parent_active_state.clone());
                    program.set_state_name(parent_active_bit, parent_active_state);

                    // Allocate bit for child rule match
                    let child_match_state =
                        format!("match_{:?}_gt_{:?}", parent_selector, child_selector);
                    let child_match_bit = self.allocate_bit(child_match_state.clone());
                    program.set_state_name(child_match_bit, child_match_state);
                }
            }
        }

        // Second pass: generate instructions
        for rule in rules {
            match rule {
                CssRule::Simple(selector) => {
                    let match_state = format!("match_{:?}", selector);
                    let active_state = format!("active_{:?}", selector);

                    let match_bit = self.state_mapping[&match_state];
                    let active_bit = self.state_mapping[&active_state];

                    // Generate instruction to check and set match bit
                    program.add_instruction(NFAInstruction::CheckAndSetBit {
                        selector: selector.clone(),
                        bit_pos: match_bit,
                    });

                    // Generate instruction to propagate to children
                    program.add_instruction(NFAInstruction::PropagateToChildren {
                        match_bit,
                        active_bit,
                    });
                }
                CssRule::Child {
                    parent_selector,
                    child_selector,
                } => {
                    let parent_active_state = format!("active_{:?}", parent_selector);
                    let child_match_state =
                        format!("match_{:?}_gt_{:?}", parent_selector, child_selector);

                    let parent_active_bit = self.state_mapping[&parent_active_state];
                    let child_match_bit = self.state_mapping[&child_match_state];

                    // Generate instruction to check parent state and set child match bit
                    program.add_instruction(NFAInstruction::CheckParentAndSetBit {
                        parent_state_bit: parent_active_bit,
                        child_selector: child_selector.clone(),
                        result_bit: child_match_bit,
                    });
                }
            }
        }

        program
    }
}

// Helper functions for parsing using external cssparser library
pub fn parse_css(css_content: &str) -> Vec<CssRule> {
    let mut rules = Vec::new();

    // Try to parse with cssparser, fall back to regex if needed
    match parse_css_with_cssparser(css_content) {
        Ok(parsed_rules) => {
            rules.extend(parsed_rules);
        }
        Err(e) => {
            eprintln!("⚠️ CSS parsing error{}", e);
            panic!("CSS parsing failed");
        }
    }

    // Remove duplicates
    rules.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));
    rules.dedup();

    rules
}

/// Parse CSS using the cssparser library for robust CSS parsing
fn parse_css_with_cssparser(css_content: &str) -> Result<Vec<CssRule>, Box<dyn std::error::Error>> {
    let mut rules = Vec::new();
    let mut input = ParserInput::new(css_content);
    let mut parser = Parser::new(&mut input);

    // Simple state machine for CSS parsing
    let mut expecting_rule_body = false;
    let mut current_selector: Option<SimpleSelector> = None;

    // Parse the CSS content
    while !parser.is_exhausted() {
        match parser.next() {
            Ok(token) => {
                if expecting_rule_body {
                    match token {
                        Token::CurlyBracketBlock => {
                            // Found rule body, add the selector if we have one
                            if let Some(selector) = current_selector.take() {
                                rules.push(CssRule::Simple(selector));
                            }
                            expecting_rule_body = false;
                        }
                        _ => {
                            // Reset if we didn't find the expected rule body
                            expecting_rule_body = false;
                            current_selector = None;
                        }
                    }
                } else {
                    match token {
                        // Type selector (e.g., "div", "p", "span")
                        Token::Ident(name) => {
                            let type_name = name.to_string().to_lowercase();
                            // Only accept common HTML elements
                            if [
                                "div", "span", "p", "a", "input", "body", "html", "h1", "h2", "h3",
                                "ul", "li", "table", "tr", "td",
                            ]
                            .contains(&type_name.as_str())
                            {
                                current_selector = Some(SimpleSelector::Type(type_name));
                                expecting_rule_body = true;
                            }
                        }
                        // ID selector (e.g., "#main", "#header")
                        Token::IDHash(id) => {
                            current_selector = Some(SimpleSelector::Id(id.to_string()));
                            expecting_rule_body = true;
                        }
                        // Class selector (e.g., ".container", ".item")
                        Token::Delim('.') => {
                            if let Ok(Token::Ident(class_name)) = parser.next() {
                                current_selector =
                                    Some(SimpleSelector::Class(class_name.to_string()));
                                expecting_rule_body = true;
                            }
                        }
                        _ => {
                            // Skip other tokens
                        }
                    }
                }
            }
            Err(_) => break, // End of input or parse error
        }
    }

    Ok(rules)
}

pub fn load_dom_from_file() -> HtmlNode {
    // Try to read Google trace data from file
    let json_data = std::fs::read_to_string(format!(
        "css-gen-op/{}/command.json",
        std::env::var("WEBSITE_NAME").unwrap()
    ))
    .expect("fail to read command.json");

    // Get the first line which should be the init command
    let first_line = json_data
        .lines()
        .next()
        .expect("File is empty or cannot read first line");

    // Parse the JSON to get the DOM tree
    let trace_data: serde_json::Value =
        serde_json::from_str(first_line).expect("Failed to parse command.json");

    // Check if it's an init command
    if trace_data["name"] != "init" {
        println!("⚠️ Expected init command, using mock data");
    }

    // Extract the node from init command
    let google_node_data = &trace_data["node"];

    // Convert JSON DOM to HtmlNode and initialize parent pointers
    let mut root = convert_json_dom_to_html_node(google_node_data);
    root.init_parent_pointers();
    root
}

// === RESULT COMPARISON UTILITIES ===

pub fn create_node_identifier(node: &HtmlNode) -> String {
    if let Some(id) = &node.id {
        format!("{}#{}", node.tag_name, id)
    } else if !node.classes.is_empty() {
        let mut class_vec: Vec<String> = node.classes.iter().cloned().collect();
        class_vec.sort(); // Sort for consistent ordering
        format!("{}.{}", node.tag_name, class_vec.join("."))
    } else {
        node.tag_name.clone()
    }
}

pub fn save_results_to_file(
    results: &[(String, Vec<usize>)],
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut output = String::new();
    for (node_id, matches) in results {
        output.push_str(&format!("{}: {:?}\n", node_id, matches));
    }
    std::fs::write(filename, output)?;
    Ok(())
}

pub fn convert_json_dom_to_html_node(json_node: &serde_json::Value) -> HtmlNode {
    let name = json_node["name"].as_str().unwrap_or("unknown");
    let mut node = HtmlNode::new(name);

    // Set ID if present
    if let Some(id) = json_node["id"].as_u64() {
        node = node.with_id(&id.to_string());
    }

    // Add classes from attributes
    if let Some(attributes) = json_node["attributes"].as_object() {
        if let Some(class_attr) = attributes.get("class") {
            if let Some(class_str) = class_attr.as_str() {
                for class_name in class_str.split_whitespace() {
                    node = node.with_class(class_name);
                }
            }
        }
    }

    // Add children recursively
    if let Some(children) = json_node["children"].as_array() {
        for child_json in children {
            let child_node = convert_json_dom_to_html_node(child_json);
            node = node.add_child(child_node);
        }
    }

    // Initialize parent pointers for the complete tree
    node.init_parent_pointers();
    node
}

// Utility functions
pub fn count_matches(node: &HtmlNode) -> usize {
    let current = if node.css_match_bitvector.as_u64() != 0 {
        1
    } else {
        0
    };
    current + node.children.iter().map(count_matches).sum::<usize>()
}

pub fn count_total_nodes(node: &HtmlNode) -> usize {
    1 + node.children.iter().map(count_total_nodes).sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dirty_marking_python_style() {
        // Create a tree: root -> child1 -> grandchild
        let mut root = HtmlNode::new("div");
        let mut child1 = HtmlNode::new("span");
        let grandchild = HtmlNode::new("p");

        child1 = child1.add_child(grandchild);
        root = root.add_child(child1);
        root.init_parent_pointers();

        // Clean all initial dirty state (nodes are dirty from construction)
        let mut initial_dirty = Vec::new();
        root.find_all_dirty_nodes_recursive(&mut initial_dirty);

        // Now nothing should be dirty
        assert!(!root.has_dirty_nodes());
        assert!(!root.is_self_dirty);
        assert!(!root.has_dirty_descendant);

        // Mark the grandchild as dirty
        if let Some(grandchild) = root.children[0].children.get_mut(0) {
            grandchild.mark_dirty();
        }

        // Check that summary bits propagated correctly:
        // - root should have dirty descendant (because child1 has dirty descendant)
        // - child1 should have dirty descendant (because grandchild is dirty)
        // - grandchild should be self dirty (but not have dirty descendant)
        assert!(!root.is_self_dirty);
        assert!(root.has_dirty_descendant);
        assert!(!root.children[0].is_self_dirty);
        assert!(root.children[0].has_dirty_descendant);
        assert!(root.children[0].children[0].is_self_dirty);
        assert!(!root.children[0].children[0].has_dirty_descendant);

        // Collect dirty nodes
        let dirty_nodes = root.collect_dirty_nodes();
        assert_eq!(dirty_nodes.len(), 1);

        // After collection, tree should be clean
        assert!(!root.has_dirty_nodes());
        assert!(!root.has_dirty_descendant);
        assert!(!root.children[0].has_dirty_descendant);
        assert!(!root.children[0].children[0].is_self_dirty);
    }

    #[test]
    fn test_multiple_dirty_nodes() {
        let mut root = HtmlNode::new("div");
        let child1 = HtmlNode::new("span");
        let child2 = HtmlNode::new("p");

        root = root.add_child(child1).add_child(child2);
        root.init_parent_pointers();

        // Clean initial dirty state
        let mut initial_dirty = Vec::new();
        root.find_all_dirty_nodes_recursive(&mut initial_dirty);

        // Mark root and second child as dirty
        root.mark_dirty();
        root.children[1].mark_dirty();

        // Check state after marking:
        // - root is self dirty and has dirty descendant
        // - child1 is not dirty
        // - child2 is self dirty but has no dirty descendant
        assert!(root.is_self_dirty);
        assert!(root.has_dirty_descendant);
        assert!(!root.children[0].is_self_dirty);
        assert!(!root.children[0].has_dirty_descendant);
        assert!(root.children[1].is_self_dirty);
        assert!(!root.children[1].has_dirty_descendant);

        // Collect dirty nodes
        let dirty_nodes = root.collect_dirty_nodes();
        assert_eq!(dirty_nodes.len(), 2);

        // Tree should be clean after collection
        assert!(!root.has_dirty_nodes());
    }

    #[test]
    fn test_ancestor_summary_propagation() {
        // Create deeper tree: root -> child -> grandchild -> great_grandchild
        let mut root = HtmlNode::new("div");
        let mut child = HtmlNode::new("span");
        let mut grandchild = HtmlNode::new("p");
        let great_grandchild = HtmlNode::new("a");

        grandchild = grandchild.add_child(great_grandchild);
        child = child.add_child(grandchild);
        root = root.add_child(child);
        root.init_parent_pointers();

        // Clean initial dirty state
        let mut initial_dirty = Vec::new();
        root.find_all_dirty_nodes_recursive(&mut initial_dirty);

        // Mark the deepest node as dirty
        root.children[0].children[0].children[0].mark_dirty();

        // Check that summary bits propagated all the way up
        assert!(!root.is_self_dirty);
        assert!(root.has_dirty_descendant);
        assert!(!root.children[0].is_self_dirty);
        assert!(root.children[0].has_dirty_descendant);
        assert!(!root.children[0].children[0].is_self_dirty);
        assert!(root.children[0].children[0].has_dirty_descendant);
        assert!(root.children[0].children[0].children[0].is_self_dirty);
        assert!(!root.children[0].children[0].children[0].has_dirty_descendant);

        // Collect dirty nodes
        let dirty_nodes = root.collect_dirty_nodes();
        assert_eq!(dirty_nodes.len(), 1);

        // All summary bits should be cleared
        assert!(!root.has_dirty_descendant);
        assert!(!root.children[0].has_dirty_descendant);
        assert!(!root.children[0].children[0].has_dirty_descendant);
    }

    #[test]
    fn test_process_dirty_nodes() {
        let mut root = HtmlNode::new("div");
        let child = HtmlNode::new("span");
        root = root.add_child(child);
        root.init_parent_pointers();

        // Clean initial dirty state
        let mut initial_dirty = Vec::new();
        root.find_all_dirty_nodes_recursive(&mut initial_dirty);

        // Mark child as dirty
        root.children[0].mark_dirty();

        let mut processed_count = 0;
        root.process_dirty_nodes(|_node_ptr| {
            processed_count += 1;
        });

        assert_eq!(processed_count, 1);
        assert!(!root.has_dirty_nodes());
    }

    #[test]
    fn test_bitvector_parent_state_tracking() {
        let mut node = HtmlNode::new("div");

        // Clean the node first (nodes start dirty from construction)
        node.mark_clean();

        // Test recording parent bit reads
        node.record_parent_bit_read(5, true);
        node.record_parent_bit_read(10, false);
        node.record_parent_bit_read(15, true);

        // Create a parent state with the same bits set
        let mut parent_state = BitVector::new();
        parent_state.set_bit(5); // bit 5 = true (matches)
        // bit 10 = false (matches)
        parent_state.set_bit(15); // bit 15 = true (matches)

        // Should not need recomputation - all tracked bits match
        assert!(!node.has_relevant_parent_state_changed_bitvector(&parent_state));
        assert!(!node.needs_self_recomputation_bitvector(&parent_state));

        // Change bit 5 from true to false
        parent_state.clear_bit(5);

        // Should need recomputation - tracked bit changed
        assert!(node.has_relevant_parent_state_changed_bitvector(&parent_state));
        assert!(node.needs_self_recomputation_bitvector(&parent_state));
    }

    #[test]
    fn test_bitvector_unused_bits_optimization() {
        let mut node = HtmlNode::new("div");

        // Only record that we read bits 5 and 10
        node.record_parent_bit_read(5, true);
        node.record_parent_bit_read(10, false);

        // Create parent state with many bits set
        let mut parent_state = BitVector::new();
        parent_state.set_bit(5); // Tracked: true
        // bit 10 = false (tracked)
        parent_state.set_bit(1); // Not tracked - should be ignored
        parent_state.set_bit(20); // Not tracked - should be ignored
        parent_state.set_bit(100); // Not tracked - should be ignored

        // Should not need recomputation - tracked bits match
        assert!(!node.has_relevant_parent_state_changed_bitvector(&parent_state));

        // Change untracked bits - should still not need recomputation (optimization!)
        parent_state.clear_bit(1);
        parent_state.clear_bit(20);
        parent_state.clear_bit(100);

        // Should still not need recomputation - untracked bits don't matter
        assert!(!node.has_relevant_parent_state_changed_bitvector(&parent_state));

        // Change a tracked bit - now should need recomputation
        parent_state.set_bit(10); // Change bit 10 from false to true
        assert!(node.has_relevant_parent_state_changed_bitvector(&parent_state));
    }

    #[test]
    fn test_bitvector_cache_setting() {
        let mut node = HtmlNode::new("div");

        let mut bits_read = BitVector::new();
        bits_read.set_bit(3);
        bits_read.set_bit(7);

        let mut values_read = BitVector::new();
        values_read.set_bit(3); // bit 3 was true
        // bit 7 was false (not set)

        node.set_parent_state_cache_bitvector(bits_read, values_read);

        // Test with matching parent state
        let mut parent_state = BitVector::new();
        parent_state.set_bit(3); // true (matches cached)
        // bit 7 = false (matches cached)

        assert!(!node.has_relevant_parent_state_changed_bitvector(&parent_state));

        // Test with non-matching parent state
        parent_state.set_bit(7); // Change bit 7 from false to true
        assert!(node.has_relevant_parent_state_changed_bitvector(&parent_state));
    }
}

pub mod generated_bitvector_functions;
pub mod generated_istate_functions;
pub mod generated_naive_functions;
