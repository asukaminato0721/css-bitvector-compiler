use regex::Regex;
use scraper::{Html, Selector as HtmlSelector};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

// --- 1. HTML Node Structure ---
#[derive(Debug, Clone)]
struct HtmlNode {
    tag_name: String,
    id: Option<String>,
    classes: HashSet<String>,
    children: Vec<HtmlNode>,
    // Stores the bitvector of *final* matching rules for this node
    css_match_bitvector: u64,
}

impl HtmlNode {
    fn new(tag_name: &str) -> Self {
        HtmlNode {
            tag_name: tag_name.to_lowercase(),
            id: None,
            classes: HashSet::new(),
            children: Vec::new(),
            css_match_bitvector: 0,
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
        parent_selector: SimpleSelector, // Selector for the parent
        child_selector: SimpleSelector,  // Selector for the current node (the child)
    },
}

// --- 3. CSS Parser ---
fn parse_css_file(css_content: &str) -> Vec<CssRule> {
    let mut rules = Vec::new();

    // Simple regex patterns for parsing CSS rules
    let simple_rule_regex = Regex::new(r"/\* Rule \d+ \([^)]+\) \*/ ([^{]+) \{\}").unwrap();
    let child_rule_regex = Regex::new(r"([^>]+)>([^{]+)").unwrap();

    for cap in simple_rule_regex.captures_iter(css_content) {
        let selector_str = cap[1].trim();

        if let Some(child_cap) = child_rule_regex.captures(selector_str) {
            // Child selector (A > B)
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
        } else {
            // Simple selector
            if let Some(simple_sel) = parse_simple_selector(selector_str) {
                rules.push(CssRule::Simple(simple_sel));
            }
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

// --- 4. HTML Parser ---
fn parse_html_file(html_content: &str) -> Option<HtmlNode> {
    let document = Html::parse_document(html_content);

    // Find the first meaningful element (div, section, p, span)
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

    // Parse attributes
    if let Some(id) = element.attr("id") {
        node = node.with_id(id);
    }

    if let Some(class_attr) = element.attr("class") {
        for class in class_attr.split_whitespace() {
            node = node.with_class(class);
        }
    }

    // Parse children
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

// --- 5. Bit Index Management ---
fn get_bit_indices_from_rules(rules: &[CssRule]) -> HashMap<String, usize> {
    let mut bit_indices = HashMap::new();
    let mut current_idx = 0;

    // Helper to add rule and its states
    let add_simple_rule =
        |selector: &SimpleSelector, map: &mut HashMap<String, usize>, idx: &mut usize| {
            let rule_key = format!("match_{:?}", selector);
            let active_key = format!("active_child_{:?}", selector);

            map.insert(rule_key, *idx);
            *idx += 1;
            map.insert(active_key, *idx);
            *idx += 1;
        };

    let add_child_rule = |parent_sel: &SimpleSelector,
                          child_sel: &SimpleSelector,
                          map: &mut HashMap<String, usize>,
                          idx: &mut usize| {
        let rule_key = format!("match_{:?}_gt_{:?}", parent_sel, child_sel);
        map.insert(rule_key, *idx);
        *idx += 1;
    };

    // Process all rules and assign bit indices
    for rule in rules {
        match rule {
            CssRule::Simple(selector) => {
                let rule_key = format!("match_{:?}", selector);
                if !bit_indices.contains_key(&rule_key) {
                    add_simple_rule(selector, &mut bit_indices, &mut current_idx);
                }
            }
            CssRule::Child {
                parent_selector,
                child_selector,
            } => {
                // Ensure parent selector has active_child state
                let parent_active_key = format!("active_child_{:?}", parent_selector);
                if !bit_indices.contains_key(&parent_active_key) {
                    add_simple_rule(parent_selector, &mut bit_indices, &mut current_idx);
                }

                // Add the child rule
                add_child_rule(
                    parent_selector,
                    child_selector,
                    &mut bit_indices,
                    &mut current_idx,
                );
            }
        }
    }

    bit_indices
}

// --- 6. node_matches_simple_selector ---
fn node_matches_simple_selector(node: &HtmlNode, selector: &SimpleSelector) -> bool {
    match selector {
        SimpleSelector::Type(tag) => node.tag_name == *tag,
        SimpleSelector::Class(class) => node.classes.contains(class),
        SimpleSelector::Id(id) => node.id.as_deref() == Some(id),
    }
}

// --- 7. process_node ---
fn process_node(
    node: &mut HtmlNode, // Mutable to store its own match result
    parent_active_child_states_bv: u64,
    rules: &[CssRule],
    bit_indices: &HashMap<String, usize>,
) -> u64 {
    // Returns: output_states_for_children_bv
    let mut current_node_match_result_bv: u64 = 0;
    let mut output_states_for_children_bv: u64 = 0;

    // --- Step 1 & 2: Evaluate all rules for the current node ---
    for rule in rules {
        match rule {
            CssRule::Simple(simple_selector) => {
                if node_matches_simple_selector(node, simple_selector) {
                    let match_idx = bit_indices[&format!("match_{:?}", simple_selector)];
                    current_node_match_result_bv |= 1 << match_idx;
                }
            }
            CssRule::Child {
                parent_selector,
                child_selector,
            } => {
                // Check if the parent had activated the necessary state
                let parent_active_idx_key = format!("active_child_{:?}", parent_selector);
                if let Some(&parent_active_idx) = bit_indices.get(&parent_active_idx_key) {
                    if (parent_active_child_states_bv & (1 << parent_active_idx)) != 0 {
                        // Parent was active, now check if current node matches the child part
                        if node_matches_simple_selector(node, child_selector) {
                            let match_idx = bit_indices
                                [&format!("match_{:?}_gt_{:?}", parent_selector, child_selector)];
                            current_node_match_result_bv |= 1 << match_idx;
                        }
                    }
                }
            }
        }
    }

    // --- Step 3: Based on *simple* matches on the current node, set active_child states for its children ---
    for rule in rules {
        if let CssRule::Simple(simple_selector) = rule {
            let match_idx_key = format!("match_{:?}", simple_selector);
            if let Some(&match_idx) = bit_indices.get(&match_idx_key) {
                if (current_node_match_result_bv & (1 << match_idx)) != 0 {
                    // This simple selector matched the current node
                    let active_idx_key = format!("active_child_{:?}", simple_selector);
                    if let Some(&active_idx) = bit_indices.get(&active_idx_key) {
                        output_states_for_children_bv |= 1 << active_idx;
                    }
                }
            }
        }
    }

    // --- Step 4: Store current node's final match result ---
    node.css_match_bitvector = current_node_match_result_bv;

    // --- Step 5: Return active_child states for children ---
    output_states_for_children_bv
}

// --- 8. DFS Traversal ---
fn dfs_visit(
    node: &mut HtmlNode,
    parent_active_child_states_bv: u64,
    rules: &[CssRule],
    bit_indices: &HashMap<String, usize>,
    path: String, // For printing node path
) {
    let active_states_for_its_children_bv =
        process_node(node, parent_active_child_states_bv, rules, bit_indices);

    println!(
        "Node: {} (Tag: {}, ID: {:?}, Classes: {:?})",
        path, node.tag_name, node.id, node.classes
    );
    println!("  Match BV (self): {:016b}", node.css_match_bitvector);
    println!(
        "  Active BV (for children): {:016b}",
        active_states_for_its_children_bv
    );
    println!("  Matches:");

    // Decode matches for readability
    for (key, &idx) in bit_indices.iter() {
        if key.starts_with("match_") && (node.css_match_bitvector & (1 << idx)) != 0 {
            println!("    - {}", key);
        }
    }
    println!("---");

    for (i, child) in node.children.iter_mut().enumerate() {
        dfs_visit(
            child,
            active_states_for_its_children_bv,
            rules,
            bit_indices,
            format!("{}/{}", path, i),
        );
    }
}

// --- 9. Test runner ---
fn run_test(css_file: &str, html_file: &str) {
    println!("=== Testing {} with {} ===", html_file, css_file);

    // Read and parse CSS
    let css_content = match fs::read_to_string(css_file) {
        Ok(content) => content,
        Err(e) => {
            println!("Error reading CSS file {}: {}", css_file, e);
            return;
        }
    };

    let rules = parse_css_file(&css_content);
    let bit_indices = get_bit_indices_from_rules(&rules);

    println!("Parsed CSS Rules: {:#?}", rules);

    // Print bit indices map with binary values
    println!("Bit Indices Map: {{");
    for (key, &value) in &bit_indices {
        println!("    \"{}\": {:04b} ({}),", key, value, value);
    }
    println!("}}");

    println!("Total bit states used: {}", bit_indices.len());

    if bit_indices.len() > 64 {
        println!("WARNING: More than 64 states defined, u64 bitvector will overflow!");
    }

    // Read and parse HTML
    let html_content = match fs::read_to_string(html_file) {
        Ok(content) => content,
        Err(e) => {
            println!("Error reading HTML file {}: {}", html_file, e);
            return;
        }
    };

    // Parse HTML and extract the root element (skip html/body wrapper if present)
    let document = Html::parse_document(&html_content);
    let meaningful_selector = HtmlSelector::parse("div, section, p, span").unwrap();

    if let Some(root_element) = document.select(&meaningful_selector).next() {
        if let Some(mut root_node) = parse_element_to_node(&root_element) {
            println!("Parsed HTML structure:");
            print_html_structure(&root_node, 0);
            println!("\n--- Running CSS Matching ---");

            // Run matching
            dfs_visit(&mut root_node, 0, &rules, &bit_indices, "root".to_string());
        } else {
            println!("Failed to parse HTML structure");
        }
    } else {
        println!("No meaningful HTML elements found");
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

fn main() {
    let tests_dir = "tests";
    let css_file = format!("{}/test.css", tests_dir);

    // Test with all HTML files
    for i in 1..=4 {
        let html_file = format!("{}/t{}.html", tests_dir, i);

        if Path::new(&html_file).exists() && Path::new(&css_file).exists() {
            run_test(&css_file, &html_file);
        } else {
            println!("Missing test files: {} or {}", css_file, html_file);
        }
    }
}
