use css_bitvector_compiler::clean::{EngineKind, binary_main};

fn main() {
    if let Err(error) = binary_main(EngineKind::Naive) {
        eprintln!("naive matcher failed: {error}");
        std::process::exit(1);
    }
}
