use super::{CompiledProgram, Engine, EngineKind, RunError, RunResult, SelectorReport, Trace};
use std::{error::Error, fs, path::PathBuf};

#[derive(Debug, Clone)]
pub struct SiteInput {
    pub name: String,
    pub css_path: PathBuf,
    pub trace_path: PathBuf,
}

impl SiteInput {
    pub fn named(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            css_path: PathBuf::from(format!("css-gen-op/{name}/{name}.css")),
            trace_path: PathBuf::from(format!("css-gen-op/{name}/command.json")),
            name,
        }
    }

    pub fn from_environment() -> Result<Self, RunError> {
        std::env::var("WEBSITE_NAME")
            .map(Self::named)
            .map_err(|_| RunError::new("WEBSITE_NAME is not set".into()))
    }
}

pub fn load_site(input: &SiteInput) -> Result<(CompiledProgram, Trace), RunError> {
    let css = fs::read_to_string(&input.css_path).map_err(|error| {
        RunError::new(format!("cannot read {}: {error}", input.css_path.display()))
    })?;
    let program = CompiledProgram::compile(&css)
        .map_err(|error| RunError::new(format!("{}: {error}", input.css_path.display())))?;
    let trace = Trace::parse(&input.trace_path)?;
    Ok((program, trace))
}

pub fn run_site(
    kind: EngineKind,
    input: &SiteInput,
) -> Result<(CompiledProgram, RunResult), RunError> {
    let (program, trace) = load_site(input)?;
    let result = Engine::new(kind, program.clone()).run(&trace)?;
    Ok((program, result))
}

pub fn run_site_with_frame_stats(
    kind: EngineKind,
    input: &SiteInput,
) -> Result<(CompiledProgram, RunResult), RunError> {
    let (program, trace) = load_site(input)?;
    let result = Engine::new(kind, program.clone())
        .with_frame_stats(true)
        .run(&trace)?;
    Ok((program, result))
}

pub fn report_selectors(label: &str, report: &SelectorReport) {
    if report.skipped_simple.is_empty() {
        println!("NOT_CONSIDERED[{label}] none");
    } else {
        println!(
            "NOT_CONSIDERED[{label}] {} selector(s)",
            report.skipped_simple.len()
        );
        for selector in &report.skipped_simple {
            println!("NOT_CONSIDERED[{label}] {selector}");
        }
    }
    if report.unsupported_pseudos.is_empty() {
        println!("PSEUDO_SKIPPED[{label}] none");
    } else {
        for (pseudo, selectors) in &report.unsupported_pseudos {
            println!(
                "PSEUDO_SKIPPED[{label}] {pseudo} -> {} selector(s)",
                selectors.len()
            );
            for selector in selectors.iter().take(5) {
                println!("PSEUDO_SKIPPED[{label}]    eg {selector}");
            }
        }
    }
    if report.unsupported.is_empty() {
        println!("UNSUPPORTED[{label}] none");
    } else {
        println!(
            "UNSUPPORTED[{label}] {} selector(s)",
            report.unsupported.len()
        );
        for selector in report.unsupported.iter().take(25) {
            println!(
                "UNSUPPORTED[{label}] {} ({})",
                selector.selector, selector.reason
            );
        }
    }
}

pub fn binary_main(kind: EngineKind) -> Result<(), Box<dyn Error>> {
    let input = SiteInput::from_environment()?;
    let log_frame_stats = env_flag("TRI_LOG_MATCH_DELTAS");
    let (program, result) = if log_frame_stats {
        run_site_with_frame_stats(kind, &input)?
    } else {
        run_site(kind, &input)?
    };
    report_selectors(kind.label(), &program.report);
    if log_frame_stats {
        for frame in &result.frames {
            println!(
                "[{}-match] frame_id={} command={} miss_delta={} node_match_changes={} total_misses={}",
                kind.label().replace('_', "-"),
                frame.frame_id,
                frame.command,
                frame.miss_delta,
                frame.node_match_changes,
                frame.total_misses
            );
        }
    }
    println!("BEGIN");
    for (selector, ids) in &result.matches {
        println!("{selector} -> {ids:?}");
    }
    println!("END");
    if kind != EngineKind::Naive {
        eprintln!("unsafe {{ MISS_CNT }} = {}", result.stats.recomputed_nodes);
    }
    if kind.uses_dependencies() {
        eprintln!(
            "unsafe {{ INPUT_CHANGE_COUNT }} = {}",
            result.stats.input_changes
        );
        eprintln!(
            "unsafe {{ INPUT_SKIP_COUNT }} = {}",
            result.stats.input_skips
        );
    }
    Ok(())
}

pub fn env_flag(name: &str) -> bool {
    std::env::var(name).is_ok_and(|value| {
        matches!(
            value.to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        )
    })
}
