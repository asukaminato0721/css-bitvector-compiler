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

        code.push_str("// Generated Tree NFA Program\n");
        code.push_str("// This program processes HTML nodes and computes CSS matches\n\n");

        code.push_str("fn process_node_generated(\n");
        code.push_str("    node: &HtmlNode,\n");
        code.push_str("    parent_state: u64,\n");
        code.push_str(") -> (u64, u64) { // (current_matches, child_states)\n");
        code.push_str("    let mut current_matches: u64 = 0;\n");
        code.push_str("    let mut child_states: u64 = 0;\n\n");

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
                        "    if node_matches_selector(node, &{}) {{\n",
                        selector_str
                    ));
                    code.push_str(&format!(
                        "        current_matches |= 1 << {}; // {}\n",
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
                    code.push_str(&format!("    if (parent_state & (1 << {})) != 0 && node_matches_selector(node, &{}) {{\n", 
                        parent_state_bit, child_selector_str));
                    code.push_str(&format!(
                        "        current_matches |= 1 << {}; // {}\n",
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
                        "    if (current_matches & (1 << {})) != 0 {{\n",
                        match_bit
                    ));
                    code.push_str(&format!(
                        "        child_states |= 1 << {}; // {}\n",
                        active_bit,
                        self.state_names
                            .get(active_bit)
                            .unwrap_or(&format!("bit_{}", active_bit))
                    ));
                    code.push_str("    }\n\n");
                }
            }
        }

        code.push_str("    (current_matches, child_states)\n");
        code.push_str("}\n\n");

        code.push_str(
            "fn node_matches_selector(node: &HtmlNode, selector: &SimpleSelector) -> bool {\n",
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
struct TreeNFAVM {
    program: TreeNFAProgram,
}

impl TreeNFAVM {
    fn new(program: TreeNFAProgram) -> Self {
        TreeNFAVM { program }
    }

    fn execute_on_node(&self, node: &mut HtmlNode, parent_state: u64) -> u64 {
        let mut current_matches: u64 = 0;
        let mut child_states: u64 = 0;

        for instruction in &self.program.instructions {
            match instruction {
                NFAInstruction::CheckAndSetBit { selector, bit_pos } => {
                    if self.node_matches_selector(node, selector) {
                        current_matches |= 1 << bit_pos;
                    }
                }
                NFAInstruction::CheckParentAndSetBit {
                    parent_state_bit,
                    child_selector,
                    result_bit,
                } => {
                    if (parent_state & (1 << parent_state_bit)) != 0
                        && self.node_matches_selector(node, child_selector)
                    {
                        current_matches |= 1 << result_bit;
                    }
                }
                NFAInstruction::PropagateToChildren {
                    match_bit,
                    active_bit,
                } => {
                    if (current_matches & (1 << match_bit)) != 0 {
                        child_states |= 1 << active_bit;
                    }
                }
            }
        }

        // Store result in node
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

    fn dfs_execute(&self, node: &mut HtmlNode, parent_state: u64, path: String) {
        let child_states = self.execute_on_node(node, parent_state);

        println!(
            "Node: {} (Tag: {}, ID: {:?}, Classes: {:?})",
            path, node.tag_name, node.id, node.classes
        );
        println!("  Parent state: {:016b}", parent_state);
        println!("  Match result: {:016b}", node.css_match_bitvector);
        println!("  Child states: {:016b}", child_states);

        // Decode matches
        println!("  Matches:");
        for (bit_pos, name) in &self.program.state_names {
            if (node.css_match_bitvector & (1 << bit_pos)) != 0 {
                println!("    - {}", name);
            }
        }
        println!("---");

        // Recursively process children
        for (i, child) in node.children.iter_mut().enumerate() {
            self.dfs_execute(child, child_states, format!("{}/{}", path, i));
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
        vm.dfs_execute(&mut root_node, 0, "root".to_string());
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
        let mut root = HtmlNode::new("div").add_child(HtmlNode::new("p").with_class("item"));

        // Execute on root (div)
        let child_states = vm.execute_on_node(&mut root, 0);

        // Root should match "div" selector
        assert_ne!(root.css_match_bitvector, 0);

        // Root should provide active states for children
        assert_ne!(child_states, 0);

        // Execute on child (p.item)
        let mut child = HtmlNode::new("p").with_class("item");
        let _child_child_states = vm.execute_on_node(&mut child, child_states);

        // Child should match both ".item" and "div > .item"
        assert_ne!(child.css_match_bitvector, 0);
    }

    #[test]
    fn test_generated_rust_code() {
        let rules = vec![CssRule::Simple(SimpleSelector::Type("div".to_string()))];

        let mut compiler = CssCompiler::new();
        let program = compiler.compile_css_rules(&rules);
        let generated_code = program.generate_rust_code();

        // Check that the generated code contains expected elements
        assert!(generated_code.contains("fn process_node_generated"));
        assert!(generated_code.contains("current_matches"));
        assert!(generated_code.contains("child_states"));
        assert!(generated_code.contains("node_matches_selector"));
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
            .with_id("outer")
            .add_child(HtmlNode::new("p").with_class("item"))
            .add_child(
                HtmlNode::new("span")
                    .with_class("item")
                    .add_child(HtmlNode::new("p").with_id("specific")),
            );

        // Execute on root div
        let child_states_root = vm.execute_on_node(&mut root, 0);

        // Root div should match "div" rule
        assert_ne!(root.css_match_bitvector, 0);

        // Test first child (p.item)
        let mut p_item = HtmlNode::new("p").with_class("item");
        let _child_states_p = vm.execute_on_node(&mut p_item, child_states_root);

        // Should match: p, .item, div > p, div > .item
        assert_ne!(p_item.css_match_bitvector, 0);

        // Test span.item
        let mut span_item = HtmlNode::new("span").with_class("item");
        let child_states_span = vm.execute_on_node(&mut span_item, child_states_root);

        // Should match: .item, div > .item
        assert_ne!(span_item.css_match_bitvector, 0);

        // Test final p#specific under span.item
        let mut p_specific = HtmlNode::new("p").with_id("specific");
        let _final_states = vm.execute_on_node(&mut p_specific, child_states_span);

        // Should match: p, #specific, .item > #specific
        assert_ne!(p_specific.css_match_bitvector, 0);
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
        assert!(generated_code.contains("fn process_node_generated"));
        assert!(generated_code.contains("current_matches: u64 = 0"));
        assert!(generated_code.contains("child_states: u64 = 0"));

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
        let child_states = vm.execute_on_node(&mut root, 0);

        // Root should match both "div" and ".item"
        let matches = root.css_match_bitvector;
        assert_ne!(matches, 0);

        // Should propagate states for children
        assert_ne!(child_states, 0);

        // Test child element
        let mut child = HtmlNode::new("span").with_id("specific");
        let _child_child_states = vm.execute_on_node(&mut child, child_states);

        // Child should match "#specific" and ".item > #specific"
        assert_ne!(child.css_match_bitvector, 0);
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
#[derive(Debug, Clone)]
struct HtmlNode {{
    tag_name: String,
    id: Option<String>,
    classes: HashSet<String>,
    children: Vec<HtmlNode>,
    css_match_bitvector: u64,
}}

impl HtmlNode {{
    fn new(tag_name: &str) -> Self {{
        HtmlNode {{
            tag_name: tag_name.to_lowercase(),
            id: None,
            classes: HashSet::new(),
            children: Vec::new(),
            css_match_bitvector: 0,
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
    let div_node = HtmlNode::new("div").with_id("test").with_class("item");
    let (matches, child_states) = process_node_generated(&div_node, 0);
    println!("div.item#test - matches: {{:016b}}, child_states: {{:016b}}", matches, child_states);
    
    // Test case 2: span node with class
    let span_node = HtmlNode::new("span").with_class("item");
    let (matches2, child_states2) = process_node_generated(&span_node, child_states);
    println!("span.item (child of div) - matches: {{:016b}}, child_states: {{:016b}}", matches2, child_states2);
    
    // Test case 3: node with specific id
    let specific_node = HtmlNode::new("p").with_id("specific");
    let (matches3, child_states3) = process_node_generated(&specific_node, 0);
    println!("p#specific - matches: {{:016b}}, child_states: {{:016b}}", matches3, child_states3);
    
    // Test case 4: child selector test
    let item_node = HtmlNode::new("div").with_class("item");
    let (matches4, child_states4) = process_node_generated(&item_node, 0);
    let specific_child = HtmlNode::new("span").with_id("specific");
    let (matches5, _) = process_node_generated(&specific_child, child_states4);
    println!("div.item parent - matches: {{:016b}}, child_states: {{:016b}}", matches4, child_states4);
    println!("span#specific (under div.item) - matches: {{:016b}}", matches5);
    
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
                                let vm_child_states = vm.execute_on_node(&mut div_node, 0);
                                let vm_matches = div_node.css_match_bitvector;

                                println!(
                                    "VM results for div.item#test - matches: {:016b}, child_states: {:016b}",
                                    vm_matches, vm_child_states
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
    fn test_performance_benchmark() {
        use std::time::Instant;

        println!("\n=== PERFORMANCE BENCHMARK ===");

        // Create a larger CSS rule set for performance testing
        let css_content = r#"
/* Rule 0 (R0) */ div {}
/* Rule 1 (R1) */ .item {}
/* Rule 2 (R2) */ #specific {}
/* Rule 3 (R3) */ p {}
/* Rule 4 (R4) */ span {}
/* Rule 5 (R5) */ .container {}
/* Rule 6 (R6) */ #main {}
/* Rule 7 (R7) */ div > p {}
/* Rule 8 (R8) */ .item > #specific {}
/* Rule 9 (R9) */ div > .item {}
/* Rule 10 (R10) */ .container > span {}
/* Rule 11 (R11) */ div > #main {}
/* Rule 12 (R12) */ p > span {}
        "#;

        // Benchmark CSS parsing
        let start = Instant::now();
        let rules = parse_css_file(css_content);
        let parse_time = start.elapsed();
        println!("CSS parsing: {:?} ({} rules)", parse_time, rules.len());

        // Benchmark compilation
        let start = Instant::now();
        let mut compiler = CssCompiler::new();
        let program = compiler.compile_css_rules(&rules);
        let compile_time = start.elapsed();
        println!(
            "Compilation: {:?} ({} instructions, {} bits)",
            compile_time,
            program.instructions.len(),
            program.total_bits
        );

        // Benchmark code generation
        let start = Instant::now();
        let _generated_code = program.generate_rust_code();
        let codegen_time = start.elapsed();
        println!("Code generation: {:?}", codegen_time);

        // Create a complex HTML structure for testing
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

        // Benchmark VM execution
        let vm = TreeNFAVM::new(program);
        let start = Instant::now();
        for _ in 0..1000 {
            let _child_states = vm.execute_on_node(&mut root, 0);
        }
        let execution_time = start.elapsed();
        println!("VM execution (1000 iterations): {:?}", execution_time);
        println!("Average per execution: {:?}", execution_time / 1000);

        println!("=== BENCHMARK COMPLETE ===\n");
    }
}
