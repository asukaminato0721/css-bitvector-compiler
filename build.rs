use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

// Include the shared logic using a path attribute.
// This makes the definitions in shared_codegen_logic.rs available in this scope.
#[path = "src/shared_codegen_logic.rs"]
mod shared_codegen_logic;

// Use the necessary items from the shared module.
use shared_codegen_logic::{
    CssCompiler, GoogleNode, parse_basic_css, TreeNFAProgram, HtmlNode, BitVector, SimpleSelector, CssRule, NFAInstruction
};
// Note: serde_json::Value is used by GoogleNode::from_json, so build.rs needs serde_json in its own build-deps.
// std::collections::{HashMap, HashSet} are used by shared types.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=css-gen-op/google.trace");
    println!("cargo:rerun-if-changed=css-gen-op/https___www.google.com_.css");
    println!("cargo:rerun-if-changed=css-gen-op/common.py");
    println!("cargo:rerun-if-changed=css-gen-op/generate.py");
    println!("cargo:rerun-if-changed=src/shared_codegen_logic.rs"); // Rerun if shared logic changes
    println!("cargo:rerun-if-changed=src/lib.rs"); // Still rerun if lib.rs changes, it might change how shared is used or re-exported

    let _out_dir = env::var("OUT_DIR")?;
    // let command_json_path = Path::new(&out_dir).join("command.json"); // Not currently used for output to OUT_DIR

    // Step 1: Run the Python script to generate command.json
    // Ensure python3 is in PATH
    let python_executable = "python3";
    let script_path = "css-gen-op/generate.py";
    let trace_file_path = "css-gen-op/google.trace";

    // The python script writes to "command.json" in its current working directory.
    // We need it to write to our OUT_DIR.
    // Simplest is to change current_dir for the command, or modify script to take output path.
    // Let's try running it from 'css-gen-op' and then moving the file.
    // Or, even better, run it from repo root but tell it where to put command.json
    // generate.py currently writes to open("command.json", "w")
    // Modifying python script is cleaner, but for now, let's copy.

    let status = Command::new(python_executable)
        .arg(script_path)
        .arg(trace_file_path)
        .current_dir(".") // Run from repo root, generate.py will create ./command.json
        .status()?;

    if !status.success() {
        return Err(format!(
            "Python script {} failed with status {}",
            script_path, status
        )
        .into());
    }
    // Copy the generated command.json to OUT_DIR for record-keeping / use by build script
    // fs::copy("command.json", &command_json_path)?;
    // Actually, the main.rs logic reads from "css-gen-op/command.json".
    // The python script also writes to "command.json" in the current dir.
    // Let's ensure it's consistently "css-gen-op/command.json"
    // The python script `css-gen-op/generate.py` writes `command.json` to its CWD.
    // The `run.sh` implies it's run from the root, so `command.json` is created at the root.
    // Then `src/main.rs` reads `css-gen-op/command.json`. This is inconsistent.
    //
    // Let's assume generate.py is run from css-gen-op directory
    // or modify it to output to a specific path.
    // Given the current plan, `css-gen-op/generate.py python3 css-gen-op/google.trace`
    // implies the script is in `css-gen-op` and trace is also there.
    //
    // Let's run it from css-gen-op, so command.json is created in css-gen-op
    let css_gen_op_dir = Path::new("css-gen-op");
    let status_py = Command::new(python_executable)
        .arg("generate.py") // script name relative to its directory
        .arg("google.trace")  // trace file relative to its directory
        .current_dir(css_gen_op_dir)
        .status()?;

    if !status_py.success() {
        return Err(format!("Python script generate.py failed with status {}", status_py).into());
    }
    // Now "css-gen-op/command.json" should exist.

    // Step 2: Porting logic from process_google_trace_with_rust
    println!("build.rs: 🔍 Processing Google Trace for CSS CodeGen Mode\n");

    let css_content = fs::read_to_string("css-gen-op/https___www.google.com_.css")
        .unwrap_or_else(|_| {
            println!("build.rs: ⚠️ Could not load Google CSS file, using basic rules");
            "div { display: block; } .gbts { color: #000; } #gb { position: relative; }".to_string()
        });

    let css_rules = parse_basic_css(&css_content);
    println!("build.rs: 📋 Loaded {} CSS rules from Google CSS", css_rules.len());

    let mut compiler = CssCompiler::new();
    let program = compiler.compile_css_rules(&css_rules);

    println!("build.rs: 🔧 Generating optimized Rust code...");
    let generated_code = program.generate_rust_code();

    // Read the first command from command.json to get initial DOM
    // This path must be correct based on where generate.py outputs it.
    let commands_content = fs::read_to_string("css-gen-op/command.json")?;
    let first_line = commands_content
        .lines()
        .next()
        .ok_or("Empty command file: css-gen-op/command.json")?;
    let command_json_value: serde_json::Value = serde_json::from_str(first_line)?;

    if command_json_value["name"] != "init" {
        return Err("First command in css-gen-op/command.json should be init".into());
    }

    // Use the GoogleNode from the library crate
    let google_node =
        GoogleNode::from_json(&command_json_value["node"]).ok_or("Failed to parse Google node from command.json")?;

    println!(
        "build.rs: 🌳 Google DOM tree contains {} nodes",
        google_node.count_nodes()
    );

    // Generate complete Rust program for Google trace testing (examples/google_trace_test.rs)
    // This reuses the generate_google_trace_program logic, which needs to be defined or ported.
    let complete_example_program = build_generate_google_trace_program(&generated_code, &google_node)?;

    let example_file_path = Path::new("examples").join("google_trace_test.rs");
    fs::create_dir_all(example_file_path.parent().unwrap())?; // Ensure examples dir exists
    fs::write(&example_file_path, complete_example_program)
        .map_err(|e| format!("build.rs: Failed to write generated example {}: {}", example_file_path.display(), e))?;

    println!("build.rs: 💾 Generated example: {}", example_file_path.display());

    // Generate functions for benchmark usage (src/generated_css_functions.rs)
    let functions_file_path = Path::new("src").join("generated_css_functions.rs");
    fs::create_dir_all(functions_file_path.parent().unwrap())?; // Ensure src dir exists
    fs::write(&functions_file_path, &generated_code)
        .map_err(|e| format!("build.rs: Failed to write generated functions {}: {}", functions_file_path.display(), e))?;

    println!("build.rs: 💾 Generated functions: {}", functions_file_path.display());

    // The original main.rs also ran the example. build.rs should not run examples.
    // It just generates files. The test or run step will execute it.

    Ok(())
}

