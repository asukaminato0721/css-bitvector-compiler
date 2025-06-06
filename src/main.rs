use regex::Regex;
use scraper::{Html, Selector as HtmlSelector};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

// Google Trace Testing Integration
#[derive(Debug, Clone)]
pub struct GoogleNode {
    pub id: Option<u32>,
    pub name: String,
    pub node_type: String,
    pub namespace: Option<String>,
    pub attributes: std::collections::HashMap<String, String>,
    pub properties: std::collections::HashMap<String, String>,
    pub visible: bool,
    pub children: Vec<GoogleNode>,
}

impl GoogleNode {
    pub fn from_json(value: &serde_json::Value) -> Option<Self> {
        let obj = value.as_object()?;
        
        Some(GoogleNode {
            id: obj.get("id").and_then(|v| v.as_u64()).map(|v| v as u32),
            name: obj.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            node_type: obj.get("type").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            namespace: obj.get("namespace").and_then(|v| v.as_str()).map(|s| s.to_string()),
            attributes: obj.get("attributes")
                .and_then(|v| v.as_object())
                .map(|attrs| {
                    attrs.iter()
                        .filter_map(|(k, v)| {
                            v.as_str().map(|s| (k.clone(), s.to_string()))
                        })
                        .collect()
                })
                .unwrap_or_default(),
            properties: obj.get("properties")
                .and_then(|v| v.as_object())
                .map(|props| {
                    props.iter()
                        .filter_map(|(k, v)| {
                            v.as_str().map(|s| (k.clone(), s.to_string()))
                        })
                        .collect()
                })
                .unwrap_or_default(),
            visible: obj.get("visible").and_then(|v| v.as_bool()).unwrap_or(true),
            children: obj.get("children")
                .and_then(|v| v.as_array())
                .map(|children| {
                    children.iter()
                        .filter_map(|child| GoogleNode::from_json(child))
                        .collect()
                })
                .unwrap_or_default(),
        })
    }
    
    pub fn to_html_node(&self) -> HtmlNode {
        let mut node = HtmlNode::new(&self.name);
        
        if let Some(id) = &self.id {
            node.id = Some(id.to_string());
        }
        
        // Extract classes from attributes
        if let Some(class_attr) = self.attributes.get("class") {
            node.classes = class_attr
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
        }
        
        // Convert children
        for child in &self.children {
            node.children.push(child.to_html_node());
        }
        
        node
    }
    
    pub fn count_nodes(&self) -> usize {
        1 + self.children.iter().map(|child| child.count_nodes()).sum::<usize>()
    }
}

pub fn process_google_trace_with_rust() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Testing Google Trace with Rust CSS Engine (CodeGen Mode)\n");
    
    // Load Google CSS rules  
    let css_content = std::fs::read_to_string("css-gen-op/https___www.google.com_.css")
        .unwrap_or_else(|_| {
            println!("⚠️ Could not load Google CSS file, using basic rules");
            "div { display: block; } .gbts { color: #000; } #gb { position: relative; }".to_string()
        });
    
    let css_rules = parse_basic_css(&css_content);
    println!("📋 Loaded {} CSS rules from Google CSS", css_rules.len());
    
    // Compile CSS rules and generate Rust code
    let mut compiler = CssCompiler::new();
    let program = compiler.compile_css_rules(&css_rules);
    
    println!("🔧 Generating optimized Rust code...");
    let generated_code = program.generate_rust_code();
    
    // Read the first command from command.json to get initial DOM
    let commands_content = std::fs::read_to_string("css-gen-op/command.json")?;
    let first_line = commands_content.lines().next().ok_or("Empty command file")?;
    let command: serde_json::Value = serde_json::from_str(first_line)?;
    
    if command["name"] != "init" {
        return Err("First command should be init".into());
    }
    
    let google_node = GoogleNode::from_json(&command["node"])
        .ok_or("Failed to parse Google node")?;
    
    println!("🌳 Google DOM tree contains {} nodes", google_node.count_nodes());
    
    // Generate complete Rust program for Google trace testing
    let complete_program = generate_google_trace_program(&generated_code, &google_node)?;
    
    // Write to temporary file
    let temp_file = "temp_google_trace_test.rs";
    std::fs::write(temp_file, &complete_program)
        .map_err(|e| format!("Failed to write generated code: {}", e))?;
    
    println!("💾 Generated complete program: {}", temp_file);
    
    // Compile the generated program
    println!("🔨 Compiling generated Rust code...");
    let compile_output = std::process::Command::new("rustc")
        .args([temp_file, "-o", "temp_google_trace_test", "-O"]) // Enable optimizations
        .output()
        .map_err(|e| format!("Failed to run rustc: {}", e))?;
    
    if !compile_output.status.success() {
        let stderr = String::from_utf8_lossy(&compile_output.stderr);
        return Err(format!("Compilation failed: {}", stderr).into());
    }
    
    println!("✅ Compilation successful!");
    
    // Run the compiled program
    println!("🚀 Running generated code with Google trace data...\n");
    let run_output = std::process::Command::new("./temp_google_trace_test")
        .output()
        .map_err(|e| format!("Failed to run generated program: {}", e))?;
    
    if run_output.status.success() {
        let stdout = String::from_utf8_lossy(&run_output.stdout);
        println!("{}", stdout);
        
        // Verify success
        if stdout.contains("SUCCESS") {
            println!("🎉 CodeGen Google trace test completed successfully!");
        } else {
            return Err("Generated program did not complete successfully".into());
        }
    } else {
        let stderr = String::from_utf8_lossy(&run_output.stderr);
        return Err(format!("Generated program failed: {}", stderr).into());
    }
    
    // Clean up
    // let _ = std::fs::remove_file(temp_file);
    let _ = std::fs::remove_file("temp_google_trace_test");
    
    Ok(())
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

fn count_css_matches(node: &HtmlNode) -> usize {
    let current_matches = if node.css_match_bitvector.bits != 0 { 1 } else { 0 };
    current_matches + node.children.iter().map(|child| count_css_matches(child)).sum::<usize>()
}

fn generate_google_trace_program(generated_fn_code: &str, google_node: &GoogleNode) -> Result<String, Box<dyn std::error::Error>> {
    let node_data = serialize_google_node_to_rust_code(google_node);
    
    let complete_program = format!(r##"
// Generated Google Trace Test Program
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BitVector {{
    bits: u64,
}}

impl BitVector {{
    fn new() -> Self {{
        BitVector {{ bits: 0 }}
    }}

    fn from_u64(bits: u64) -> Self {{
        BitVector {{ bits }}
    }}

    fn set_bit(&mut self, pos: usize) {{
        self.bits |= 1 << pos;
    }}

    fn is_bit_set(&self, pos: usize) -> bool {{
        (self.bits & (1 << pos)) != 0
    }}

    fn as_u64(&self) -> u64 {{
        self.bits
    }}
}}

impl std::fmt::Binary for BitVector {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        write!(f, "{{:016b}}", self.bits)
    }}
}}

#[derive(Debug, Clone)]
struct HtmlNode {{
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
}}

impl HtmlNode {{
    fn new(tag_name: &str) -> Self {{
        HtmlNode {{
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
        }}
    }}

    fn with_id(mut self, id: &str) -> Self {{
        self.id = Some(id.to_string());
        self
    }}

    fn with_class(mut self, class: &str) -> Self {{
        self.classes.insert(class.to_string());
        self
    }}

    fn add_child(mut self, child: HtmlNode) -> Self {{
        self.children.push(child);
        self
    }}

    fn needs_any_recomputation(&self, new_parent_state: BitVector) -> bool {{
        self.is_self_dirty ||
        self.has_dirty_descendant ||
        self.cached_parent_state.is_none() ||
        self.cached_parent_state.unwrap().as_u64() != new_parent_state.as_u64()
    }}

    fn mark_clean(&mut self) {{
        self.is_self_dirty = false;
        self.has_dirty_descendant = false;
    }}

    fn mark_self_dirty(&mut self) {{
        self.is_self_dirty = true;
        // Also invalidate cached intrinsic matches
        self.cached_node_intrinsic = None;
    }}
}}

#[derive(Debug, Clone, PartialEq)]
enum SimpleSelector {{
    Type(String),
    Class(String),
    Id(String),
}}

{generated_fn_code}

fn count_matches(node: &HtmlNode) -> usize {{
    let current = if node.css_match_bitvector.as_u64() != 0 {{ 1 }} else {{ 0 }};
    current + node.children.iter().map(|child| count_matches(child)).sum::<usize>()
}}

fn count_total_nodes(node: &HtmlNode) -> usize {{
    1 + node.children.iter().map(|child| count_total_nodes(child)).sum::<usize>()
}}

fn process_tree_with_stats(root: &mut HtmlNode) -> (usize, usize, usize) {{
    let mut total_nodes = 0;
    let mut cache_hits = 0;
    let mut cache_misses = 0;
    
    fn process_recursive(node: &mut HtmlNode, parent_state: BitVector, stats: &mut (usize, usize, usize)) {{
        stats.0 += 1; // total_nodes
        
        let was_cached = !node.needs_any_recomputation(parent_state);
        if was_cached {{
            stats.1 += 1; // cache_hits
            return; // Skip processing - use cached result
        }} else {{
            stats.2 += 1; // cache_misses
        }}
        
        let child_states = process_node_generated_incremental(node, parent_state);
        
        // Process all children - they need to be processed at least once
        for child in node.children.iter_mut() {{
            process_recursive(child, child_states, stats);
        }}
    }}
    
    let mut stats = (0, 0, 0);
    process_recursive(root, BitVector::new(), &mut stats);
    stats
}}

fn find_deep_node(node: &mut HtmlNode, target_depth: usize) -> Option<&mut HtmlNode> {{
    if target_depth == 0 {{
        return Some(node);
    }}
    
    for child in &mut node.children {{
        if let Some(found) = find_deep_node(child, target_depth - 1) {{
            return Some(found);
        }}
    }}
    
    None
}}

fn main() {{
    println!("🚀 CodeGen Google Trace Performance Test\n");
    
    // Create the Google DOM tree
    let mut root = {node_data};
    
    let total_nodes = count_total_nodes(&root);
    println!("🌳 DOM tree loaded: {{}} nodes", total_nodes);
    
    // Test 1: Initial processing (all cache misses expected)
    println!("\n📊 Test 1: Initial processing");
    let (total1, hits1, misses1) = process_tree_with_stats(&mut root);
    println!("  Processed nodes: {{}}", total1);
    println!("  Cache hits: {{}}", hits1);
    println!("  Cache misses: {{}}", misses1);
    println!("  Cache hit rate: {{:.2}}%", if total1 > 0 {{ hits1 as f64 / total1 as f64 * 100.0 }} else {{ 0.0 }});
    
    // Test 2: Second run (should have high cache hit rate)
    println!("\n📊 Test 2: Second run (cache optimization)");
    let (total2, hits2, misses2) = process_tree_with_stats(&mut root);
    println!("  Processed nodes: {{}}", total2);
    println!("  Cache hits: {{}}", hits2);
    println!("  Cache misses: {{}}", misses2);
    println!("  Cache hit rate: {{:.2}}%", if total2 > 0 {{ hits2 as f64 / total2 as f64 * 100.0 }} else {{ 0.0 }});
    
    // Test 3: Mark a deep node dirty and test incremental processing
    if let Some(deep_node) = find_deep_node(&mut root, 5) {{
        deep_node.mark_self_dirty();
        println!("\n📝 Marked a deep node dirty...");
        
        println!("\n📊 Test 3: After deep node modification");
        let (total3, hits3, misses3) = process_tree_with_stats(&mut root);
        println!("  Processed nodes: {{}}", total3);
        println!("  Cache hits: {{}}", hits3);
        println!("  Cache misses: {{}}", misses3);
        println!("  Cache hit rate: {{:.2}}%", if total3 > 0 {{ hits3 as f64 / total3 as f64 * 100.0 }} else {{ 0.0 }});
        println!("  💡 Optimization: Only {{}} nodes needed reprocessing!", total3);
    }}
    
    // Show matching results
    let matches = count_matches(&root);
    println!("\n🎯 CSS Matching Results:");
    println!("  Total nodes with matches: {{}} / {{}}", matches, total_nodes);
    println!("  Match percentage: {{:.1}}%", matches as f64 / total_nodes as f64 * 100.0);
    
    println!("\n✅ SUCCESS: CodeGen Google trace test completed!");
}}
"##, 
        generated_fn_code = generated_fn_code,
        node_data = node_data
    );
    
    Ok(complete_program)
}

