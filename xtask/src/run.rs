use crate::context::{Workspace, has_flag, value_after};
use css_bitvector_compiler::clean::{Engine, EngineKind, SiteInput, load_site};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SiteResult {
    pub schema_version: u32,
    pub site: String,
    pub parity: bool,
    pub matches: BTreeMap<String, Vec<u64>>,
    pub engines: BTreeMap<String, EngineResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct EngineResult {
    pub recomputed_nodes: usize,
    pub input_changes: usize,
    pub input_skips: usize,
    pub visited_nodes: usize,
}

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
        let result = evaluate_site(site)?;
        println!(
            "==> {}: naive == bit == tri == rec_tri == quad",
            result.site
        );
        if update {
            write_result(workspace.root(), &result)?;
        }
    }
    println!(
        "checked {} site(s){}",
        sites.len(),
        if update {
            " and updated one results.json per site"
        } else {
            ""
        }
    );
    Ok(())
}

pub(crate) fn discover_sites(root: &Path) -> Result<Vec<String>, Box<dyn Error>> {
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

fn evaluate_site(site: &str) -> Result<SiteResult, Box<dyn Error>> {
    let (program, trace) = load_site(&SiteInput::named(site))?;
    let mut engines = BTreeMap::new();
    let mut baseline = None;
    for (name, kind) in [
        ("naive", EngineKind::Naive),
        ("bit", EngineKind::Bit),
        ("tri", EngineKind::Tri),
        ("rec_tri", EngineKind::RecursiveTri),
        ("quad", EngineKind::Quad),
    ] {
        let result = Engine::new(kind, program.clone()).run(&trace)?;
        if let Some(expected) = &baseline {
            if result.matches != *expected {
                return Err(format!("{site}: {name} differs from naive").into());
            }
        } else {
            baseline = Some(result.matches);
        }
        engines.insert(
            name.to_string(),
            EngineResult {
                recomputed_nodes: result.stats.recomputed_nodes,
                input_changes: result.stats.input_changes,
                input_skips: result.stats.input_skips,
                visited_nodes: result.stats.visited_nodes,
            },
        );
    }
    Ok(SiteResult {
        schema_version: 3,
        site: site.to_string(),
        parity: true,
        matches: baseline.unwrap_or_default(),
        engines,
    })
}

fn write_result(root: &Path, result: &SiteResult) -> Result<(), Box<dyn Error>> {
    let path = root
        .join("css-gen-op")
        .join(&result.site)
        .join("results.json");
    let contents = serde_json::to_vec_pretty(result)?;
    atomic_write(&path, &contents)
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
    use super::{EngineResult, SiteResult};
    use std::collections::BTreeMap;

    #[test]
    fn result_schema_round_trips() {
        let result = SiteResult {
            schema_version: 3,
            site: "testcase".into(),
            parity: true,
            matches: BTreeMap::new(),
            engines: BTreeMap::from([(
                "bit".into(),
                EngineResult {
                    recomputed_nodes: 1,
                    input_changes: 0,
                    input_skips: 0,
                    visited_nodes: 1,
                },
            )]),
        };
        let encoded = serde_json::to_string(&result).unwrap();
        let decoded: SiteResult = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded.site, "testcase");
        assert!(decoded.parity);
    }
}