// Ported from main.rs:generate_google_trace_program
// Minor changes: takes &GoogleNode from css_bitvector_compiler::GoogleNode
fn build_generate_google_trace_program(
    generated_fn_code: &str,
    _google_node: &GoogleNode, // Parameter kept for signature consistency, though not used in current body
) -> Result<String, Box<dyn std::error::Error>> {
    let mut program_content = String::new();

    // 1. Import types and functions from the library crate
    program_content.push_str("use css_bitvector_compiler::*;\n");
    // If GoogleNode is needed by the example, it's already part of the import.
    // If HtmlNode is needed by the example for tree construction, it's also imported.

    program_content.push_str("\n");

    // 2. Add the generated CSS processing functions
    program_content.push_str("// Generated CSS processing functions (from build.rs)\n");
    program_content.push_str(generated_fn_code);
    program_content.push_str("\n\n");

    // 3. Add a main function for the example
    // The original example in main.rs had an empty main, which implies it was a lib or just for compilation check.
    // Let's keep it simple. If it needs to *do* something, that logic would be here.
    // For now, an empty main is fine if the purpose is to check compilation of generated code.
    program_content.push_str("fn main() {\n");
    program_content.push_str("    // This is the main function for the generated example.\n");
    program_content.push_str("    // It can be used to run tests or benchmarks with the generated code.\n");
    program_content.push_str("    // For now, it's empty, ensuring the generated code compiles.\n");
    program_content.push_str("    println!(\"Generated example google_trace_test.rs executed.\");\n");
    // To actually use the DOM, we'd need to serialize google_node or reconstruct it.
    // The original `process_google_trace_with_rust` didn't pass the DOM to the *example's main*.
    // It was used to generate `complete_program`. The example itself was minimal.
    program_content.push_str("}\n");

    Ok(program_content)
}
