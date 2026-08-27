use css_bitvector_compiler::clean::{EngineKind, binary_main};

fn main() {
    if let Err(error) = binary_main(EngineKind::Bit, Some("dot.dot")) {
        eprintln!("bit matcher failed: {error}");
        std::process::exit(1);
    }
}
