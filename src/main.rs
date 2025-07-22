use std::path::Path;

use css_bitvector_compiler::{CssCompiler, parse_css};

fn main() {
    let website = std::env::var("WEBSITE_NAME").unwrap();
    let css_content = std::fs::read_to_string(format!("css-gen-op/{website}/{website}.css",))
        .expect("fail to read css file");

    let css_rules = parse_css(&css_content);

    let mut compiler = CssCompiler::new();
    let program = compiler.compile_css_rules(&css_rules);
    std::fs::write(
        "src/generated_istate_functions.rs",
        &program.generate_istate_code(),
    )
    .unwrap();
    std::fs::write(
        "src/generated_bitvector_functions.rs",
        program.generate_bitvector_code(),
    )
    .unwrap();
    std::fs::write(
        "src/generated_naive_functions.rs",
        program.generate_naive_rust_code(),
    )
    .unwrap();
    // for read the gen code
    let p = Path::new("css-gen-op").join(website);
    std::fs::write(
        p.join("generated_istate_functions.rs"),
        &program.generate_istate_code(),
    )
    .unwrap();
    std::fs::write(
        p.join("generated_bitvector_functions.rs"),
        program.generate_bitvector_code(),
    )
    .unwrap();
    std::fs::write(
        p.join("generated_naive_functions.rs"),
        program.generate_naive_rust_code(),
    )
    .unwrap();
}
