use crate::context::{Workspace, has_flag, value_after};
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

struct EngineSpec {
    binary: &'static str,
    log_name: &'static str,
    frame_stats: bool,
}

const ENGINES: &[EngineSpec] = &[
    EngineSpec {
        binary: "naive",
        log_name: "tmp.txt",
        frame_stats: false,
    },
    EngineSpec {
        binary: "bit",
        log_name: "bit_tmp.txt",
        frame_stats: false,
    },
    EngineSpec {
        binary: "tri",
        log_name: "tri_tmp.txt",
        frame_stats: true,
    },
    EngineSpec {
        binary: "rec_tri",
        log_name: "rec_tri_tmp.txt",
        frame_stats: true,
    },
];

pub fn execute(workspace: &Workspace, arguments: &[String]) -> Result<(), Box<dyn Error>> {
    let update = has_flag(arguments, "--update");
    let sites = if has_flag(arguments, "--all") {
        discover_sites(workspace.root())?
    } else {
        vec![
            value_after(arguments, "--site")
                .ok_or("run requires --site <name> or --all")?
                .to_string(),
        ]
    };
    for site in &sites {
        run_site(workspace, site, update)?;
    }
    println!(
        "checked {} site(s){}",
        sites.len(),
        if update { " and updated logs" } else { "" }
    );
    Ok(())
}

fn discover_sites(root: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    let mut sites = Vec::new();
    for entry in fs::read_dir(root.join("css-gen-op"))? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        if name == "reddit" {
            continue;
        }
        let directory = entry.path();
        if directory.join(format!("{name}.css")).is_file()
            && directory.join("command.json").is_file()
        {
            sites.push(name);
        }
    }
    sites.sort();
    Ok(sites)
}

fn run_site(workspace: &Workspace, site: &str, update: bool) -> Result<(), Box<dyn Error>> {
    println!("==> compare {site}");
    let mut outputs = Vec::new();
    for engine in ENGINES {
        let output = run_engine(workspace, site, engine)?;
        let matches = extract_matches(&output)?;
        if update {
            let path = workspace
                .root()
                .join("css-gen-op")
                .join(site)
                .join(engine.log_name);
            atomic_write(&path, output.as_bytes())?;
        }
        outputs.push((engine.binary, matches));
    }
    let baseline = &outputs[0].1;
    for (engine, matches) in &outputs[1..] {
        if matches != baseline {
            return Err(format!(
                "{site}: {engine} differs from naive\n{}",
                first_difference(baseline, matches)
            )
            .into());
        }
    }
    println!("    OK: naive == bit == tri == rec_tri");
    Ok(())
}

fn run_engine(
    workspace: &Workspace,
    site: &str,
    engine: &EngineSpec,
) -> Result<String, Box<dyn Error>> {
    let mut command = workspace.cargo();
    command
        .args([
            "run",
            "--quiet",
            "--release",
            "--package",
            "css-bitvector-compiler",
            "--bin",
            engine.binary,
        ])
        .env("WEBSITE_NAME", site)
        .env("CSS_BV_NO_DOT", "1");
    if engine.frame_stats {
        command.env("TRI_LOG_MATCH_DELTAS", "1");
    }
    capture_combined(workspace, engine.binary, &mut command)
}

fn capture_combined(
    workspace: &Workspace,
    label: &str,
    command: &mut Command,
) -> Result<String, Box<dyn Error>> {
    let output = workspace.capture(label, command)?;
    let mut combined = String::from_utf8(output.stdout)?;
    combined.push_str(&String::from_utf8(output.stderr)?);
    Ok(combined)
}

fn extract_matches(output: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut collecting = false;
    let mut saw_begin = false;
    let mut matches = Vec::new();
    for line in output.lines() {
        match line {
            "BEGIN" => {
                collecting = true;
                saw_begin = true;
            }
            "END" => collecting = false,
            _ if collecting => matches.push(line.to_string()),
            _ => {}
        }
    }
    if !saw_begin {
        return Err("engine output does not contain BEGIN/END markers".into());
    }
    matches.sort();
    Ok(matches)
}

fn first_difference(expected: &[String], actual: &[String]) -> String {
    let length = expected.len().max(actual.len());
    for index in 0..length {
        if expected.get(index) != actual.get(index) {
            return format!(
                "first difference at line {}: naive={:?}, actual={:?}",
                index + 1,
                expected.get(index),
                actual.get(index)
            );
        }
    }
    "outputs have different metadata".into()
}

fn atomic_write(path: &Path, contents: &[u8]) -> Result<(), Box<dyn Error>> {
    let temporary = temporary_path(path);
    fs::write(&temporary, contents)?;
    fs::rename(&temporary, path)?;
    Ok(())
}

fn temporary_path(path: &Path) -> PathBuf {
    let mut value = path.as_os_str().to_os_string();
    value.push(".xtask-tmp");
    PathBuf::from(value)
}

#[cfg(test)]
mod tests {
    use super::extract_matches;

    #[test]
    fn extracts_and_sorts_match_section() {
        let matches = extract_matches("noise\nBEGIN\nb\na\nEND\nmore").unwrap();
        assert_eq!(matches, ["a", "b"]);
    }
}
