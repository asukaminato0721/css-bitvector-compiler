use css_bitvector_compiler::clean::{Engine, EngineKind, SiteInput, load_site};
use std::{env, error::Error};

const SAMPLES: usize = 7;

fn main() {
    if let Err(error) = run() {
        eprintln!("benchmark failed: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let (site, targets) = parse_args()?;
    let input = SiteInput::named(site);
    let (program, trace) = load_site(&input)?;
    let mut results = Vec::new();

    for target in targets {
        let kind = parse_kind(&target)?;
        if kind == EngineKind::ExperimentalQuad {
            eprintln!("warning: quad is experimental and excluded from default results");
        }
        // Untimed warmup.
        std::hint::black_box(Engine::new(kind, program.clone()).run(&trace)?);
        let mut samples = Vec::with_capacity(SAMPLES);
        for _ in 0..SAMPLES {
            let result = Engine::new(kind, program.clone()).run(&trace)?;
            samples.push(std::hint::black_box(result.stats.cycles));
        }
        samples.sort_unstable();
        results.push((target, samples[SAMPLES / 2]));
    }

    println!("\nrdtsc cycles (median of {SAMPLES}, lower is faster)");
    let Some((base_name, base_cycles)) = results.first().cloned() else {
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

fn parse_args() -> Result<(String, Vec<String>), Box<dyn Error>> {
    let mut args = env::args().skip(1);
    let mut site = None;
    let mut targets = Vec::new();
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--site" | "--website" | "-w" => {
                site = Some(args.next().ok_or("--site requires a value")?);
            }
            _ => targets.extend(
                argument
                    .split(',')
                    .filter(|name| !name.trim().is_empty())
                    .map(|name| name.trim().to_string()),
            ),
        }
    }
    let site = site
        .or_else(|| env::var("WEBSITE_NAME").ok())
        .ok_or("WEBSITE_NAME is not set; pass --site <name>")?;
    if targets.is_empty() {
        targets = ["bit", "tri", "rec_tri"]
            .into_iter()
            .map(str::to_string)
            .collect();
    }
    Ok((site, targets))
}

fn parse_kind(name: &str) -> Result<EngineKind, Box<dyn Error>> {
    match name {
        "naive" => Ok(EngineKind::Naive),
        "bit" => Ok(EngineKind::Bit),
        "tri" => Ok(EngineKind::Tri),
        "rec_tri" | "rec-tri" => Ok(EngineKind::RecursiveTri),
        "quad" => Ok(EngineKind::ExperimentalQuad),
        _ => Err(
            format!("unknown target `{name}`; expected naive, bit, tri, rec_tri, or quad").into(),
        ),
    }
}
