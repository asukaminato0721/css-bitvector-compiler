use crate::{
    context::{Workspace, value_after},
    run::SiteResult,
};
use std::{error::Error, fs, path::PathBuf};

pub fn execute(workspace: &Workspace, arguments: &[String]) -> Result<(), Box<dyn Error>> {
    let base = value_after(arguments, "--base")
        .map(PathBuf::from)
        .unwrap_or_else(|| workspace.root().join("css-gen-op"));
    let output_directory = value_after(arguments, "--output-dir")
        .map(PathBuf::from)
        .unwrap_or_else(|| workspace.root().to_path_buf());
    fs::create_dir_all(&output_directory)?;
    let results = collect_results(&base)?;
    if results.is_empty() {
        return Err(
            "no results.json files found; run `cargo xtask run --all --update` first".into(),
        );
    }
    atomic_write(
        &output_directory.join("misscnt.html"),
        html(&results).as_bytes(),
    )?;
    println!(
        "generated one misscnt.html report for {} site(s) in {}",
        results.len(),
        output_directory.display()
    );
    Ok(())
}

fn collect_results(base: &std::path::Path) -> Result<Vec<SiteResult>, Box<dyn Error>> {
    let mut results = Vec::new();
    for entry in fs::read_dir(base)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let path = entry.path().join("results.json");
        if !path.is_file() {
            continue;
        }
        let result: SiteResult = serde_json::from_slice(&fs::read(&path)?)?;
        if result.schema_version != 2 {
            return Err(format!(
                "{} uses unsupported results schema {}",
                path.display(),
                result.schema_version
            )
            .into());
        }
        results.push(result);
    }
    results.sort_by(|left, right| left.site.cmp(&right.site));
    Ok(results)
}

fn html(results: &[SiteResult]) -> String {
    let body = results
        .iter()
        .map(|result| {
            let bit = &result.engines["bit"];
            let tri = &result.engines["tri"];
            let rec_tri = &result.engines["rec_tri"];
            format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td><span class=\"pill {}\">{}</span></td></tr>",
                escape(&result.site),
                bit.recomputed_nodes,
                tri.recomputed_nodes,
                rec_tri.recomputed_nodes,
                tri.input_skips,
                rec_tri.input_skips,
                if result.parity { "ok" } else { "diff" },
                if result.parity { "OK" } else { "DIFF" }
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>CSS Bitvector Results</title><style>{STYLE}</style></head><body><main><h1>CSS Bitvector Results</h1><p>One consolidated results.json per site.</p><table><thead><tr><th>Site</th><th>Bit misses</th><th>Tri misses</th><th>Rec tri misses</th><th>Tri skips</th><th>Rec tri skips</th><th>Parity</th></tr></thead><tbody>{body}</tbody></table></main></body></html>"
    )
}

fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn atomic_write(path: &std::path::Path, contents: &[u8]) -> Result<(), Box<dyn Error>> {
    let temporary = path.with_extension("xtask-tmp");
    fs::write(&temporary, contents)?;
    fs::rename(&temporary, path)?;
    Ok(())
}

const STYLE: &str = r#"
:root{font-family:system-ui,sans-serif;color:#172033;background:#f5f7fb}body{margin:0;padding:2rem}main{max-width:1100px;margin:auto}table{width:100%;border-collapse:collapse;background:white;box-shadow:0 8px 24px #17203318}th,td{padding:.7rem 1rem;border-bottom:1px solid #e4e8f0;text-align:left}th{background:#172033;color:white}.pill{display:inline-block;padding:.15rem .55rem;border-radius:999px;font-weight:650}.ok{background:#dcfce7;color:#166534}.diff{background:#fee2e2;color:#991b1b}@media(prefers-color-scheme:dark){:root{color:#e5e7eb;background:#111827}table{background:#1f2937}th{background:#030712}th,td{border-color:#374151}}
"#;

#[cfg(test)]
mod tests {
    use super::escape;

    #[test]
    fn escapes_site_names() {
        assert_eq!(escape("a<&\""), "a&lt;&amp;&quot;");
    }
}
