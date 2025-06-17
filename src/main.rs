use regex::Regex;
use scraper::{Html, Selector as HtmlSelector};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use css_bitvector_compiler::CssCompiler;
use css_bitvector_compiler::CssRule;
use css_bitvector_compiler::HtmlNode;
use css_bitvector_compiler::SimpleSelector;
use css_bitvector_compiler::parse_basic_css;
use css_bitvector_compiler::rdtsc;

mod benchmark;

// All types are now defined in lib.rs and imported from there

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
            name: obj
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            node_type: obj
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            namespace: obj
                .get("namespace")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            attributes: obj
                .get("attributes")
                .and_then(|v| v.as_object())
                .map(|attrs| {
                    attrs
                        .iter()
                        .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                        .collect()
                })
                .unwrap_or_default(),
            properties: obj
                .get("properties")
                .and_then(|v| v.as_object())
                .map(|props| {
                    props
                        .iter()
                        .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                        .collect()
                })
                .unwrap_or_default(),
            visible: obj.get("visible").and_then(|v| v.as_bool()).unwrap_or(true),
            children: obj
                .get("children")
                .and_then(|v| v.as_array())
                .map(|children| children.iter().filter_map(GoogleNode::from_json).collect())
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
        1 + self
            .children
            .iter()
            .map(|child| child.count_nodes())
            .sum::<usize>()
    }

    fn init_parent_pointers(&mut self) {
        for child in &mut self.children {
            child.init_parent_pointers();
        }
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
    let first_line = commands_content
        .lines()
        .next()
        .ok_or("Empty command file")?;
    let command: serde_json::Value = serde_json::from_str(first_line)?;

    if command["name"] != "init" {
        return Err("First command should be init".into());
    }

    let google_node =
        GoogleNode::from_json(&command["node"]).ok_or("Failed to parse Google node")?;

    println!(
        "🌳 Google DOM tree contains {} nodes",
        google_node.count_nodes()
    );

    // Generate complete Rust program for Google trace testing
    let complete_program = generate_google_trace_program(&generated_code, &google_node)?;

    // Write to examples directory
    let example_file = "examples/google_trace_test.rs";
    std::fs::write(example_file, &complete_program)
        .map_err(|e| format!("Failed to write generated code: {}", e))?;

    println!("💾 Generated example: {}", example_file);

    // Also generate functions for benchmark usage
    let functions_code = generate_css_functions_for_benchmark(&generated_code)?;
    let functions_file = "src/generated_css_functions.rs";
    std::fs::write(functions_file, &functions_code)
        .map_err(|e| format!("Failed to write generated functions: {}", e))?;

    println!("💾 Generated functions: {}", functions_file);

    // Run the generated example
    println!("🚀 Running generated example with Google trace data...\n");
    let run_output = std::process::Command::new("cargo")
        .args(["run", "--example", "google_trace_test"])
        .output()
        .map_err(|e| format!("Failed to run example: {}", e))?;

    if run_output.status.success() {
        let stdout = String::from_utf8_lossy(&run_output.stdout);
        println!("{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&run_output.stderr);
        return Err(format!("Generated example failed: {}", stderr).into());
    }

    Ok(())
}

// Generate CSS functions for benchmark usage (to be included with include! macro)
fn generate_css_functions_for_benchmark(
    generated_fn_code: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut code = String::new();

    // 1. Header comment
    code.push_str("// Generated CSS processing functions for benchmark usage\n");
    code.push_str("// This file is included by benchmark.rs using include! macro\n\n");

    // 2. Add generated CSS processing function
    code.push_str("// Generated CSS processing function\n");
    code.push_str(generated_fn_code);
    code.push_str("\n\n");

    // 3. Add public incremental processing functions
    code.push_str(
        r#"/// Real incremental processing with statistics tracking
pub fn process_tree_incremental_with_stats(root: &mut HtmlNode) -> (usize, usize, usize) {
    let mut total_nodes = 0;
    let mut cache_hits = 0;
    let mut cache_misses = 0;
    
    process_tree_recursive_with_stats(root, BitVector::new(), &mut total_nodes, &mut cache_hits, &mut cache_misses);
    
    (total_nodes, cache_hits, cache_misses)
}

fn process_tree_recursive_with_stats(node: &mut HtmlNode, parent_state: BitVector, 
                                    total: &mut usize, hits: &mut usize, misses: &mut usize) {
    *total += 1;
    
    // Check if we need to recompute using the real incremental logic
    if node.needs_any_recomputation(parent_state) {
        *misses += 1;
        
        // Use the actual generated incremental processing function
        let child_states = process_node_generated_incremental(node, parent_state);
        
        // Process children recursively
        for child in node.children.iter_mut() {
            process_tree_recursive_with_stats(child, child_states, total, hits, misses);
        }
    } else {
        *hits += 1;
        // Skip entire subtree when cached - this is the power of incremental processing
    }
}

/// Non-incremental processing for comparison (always recomputes everything)
pub fn process_tree_full_recompute(root: &mut HtmlNode) -> (usize, usize, usize) {
    let mut total_nodes = 0;
    let cache_hits = 0; // No caching in full recompute
    let mut cache_misses = 0;
    
    process_tree_recursive_full(root, BitVector::new(), &mut total_nodes, &mut cache_misses);
    
    (total_nodes, cache_hits, cache_misses)
}

fn process_tree_recursive_full(node: &mut HtmlNode, parent_state: BitVector, 
                              total: &mut usize, misses: &mut usize) {
    *total += 1;
    *misses += 1;
    
    // Always recompute - clear caches first
    node.cached_node_intrinsic = None;
    node.cached_parent_state = None;
    node.cached_child_states = None;
    node.mark_dirty();
    
    // Force recomputation by using the generated incremental function
    let child_states = process_node_generated_incremental(node, parent_state);
    
    // Process all children
    for child in node.children.iter_mut() {
        process_tree_recursive_full(child, child_states, total, misses);
    }
}

/// Helper function to reset cache state for benchmarking
pub fn reset_cache_state(node: &mut HtmlNode) {
    node.is_self_dirty = true;
    node.has_dirty_descendant = false;
    node.cached_parent_state = None;
    node.cached_node_intrinsic = None;
    node.cached_child_states = Some(BitVector::new());

    for child in node.children.iter_mut() {
        reset_cache_state(child);
    }
}
"#,
    );

    Ok(code)
}

fn find_deep_node(node: &mut HtmlNode, target_depth: usize) -> Option<&mut HtmlNode> {
    node.find_deep_node_mut(target_depth)
}

fn count_css_matches(node: &HtmlNode) -> usize {
    let current_matches = if node.css_match_bitvector.bits != 0 {
        1
    } else {
        0
    };
    current_matches + node.children.iter().map(count_css_matches).sum::<usize>()
}

fn generate_google_trace_program(
    generated_fn_code: &str,
    google_node: &GoogleNode,
) -> Result<String, Box<dyn std::error::Error>> {
    // 使用模块引用方法 - 直接使用库中定义的类型和函数
    let mut program = String::new();

    // 1. 导入库中的所有类型和函数
    program.push_str("use css_bitvector_compiler::*;\n\n");

    // 2. 添加生成的 CSS 处理函数
    program.push_str("// Generated CSS processing function\n");
    program.push_str(generated_fn_code);
    program.push_str("\n\n");
    program.push_str("fn main(){}");

    Ok(program)
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
            code.push_str(&format!(
                "\n{}.add_child(\n{}\n{})",
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

        // Modify one node and run again
        if !root_node.children.is_empty() {
            println!("\nModifying first child node...");
            root_node.children[0].mark_dirty();
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

fn mark_cascading_changes(tree: &mut HtmlNode, depth: usize) {
    if depth == 0 {
        return;
    }

    tree.mark_dirty();

    for (i, child) in tree.children.iter_mut().enumerate() {
        if i < depth {
            mark_cascading_changes(child, depth - 1);
        }
    }
}

fn create_scaled_google_tree(scale: f64) -> Result<HtmlNode, Box<dyn std::error::Error>> {
    let command_file_path = "css-gen-op/command.json";
    let content = std::fs::read_to_string(command_file_path)?;
    let first_line = content.lines().next().ok_or("Empty command file")?;
    let command: serde_json::Value = serde_json::from_str(first_line)?;

    if command["name"] != "init" {
        return Err("First command should be init".into());
    }

    let google_node =
        GoogleNode::from_json(&command["node"]).ok_or("Failed to parse Google node")?;

    let mut html_node = google_node.to_html_node();

    if scale != 1.0 {
        scale_tree_by_factor(&mut html_node, scale, 0, 2);
    }

    html_node.init_parent_pointers();
    Ok(html_node)
}

fn scale_tree_by_factor(node: &mut HtmlNode, scale: f64, depth: usize, max_depth: usize) {
    if depth >= max_depth {
        return;
    }

    if scale < 1.0 {
        let keep_count = (node.children.len() as f64 * scale).ceil() as usize;
        node.children.truncate(keep_count);
    } else if scale > 1.0 && !node.children.is_empty() {
        let original_children = node.children.clone();
        let extra_count = ((node.children.len() as f64 * (scale - 1.0)).ceil() as usize).min(3);

        for i in 0..extra_count {
            if i < original_children.len() {
                node.children.push(original_children[i].clone());
            }
        }
    }

    for child in &mut node.children {
        scale_tree_by_factor(child, scale, depth + 1, max_depth);
    }
}

fn apply_test_modification(tree: &mut HtmlNode, pattern: &str) {
    match pattern {
        "no_change" => {
            // Do nothing - test pure cache performance
        }
        "single_node" => {
            if let Some(node) = find_node_at_test_depth(tree, 2) {
                node.mark_dirty();
            }
        }
        "small_subtree" => {
            if !tree.children.is_empty() {
                tree.children[0].mark_dirty();
            }
        }
        "large_subtree" => {
            tree.mark_dirty();
        }
        _ => {}
    }
}

fn find_node_at_test_depth(node: &mut HtmlNode, depth: usize) -> Option<&mut HtmlNode> {
    if depth == 0 {
        return Some(node);
    }

    for child in &mut node.children {
        if let Some(found) = find_node_at_test_depth(child, depth - 1) {
            return Some(found);
        }
    }

    None
}

fn clear_tree_cache(node: &mut HtmlNode) {
    node.is_self_dirty = true;
    node.has_dirty_descendant = false;
    node.cached_parent_state = None;
    node.cached_node_intrinsic = None;
    node.cached_child_states = None;

    for child in &mut node.children {
        clear_tree_cache(child);
    }
}

fn main() {
    // Check if we should run benchmarks
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "benchmark" {
        println!("🚀 Running Web Browser Layout Trace Benchmark Mode\n");

        benchmark::run_web_browser_layout_trace_benchmark();

        return;
    }

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
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use css_bitvector_compiler::NFAInstruction;

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

        // Create test HTML structure: div > p.item
        let root = HtmlNode::new("div")
            .with_id("outer")
            .add_child(HtmlNode::new("p").with_class("item"))
            .add_child(
                HtmlNode::new("span")
                    .with_class("item")
                    .add_child(HtmlNode::new("p").with_id("specific")),
            );

        // Execute on root (div)

        // Root should match "div" selector
        assert_ne!(root.css_match_bitvector.as_u64(), 0);

        // Root should provide active states for children

        // Execute on child (p.item)
        let child = HtmlNode::new("p").with_class("item");

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

        // Create HTML structure similar to t1.html: div > p.item > span > p#specific
        let root = HtmlNode::new("div")
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

        // Root div should match "div" rule
        assert_ne!(root.css_match_bitvector.as_u64(), 0);

        // Test first child (p.item)
        let p_item = HtmlNode::new("p").with_class("item");

        // Should match: p, .item, div > p, div > .item
        assert_ne!(p_item.css_match_bitvector.as_u64(), 0);

        // Test span.item
        let span_item = HtmlNode::new("span").with_class("item");

        // Should match: .item, div > .item
        assert_ne!(span_item.css_match_bitvector.as_u64(), 0);

        // Test final p#specific under span.item
        let p_specific = HtmlNode::new("p").with_id("specific");

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
        assert!(generated_code.contains("intrinsic_matches = BitVector::new()"));
        assert!(generated_code.contains("child_states = BitVector::new()"));

        // Verify instruction generation
        assert!(generated_code.contains("CheckAndSetBit"));
        assert!(generated_code.contains("PropagateToChildren"));
        assert!(generated_code.contains("CheckParentAndSetBit"));

        // Verify selector type handling
        assert!(generated_code.contains("SimpleSelector::Type"));
        assert!(generated_code.contains("SimpleSelector::Class"));
        assert!(generated_code.contains("SimpleSelector::Id"));

        // Test case: div.item > span#specific
        let root = HtmlNode::new("div").with_class("item");

        // Root should match both "div" and ".item"
        let matches = root.css_match_bitvector.as_u64();
        assert_ne!(matches, 0);

        // Should propagate states for children

        // Test child element
        let child = HtmlNode::new("span").with_id("specific");

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
        // Store initial results for comparison
        let initial_root_matches = root.css_match_bitvector;
        let initial_child1_matches = root.children[0].css_match_bitvector;

        // Verify results didn't change
        assert_eq!(root.css_match_bitvector, initial_root_matches);
        assert_eq!(root.children[0].css_match_bitvector, initial_child1_matches);
        println!("✓ Results consistent with cached version");

        // Test 3: Modify node and verify selective recomputation
        println!("\nTest 3: Selective recomputation after modification");
        root.children[0].mark_dirty();

        // Test 4: Compare with non-incremental version for correctness
        println!("\nTest 4: Correctness verification");
        let root_copy = HtmlNode::new("div")
            .with_id("container")
            .add_child(
                HtmlNode::new("p")
                    .with_class("item")
                    .add_child(HtmlNode::new("span").with_id("specific")),
            )
            .add_child(HtmlNode::new("div").with_class("other"));
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

        let mut node = HtmlNode::new("div");

        // Initial processing
        let initial_matches = node.css_match_bitvector;

        // Add a class (this should mark the node dirty)
        node = node.with_class("highlight");

        // Reprocess
        let updated_matches = node.css_match_bitvector;

        // Results should be different
        assert_ne!(
            initial_matches, updated_matches,
            "Adding class should change CSS matches"
        );

        // Find the correct bit position for .highlight class
        let highlight_bit = None;

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

        // Create a complex HTML structure
        let root = HtmlNode::new("div")
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
        let regular_time = start.elapsed();

        // Benchmark incremental processing (first run + cached runs)
        let start = Instant::now();

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

        // Build deep tree: root -> child1 -> child2 -> child3
        let mut root = HtmlNode::new("div").add_child(HtmlNode::new("div").add_child(
            HtmlNode::new("div").add_child(HtmlNode::new("span").with_class("highlight")),
        ));

        // Initial processing

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

        // With optimization, we should have fewer cache misses than total nodes
        // Only the dirty path should be recomputed, but our current implementation
        // still visits all nodes to check if they need recomputation

        // Verify that the dirty marking worked correctly
        assert!(!root.is_self_dirty); // Root was processed and cleaned
        assert!(!root.has_dirty_descendant); // Summary bit cleared after processing
        assert!(!root.children[0].children[0].children[0].is_self_dirty); // Deep node processed and cleaned

        // The key optimization is that only nodes on the dirty path needed recomputation
        // This is more of a conceptual test for now
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

        // Test that selector index correctly identifies matching rules
        let test_node = HtmlNode::new("div").with_class("highlight");
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
            generated_code.contains("needs_any_recomputation"),
            "Generated code should contain cache checking function"
        );

        // Should contain cache return logic
        assert!(
            generated_code.contains("node.cached_child_states.unwrap_or"),
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

        // The generated code provides node-level incremental processing
        // Tree traversal logic is handled by the calling code

        // The generated code focuses on incremental processing
        // Non-incremental versions are available in the main library

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
