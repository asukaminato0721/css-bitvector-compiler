use crate::context::{Workspace, value_after};
use css_bitvector_compiler::clean::CompiledProgram;
use std::{cmp::Reverse, error::Error, fs, path::PathBuf};

pub fn execute(workspace: &Workspace, arguments: &[String]) -> Result<(), Box<dyn Error>> {
    let path = if let Some(path) = value_after(arguments, "--css") {
        PathBuf::from(path)
    } else {
        let site = value_after(arguments, "--site").ok_or("stats requires --site or --css")?;
        workspace
            .root()
            .join("css-gen-op")
            .join(site)
            .join(format!("{site}.css"))
    };
    let program = CompiledProgram::compile(&fs::read_to_string(&path)?)?;
    let supported = program.selectors.len();
    let skipped = program.report.skipped_simple.len();
    let unsupported = program.report.unsupported.len();
    let total = supported + skipped + unsupported;
    println!("CSS stats for {}", path.display());
    if total == 0 {
        println!("No selectors found.");
        return Ok(());
    }
    println!(
        "Supported selectors: {supported} / {total} ({:.2}%)",
        supported as f64 / total as f64 * 100.0
    );
    println!("Skipped simple selectors: {skipped}");
    println!("Unsupported selectors: {unsupported}");
    if !program.report.unsupported_pseudos.is_empty() {
        println!("\nTop unsupported pseudo-classes:");
        let mut pseudos: Vec<_> = program.report.unsupported_pseudos.iter().collect();
        pseudos.sort_by_key(|entry| Reverse(entry.1.len()));
        for (pseudo, selectors) in pseudos.into_iter().take(10) {
            println!("  {pseudo:<24} {}", selectors.len());
        }
    }
    if !program.report.unsupported.is_empty() {
        println!("\nUnsupported examples:");
        for selector in program.report.unsupported.iter().take(10) {
            println!("  {} ({})", selector.selector, selector.reason);
        }
    }
    Ok(())
}
