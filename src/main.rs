use css_bitvector_compiler::{CssCompiler, HtmlNode, parse_css};

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
}

pub fn codegen() -> Result<(), Box<dyn std::error::Error>> {
    let css_content = std::fs::read_to_string(format!(
        "css-gen-op/{}/{}.css",
        std::env::var("WEBSITE_NAME").unwrap(),
        std::env::var("WEBSITE_NAME").unwrap()
    ))
    .expect("fail to read css file");

    let css_rules = parse_css(&css_content);
    println!("📋 Loaded {} CSS rules from CSS", css_rules.len());

    let mut compiler = CssCompiler::new();
    let program = compiler.compile_css_rules(&css_rules);

    println!("🔧 Generating optimized Rust code...");
    let generated_code = program.generate_rust_istate_code();

    // Read the first command from command.json to get initial DOM
    let commands_content = std::fs::read_to_string(format!(
        "css-gen-op/{}/command.json",
        std::env::var("WEBSITE_NAME").unwrap()
    ))?;
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

    Ok(())
}

fn generate_google_trace_program(
    _generated_fn_code: &str,
    _google_node: &GoogleNode,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut program = String::new();

    program.push_str("use css_bitvector_compiler::*;\n");

    program.push_str(r#"fn collect_all_matches(node: &mut HtmlNode, parent_state: &BitVector, results: &mut Vec<(String, Vec<usize>)>) {
    // Process this node
    let child_states = process_node_generated_incremental(node, parent_state);
    
    // Collect matches for this node
    let mut matches = Vec::new();
    for i in 0..BITVECTOR_CAPACITY {
        if node.css_match_bitvector.is_bit_set(i) {
            matches.push(i);
        }
    }
    
    // Create node identifier using utility function
    let node_id = create_node_identifier(node);
    results.push((node_id, matches));
    
    // Process children
    for child in &mut node.children {
        collect_all_matches(child, &child_states, results);
    }
}

"#);

    // 4. 添加主函数
    program.push_str(r#"fn main() {
    println!("🚀 Testing OPTIMIZED Layout Calculation with Google Trace");
    
    // Load Google DOM tree
    let mut root = load_dom_from_file();
    println!("✅ Loaded Google DOM tree successfully!");
    
    let initial_state = BitVector::with_capacity(BITVECTOR_CAPACITY);
    let mut optimized_results = Vec::new();
    
    // Collect all matching results
    collect_all_matches(&mut root, &initial_state, &mut optimized_results);
    
    println!("📊 OPTIMIZED Results Summary:");
    println!("Total nodes processed: {}", optimized_results.len());
    
    // Output first few nodes for verification
    println!("\n🔍 First 10 nodes with their CSS rule matches:");
    for (i, (node_id, matches)) in optimized_results.iter().take(10).enumerate() {
        println!("Node {}: {} -> {} rules matched", i+1, node_id, matches.len());
        if !matches.is_empty() {
            let rule_list: Vec<String> = matches.iter().take(5).map(|&r| r.to_string()).collect();
            println!("  Rules: {} {}", rule_list.join(", "), if matches.len() > 5 { "..." } else { "" });
        }
    }
    
    // Save results to file for comparison
    if let Err(e) = save_results_to_file(&optimized_results, "optimized_results.txt") {
        println!("Failed to save optimized results: {}", e);
        return;
    }
    
    println!("\n💾 Full optimized results saved to: optimized_results.txt");
    println!("🔄 Run the naive example to compare results!");
}"#);

    Ok(program)
}

fn main() {
    // Test Google trace integration
    println!("\n=== GOOGLE TRACE INTEGRATION TEST ===");
    if let Err(e) = codegen() {
        println!("Google trace test failed: {}", e);
        println!("This is expected if css-gen-op files are not available");
    }
}
