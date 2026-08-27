use crate::context::{Workspace, value_after};
use std::{
    collections::BTreeMap,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

struct Row {
    site: String,
    bit: String,
    tri: String,
    rec_tri: String,
    bit_status: &'static str,
    tri_status: &'static str,
    rec_tri_status: &'static str,
}

pub fn execute(workspace: &Workspace, arguments: &[String]) -> Result<(), Box<dyn Error>> {
    let base = value_after(arguments, "--base")
        .map(PathBuf::from)
        .unwrap_or_else(|| workspace.root().join("css-gen-op"));
    let output_directory = value_after(arguments, "--output-dir")
        .map(PathBuf::from)
        .unwrap_or_else(|| workspace.root().to_path_buf());
    fs::create_dir_all(&output_directory)?;
    let rows = collect_rows(&base)?;
    atomic_write(
        &output_directory.join("misscnt.md"),
        markdown(&rows).as_bytes(),
    )?;
    atomic_write(
        &output_directory.join("misscnt.html"),
        html(&rows).as_bytes(),
    )?;
    println!(
        "generated misscnt.md and misscnt.html for {} site(s) in {}",
        rows.len(),
        output_directory.display()
    );
    Ok(())
}

fn collect_rows(base: &Path) -> Result<Vec<Row>, Box<dyn Error>> {
    let mut rows = Vec::new();
    for entry in fs::read_dir(base)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() || entry.file_name() == "reddit" {
            continue;
        }
        let directory = entry.path();
        let baseline = read_log(&directory.join("tmp.txt"));
        let bit = read_log(&directory.join("bit_tmp.txt"));
        let tri = read_log(&directory.join("tri_tmp.txt"));
        let rec_tri = read_log(&directory.join("rec_tri_tmp.txt"));
        if baseline.is_none() && bit.is_none() && tri.is_none() && rec_tri.is_none() {
            continue;
        }
        rows.push(Row {
            site: entry.file_name().to_string_lossy().into_owned(),
            bit: bit
                .as_ref()
                .and_then(|log| log.misses.clone())
                .unwrap_or("-".into()),
            tri: tri
                .as_ref()
                .and_then(|log| log.misses.clone())
                .unwrap_or("-".into()),
            rec_tri: rec_tri
                .as_ref()
                .and_then(|log| log.misses.clone())
                .unwrap_or("-".into()),
            bit_status: status(baseline.as_ref(), bit.as_ref()),
            tri_status: status(baseline.as_ref(), tri.as_ref()),
            rec_tri_status: status(baseline.as_ref(), rec_tri.as_ref()),
        });
    }
    rows.sort_by(|left, right| left.site.cmp(&right.site));
    Ok(rows)
}

struct Log {
    misses: Option<String>,
    matches: Vec<String>,
}

fn read_log(path: &Path) -> Option<Log> {
    let contents = fs::read_to_string(path).ok()?;
    Some(Log {
        misses: miss_count(&contents),
        matches: match_lines(&contents),
    })
}

fn miss_count(contents: &str) -> Option<String> {
    contents.lines().rev().find_map(|line| {
        let (_, suffix) = line.split_once("MISS_CNT } =")?;
        let value = suffix.trim();
        value
            .chars()
            .all(|character| character.is_ascii_digit())
            .then(|| value.into())
    })
}

fn match_lines(contents: &str) -> Vec<String> {
    let mut collecting = false;
    let mut result = Vec::new();
    for line in contents.lines() {
        match line {
            "BEGIN" => collecting = true,
            "END" => collecting = false,
            _ if collecting => result.push(line.to_string()),
            _ => {}
        }
    }
    result.sort();
    result
}

fn status(baseline: Option<&Log>, candidate: Option<&Log>) -> &'static str {
    match (baseline, candidate) {
        (Some(baseline), Some(candidate)) if baseline.matches == candidate.matches => "OK",
        (Some(_), Some(_)) => "DIFF",
        _ => "MISSING",
    }
}

fn markdown(rows: &[Row]) -> String {
    let mut output = String::from(
        "| Folder | MISS\\_CNT | TRI MISS\\_CNT | REC\\_TRI MISS\\_CNT | bit vs tmp | tri vs tmp | rec_tri vs tmp |\n\
         |---|---:|---:|---:|:---:|:---:|:---:|\n",
    );
    for row in rows {
        output.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} |\n",
            row.site,
            row.bit,
            row.tri,
            row.rec_tri,
            row.bit_status,
            row.tri_status,
            row.rec_tri_status
        ));
    }
    output
}

fn html(rows: &[Row]) -> String {
    let mut counts = BTreeMap::new();
    for status in rows
        .iter()
        .flat_map(|row| [row.bit_status, row.tri_status, row.rec_tri_status])
    {
        *counts.entry(status).or_insert(0usize) += 1;
    }
    let summary = counts
        .into_iter()
        .map(|(status, count)| {
            format!(
                "<span class=\"pill {}\">{}: {}</span>",
                class(status),
                status,
                count
            )
        })
        .collect::<Vec<_>>()
        .join(" ");
    let body = rows
        .iter()
        .map(|row| {
            format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td>{}{}{}</tr>",
                escape(&row.site),
                row.bit,
                row.tri,
                row.rec_tri,
                status_cell(row.bit_status),
                status_cell(row.tri_status),
                status_cell(row.rec_tri_status)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>Miss Count Report</title><style>{STYLE}</style></head><body><main><h1>Miss Count Report</h1><p>{summary}</p><table><thead><tr><th>Site</th><th>Bit</th><th>Tri</th><th>Rec tri</th><th>Bit parity</th><th>Tri parity</th><th>Rec tri parity</th></tr></thead><tbody>{body}</tbody></table></main></body></html>"
    )
}

fn status_cell(status: &str) -> String {
    format!(
        "<td><span class=\"pill {}\">{status}</span></td>",
        class(status)
    )
}

fn class(status: &str) -> &'static str {
    match status {
        "OK" => "ok",
        "DIFF" => "diff",
        _ => "missing",
    }
}

fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn atomic_write(path: &Path, contents: &[u8]) -> Result<(), Box<dyn Error>> {
    let temporary = path.with_extension("xtask-tmp");
    fs::write(&temporary, contents)?;
    fs::rename(&temporary, path)?;
    Ok(())
}

const STYLE: &str = r#"
:root{font-family:system-ui,sans-serif;color:#172033;background:#f5f7fb}body{margin:0;padding:2rem}main{max-width:1100px;margin:auto}table{width:100%;border-collapse:collapse;background:white;box-shadow:0 8px 24px #17203318}th,td{padding:.7rem 1rem;border-bottom:1px solid #e4e8f0;text-align:left}th{background:#172033;color:white}.pill{display:inline-block;padding:.15rem .55rem;border-radius:999px;font-weight:650}.ok{background:#dcfce7;color:#166534}.diff{background:#fee2e2;color:#991b1b}.missing{background:#fef3c7;color:#92400e}@media(prefers-color-scheme:dark){:root{color:#e5e7eb;background:#111827}table{background:#1f2937}th{background:#030712}th,td{border-color:#374151}}
"#;

#[cfg(test)]
mod tests {
    use super::{match_lines, miss_count};

    #[test]
    fn parses_engine_log() {
        let log = "noise\nBEGIN\nb\na\nEND\nunsafe { MISS_CNT } = 42\n";
        assert_eq!(miss_count(log).as_deref(), Some("42"));
        assert_eq!(match_lines(log), ["a", "b"]);
    }
}
