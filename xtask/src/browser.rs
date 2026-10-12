//! Native measurements and exact per-frame oracles for the browser experiment.
use crate::context::{Workspace, value_after};
use css_bitvector_compiler::clean::{CompiledProgram, Engine, EngineKind, Trace, TraceFrame};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{error::Error, fs, hint::black_box, process::Command};

#[derive(Deserialize)]
struct Fixture {
    name: String,
    nodes: usize,
    css: String,
    frames: Vec<Value>,
}

pub fn execute(workspace: &Workspace, arguments: &[String]) -> Result<(), Box<dyn Error>> {
    let input = value_after(arguments, "--input").ok_or("missing --input")?;
    let output = value_after(arguments, "--output").ok_or("missing --output")?;
    let oracle = value_after(arguments, "--oracle").ok_or("missing --oracle")?;
    let samples: usize = value_after(arguments, "--samples").unwrap_or("7").parse()?;
    if samples == 0 || samples.is_multiple_of(2) {
        return Err("--samples must be a positive odd number".into());
    }
    let fixtures: Vec<Fixture> = serde_json::from_str(&fs::read_to_string(input)?)?;
    let kinds = [
        EngineKind::Bit,
        EngineKind::Tri,
        EngineKind::RecursiveTri,
        EngineKind::Quad,
    ];
    let mut results = Vec::new();
    let mut oracles = Vec::new();
    for fixture in fixtures {
        let program = CompiledProgram::compile(&fixture.css)?;
        if program.selectors.is_empty()
            || !program.report.unsupported.is_empty()
            || !program.report.skipped_simple.is_empty()
        {
            return Err(format!("{}: fixture selectors were excluded", fixture.name).into());
        }
        let trace = Trace {
            frames: fixture
                .frames
                .iter()
                .enumerate()
                .map(|(frame_id, command)| {
                    Ok(TraceFrame {
                        frame_id,
                        command: serde_json::from_value(command.clone())?,
                    })
                })
                .collect::<Result<_, serde_json::Error>>()?,
        };
        let first = trace.frames.first().ok_or("empty fixture")?;
        let initial = Trace {
            frames: vec![first.clone()],
        };
        let mut engines: Vec<_> = std::iter::once(EngineKind::Naive)
            .chain(kinds)
            .map(|kind| Engine::new(kind, program.clone()))
            .collect();
        let mut expected = Vec::new();
        for frame in &trace.frames {
            let reference = engines[0].comparison_step(frame)?;
            for engine in &mut engines[1..] {
                if engine.comparison_step(frame)? != reference {
                    return Err(
                        format!("{}: mismatch at frame {}", fixture.name, frame.frame_id).into(),
                    );
                }
            }
            expected.push(reference);
        }
        let mut scenarios = Vec::new();
        for (workload, selected) in [("initial", &initial), ("trace", &trace)] {
            let mut times = vec![Vec::new(); kinds.len()];
            for round in 0..=samples {
                for offset in 0..kinds.len() {
                    let index = (round + offset) % kinds.len();
                    let result =
                        Engine::new(kinds[index], program.clone()).comparison_run(selected)?;
                    let elapsed = black_box(result).elapsed_ns;
                    if round > 0 {
                        times[index].push(elapsed);
                    }
                }
                eprintln!(
                    "{} {workload}: native round {round}/{samples}",
                    fixture.name
                );
            }
            let measurements: Vec<_> = times.into_iter().enumerate().map(|(index, values)| {
                let mut sorted = values.clone();
                sorted.sort_unstable();
                json!({"engine": kinds[index].label(), "samples_ns": values,
                    "median_ns": sorted[samples / 2], "min_ns": sorted[0], "max_ns": sorted[samples - 1]})
            }).collect();
            scenarios.push(json!({"workload": workload, "measurements": measurements}));
        }
        results.push(json!({"name": fixture.name, "nodes": fixture.nodes,
            "selectors": program.selectors.len(), "frames": trace.frames.len(), "scenarios": scenarios}));
        oracles.push(
            json!({"name": fixture.name, "nodes": fixture.nodes, "css": fixture.css,
            "selectors": program.selectors.iter().map(|s| &s.text).collect::<Vec<_>>(),
            "frames": fixture.frames, "expected": expected}),
        );
    }
    let hashes = Command::new("sha256sum")
        .args([
            "src/clean/engine.rs",
            "src/clean/engine/comparison.rs",
            "src/clean/compiler.rs",
            "src/clean/logic.rs",
            "xtask/src/browser.rs",
            "dev/browser-benchmark.mjs",
            input,
        ])
        .output()?;
    let report = json!({"samples": samples, "warmups": 1, "release_build": !cfg!(debug_assertions),
        "cpu_affinity": fs::read_to_string("/proc/self/status")?.lines()
            .find(|line| line.starts_with("Cpus_allowed_list:")).unwrap_or("unknown"),
        "source_hashes": String::from_utf8_lossy(&hashes.stdout), "results": results});
    fs::write(
        workspace.root().join(output),
        serde_json::to_string_pretty(&report)? + "\n",
    )?;
    fs::write(oracle, serde_json::to_vec(&oracles)?)?;
    Ok(())
}