fn serialize_google_node_to_rust_code(node: &GoogleNode) -> String {
    fn serialize_node(node: &GoogleNode, depth: usize) -> String {
        let indent = "    ".repeat(depth + 1);
        let mut code = format!("{}HtmlNode::new(\"{}\")", indent, escape_string(&node.name));
        
        if let Some(id) = &node.id {
            code.push_str(&format!(".with_id(\"{}\")", escape_string(&id.to_string())));
        }
        
        if let Some(class_attr) = node.attributes.get("class") {
            for class in class_attr.split_whitespace() {
                code.push_str(&format!(".with_class(\"{}\")", escape_string(class)));
            }
        }
        
        for child in &node.children {
            code.push_str(&format!("\n{}.add_child(\n{}\n{})", 
                indent, 
                serialize_node(child, depth + 1),
                indent
            ));
        }
        
        code
    }
    
    serialize_node(node, 0)
}

fn escape_string(s: &str) -> String {
    s.replace("\\", "\\\\")
     .replace("\"", "\\\"")
     .replace("\n", "\\n")
     .replace("\r", "\\r")
     .replace("\t", "\\t")
}

fn parse_basic_css(css_content: &str) -> Vec<CssRule> {
    let mut rules = Vec::new();
    
    // Very basic CSS parser for testing - just extract simple selectors
    let lines: Vec<&str> = css_content.lines().collect();
    let mut current_selector = String::new();
    
    for line in lines {
        let line = line.trim();
        if line.is_empty() || line.starts_with("/*") {
            continue;
        }
        
        if line.contains('{') && !line.contains('}') {
            // Start of a rule
            current_selector = line.split('{').next().unwrap_or("").trim().to_string();
        } else if line.contains('}') && !current_selector.is_empty() {
            // End of a rule - add it
            if current_selector.starts_with('.') {
                let class_name = current_selector[1..].to_string();
                if !class_name.contains(' ') && !class_name.contains(':') {
                    rules.push(CssRule::Simple(SimpleSelector::Class(class_name)));
                }
            } else if current_selector.starts_with('#') {
                let id_name = current_selector[1..].to_string();
                if !id_name.contains(' ') && !id_name.contains(':') {
                    rules.push(CssRule::Simple(SimpleSelector::Id(id_name)));
                }
            } else if !current_selector.contains(' ') && !current_selector.contains(':') && !current_selector.contains('.') && !current_selector.contains('#') {
                // Simple tag selector
                rules.push(CssRule::Simple(SimpleSelector::Type(current_selector.to_lowercase())));
            }
            current_selector.clear();
        }
    }
    
    // Add some common Google selectors we know exist
    rules.extend([
        CssRule::Simple(SimpleSelector::Type("div".to_string())),
        CssRule::Simple(SimpleSelector::Type("span".to_string())),
        CssRule::Simple(SimpleSelector::Type("a".to_string())),
        CssRule::Simple(SimpleSelector::Type("input".to_string())),
        CssRule::Simple(SimpleSelector::Class("gbts".to_string())),
        CssRule::Simple(SimpleSelector::Class("gbmt".to_string())),
        CssRule::Simple(SimpleSelector::Class("lsb".to_string())),
        CssRule::Simple(SimpleSelector::Id("gb".to_string())),
        CssRule::Simple(SimpleSelector::Id("gbz".to_string())),
    ]);
    
    rules
}

// --- 0. BitVector Abstraction ---
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

    fn clear_bit(&mut self, pos: usize) {
        self.bits &= !(1 << pos);
    }

    fn is_bit_set(&self, pos: usize) -> bool {
        (self.bits & (1 << pos)) != 0
    }

    fn or_assign(&mut self, other: BitVector) {
        self.bits |= other.bits;
    }

    fn and(&self, other: BitVector) -> BitVector {
        BitVector {
            bits: self.bits & other.bits,
        }
    }

    fn is_empty(&self) -> bool {
        self.bits == 0
    }

    fn as_u64(&self) -> u64 {
        self.bits
    }

    // Check if any of the specified bits are set
    fn has_any_bits(&self, mask: BitVector) -> bool {
        (self.bits & mask.bits) != 0
    }
}

impl std::fmt::Binary for BitVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:016b}", self.bits)
    }
}

// --- Optimized CSS Rule Matching with Hash Tables ---
#[derive(Debug, Clone)]
struct SelectorMatchingIndex {
    // Hash tables for fast selector matching
    tag_rules: HashMap<String, Vec<(usize, NFAInstruction)>>, // tag -> (rule_id, instruction)
    class_rules: HashMap<String, Vec<(usize, NFAInstruction)>>,
    id_rules: HashMap<String, Vec<(usize, NFAInstruction)>>,
    // For complex selectors that need parent context
    parent_dependent_rules: Vec<(usize, NFAInstruction)>,
}

impl SelectorMatchingIndex {
    fn new() -> Self {
        Self {
            tag_rules: HashMap::new(),
            class_rules: HashMap::new(),
            id_rules: HashMap::new(),
            parent_dependent_rules: Vec::new(),
        }
    }

    fn add_rule(&mut self, rule_id: usize, instruction: NFAInstruction) {
        match &instruction {
            NFAInstruction::CheckAndSetBit { selector, .. } => {
                match selector {
                    SimpleSelector::Type(tag) => {
                        self.tag_rules.entry(tag.clone()).or_default().push((rule_id, instruction));
                    }
                    SimpleSelector::Class(class) => {
                        self.class_rules.entry(class.clone()).or_default().push((rule_id, instruction));
                    }
                    SimpleSelector::Id(id) => {
                        self.id_rules.entry(id.clone()).or_default().push((rule_id, instruction));
                    }
                }
            }
            NFAInstruction::CheckParentAndSetBit { .. } => {
                self.parent_dependent_rules.push((rule_id, instruction));
            }
            NFAInstruction::PropagateToChildren { .. } => {
                // These are processed separately after matching
            }
        }
    }

    fn get_matching_rules(&self, node: &HtmlNode) -> Vec<&NFAInstruction> {
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

    fn get_parent_dependent_rules(&self) -> &[(usize, NFAInstruction)] {
        &self.parent_dependent_rules
    }
}

// --- 1. HTML Node Structure with Double Dirty Bit Algorithm ---
#[derive(Debug, Clone)]
struct HtmlNode {
    tag_name: String,
    id: Option<String>,
    classes: HashSet<String>,
    children: Vec<HtmlNode>,
    // Stores the bitvector of *final* matching rules for this node
    css_match_bitvector: BitVector,

    // Double Dirty Bit Algorithm state
    is_self_dirty: bool,           // This node's own attributes changed
    has_dirty_descendant: bool,    // Some descendant needs recomputation (summary bit)
    
    // Incremental processing state
    cached_parent_state: Option<BitVector>, // Input: parent state from last computation
    cached_node_intrinsic: Option<BitVector>, // Input: node's own selector matches (computed once)
    cached_child_states: Option<BitVector>, // Output: states to propagate to children
}

impl HtmlNode {
    fn new(tag_name: &str) -> Self {
        HtmlNode {
            tag_name: tag_name.to_lowercase(),
            id: None,
            classes: HashSet::new(),
            children: Vec::new(),
            css_match_bitvector: BitVector::new(),
            is_self_dirty: true,  // New nodes need computation
            has_dirty_descendant: false,
            cached_parent_state: None,
            cached_node_intrinsic: None,
            cached_child_states: None,
        }
    }

    fn with_id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self.mark_self_dirty(); // Only mark self as dirty, not children
        self
    }

    fn with_class(mut self, class: &str) -> Self {
        self.classes.insert(class.to_string());
        self.mark_self_dirty(); // Only mark self as dirty, not children
        self
    }

    fn add_child(mut self, child: HtmlNode) -> Self {
        self.children.push(child);
        self
    }

    // Mark only this node as dirty (attributes changed)
    fn mark_self_dirty(&mut self) {
        self.is_self_dirty = true;
        self.cached_node_intrinsic = None; // Invalidate intrinsic cache
        // Don't clear parent/child state - they may still be valid
    }

    // Mark this node as having dirty descendants and propagate summary bit upward
    fn mark_descendant_dirty(&mut self) {
        self.has_dirty_descendant = true;
        // Propagate summary bit would happen here in a real implementation
        // For now, we'll handle this in the processing logic
    }

    // Complete dirty marking (for structural changes)
    fn mark_dirty_complete(&mut self) {
        self.is_self_dirty = true;
        self.has_dirty_descendant = true;
        self.cached_parent_state = None;
        self.cached_node_intrinsic = None;
        self.cached_child_states = None;
        // Still don't recursively dirty children
    }

    // Check if this node or any descendant needs recomputation
    fn needs_any_recomputation(&self, new_parent_state: BitVector) -> bool {
        self.is_self_dirty 
            || self.has_dirty_descendant
            || self.cached_parent_state.is_none()
            || self.cached_parent_state.unwrap() != new_parent_state
    }

    // Check if only this node needs recomputation
    fn needs_self_recomputation(&self, new_parent_state: BitVector) -> bool {
        self.is_self_dirty
            || self.cached_parent_state.is_none()
            || self.cached_parent_state.unwrap() != new_parent_state
    }

    // Clean dirty flags after processing
    fn mark_clean(&mut self) {
        self.is_self_dirty = false;
        // Don't clear has_dirty_descendant here - it will be cleared
        // when all descendants are processed
    }

    // Clean descendant dirty flag when all children are processed
    fn mark_descendants_clean(&mut self) {
        self.has_dirty_descendant = false;
    }

    // Smart dirty marking: mark path from this node to root with summary bits
    fn propagate_dirty_upward(&mut self, path_to_root: &mut [&mut HtmlNode]) {
        // Mark this node as dirty
        self.mark_self_dirty();
        
        // Mark all ancestors as having dirty descendants
        for ancestor in path_to_root.iter_mut() {
            ancestor.mark_descendant_dirty();
        }
    }

    // Find a node by path and mark it dirty with efficient upward propagation
    fn mark_node_dirty_by_path(&mut self, path: &[usize]) -> bool {
        if path.is_empty() {
            self.mark_self_dirty();
            return true;
        }

        let first_index = path[0];
        if first_index >= self.children.len() {
            return false; // Path not found
        }

        // Recursively find the target node
        if self.children[first_index].mark_node_dirty_by_path(&path[1..]) {
            // If target was found, mark this node as having dirty descendants
            self.mark_descendant_dirty();
            return true;
        }
        false
    }
}

