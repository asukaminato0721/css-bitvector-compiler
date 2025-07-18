use css_bitvector_compiler::{CssCompiler, parse_css};

fn main() {
    let css_content = std::fs::read_to_string(format!(
        "css-gen-op/{}/{}.css",
        std::env::var("WEBSITE_NAME").unwrap(),
        std::env::var("WEBSITE_NAME").unwrap()
    ))
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
}
