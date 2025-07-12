use css_bitvector_compiler::{
    CssCompiler, convert_json_dom_to_html_node, count_total_nodes, parse_css,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 CSS Naive Layout Code Generator");
    println!("📋 Generating layout calculation code without cache or optimization\n");

    // Load Google CSS rules (same as main binary for consistency)
    let css_content = std::fs::read_to_string(format!(
        "css-gen-op/{}/{}.css",
        std::env::var("WEBSITE_NAME").unwrap(),
        std::env::var("WEBSITE_NAME").unwrap()
    ))
    .unwrap_or_else(|_| {
        println!("⚠️ Could not load Google CSS file, using basic rules");
        "div { display: block; } .gbts { color: #000; } #gb { position: relative; }".to_string()
    });

    let css_rules = parse_css(&css_content);
    println!("📋 Loaded {} CSS rules from Google CSS", css_rules.len());

    // Compile CSS rules
    let mut compiler = CssCompiler::new();
    let program = compiler.compile_css_rules(&css_rules);

    println!("🔧 Generating naive Rust code (no caching, no optimization)...");
    let naive_code = program.generate_naive_rust_code();

    // Read the first command from command.json to get initial DOM (for consistency)
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

    let node = convert_json_dom_to_html_node(&command["node"]);

    println!(
        "🌳 Google DOM tree contains {} nodes",
        count_total_nodes(&node)
    );
    // Also generate naive functions for direct usage
    let functions_file = "src/generated_naive_functions.rs";
    std::fs::write(functions_file, &naive_code)
        .map_err(|e| format!("Failed to write generated functions: {}", e))?;

    println!("💾 Generated naive functions: {}", functions_file);

    Ok(())
}