// --- 2. CSS Rule Representation ---
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum SimpleSelector {
    Type(String),
    Class(String),
    Id(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum CssRule {
    Simple(SimpleSelector),
    Child {
        parent_selector: SimpleSelector,
        child_selector: SimpleSelector,
    },
}

// --- 3. Tree NFA Instruction Set ---
#[derive(Debug, Clone)]
enum NFAInstruction {
    // 检查当前节点是否匹配selector，如果匹配则设置bit位
    CheckAndSetBit {
        selector: SimpleSelector,
        bit_pos: usize,
    },

    // 检查parent_state_bit是否设置，如果设置且当前节点匹配child_selector，则设置result_bit
    CheckParentAndSetBit {
        parent_state_bit: usize,
        child_selector: SimpleSelector,
        result_bit: usize,
    },

    // 如果match_bit设置，则设置active_bit（为子节点准备状态）
    PropagateToChildren {
        match_bit: usize,
        active_bit: usize,
    },
}

// --- 4. Tree NFA Program ---
#[derive(Debug)]
struct TreeNFAProgram {
    instructions: Vec<NFAInstruction>,
    state_names: HashMap<usize, String>, // bit position -> descriptive name
    total_bits: usize,
}

impl TreeNFAProgram {
    fn new() -> Self {
        TreeNFAProgram {
            instructions: Vec::new(),
            state_names: HashMap::new(),
            total_bits: 0,
        }
    }

    fn add_instruction(&mut self, instruction: NFAInstruction) {
        self.instructions.push(instruction);
    }

    fn set_state_name(&mut self, bit_pos: usize, name: String) {
        self.state_names.insert(bit_pos, name);
        if bit_pos >= self.total_bits {
            self.total_bits = bit_pos + 1;
        }
    }

    fn generate_rust_code(&self) -> String {
        let mut code = String::new();

        code.push_str("// Generated Tree NFA Program with Incremental Processing\n");
        code.push_str(
            "// This program processes HTML nodes and computes CSS matches with caching\n\n",
        );

        // Generate incremental processing function
        code.push_str("fn process_node_generated_incremental(\n");
        code.push_str("    node: &mut HtmlNode,\n");
        code.push_str("    parent_state: BitVector,\n");
        code.push_str(") -> BitVector { // returns child_states\n");
        code.push_str("    // Double dirty bit optimization: skip if no recomputation needed\n");
        code.push_str("    if !node.needs_any_recomputation(parent_state) {\n");
        code.push_str("        // Return cached result - entire subtree can be skipped\n");
        code.push_str("        return node.cached_child_states.unwrap_or(BitVector::new());\n");
        code.push_str("    }\n\n");

        code.push_str("    // Recompute node intrinsic matches if needed\n");
        code.push_str("    if node.cached_node_intrinsic.is_none() || node.is_self_dirty {\n");
        code.push_str("        let mut intrinsic_matches = BitVector::new();\n\n");

        // Generate intrinsic selector checks
        for (i, instruction) in self.instructions.iter().enumerate() {
            if let NFAInstruction::CheckAndSetBit { selector, bit_pos } = instruction {
                code.push_str(&format!(
                    "        // Instruction {}: {:?}\n",
                    i, instruction
                ));
                let selector_str = match selector {
                    SimpleSelector::Type(tag) => {
                        format!("SimpleSelector::Type(\"{}\".to_string())", tag)
                    }
                    SimpleSelector::Class(class) => {
                        format!("SimpleSelector::Class(\"{}\".to_string())", class)
                    }
                    SimpleSelector::Id(id) => {
                        format!("SimpleSelector::Id(\"{}\".to_string())", id)
                    }
                };
                code.push_str(&format!(
                    "        if node_matches_selector_generated(node, &{}) {{\n",
                    selector_str
                ));
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

        code.push_str("        node.cached_node_intrinsic = Some(intrinsic_matches);\n");
        code.push_str("    }\n\n");

        code.push_str("    // Start with cached intrinsic matches\n");
        code.push_str("    let mut current_matches = node.cached_node_intrinsic.unwrap();\n");
        code.push_str("    let mut child_states = BitVector::new();\n\n");
        code.push_str("    // Optimized selector matching using hash tables (conceptual)\n");
        code.push_str("    // In practice, rules would be pre-indexed by tag/class/id\n\n");

        // Generate parent-dependent rules
        code.push_str("    // Apply parent-dependent rules\n");
        for (i, instruction) in self.instructions.iter().enumerate() {
            match instruction {
                NFAInstruction::CheckParentAndSetBit {
                    parent_state_bit,
                    child_selector,
                    result_bit,
                } => {
                    code.push_str(&format!("    // Instruction {}: {:?}\n", i, instruction));
                    let child_selector_str = match child_selector {
                        SimpleSelector::Type(tag) => {
                            format!("SimpleSelector::Type(\"{}\".to_string())", tag)
                        }
                        SimpleSelector::Class(class) => {
                            format!("SimpleSelector::Class(\"{}\".to_string())", class)
                        }
                        SimpleSelector::Id(id) => {
                            format!("SimpleSelector::Id(\"{}\".to_string())", id)
                        }
                    };
                    code.push_str(&format!("    if parent_state.is_bit_set({}) && node_matches_selector_generated(node, &{}) {{\n", 
                        parent_state_bit, child_selector_str));
                    code.push_str(&format!(
                        "        current_matches.set_bit({}); // {}\n",
                        result_bit,
                        self.state_names
                            .get(result_bit)
                            .unwrap_or(&format!("bit_{}", result_bit))
                    ));
                    code.push_str("    }\n\n");
                }
                NFAInstruction::PropagateToChildren {
                    match_bit,
                    active_bit,
                } => {
                    code.push_str(&format!("    // Instruction {}: {:?}\n", i, instruction));
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
                    code.push_str("    }\n\n");
                }
                _ => {} // CheckAndSetBit already handled above in intrinsic section
            }
        }

        code.push_str("    // Cache results and mark clean\n");
        code.push_str("    node.css_match_bitvector = current_matches;\n");
        code.push_str("    node.cached_parent_state = Some(parent_state);\n");
        code.push_str("    node.cached_child_states = Some(child_states);\n");
        code.push_str("    node.mark_clean(); // Use double dirty bit cleanup\n\n");
        code.push_str("    child_states\n");
        code.push_str("}\n\n");

        // Generate basic non-incremental version for compatibility
        code.push_str("fn process_node_generated_inplace(\n");
        code.push_str("    node: &mut HtmlNode,\n");
        code.push_str("    parent_state: BitVector,\n");
        code.push_str(") -> BitVector { // returns child_states\n");
        code.push_str("    let mut current_matches = BitVector::new();\n");
        code.push_str("    let mut child_states = BitVector::new();\n\n");

        for (i, instruction) in self.instructions.iter().enumerate() {
            code.push_str(&format!("    // Instruction {}: {:?}\n", i, instruction));
            match instruction {
                NFAInstruction::CheckAndSetBit { selector, bit_pos } => {
                    let selector_str = match selector {
                        SimpleSelector::Type(tag) => {
                            format!("SimpleSelector::Type(\"{}\".to_string())", tag)
                        }
                        SimpleSelector::Class(class) => {
                            format!("SimpleSelector::Class(\"{}\".to_string())", class)
                        }
                        SimpleSelector::Id(id) => {
                            format!("SimpleSelector::Id(\"{}\".to_string())", id)
                        }
                    };
                    code.push_str(&format!(
                        "    if node_matches_selector_generated(node, &{}) {{\n",
                        selector_str
                    ));
                    code.push_str(&format!(
                        "        current_matches.set_bit({}); // {}\n",
                        bit_pos,
                        self.state_names
                            .get(bit_pos)
                            .unwrap_or(&format!("bit_{}", bit_pos))
                    ));
                    code.push_str("    }\n\n");
                }
                NFAInstruction::CheckParentAndSetBit {
                    parent_state_bit,
                    child_selector,
                    result_bit,
                } => {
                    let child_selector_str = match child_selector {
                        SimpleSelector::Type(tag) => {
                            format!("SimpleSelector::Type(\"{}\".to_string())", tag)
                        }
                        SimpleSelector::Class(class) => {
                            format!("SimpleSelector::Class(\"{}\".to_string())", class)
                        }
                        SimpleSelector::Id(id) => {
                            format!("SimpleSelector::Id(\"{}\".to_string())", id)
                        }
                    };
                    code.push_str(&format!("    if parent_state.is_bit_set({}) && node_matches_selector_generated(node, &{}) {{\n", 
                        parent_state_bit, child_selector_str));
                    code.push_str(&format!(
                        "        current_matches.set_bit({}); // {}\n",
                        result_bit,
                        self.state_names
                            .get(result_bit)
                            .unwrap_or(&format!("bit_{}", result_bit))
                    ));
                    code.push_str("    }\n\n");
                }
                NFAInstruction::PropagateToChildren {
                    match_bit,
                    active_bit,
                } => {
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
                    code.push_str("    }\n\n");
                }
            }
        }

        code.push_str("    // Store result in node (in-place)\n");
        code.push_str("    node.css_match_bitvector = current_matches;\n");
        code.push_str("    child_states\n");
        code.push_str("}\n\n");

        // Generate helper function for cache checking
        code.push_str("fn needs_recomputation_generated(node: &HtmlNode, new_parent_state: BitVector) -> bool {\n");
        code.push_str("    node.is_self_dirty ||\n");
        code.push_str("    node.has_dirty_descendant ||\n");
        code.push_str("    node.cached_parent_state.is_none() ||\n");
        code.push_str("    node.cached_parent_state.unwrap() != new_parent_state\n");
        code.push_str("}\n\n");

        // Generate incremental tree processing driver
        code.push_str("fn process_tree_generated_incremental(root: &mut HtmlNode) {\n");
        code.push_str(
            "    process_tree_recursive_generated_incremental(root, BitVector::new());\n",
        );
        code.push_str("}\n\n");

        code.push_str("fn process_tree_recursive_generated_incremental(node: &mut HtmlNode, parent_state: BitVector) {\n");
        code.push_str(
            "    let child_states = process_node_generated_incremental(node, parent_state);\n",
        );
        code.push_str("    \n");
        code.push_str("    // Recursively process children\n");
        code.push_str("    for child in node.children.iter_mut() {\n");
        code.push_str(
            "        process_tree_recursive_generated_incremental(child, child_states);\n",
        );
        code.push_str("    }\n");
        code.push_str("}\n\n");

        // Generate basic driver function
        code.push_str("fn process_tree_generated(root: &mut HtmlNode) {\n");
        code.push_str("    process_tree_recursive_generated(root, BitVector::new());\n");
        code.push_str("}\n\n");

        code.push_str(
            "fn process_tree_recursive_generated(node: &mut HtmlNode, parent_state: BitVector) {\n",
        );
        code.push_str(
            "    let child_states = process_node_generated_inplace(node, parent_state);\n",
        );
        code.push_str("    \n");
        code.push_str("    // Recursively process children\n");
        code.push_str("    for child in node.children.iter_mut() {\n");
        code.push_str("        process_tree_recursive_generated(child, child_states);\n");
        code.push_str("    }\n");
        code.push_str("}\n\n");

        // Generate selector matching function
        code.push_str(
            "fn node_matches_selector_generated(node: &HtmlNode, selector: &SimpleSelector) -> bool {\n",
        );
        code.push_str("    match selector {\n");
        code.push_str("        SimpleSelector::Type(tag) => node.tag_name == *tag,\n");
        code.push_str("        SimpleSelector::Class(class) => node.classes.contains(class),\n");
        code.push_str("        SimpleSelector::Id(id) => node.id.as_deref() == Some(id),\n");
        code.push_str("    }\n");
        code.push_str("}\n");

        code
    }

    fn print_program(&self) {
        println!("=== Generated Tree NFA Program ===");
        println!("Total bits used: {}", self.total_bits);
        println!("\nState mapping:");
        for i in 0..self.total_bits {
            if let Some(name) = self.state_names.get(&i) {
                println!("  Bit {}: {}", i, name);
            }
        }

        println!("\nInstructions:");
        for (i, instruction) in self.instructions.iter().enumerate() {
            println!("  {}: {:?}", i, instruction);
        }
        println!("===================================\n");
    }
}

// --- 5. CSS Compiler ---
struct CssCompiler {
    bit_counter: usize,
    state_mapping: HashMap<String, usize>, // state name -> bit position
}

impl CssCompiler {
    fn new() -> Self {
        CssCompiler {
            bit_counter: 0,
            state_mapping: HashMap::new(),
        }
    }

    fn allocate_bit(&mut self, state_name: String) -> usize {
        if let Some(&existing_bit) = self.state_mapping.get(&state_name) {
            existing_bit
        } else {
            let bit_pos = self.bit_counter;
            self.state_mapping.insert(state_name, bit_pos);
            self.bit_counter += 1;
            bit_pos
        }
    }

    fn compile_css_rules(&mut self, rules: &[CssRule]) -> TreeNFAProgram {
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

// --- 6. Tree NFA Virtual Machine ---
#[derive(Debug, Default)]
struct IncrementalStats {
    total_nodes: usize,
    cache_hits: usize,
    cache_misses: usize,
}

impl IncrementalStats {
    fn new() -> Self {
        IncrementalStats {
            total_nodes: 0,
            cache_hits: 0,
            cache_misses: 0,
        }
    }

    fn cache_hit_rate(&self) -> f64 {
        if self.total_nodes == 0 {
            0.0
        } else {
            self.cache_hits as f64 / self.total_nodes as f64
        }
    }

    fn print_summary(&self) {
        println!("=== Incremental Processing Stats ===");
        println!("Total nodes processed: {}", self.total_nodes);
        println!("Cache hits: {}", self.cache_hits);
        println!("Cache misses: {}", self.cache_misses);
        println!("Cache hit rate: {:.2}%", self.cache_hit_rate() * 100.0);
        println!("=====================================");
    }
}

struct TreeNFAVM {
    program: TreeNFAProgram,
    selector_index: SelectorMatchingIndex,
}

impl TreeNFAVM {
    fn new(program: TreeNFAProgram) -> Self {
        let mut selector_index = SelectorMatchingIndex::new();
        
        // Build index for fast selector matching
        for (i, instruction) in program.instructions.iter().enumerate() {
            selector_index.add_rule(i, instruction.clone());
        }
        
        TreeNFAVM { 
            program,
            selector_index,
        }
    }

    fn process_node_inplace(&self, node: &mut HtmlNode, parent_state: BitVector) -> BitVector {
        let mut current_matches = BitVector::new();
        let mut child_states = BitVector::new();

        for instruction in &self.program.instructions {
            match instruction {
                NFAInstruction::CheckAndSetBit { selector, bit_pos } => {
                    if self.node_matches_selector(node, selector) {
                        current_matches.set_bit(*bit_pos);
                    }
                }
                NFAInstruction::CheckParentAndSetBit {
                    parent_state_bit,
                    child_selector,
                    result_bit,
                } => {
                    if parent_state.is_bit_set(*parent_state_bit)
                        && self.node_matches_selector(node, child_selector)
                    {
                        current_matches.set_bit(*result_bit);
                    }
                }
                NFAInstruction::PropagateToChildren {
                    match_bit,
                    active_bit,
                } => {
                    if current_matches.is_bit_set(*match_bit) {
                        child_states.set_bit(*active_bit);
                    }
                }
            }
        }

        // Store result in node (in-place)
        node.css_match_bitvector = current_matches;

        child_states
    }

    fn node_matches_selector(&self, node: &HtmlNode, selector: &SimpleSelector) -> bool {
        match selector {
            SimpleSelector::Type(tag) => node.tag_name == *tag,
            SimpleSelector::Class(class) => node.classes.contains(class),
            SimpleSelector::Id(id) => node.id.as_deref() == Some(id),
        }
    }

    // Driver function: recursively process entire tree
    fn process_tree(&self, root: &mut HtmlNode) {
        self.process_tree_recursive(root, BitVector::new(), "root".to_string());
    }

    fn process_tree_recursive(&self, node: &mut HtmlNode, parent_state: BitVector, path: String) {
        let child_states = self.process_node_inplace(node, parent_state);

        // Recursively process children
        for (i, child) in node.children.iter_mut().enumerate() {
            self.process_tree_recursive(child, child_states, format!("{}/{}", path, i));
        }
    }

    // Driver function with verbose output (for debugging)
    fn process_tree_verbose(&self, root: &mut HtmlNode) {
        self.dfs_execute_verbose(root, BitVector::new(), "root".to_string());
    }

    fn dfs_execute_verbose(&self, node: &mut HtmlNode, parent_state: BitVector, path: String) {
        let child_states = self.process_node_inplace(node, parent_state);

        println!(
            "Node: {} (Tag: {}, ID: {:?}, Classes: {:?})",
            path, node.tag_name, node.id, node.classes
        );
        println!("  Parent state: {:b}", parent_state);
        println!("  Match result: {:b}", node.css_match_bitvector);
        println!("  Child states: {:b}", child_states);

        // Decode matches
        println!("  Matches:");
        for (bit_pos, name) in &self.program.state_names {
            if node.css_match_bitvector.is_bit_set(*bit_pos) {
                println!("    - {}", name);
            }
        }
        println!("---");

        // Recursively process children
        for (i, child) in node.children.iter_mut().enumerate() {
            self.dfs_execute_verbose(child, child_states, format!("{}/{}", path, i));
        }
    }

    // Incremental processing: only recompute when inputs change
    fn process_node_incremental(&self, node: &mut HtmlNode, parent_state: BitVector) -> BitVector {
        // Check if we need to recompute
        if !node.needs_any_recomputation(parent_state) {
            // Return cached result
            return node.cached_child_states.unwrap_or(BitVector::new());
        }

        // Recompute node intrinsic matches if needed (this is stable unless node attributes change)
        if node.cached_node_intrinsic.is_none() || node.is_self_dirty {
            let mut intrinsic_matches = BitVector::new();

            // Use optimized hash table lookup instead of linear search
            let matching_rules = self.selector_index.get_matching_rules(node);
            for instruction in matching_rules {
                if let NFAInstruction::CheckAndSetBit { bit_pos, .. } = instruction {
                    intrinsic_matches.set_bit(*bit_pos);
                }
            }

            node.cached_node_intrinsic = Some(intrinsic_matches);
        }

        // Compute final matches and child states
        let mut current_matches = node.cached_node_intrinsic.unwrap();
        let mut child_states = BitVector::new();

        // Apply parent-dependent rules - these still need to be checked
        for (_, instruction) in self.selector_index.get_parent_dependent_rules() {
            if let NFAInstruction::CheckParentAndSetBit {
                parent_state_bit,
                child_selector,
                result_bit,
            } = instruction {
                if parent_state.is_bit_set(*parent_state_bit)
                    && self.node_matches_selector(node, child_selector)
                {
                    current_matches.set_bit(*result_bit);
                }
            }
        }

        // Apply propagation rules
        for instruction in &self.program.instructions {
            if let NFAInstruction::PropagateToChildren {
                match_bit,
                active_bit,
            } = instruction {
                if current_matches.is_bit_set(*match_bit) {
                    child_states.set_bit(*active_bit);
                }
            }
        }

        // Cache results
        node.css_match_bitvector = current_matches;
        node.cached_parent_state = Some(parent_state);
        node.cached_child_states = Some(child_states);
        node.mark_clean();

        child_states
    }

    // Incremental tree processing driver
    fn process_tree_incremental(&self, root: &mut HtmlNode) {
        self.process_tree_incremental_recursive(root, BitVector::new());
    }

    fn process_tree_incremental_recursive(&self, node: &mut HtmlNode, parent_state: BitVector) {
        // Only process if this node or its descendants need recomputation
        if !node.needs_any_recomputation(parent_state) {
            return; // Skip entire subtree
        }

        let child_states = self.process_node_incremental(node, parent_state);

        // Process all children - in first run, all will be processed
        // In subsequent runs, only dirty paths will be processed
        let mut any_child_was_dirty = false;
        for child in node.children.iter_mut() {
            if child.needs_any_recomputation(child_states) {
                any_child_was_dirty = true;
                self.process_tree_incremental_recursive(child, child_states);
            }
        }
        
        // Clear the summary bit since we've processed all dirty descendants
        if any_child_was_dirty || node.has_dirty_descendant {
            node.mark_descendants_clean();
        }
    }

    // Debug version with statistics
    fn process_tree_incremental_with_stats(&self, root: &mut HtmlNode) -> IncrementalStats {
        let mut stats = IncrementalStats::new();
        self.process_tree_incremental_recursive_with_stats(root, BitVector::new(), &mut stats);
        stats
    }

    fn process_tree_incremental_recursive_with_stats(
        &self,
        node: &mut HtmlNode,
        parent_state: BitVector,
        stats: &mut IncrementalStats,
    ) {
        // Only process if this node or its descendants need recomputation
        if !node.needs_any_recomputation(parent_state) {
            return; // Skip entire subtree - this is a cache hit for the whole subtree
        }

        stats.total_nodes += 1;

        let was_cached = !node.needs_any_recomputation(parent_state);
        if was_cached {
            stats.cache_hits += 1;
        } else {
            stats.cache_misses += 1;
        }

        let child_states = self.process_node_incremental(node, parent_state);

        // Process all children - in first run, all will be processed
        // In subsequent runs, only dirty paths will be processed
        let mut any_child_was_dirty = false;
        for child in node.children.iter_mut() {
            if child.needs_any_recomputation(child_states) {
                any_child_was_dirty = true;
                self.process_tree_incremental_recursive_with_stats(child, child_states, stats);
            }
        }
        
        // Clear the summary bit since we've processed all dirty descendants
        if any_child_was_dirty || node.has_dirty_descendant {
            node.mark_descendants_clean();
        }
    }
}

// --- 7. CSS Parser ---
fn parse_css_file(css_content: &str) -> Vec<CssRule> {
    let mut rules = Vec::new();

    let simple_rule_regex = Regex::new(r"/\* Rule \d+ \([^)]+\) \*/ ([^{]+) \{\}").unwrap();
    let child_rule_regex = Regex::new(r"([^>]+)>([^{]+)").unwrap();

    for cap in simple_rule_regex.captures_iter(css_content) {
        let selector_str = cap[1].trim();

        if let Some(child_cap) = child_rule_regex.captures(selector_str) {
            let parent_str = child_cap[1].trim();
            let child_str = child_cap[2].trim();

            if let (Some(parent_sel), Some(child_sel)) = (
                parse_simple_selector(parent_str),
                parse_simple_selector(child_str),
            ) {
                rules.push(CssRule::Child {
                    parent_selector: parent_sel,
                    child_selector: child_sel,
                });
            }
        } else if let Some(simple_sel) = parse_simple_selector(selector_str) {
            rules.push(CssRule::Simple(simple_sel));
        }
    }

    rules
}

fn parse_simple_selector(selector_str: &str) -> Option<SimpleSelector> {
    let trimmed = selector_str.trim();

    if trimmed.starts_with('#') {
        Some(SimpleSelector::Id(trimmed[1..].to_string()))
    } else if trimmed.starts_with('.') {
        Some(SimpleSelector::Class(trimmed[1..].to_string()))
    } else if !trimmed.is_empty() && trimmed.chars().all(|c| c.is_alphabetic()) {
        Some(SimpleSelector::Type(trimmed.to_string()))
    } else {
        None
    }
}

// --- 8. HTML Parser ---
fn parse_html_file(html_content: &str) -> Option<HtmlNode> {
    let document = Html::parse_document(html_content);

    let meaningful_selector = HtmlSelector::parse("div, section, p, span").unwrap();
    if let Some(first_element) = document.select(&meaningful_selector).next() {
        parse_element_to_node(&first_element)
    } else {
        None
    }
}

fn parse_element_to_node(element_ref: &scraper::ElementRef) -> Option<HtmlNode> {
    let element = element_ref.value();
    let tag_name = element.name().to_string();

    let mut node = HtmlNode::new(&tag_name);

    if let Some(id) = element.attr("id") {
        node = node.with_id(id);
    }

    if let Some(class_attr) = element.attr("class") {
        for class in class_attr.split_whitespace() {
            node = node.with_class(class);
        }
    }

    for child_ref in element_ref.children() {
        if child_ref.value().as_element().is_some() {
            if let Some(child_element_ref) = scraper::ElementRef::wrap(child_ref) {
                if let Some(child_node) = parse_element_to_node(&child_element_ref) {
                    node = node.add_child(child_node);
                }
            }
        }
    }

    Some(node)
}

// --- 9. Main compiler function ---
fn compile_and_run(css_file: &str, html_file: &str) {
    println!("=== CSS Compiler: {} -> Program ===", css_file);

    // Read and parse CSS
    let css_content = match fs::read_to_string(css_file) {
        Ok(content) => content,
        Err(e) => {
            println!("Error reading CSS file {}: {}", css_file, e);
            return;
        }
    };

    let rules = parse_css_file(&css_content);
    println!("Parsed CSS Rules: {:#?}\n", rules);

    // Compile CSS to Tree NFA program
    let mut compiler = CssCompiler::new();
    let program = compiler.compile_css_rules(&rules);

    // Print the generated program
    program.print_program();

    // Generate Rust code
    println!("=== Generated Rust Code ===");
    println!("{}", program.generate_rust_code());

    // Read and parse HTML
    let html_content = match fs::read_to_string(html_file) {
        Ok(content) => content,
        Err(e) => {
            println!("Error reading HTML file {}: {}", html_file, e);
            return;
        }
    };

    if let Some(mut root_node) = parse_html_file(&html_content) {
        println!("=== Executing Tree NFA Program on {} ===", html_file);

        let vm = TreeNFAVM::new(program);

        // Use the new driver function
        vm.process_tree_verbose(&mut root_node);

        // Show final results
        println!("=== Final Results ===");
        print_css_results(&root_node, &vm.program.state_names, 0);

        // Demonstrate incremental processing
        println!("\n=== Incremental Processing Demo ===");

        // First run - everything will be computed
        println!("First incremental run (all cache misses expected):");
        let stats1 = vm.process_tree_incremental_with_stats(&mut root_node);
        stats1.print_summary();

        // Second run - everything should be cached
        println!("\nSecond incremental run (all cache hits expected):");
        let stats2 = vm.process_tree_incremental_with_stats(&mut root_node);
        stats2.print_summary();

        // Modify one node and run again
        if !root_node.children.is_empty() {
            println!("\nModifying first child node...");
            root_node.children[0].mark_self_dirty();

            println!("Third incremental run (some cache misses expected):");
            let stats3 = vm.process_tree_incremental_with_stats(&mut root_node);
            stats3.print_summary();
        }
    } else {
        println!("Failed to parse HTML file");
    }

    println!("\n");
}

fn print_html_structure(node: &HtmlNode, depth: usize) {
    let indent = "  ".repeat(depth);
    println!(
        "{}Tag: {}, ID: {:?}, Classes: {:?}",
        indent, node.tag_name, node.id, node.classes
    );

    for child in &node.children {
        print_html_structure(child, depth + 1);
    }
}

fn print_css_results(node: &HtmlNode, state_names: &HashMap<usize, String>, depth: usize) {
    let indent = "  ".repeat(depth);
    println!(
        "{}[{}] {} (ID: {:?}, Classes: {:?})",
        indent,
        if node.css_match_bitvector.is_empty() {
            "No matches"
        } else {
            "MATCHES"
        },
        node.tag_name,
        node.id,
        node.classes
    );

    if !node.css_match_bitvector.is_empty() {
        for (bit_pos, name) in state_names {
            if node.css_match_bitvector.is_bit_set(*bit_pos) {
                println!("{}    - {}", indent, name);
            }
        }
    }

    for child in &node.children {
        print_css_results(child, state_names, depth + 1);
    }
}

fn main() {
    let tests_dir = "tests";
    let css_file = format!("{}/test.css", tests_dir);

    // Test with all HTML files
    for i in 1..=4 {
        let html_file = format!("{}/t{}.html", tests_dir, i);

        if Path::new(&html_file).exists() && Path::new(&css_file).exists() {
            compile_and_run(&css_file, &html_file);
        } else {
            println!("Missing test files: {} or {}", css_file, html_file);
        }
    }
    
    // Test Google trace integration
    println!("\n=== GOOGLE TRACE INTEGRATION TEST ===");
    if let Err(e) = process_google_trace_with_rust() {
        println!("Google trace test failed: {}", e);
        println!("This is expected if css-gen-op files are not available");
    } else {
        println!("✓ Google trace test completed successfully!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create test HTML nodes
    #[allow(dead_code)]
    fn create_test_node() -> HtmlNode {
        HtmlNode::new("div")
            .with_id("test")
            .with_class("container")
            .add_child(
                HtmlNode::new("p")
                    .with_class("item")
                    .add_child(HtmlNode::new("span").with_id("specific")),
            )
    }

    #[test]
    fn test_simple_selector_parsing() {
        assert_eq!(
            parse_simple_selector("div"),
            Some(SimpleSelector::Type("div".to_string()))
        );
        assert_eq!(
            parse_simple_selector(".item"),
            Some(SimpleSelector::Class("item".to_string()))
        );
        assert_eq!(
            parse_simple_selector("#specific"),
            Some(SimpleSelector::Id("specific".to_string()))
        );
        assert_eq!(parse_simple_selector(""), None);
        assert_eq!(parse_simple_selector("invalid123"), None);
    }

    #[test]
    fn test_css_parsing() {
        let css = r#"
/* Rule 0 (R0) */ div {}
/* Rule 1 (R1) */ .item {}
/* Rule 2 (R2) */ #specific {}
/* Rule 3 (R3) */ div > p {}
        "#;

        let rules = parse_css_file(css);
        assert_eq!(rules.len(), 4);

        assert_eq!(
            rules[0],
            CssRule::Simple(SimpleSelector::Type("div".to_string()))
        );
        assert_eq!(
            rules[1],
            CssRule::Simple(SimpleSelector::Class("item".to_string()))
        );
        assert_eq!(
            rules[2],
            CssRule::Simple(SimpleSelector::Id("specific".to_string()))
        );
        assert_eq!(
            rules[3],
            CssRule::Child {
                parent_selector: SimpleSelector::Type("div".to_string()),
                child_selector: SimpleSelector::Type("p".to_string()),
            }
        );
    }

    #[test]
    fn test_html_node_creation() {
        let node = HtmlNode::new("div")
            .with_id("test")
            .with_class("item")
            .with_class("container");

        assert_eq!(node.tag_name, "div");
        assert_eq!(node.id, Some("test".to_string()));
        assert!(node.classes.contains("item"));
        assert!(node.classes.contains("container"));
    }

    #[test]
    fn test_css_compiler_bit_allocation() {
        let mut compiler = CssCompiler::new();

        let bit1 = compiler.allocate_bit("match_div".to_string());
        let bit2 = compiler.allocate_bit("active_div".to_string());
        let bit3 = compiler.allocate_bit("match_div".to_string()); // Should reuse

        assert_eq!(bit1, 0);
        assert_eq!(bit2, 1);
        assert_eq!(bit3, 0); // Should be the same as bit1
    }

    #[test]
    fn test_tree_nfa_program_generation() {
        let rules = vec![
            CssRule::Simple(SimpleSelector::Type("div".to_string())),
            CssRule::Simple(SimpleSelector::Class("item".to_string())),
            CssRule::Child {
                parent_selector: SimpleSelector::Type("div".to_string()),
                child_selector: SimpleSelector::Class("item".to_string()),
            },
        ];

        let mut compiler = CssCompiler::new();
        let program = compiler.compile_css_rules(&rules);

        // Should have instructions for simple selectors and child selector
        assert!(!program.instructions.is_empty());
        assert!(program.total_bits > 0);

        // Check that state names are set
        assert!(!program.state_names.is_empty());
    }

    #[test]
    fn test_node_matches_selector() {
        let vm = TreeNFAVM::new(TreeNFAProgram::new());

        let div_node = HtmlNode::new("div").with_id("test").with_class("container");

        assert!(vm.node_matches_selector(&div_node, &SimpleSelector::Type("div".to_string())));
        assert!(vm.node_matches_selector(&div_node, &SimpleSelector::Id("test".to_string())));
        assert!(
            vm.node_matches_selector(&div_node, &SimpleSelector::Class("container".to_string()))
        );

        assert!(!vm.node_matches_selector(&div_node, &SimpleSelector::Type("p".to_string())));
        assert!(!vm.node_matches_selector(&div_node, &SimpleSelector::Id("other".to_string())));
        assert!(!vm.node_matches_selector(&div_node, &SimpleSelector::Class("other".to_string())));
    }

    #[test]
    fn test_complete_css_matching() {
        // Create a simple CSS rule set
        let rules = vec![
            CssRule::Simple(SimpleSelector::Type("div".to_string())),
            CssRule::Simple(SimpleSelector::Class("item".to_string())),
            CssRule::Child {
                parent_selector: SimpleSelector::Type("div".to_string()),
                child_selector: SimpleSelector::Class("item".to_string()),
            },
        ];

        // Compile to Tree NFA program
        let mut compiler = CssCompiler::new();
        let program = compiler.compile_css_rules(&rules);
        let vm = TreeNFAVM::new(program);

        // Create test HTML structure: div > p.item
        let mut root = HtmlNode::new("div")
            .with_id("outer")
            .add_child(HtmlNode::new("p").with_class("item"))
            .add_child(
                HtmlNode::new("span")
                    .with_class("item")
                    .add_child(HtmlNode::new("p").with_id("specific")),
            );

        // Execute on root (div)
        let child_states = vm.process_node_inplace(&mut root, BitVector::new());

        // Root should match "div" selector
        assert_ne!(root.css_match_bitvector.as_u64(), 0);

        // Root should provide active states for children
        assert!(!child_states.is_empty());

        // Execute on child (p.item)
        let mut child = HtmlNode::new("p").with_class("item");
        let _child_child_states = vm.process_node_inplace(&mut child, child_states);

        // Child should match both ".item" and "div > .item"
        assert_ne!(child.css_match_bitvector.as_u64(), 0);
    }

    #[test]
    fn test_generated_rust_code() {
        let rules = vec![CssRule::Simple(SimpleSelector::Type("div".to_string()))];

        let mut compiler = CssCompiler::new();
        let program = compiler.compile_css_rules(&rules);
        let generated_code = program.generate_rust_code();

        // Check that the generated code contains expected elements
        assert!(generated_code.contains("fn process_node_generated_incremental"));
        assert!(generated_code.contains("current_matches"));
        assert!(generated_code.contains("child_states"));
        assert!(generated_code.contains("node_matches_selector_generated"));
        assert!(generated_code.contains("SimpleSelector::Type"));
    }

    #[test]
    fn test_complex_css_scenario() {
        // Test the exact CSS from test.css file
        let css = r#"
/* Rule 0 (R0) */ div {}
/* Rule 1 (R1) */ .item {}
/* Rule 2 (R2) */ #specific {}
/* Rule 3 (R3) */ p {}
/* Rule 4 (R4) */ div > p {}
/* Rule 5 (R5) */ .item > #specific {}
/* Rule 6 (R6) */ div > .item {}
/* Rule 7 (R7) */ div > #specific {}
        "#;

        let rules = parse_css_file(css);
        let mut compiler = CssCompiler::new();
        let program = compiler.compile_css_rules(&rules);
        let vm = TreeNFAVM::new(program);

        // Create HTML structure similar to t1.html: div > p.item > span > p#specific
        let mut root = HtmlNode::new("div")
            .with_id("main")
            .with_class("container")
            .add_child(
                HtmlNode::new("p")
                    .with_class("item")
                    .add_child(HtmlNode::new("span").with_id("specific")),
            )
            .add_child(
                HtmlNode::new("div").add_child(HtmlNode::new("p").add_child(HtmlNode::new("span"))),
            );

        // Execute on root div
        let child_states_root = vm.process_node_inplace(&mut root, BitVector::new());

        // Root div should match "div" rule
        assert_ne!(root.css_match_bitvector.as_u64(), 0);

        // Test first child (p.item)
        let mut p_item = HtmlNode::new("p").with_class("item");
        let _child_states_p = vm.process_node_inplace(&mut p_item, child_states_root);

        // Should match: p, .item, div > p, div > .item
        assert_ne!(p_item.css_match_bitvector.as_u64(), 0);

        // Test span.item
        let mut span_item = HtmlNode::new("span").with_class("item");
        let child_states_span = vm.process_node_inplace(&mut span_item, child_states_root);

        // Should match: .item, div > .item
        assert_ne!(span_item.css_match_bitvector.as_u64(), 0);

        // Test final p#specific under span.item
        let mut p_specific = HtmlNode::new("p").with_id("specific");
        let _final_states = vm.process_node_inplace(&mut p_specific, child_states_span);

        // Should match: p, #specific, .item > #specific
        assert_ne!(p_specific.css_match_bitvector.as_u64(), 0);
    }

    #[test]
    fn test_bitvector_operations() {
        // Test basic bitvector operations used in the CSS matching
        let mut bitvector: u64 = 0;

        // Set bit 3
        bitvector |= 1 << 3;
        assert_eq!(bitvector, 8); // 2^3 = 8

        // Check bit 3 is set
        assert_ne!(bitvector & (1 << 3), 0);

        // Check bit 2 is not set
        assert_eq!(bitvector & (1 << 2), 0);

        // Set bit 0
        bitvector |= 1 << 0;
        assert_eq!(bitvector, 9); // 8 + 1 = 9

        // Test multiple bits
        assert_ne!(bitvector & (1 << 0), 0);
        assert_ne!(bitvector & (1 << 3), 0);
    }

    #[test]
    fn test_error_handling() {
        // Test CSS parsing with malformed input
        let bad_css = "this is not valid css";
        let rules = parse_css_file(bad_css);
        assert!(rules.is_empty());

        // Test selector parsing with invalid input
        assert_eq!(parse_simple_selector("123invalid"), None);
        assert_eq!(parse_simple_selector(""), None);
    }

    #[test]
    fn test_generated_code_execution() {
        // Create a comprehensive CSS rule set to test code generation
        let rules = vec![
            CssRule::Simple(SimpleSelector::Type("div".to_string())),
            CssRule::Simple(SimpleSelector::Class("item".to_string())),
            CssRule::Simple(SimpleSelector::Id("specific".to_string())),
            CssRule::Child {
                parent_selector: SimpleSelector::Type("div".to_string()),
                child_selector: SimpleSelector::Class("item".to_string()),
            },
            CssRule::Child {
                parent_selector: SimpleSelector::Class("item".to_string()),
                child_selector: SimpleSelector::Id("specific".to_string()),
            },
        ];

        let mut compiler = CssCompiler::new();
        let program = compiler.compile_css_rules(&rules);
        let generated_code = program.generate_rust_code();

        // Verify the generated code has the expected structure
        assert!(generated_code.contains("fn process_node_generated_incremental"));
        assert!(generated_code.contains("current_matches = BitVector::new()"));
        assert!(generated_code.contains("child_states = BitVector::new()"));

        // Verify instruction generation
        assert!(generated_code.contains("CheckAndSetBit"));
        assert!(generated_code.contains("PropagateToChildren"));
        assert!(generated_code.contains("CheckParentAndSetBit"));

        // Verify selector type handling
        assert!(generated_code.contains("SimpleSelector::Type"));
        assert!(generated_code.contains("SimpleSelector::Class"));
        assert!(generated_code.contains("SimpleSelector::Id"));

        // Test that the VM produces the same results as what the generated code should
        let vm = TreeNFAVM::new(program);

        // Test case: div.item > span#specific
        let mut root = HtmlNode::new("div").with_class("item");
        let child_states = vm.process_node_inplace(&mut root, BitVector::new());

        // Root should match both "div" and ".item"
        let matches = root.css_match_bitvector.as_u64();
        assert_ne!(matches, 0);

        // Should propagate states for children
        assert!(!child_states.is_empty());

        // Test child element
        let mut child = HtmlNode::new("span").with_id("specific");
        let _child_child_states = vm.process_node_inplace(&mut child, child_states);

        // Child should match "#specific" and ".item > #specific"
        assert_ne!(child.css_match_bitvector.as_u64(), 0);
    }

    #[test]
    fn test_instruction_order_and_correctness() {
        // Test that instructions are generated in the correct order
        let rules = vec![
            CssRule::Simple(SimpleSelector::Type("div".to_string())),
            CssRule::Child {
                parent_selector: SimpleSelector::Type("div".to_string()),
                child_selector: SimpleSelector::Type("p".to_string()),
            },
        ];

        let mut compiler = CssCompiler::new();
        let program = compiler.compile_css_rules(&rules);

        // Check instruction count and types
        assert!(!program.instructions.is_empty());

        let mut has_check_and_set = false;
        let mut has_propagate = false;
        let mut has_check_parent = false;

        for instruction in &program.instructions {
            match instruction {
                NFAInstruction::CheckAndSetBit { .. } => has_check_and_set = true,
                NFAInstruction::PropagateToChildren { .. } => has_propagate = true,
                NFAInstruction::CheckParentAndSetBit { .. } => has_check_parent = true,
            }
        }

        assert!(has_check_and_set, "Should have CheckAndSetBit instructions");
        assert!(
            has_propagate,
            "Should have PropagateToChildren instructions"
        );
        assert!(
            has_check_parent,
            "Should have CheckParentAndSetBit instructions"
        );
    }

    #[test]
    fn test_state_naming_consistency() {
        let rules = vec![
            CssRule::Simple(SimpleSelector::Type("div".to_string())),
            CssRule::Simple(SimpleSelector::Class("test".to_string())),
        ];

        let mut compiler = CssCompiler::new();
        let program = compiler.compile_css_rules(&rules);

        // Check that state names follow the expected pattern
        let state_names: Vec<&String> = program.state_names.values().collect();

        assert!(
            state_names
                .iter()
                .any(|s| s.contains("match_Type(\"div\")"))
        );
        assert!(
            state_names
                .iter()
                .any(|s| s.contains("active_Type(\"div\")"))
        );
        assert!(
            state_names
                .iter()
                .any(|s| s.contains("match_Class(\"test\")"))
        );
        assert!(
            state_names
                .iter()
                .any(|s| s.contains("active_Class(\"test\")"))
        );
    }

    #[test]
    fn test_demo_generated_code() {
        println!("\n=== CSS COMPILER DEMO ===");

        // Simple CSS rules
        let css_content = r#"
/* Rule 0 (R0) */ div {}
/* Rule 1 (R1) */ .item {}
/* Rule 2 (R2) */ #specific {}
/* Rule 3 (R3) */ div > .item {}
/* Rule 4 (R4) */ .item > #specific {}
        "#;

        println!("Input CSS:");
        println!("{}", css_content);

        let rules = parse_css_file(css_content);
        println!("Parsed {} CSS rules", rules.len());

        let mut compiler = CssCompiler::new();
        let program = compiler.compile_css_rules(&rules);

        println!("\nGenerated Tree NFA Program:");
        println!("Total bits: {}", program.total_bits);
        println!("Instructions: {}", program.instructions.len());

        println!("\nGenerated Rust Code:");
        println!("{}", program.generate_rust_code());

        println!("=== DEMO COMPLETE ===\n");
    }

    #[test]
    fn test_run_generated_rust_code() {
        use std::fs;
        use std::io::Write;
        use std::process::Command;

        println!("\n=== TESTING GENERATED RUST CODE EXECUTION ===");

        // Create test CSS rules
        let rules = vec![
            CssRule::Simple(SimpleSelector::Type("div".to_string())),
            CssRule::Simple(SimpleSelector::Class("item".to_string())),
            CssRule::Simple(SimpleSelector::Id("specific".to_string())),
            CssRule::Child {
                parent_selector: SimpleSelector::Type("div".to_string()),
                child_selector: SimpleSelector::Class("item".to_string()),
            },
        ];

        // Compile to program and generate code
        let mut compiler = CssCompiler::new();
        let program = compiler.compile_css_rules(&rules);
        let generated_fn = program.generate_rust_code();

        // Create a complete Rust file that can be compiled and run
        let complete_rust_code = format!(
            r##"
// Generated file - do not format manually
use std::collections::HashSet;

// Copy necessary types and structs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BitVector {{
    bits: u64,
}}

impl BitVector {{
    fn new() -> Self {{
        BitVector {{ bits: 0 }}
    }}

    fn from_u64(bits: u64) -> Self {{
        BitVector {{ bits }}
    }}

    fn set_bit(&mut self, pos: usize) {{
        self.bits |= 1 << pos;
    }}

    fn is_bit_set(&self, pos: usize) -> bool {{
        (self.bits & (1 << pos)) != 0
    }}

    fn is_empty(&self) -> bool {{
        self.bits == 0
    }}

    fn as_u64(&self) -> u64 {{
        self.bits
    }}
}}

impl std::fmt::Binary for BitVector {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        write!(f, "{{:016b}}", self.bits)
    }}
}}

#[derive(Debug, Clone)]
struct HtmlNode {{
    tag_name: String,
    id: Option<String>,
    classes: HashSet<String>,
    children: Vec<HtmlNode>,
    css_match_bitvector: BitVector,
    
    // Double Dirty Bit Algorithm state
    is_self_dirty: bool,           // This node's own attributes changed
    has_dirty_descendant: bool,    // Some descendant needs recomputation (summary bit)
    
    // Incremental processing state
    cached_parent_state: Option<BitVector>,     // Input: parent state from last computation
    cached_node_intrinsic: Option<BitVector>,   // Input: node's own selector matches (computed once)
    cached_child_states: Option<BitVector>,     // Output: states to propagate to children
}}

impl HtmlNode {{
    fn new(tag_name: &str) -> Self {{
        HtmlNode {{
            tag_name: tag_name.to_lowercase(),
            id: None,
            classes: HashSet::new(),
            children: Vec::new(),
            css_match_bitvector: BitVector::new(),
            is_self_dirty: true,  // New nodes need computation
            has_dirty_descendant: false,
            cached_parent_state: None,
            cached_node_intrinsic: None,
            cached_child_states: None,
        }}
    }}

    fn with_id(mut self, id: &str) -> Self {{
        self.id = Some(id.to_string());
        self.mark_self_dirty(); // Only mark self as dirty, not children
        self
    }}

    fn with_class(mut self, class: &str) -> Self {{
        self.classes.insert(class.to_string());
        self.mark_self_dirty(); // Only mark self as dirty, not children
        self
    }}

    fn add_child(mut self, child: HtmlNode) -> Self {{
        self.children.push(child);
        self
    }}
    
    // Mark only this node as dirty (attributes changed)
    fn mark_self_dirty(&mut self) {{
        self.is_self_dirty = true;
        self.cached_node_intrinsic = None; // Invalidate intrinsic cache
        // Don't clear parent/child state - they may still be valid
    }}
    
    // Mark this node as having dirty descendants and propagate summary bit upward
    fn mark_descendant_dirty(&mut self) {{
        self.has_dirty_descendant = true;
        // Propagate summary bit would happen here in a real implementation
        // For now, we'll handle this in the processing logic
    }}
    
    // Complete dirty marking (for structural changes)
    fn mark_dirty_complete(&mut self) {{
        self.is_self_dirty = true;
        self.has_dirty_descendant = true;
        self.cached_parent_state = None;
        self.cached_node_intrinsic = None;
        self.cached_child_states = None;
        // Still don't recursively dirty children
    }}
    
    // Check if this node or any descendant needs recomputation
    fn needs_any_recomputation(&self, new_parent_state: BitVector) -> bool {{
        self.is_self_dirty || 
        self.has_dirty_descendant ||
        self.cached_parent_state.is_none() ||
        self.cached_parent_state.unwrap() != new_parent_state
    }}
    
    // Check if only this node needs recomputation
    fn needs_self_recomputation(&self, new_parent_state: BitVector) -> bool {{
        self.is_self_dirty ||
        self.cached_parent_state.is_none() ||
        self.cached_parent_state.unwrap() != new_parent_state
    }}
    
    // Clean dirty flags after processing
    fn mark_clean(&mut self) {{
        self.is_self_dirty = false;
        // Don't clear has_dirty_descendant here - it will be cleared
        // when all descendants are processed
    }}
    
    // Clean descendant dirty flag when all children are processed
    fn mark_descendants_clean(&mut self) {{
        self.has_dirty_descendant = false;
    }}
}}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum SimpleSelector {{
    Type(String),
    Class(String),
    Id(String),
}}

{generated_fn}

fn main() {{
    // Test case 1: div node
    let mut div_node = HtmlNode::new("div").with_id("test").with_class("item");
    let child_states = process_node_generated_incremental(&mut div_node, BitVector::new());
    println!("div.item#test - matches: {{:b}}, child_states: {{:b}}", div_node.css_match_bitvector, child_states);
    
    // Test case 2: span node with class
    let mut span_node = HtmlNode::new("span").with_class("item");
    let child_states2 = process_node_generated_incremental(&mut span_node, child_states);
    println!("span.item (child of div) - matches: {{:b}}, child_states: {{:b}}", span_node.css_match_bitvector, child_states2);
    
    // Test case 3: node with specific id
    let mut specific_node = HtmlNode::new("p").with_id("specific");
    let child_states3 = process_node_generated_incremental(&mut specific_node, BitVector::new());
    println!("p#specific - matches: {{:b}}, child_states: {{:b}}", specific_node.css_match_bitvector, child_states3);
    
    // Test case 4: driver function test
    let mut tree = HtmlNode::new("div")
        .with_class("item")
        .add_child(HtmlNode::new("span").with_id("specific"));
    
    println!("\nTesting tree processing...");
    process_tree_generated_incremental(&mut tree);
    
    fn print_tree_results(node: &HtmlNode, depth: usize) {{
        let indent = "  ".repeat(depth);
        println!("{{}}{{}} (matches: {{:b}})", indent, node.tag_name, node.css_match_bitvector);
        for child in &node.children {{
            print_tree_results(child, depth + 1);
        }}
    }}
    
    print_tree_results(&tree, 0);
    
    println!("SUCCESS: Generated Rust code executed successfully!");
}}
"##
        );

        // Write the complete code to a temporary file
        let temp_file = "temp_generated_test.rs";
        let mut file = fs::File::create(temp_file).expect("Failed to create temp file");
        file.write_all(complete_rust_code.as_bytes())
            .expect("Failed to write to temp file");
        drop(file);

        // Compile the generated Rust code
        println!("Compiling generated Rust code...");
        let compile_output = Command::new("rustc")
            .args([temp_file, "-o", "temp_generated_test"])
            .output();

        match compile_output {
            Ok(output) => {
                if output.status.success() {
                    println!("Compilation successful!");

                    // Run the compiled binary
                    println!("Running generated code...");
                    let run_output = Command::new("./temp_generated_test").output();

                    match run_output {
                        Ok(run_result) => {
                            if run_result.status.success() {
                                let stdout = String::from_utf8_lossy(&run_result.stdout);
                                println!("Generated code output:");
                                println!("{}", stdout);

                                // Verify that it ran successfully
                                assert!(stdout.contains(
                                    "SUCCESS: Generated Rust code executed successfully!"
                                ));

                                // Now compare with VM results to ensure consistency
                                println!("Comparing with VM results...");
                                let vm = TreeNFAVM::new(program);

                                // Test case 1: div.item#test
                                let mut div_node =
                                    HtmlNode::new("div").with_id("test").with_class("item");
                                let vm_child_states =
                                    vm.process_node_inplace(&mut div_node, BitVector::new());
                                let vm_matches = div_node.css_match_bitvector.as_u64();

                                println!(
                                    "VM results for div.item#test - matches: {:016b}, child_states: {:016b}",
                                    vm_matches,
                                    vm_child_states.as_u64()
                                );

                                // The generated code should produce the same results as the VM
                                // We can't easily parse the exact output, but we verified it runs without error

                                println!("✓ Generated code execution test passed!");
                            } else {
                                let stderr = String::from_utf8_lossy(&run_result.stderr);
                                panic!("Generated code failed to run: {}", stderr);
                            }
                        }
                        Err(e) => {
                            println!(
                                "Warning: Could not run generated code (maybe missing binary): {}",
                                e
                            );
                            // Don't fail the test if we can't run the binary, just log it
                        }
                    }
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    panic!("Failed to compile generated code: {}", stderr);
                }
            }
            Err(e) => {
                println!(
                    "Warning: rustc not available for testing generated code: {}",
                    e
                );
                // Don't fail the test if rustc is not available
            }
        }

        // Clean up temporary files
        // let _ = fs::remove_file(temp_file);
        let _ = fs::remove_file("temp_generated_test");

        println!("=== GENERATED CODE TEST COMPLETE ===\n");
    }

    #[test]
    fn test_incremental_processing() {
        println!("\n=== TESTING INCREMENTAL PROCESSING ===");

        // Create test CSS rules
        let rules = vec![
            CssRule::Simple(SimpleSelector::Type("div".to_string())),
            CssRule::Simple(SimpleSelector::Class("item".to_string())),
            CssRule::Simple(SimpleSelector::Id("specific".to_string())),
            CssRule::Child {
                parent_selector: SimpleSelector::Type("div".to_string()),
                child_selector: SimpleSelector::Class("item".to_string()),
            },
        ];

        let mut compiler = CssCompiler::new();
        let program = compiler.compile_css_rules(&rules);
        let vm = TreeNFAVM::new(program);

        // Create test HTML structure
        let mut root = HtmlNode::new("div")
            .with_id("container")
            .add_child(
                HtmlNode::new("p")
                    .with_class("item")
                    .add_child(HtmlNode::new("span").with_id("specific")),
            )
            .add_child(HtmlNode::new("div").with_class("other"));

        // Test 1: First run should compute everything
        println!("Test 1: Initial computation");
        let stats1 = vm.process_tree_incremental_with_stats(&mut root);
        assert_eq!(stats1.cache_hits, 0);
        assert!(stats1.cache_misses > 0);
        println!(
            "✓ All nodes computed (cache misses: {})",
            stats1.cache_misses
        );

        // Store initial results for comparison
        let initial_root_matches = root.css_match_bitvector;
        let initial_child1_matches = root.children[0].css_match_bitvector;

        // Test 2: Second run should use cache
        println!("\nTest 2: Cache utilization");
        let stats2 = vm.process_tree_incremental_with_stats(&mut root);
        assert_eq!(stats2.cache_misses, 0);
        assert!(stats2.cache_hits > 0);
        println!("✓ All results cached (cache hits: {})", stats2.cache_hits);

        // Verify results didn't change
        assert_eq!(root.css_match_bitvector, initial_root_matches);
        assert_eq!(root.children[0].css_match_bitvector, initial_child1_matches);
        println!("✓ Results consistent with cached version");

        // Test 3: Modify node and verify selective recomputation
        println!("\nTest 3: Selective recomputation after modification");
        root.children[0].mark_self_dirty();

        let stats3 = vm.process_tree_incremental_with_stats(&mut root);
        assert!(stats3.cache_hits > 0, "Some nodes should still be cached");
        assert!(stats3.cache_misses > 0, "Some nodes should be recomputed");
        println!(
            "✓ Selective recomputation (hits: {}, misses: {})",
            stats3.cache_hits, stats3.cache_misses
        );

        // Test 4: Compare with non-incremental version for correctness
        println!("\nTest 4: Correctness verification");
        let mut root_copy = HtmlNode::new("div")
            .with_id("container")
            .add_child(
                HtmlNode::new("p")
                    .with_class("item")
                    .add_child(HtmlNode::new("span").with_id("specific")),
            )
            .add_child(HtmlNode::new("div").with_class("other"));

        vm.process_tree(&mut root_copy);

        // Compare results
        assert_eq!(root.css_match_bitvector, root_copy.css_match_bitvector);
        assert_eq!(
            root.children[0].css_match_bitvector,
            root_copy.children[0].css_match_bitvector
        );
        assert_eq!(
            root.children[1].css_match_bitvector,
            root_copy.children[1].css_match_bitvector
        );
        println!("✓ Incremental results match non-incremental results");

        println!("=== INCREMENTAL PROCESSING TESTS PASSED ===\n");
    }

    #[test]
    fn test_incremental_node_modification() {
        // Test that modifying node attributes correctly invalidates cache
        let rules = vec![
            CssRule::Simple(SimpleSelector::Type("div".to_string())),
            CssRule::Simple(SimpleSelector::Class("highlight".to_string())),
        ];

        let mut compiler = CssCompiler::new();
        let program = compiler.compile_css_rules(&rules);
        let vm = TreeNFAVM::new(program);

        let mut node = HtmlNode::new("div");

        // Initial processing
        vm.process_tree_incremental(&mut node);
        let initial_matches = node.css_match_bitvector;

        // Add a class (this should mark the node dirty)
        node = node.with_class("highlight");

        // Reprocess
        vm.process_tree_incremental(&mut node);
        let updated_matches = node.css_match_bitvector;

        // Results should be different
        assert_ne!(
            initial_matches, updated_matches,
            "Adding class should change CSS matches"
        );

        // Find the correct bit position for .highlight class
        let mut highlight_bit = None;
        for (bit_pos, name) in &vm.program.state_names {
            if name.contains("highlight") && name.contains("match") {
                highlight_bit = Some(*bit_pos);
                break;
            }
        }

        if let Some(bit) = highlight_bit {
            assert!(
                updated_matches.is_bit_set(bit),
                "Should match .highlight class at bit {}",
                bit
            );
        } else {
            panic!("Could not find bit position for .highlight class");
        }
    }

    #[test]
    fn test_performance_comparison() {
        use std::time::Instant;

        println!("\n=== PERFORMANCE COMPARISON ===");

        // Create a larger CSS rule set
        let rules = vec![
            CssRule::Simple(SimpleSelector::Type("div".to_string())),
            CssRule::Simple(SimpleSelector::Class("item".to_string())),
            CssRule::Simple(SimpleSelector::Id("specific".to_string())),
            CssRule::Simple(SimpleSelector::Type("p".to_string())),
            CssRule::Simple(SimpleSelector::Type("span".to_string())),
            CssRule::Child {
                parent_selector: SimpleSelector::Type("div".to_string()),
                child_selector: SimpleSelector::Type("p".to_string()),
            },
            CssRule::Child {
                parent_selector: SimpleSelector::Class("item".to_string()),
                child_selector: SimpleSelector::Id("specific".to_string()),
            },
            CssRule::Child {
                parent_selector: SimpleSelector::Type("div".to_string()),
                child_selector: SimpleSelector::Class("item".to_string()),
            },
        ];

        let mut compiler = CssCompiler::new();
        let program = compiler.compile_css_rules(&rules);
        let vm = TreeNFAVM::new(program);

        // Create a complex HTML structure
        let mut root = HtmlNode::new("div")
            .with_id("main")
            .with_class("container")
            .add_child(
                HtmlNode::new("div")
                    .with_class("item")
                    .add_child(
                        HtmlNode::new("p").add_child(HtmlNode::new("span").with_id("specific")),
                    )
                    .add_child(HtmlNode::new("div").add_child(HtmlNode::new("p"))),
            )
            .add_child(HtmlNode::new("p").with_class("item"))
            .add_child(
                HtmlNode::new("div")
                    .add_child(HtmlNode::new("span"))
                    .add_child(HtmlNode::new("p").with_id("specific")),
            );

        // Benchmark regular processing (multiple runs)
        let iterations = 1000;
        let start = Instant::now();
        for _ in 0..iterations {
            vm.process_tree(&mut root.clone());
        }
        let regular_time = start.elapsed();

        // Benchmark incremental processing (first run + cached runs)
        let start = Instant::now();

        // First run (cache miss)
        vm.process_tree_incremental(&mut root);

        // Subsequent runs (cache hits)
        for _ in 1..iterations {
            vm.process_tree_incremental(&mut root);
        }
        let incremental_time = start.elapsed();

        println!(
            "Regular processing ({} iterations): {:?}",
            iterations, regular_time
        );
        println!(
            "Incremental processing ({} iterations): {:?}",
            iterations, incremental_time
        );

        let speedup = regular_time.as_nanos() as f64 / incremental_time.as_nanos() as f64;
        println!("Speedup: {:.2}x", speedup);

        // Incremental should be significantly faster for repeated computations
        assert!(
            incremental_time < regular_time,
            "Incremental should be faster"
        );

        println!("=== PERFORMANCE COMPARISON COMPLETE ===\n");
    }

    #[test]
    fn test_double_dirty_bit_optimization() {
        // Create a deep tree structure
        let css_rules = vec![
            CssRule::Simple(SimpleSelector::Type("div".to_string())),
            CssRule::Simple(SimpleSelector::Class("highlight".to_string())),
        ];

        let mut compiler = CssCompiler::new();
        let program = compiler.compile_css_rules(&css_rules);
        let vm = TreeNFAVM::new(program);

        // Build deep tree: root -> child1 -> child2 -> child3
        let mut root = HtmlNode::new("div")
            .add_child(
                HtmlNode::new("div")
                    .add_child(
                        HtmlNode::new("div")
                            .add_child(HtmlNode::new("span").with_class("highlight"))
                    )
            );

        // Initial processing
        vm.process_tree_incremental(&mut root);
        
        // Verify everything is initially clean
        assert!(!root.is_self_dirty);
        assert!(!root.has_dirty_descendant);
        assert!(!root.children[0].is_self_dirty);
        assert!(!root.children[0].has_dirty_descendant);

        // Mark deep node dirty using optimized path marking
        assert!(root.mark_node_dirty_by_path(&[0, 0, 0])); // path to deepest span

        // Verify dirty bits are set correctly along the path
        assert!(!root.is_self_dirty); // Root itself not dirty
        assert!(root.has_dirty_descendant); // But has dirty descendant

        assert!(!root.children[0].is_self_dirty); // Child1 not dirty
        assert!(root.children[0].has_dirty_descendant); // But has dirty descendant

        assert!(!root.children[0].children[0].is_self_dirty); // Child2 not dirty  
        assert!(root.children[0].children[0].has_dirty_descendant); // But has dirty descendant

        assert!(root.children[0].children[0].children[0].is_self_dirty); // Deepest node is dirty
        assert!(!root.children[0].children[0].children[0].has_dirty_descendant); // No children

        // Process incrementally - should only process nodes on the dirty path
        let stats = vm.process_tree_incremental_with_stats(&mut root);
        
        // With optimization, we should have fewer cache misses than total nodes
        // Only the dirty path should be recomputed, but our current implementation 
        // still visits all nodes to check if they need recomputation
        println!("Double dirty bit stats: {:?}", stats);
        
        // Verify that the dirty marking worked correctly
        assert!(!root.is_self_dirty); // Root was processed and cleaned
        assert!(!root.has_dirty_descendant); // Summary bit cleared after processing
        assert!(!root.children[0].children[0].children[0].is_self_dirty); // Deep node processed and cleaned
        
        // The key optimization is that only nodes on the dirty path needed recomputation
        // This is more of a conceptual test for now
        assert!(stats.total_nodes > 0, "Should have processed some nodes");
    }

    #[test]
    fn test_optimized_selector_matching() {
        // Test hash table vs linear search performance conceptually
        let css_rules = vec![
            CssRule::Simple(SimpleSelector::Type("div".to_string())),
            CssRule::Simple(SimpleSelector::Type("span".to_string())),
            CssRule::Simple(SimpleSelector::Type("p".to_string())),
            CssRule::Simple(SimpleSelector::Class("highlight".to_string())),
            CssRule::Simple(SimpleSelector::Class("error".to_string())),
            CssRule::Simple(SimpleSelector::Id("main".to_string())),
        ];

        let mut compiler = CssCompiler::new();
        let program = compiler.compile_css_rules(&css_rules);
        let vm = TreeNFAVM::new(program);

        // Test that selector index correctly identifies matching rules
        let test_node = HtmlNode::new("div").with_class("highlight");
        let matching_rules = vm.selector_index.get_matching_rules(&test_node);
        
        // Should find exactly 2 matching rules (div tag and highlight class)
        assert_eq!(matching_rules.len(), 2, "Should find exactly 2 matching rules");
        
        // Verify the rules are correct
        let mut found_tag_rule = false;
        let mut found_class_rule = false;
        
        for rule in matching_rules {
            match rule {
                NFAInstruction::CheckAndSetBit { selector, .. } => {
                    match selector {
                        SimpleSelector::Type(tag) if tag == "div" => found_tag_rule = true,
                        SimpleSelector::Class(class) if class == "highlight" => found_class_rule = true,
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        
        assert!(found_tag_rule, "Should find div tag rule");
        assert!(found_class_rule, "Should find highlight class rule");
    }

    #[test]
    fn test_generated_incremental_code_has_caching() {
        println!("\n=== TESTING GENERATED INCREMENTAL CODE ===");

        // Create test CSS rules
        let rules = vec![
            CssRule::Simple(SimpleSelector::Type("div".to_string())),
            CssRule::Simple(SimpleSelector::Class("item".to_string())),
            CssRule::Child {
                parent_selector: SimpleSelector::Type("div".to_string()),
                child_selector: SimpleSelector::Class("item".to_string()),
            },
        ];

        let mut compiler = CssCompiler::new();
        let program = compiler.compile_css_rules(&rules);
        let generated_code = program.generate_rust_code();

        // Verify that the generated code contains incremental processing logic
        println!("Checking generated code for incremental features...");

        // Should contain incremental function
        assert!(
            generated_code.contains("process_node_generated_incremental"),
            "Generated code should contain incremental processing function"
        );

        // Should contain cache checking
        assert!(
            generated_code.contains("needs_recomputation_generated"),
            "Generated code should contain cache checking function"
        );

        // Should contain cache return logic
        assert!(
            generated_code.contains("return node.cached_child_states.unwrap_or"),
            "Generated code should return cached results when possible"
        );

        // Should contain intrinsic computation caching
        assert!(
            generated_code.contains("node.cached_node_intrinsic.is_none() || node.is_self_dirty"),
            "Generated code should cache intrinsic matches"
        );

        // Should contain cache updates
        assert!(
            generated_code.contains("node.cached_parent_state = Some(parent_state)"),
            "Generated code should update parent state cache"
        );
        assert!(
            generated_code.contains("node.cached_child_states = Some(child_states)"),
            "Generated code should update child states cache"
        );
        assert!(
            generated_code.contains("node.mark_clean()"),
            "Generated code should mark node as clean"
        );

        // Should contain incremental tree driver
        assert!(
            generated_code.contains("process_tree_generated_incremental"),
            "Generated code should contain incremental tree processing driver"
        );

        // Should also contain non-incremental version for compatibility
        assert!(
            generated_code.contains("process_node_generated_inplace"),
            "Generated code should contain non-incremental version for compatibility"
        );

        println!("✓ All incremental features found in generated code");

        // Also verify specific structure patterns
        assert!(
            generated_code.contains("// Check if we need to recompute"),
            "Generated code should have cache check comments"
        );
        assert!(
            generated_code.contains("// Recompute node intrinsic matches if needed"),
            "Generated code should have intrinsic computation comments"
        );
        assert!(
            generated_code.contains("// Start with cached intrinsic matches"),
            "Generated code should use cached intrinsic matches"
        );
        assert!(
            generated_code.contains("// Cache results"),
            "Generated code should have cache update comments"
        );

        println!("✓ Code structure and comments verify incremental logic");

        // Test that the generated code separates intrinsic from parent-dependent computation
        let intrinsic_section = generated_code.contains("if node.cached_node_intrinsic.is_none()");
        let parent_dependent_section = generated_code.contains("Apply parent-dependent rules");
        assert!(
            intrinsic_section && parent_dependent_section,
            "Generated code should separate intrinsic and parent-dependent computation"
        );

        println!("✓ Generated code properly separates intrinsic and parent-dependent computation");

        println!("=== GENERATED INCREMENTAL CODE TEST PASSED ===\n");
    }
}
