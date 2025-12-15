use css_bitvector_compiler::{parse_css_with_pseudo, partition_simple_selectors};
use std::{error::Error, fs, path::PathBuf};

struct CssInput {
    label: String,
    path: PathBuf,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("css stat failed: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let input = resolve_input()?;
    let css = fs::read_to_string(&input.path)?;
    let parsed = parse_css_with_pseudo(&css);
    let (supported_selectors, skipped_simple) =
        partition_simple_selectors(parsed.selectors.clone());

    let mut pseudo_breakdown: Vec<(String, usize)> = parsed
        .pseudo_selectors
        .iter()
        .map(|(pseudo, selectors)| (pseudo.clone(), selectors.len()))
        .collect();
    pseudo_breakdown.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    let pseudo_total: usize = pseudo_breakdown.iter().map(|(_, count)| *count).sum();
    let unsupported_total = parsed.unsupported_selectors.len();

    let total_unique =
        supported_selectors.len() + skipped_simple.len() + pseudo_total + unsupported_total;

    println!("CSS stats for {}", input.label);
    println!("Source: {}", input.path.display());
    if total_unique == 0 {
        println!("No selectors found.");
        return Ok(());
    }

    let supported_pct = (supported_selectors.len() as f64 / total_unique as f64) * 100.0;
    println!(
        "Supported selectors: {} / {} ({supported_pct:.2}%)",
        supported_selectors.len(),
        total_unique
    );
    println!("Skipped simple selectors: {}", skipped_simple.len());
    println!("Unsupported pseudo-derived selectors: {pseudo_total}");
    println!("Unsupported complex selectors: {}", unsupported_total);

    if !pseudo_breakdown.is_empty() {
        println!("\nTop unsupported pseudo-classes:");
        for (pseudo, count) in pseudo_breakdown.iter().take(10) {
            println!("  {pseudo:<20} {count}");
        }
        if pseudo_breakdown.len() > 10 {
            println!("  ...");
        }
    }

    if !parsed.unsupported_selectors.is_empty() {
        println!("\nExamples of unsupported selectors:");
        for selector in parsed.unsupported_selectors.iter().take(10) {
            println!("  {selector}");
        }
        if parsed.unsupported_selectors.len() > 10 {
            println!("  ...");
        }
    }

    if !skipped_simple.is_empty() {
        println!("\nExamples of skipped simple selectors:");
        for selector in skipped_simple.iter().take(10) {
            println!("  {selector}");
        }
        if skipped_simple.len() > 10 {
            println!("  ...");
        }
    }

    if !supported_selectors.is_empty() {
        println!("\nSupported selector sample:");
        for selector in supported_selectors.iter().take(25) {
            println!("  {selector}");
        }
        if supported_selectors.len() > 25 {
            println!("  ...");
        }
    }

    Ok(())
}

fn resolve_input() -> Result<CssInput, Box<dyn Error>> {
    if let Some(path) = std::env::args().nth(1) {
        return Ok(CssInput {
            label: derive_label(&path),
            path: PathBuf::from(path),
        });
    }

    if let Ok(path) = std::env::var("CSS_PATH") {
        return Ok(CssInput {
            label: derive_label(&path),
            path: PathBuf::from(path),
        });
    }

    let site = std::env::var("WEBSITE_NAME")?;
    let default_path = format!("css-gen-op/{0}/{0}.css", site);
    Ok(CssInput {
        label: site,
        path: PathBuf::from(default_path),
    })
}

fn derive_label(path: &str) -> String {
    PathBuf::from(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| path.to_string())
}
