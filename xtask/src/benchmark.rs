use crate::context::value_after;
use css_bitvector_compiler::clean::{Engine, EngineKind, SiteInput, load_site};
use std::error::Error;

const SAMPLES: usize = 7;

pub fn execute(arguments: &[String]) -> Result<(), Box<dyn Error>> {
    let site = value_after(arguments, "--site").ok_or("benchmark requires --site <name>")?;
    let targets = arguments
        .iter()
        .rfind(|argument| !argument.starts_with('-') && argument.as_str() != site)
        .map(String::as_str)
        .unwrap_or("bit,tri,rec_tri");
    let kinds = targets
        .split(',')
        .map(parse_kind)
        .collect::<Result<Vec<_>, _>>()?;
    let (program, trace) = load_site(&SiteInput::named(site))?;
    let mut results = Vec::new();
    for (name, kind) in kinds {
        std::hint::black_box(Engine::new(kind, program.clone()).run(&trace)?);
        let mut samples = Vec::with_capacity(SAMPLES);
        for _ in 0..SAMPLES {
            let result = Engine::new(kind, program.clone()).run(&trace)?;
            samples.push(std::hint::black_box(result.stats.cycles));
        }
        samples.sort_unstable();
        results.push((name, samples[SAMPLES / 2]));
    }
    println!("rdtsc cycles (median of {SAMPLES}, lower is faster)");
    let Some((base_name, base_cycles)) = results.first().copied() else {
        return Ok(());
    };
    let base_cycles = base_cycles.max(1);
    for (name, cycles) in results {
        println!(
            "{name:<8} {cycles:>12}  x{:.2} vs {base_name}",
            cycles as f64 / base_cycles as f64
        );
    }
    Ok(())
}

fn parse_kind(name: &str) -> Result<(&str, EngineKind), Box<dyn Error>> {
    match name {
        "naive" => Ok((name, EngineKind::Naive)),
        "bit" => Ok((name, EngineKind::Bit)),
        "tri" => Ok((name, EngineKind::Tri)),
        "rec_tri" | "rec-tri" => Ok((name, EngineKind::RecursiveTri)),
        _ => Err(format!("unknown engine `{name}`; expected naive, bit, tri, or rec_tri").into()),
    }
}
