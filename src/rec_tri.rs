use css_bitvector_compiler::clean::{EngineKind, binary_main};

fn main() {
    if let Err(error) = binary_main(EngineKind::RecursiveTri, Some("dot_rec_tri.dot")) {
        eprintln!("recursive-tri matcher failed: {error}");
        std::process::exit(1);
    }
}
