use css_bitvector_compiler::clean::{EngineKind, binary_main};

fn main() {
    if let Err(error) = binary_main(EngineKind::Tri, Some("dot_tri.dot")) {
        eprintln!("tri matcher failed: {error}");
        std::process::exit(1);
    }
}
