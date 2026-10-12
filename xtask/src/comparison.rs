use crate::context::{Workspace, has_flag, value_after};
use css_bitvector_compiler::clean::{
    Engine, EngineKind, ParcelProgram, SiteInput, Trace, TraceCommand, load_site,
};
use serde::Serialize;
use std::{error::Error, fs, hint::black_box, process::Command};

const SITES: &[&str] = &[
    "a_to_b",
    "amazon",
    "bilibili",
    "bing",
    "bootstrap",
    "google",
    "netflix",
    "testcase",
    "tiktok",
    "whatsapp",
    "wikipedia",
    "yahoo",
    "youtube",
];
const KINDS: &[EngineKind] = &[
    EngineKind::Bit,
    EngineKind::Tri,
    EngineKind::RecursiveTri,
    EngineKind::Quad,
];

#[derive(Serialize)]
struct Measurement {
    engine: String,
    samples_ns: Vec<u128>,
    median_ns: u128,
    min_ns: u128,
    max_ns: u128,
}
#[derive(Serialize)]
struct Scenario {
    workload: &'static str,
    frames: usize,
    measurements: Vec<Measurement>,
}
#[derive(Serialize)]
struct SiteResult {
    site: String,
    selectors: usize,
    skipped_simple: usize,
    unsupported: usize,
    checked_frames: usize,
    scenarios: Vec<Scenario>,
}
#[derive(Serialize)]
struct Report {
    baseline: &'static str,
    baseline_policy: &'static str,
    command: Vec<String>,
    rustc: String,
    cpu: String,
    cpu_affinity: String,
    cpu4_governor: Option<String>,
    kernel: String,
    revision: String,
    working_tree_dirty: bool,
    captured_at_utc: String,
    source_hashes: String,
    timer: &'static str,
    release_build: bool,
    warmups: usize,
    samples: usize,
    results: Vec<SiteResult>,
}
fn output(command: &str, args: &[&str]) -> String {
    Command::new(command)
        .args(args)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_owned())
        .unwrap_or_default()
}

pub fn execute(workspace: &Workspace, arguments: &[String]) -> Result<(), Box<dyn Error>> {
    let samples: usize = value_after(arguments, "--samples").unwrap_or("7").parse()?;
    if samples == 0 || samples.is_multiple_of(2) {
        return Err("--samples must be a positive odd number".into());
    }
    let sites = if has_flag(arguments, "--all") {
        SITES.to_vec()
    } else {
        vec![value_after(arguments, "--site").ok_or("use --site <name> or --all")?]
    };
    let mut report = Report {
        baseline: "parcel_selectors 0.28.2",
        baseline_policy: "full rematch after each frame; per-frame nth cache; no browser style invalidation or ancestor bloom filter",
        command: std::env::args().collect(),
        rustc: output("rustc", &["-Vv"]),
        cpu: fs::read_to_string("/proc/cpuinfo")
            .unwrap_or_default()
            .lines()
            .find(|line| line.starts_with("model name"))
            .unwrap_or("unknown")
            .to_owned(),
        cpu_affinity: fs::read_to_string("/proc/self/status")
            .unwrap_or_default()
            .lines()
            .find(|line| line.starts_with("Cpus_allowed_list:"))
            .unwrap_or("unknown")
            .to_owned(),
        kernel: output("uname", &["-srmo"]),
        cpu4_governor: fs::read_to_string("/sys/devices/system/cpu/cpu4/cpufreq/scaling_governor")
            .ok()
            .map(|s| s.trim().to_owned()),
        revision: output("git", &["rev-parse", "HEAD"]),
        working_tree_dirty: !output("git", &["status", "--porcelain"]).is_empty(),
        captured_at_utc: output("date", &["-u", "+%Y-%m-%dT%H:%M:%SZ"]),
        source_hashes: output(
            "sha256sum",
            &[
                "Cargo.lock",
                "src/clean/engine.rs",
                "src/clean/engine/arena.rs",
                "src/clean/engine/names.rs",
                "src/clean/engine/trace_index.rs",
                "Cargo.toml",
                "src/clean/engine/comparison.rs",
                "src/clean/compiler.rs",
                "src/clean/logic.rs",
                "xtask/src/comparison.rs",
            ],
        ),
        timer: "std::time::Instant, nanoseconds",
        release_build: !cfg!(debug_assertions),
        warmups: 1,
        samples,
        results: Vec::new(),
    };
    for site in sites {
        let input = SiteInput::named(site);
        let css = fs::read_to_string(&input.css_path)?;
        let (program, trace) = load_site(&input)?;
        let parcel = ParcelProgram::parse(&css, &program)?;
        eprintln!(
            "{site}: checking {} selectors across {} frames",
            program.selectors.len(),
            trace.frames.len()
        );
        parcel.check(&program, &trace)?;
        let first = trace.frames.first().ok_or("empty trace")?;
        if !matches!(first.command, TraceCommand::Init { .. }) {
            return Err("trace must start with init".into());
        }
        let initial = Trace {
            frames: vec![first.clone()],
        };
        let mut scenarios = Vec::new();
        for (workload, workload_trace) in [("initial", &initial), ("trace", &trace)] {
            let mut times = vec![Vec::new(); KINDS.len() + 1];
            for round in 0..=samples {
                // Rotate order to distribute warmup, temperature, and frequency effects.
                for offset in 0..times.len() {
                    let index = (offset + round) % times.len();
                    let result = if index == KINDS.len() {
                        parcel.run(&program, workload_trace)?
                    } else {
                        Engine::new(KINDS[index], program.clone()).comparison_run(workload_trace)?
                    };
                    let elapsed = black_box(result).elapsed_ns;
                    if round > 0 {
                        times[index].push(elapsed);
                    }
                }
            }
            let measurements: Vec<_> = times
                .into_iter()
                .enumerate()
                .map(|(index, times)| {
                    let mut sorted = times.clone();
                    sorted.sort_unstable();
                    Measurement {
                        engine: if index == KINDS.len() {
                            "parcel_selectors".into()
                        } else {
                            KINDS[index].label().into()
                        },
                        median_ns: sorted[samples / 2],
                        min_ns: sorted[0],
                        max_ns: sorted[samples - 1],
                        samples_ns: times,
                    }
                })
                .collect();
            println!(
                "{site} {workload}: median milliseconds ({samples} samples, per-frame parity passed)"
            );
            let baseline = measurements.last().unwrap().median_ns as f64;
            for item in &measurements {
                println!(
                    "  {:<18} {:>12.3} ms  parcel/time = {:.3}x",
                    item.engine,
                    item.median_ns as f64 / 1_000_000.0,
                    baseline / item.median_ns.max(1) as f64
                );
            }
            scenarios.push(Scenario {
                workload,
                frames: workload_trace.frames.len(),
                measurements,
            });
        }
        report.results.push(SiteResult {
            site: site.to_owned(),
            selectors: program.selectors.len(),
            skipped_simple: program.report.skipped_simple.len(),
            unsupported: program.report.unsupported.len(),
            checked_frames: trace.frames.len(),
            scenarios,
        });
        // Save each completed site so a long run retains its verified measurements.
        if let Some(path) = value_after(arguments, "--output") {
            fs::write(
                workspace.root().join(path),
                serde_json::to_string_pretty(&report)? + "\n",
            )?;
        }
    }
    Ok(())
}
