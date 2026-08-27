use css_bitvector_compiler::rdtsc;
use std::env;

mod bit {
    include!("bit.rs");
    pub fn bench_match_only() -> u64 {
        let mut dom = DOM::new();
        let website_name = super::website_name();
        let parsed = parse_css_with_pseudo(
            &std::fs::read_to_string(format!("css-gen-op/{0}/{0}.css", website_name,)).unwrap(),
        );
        let ParsedSelectors {
            mut selectors,
            mut pseudo_selectors,
            ..
        } = parsed;
        selectors.extend(drain_supported_pseudo_selectors(&mut pseudo_selectors));
        selectors.sort();
        selectors.dedup();
        let (selectors, _) = partition_simple_selectors(selectors);
        let mut s = unsafe { STATE };
        let nfa = generate_nfa(&selectors, &mut dom.selector_manager, &mut s);
        unsafe {
            STATE = s;
        }
        let frames = parse_trace();
        let start = super::rdtsc();
        for frame in &frames {
            apply_frame(&mut dom, frame, &nfa);
        }
        std::hint::black_box(&dom);
        let end = super::rdtsc();
        end.wrapping_sub(start)
    }
}

mod tri {
    include!("tri.rs");
    pub fn bench_match_only() -> u64 {
        let mut dom = DOM::new();
        let website_name = super::website_name();
        let parsed = parse_css_with_pseudo(
            &std::fs::read_to_string(format!("css-gen-op/{0}/{0}.css", website_name)).unwrap(),
        );
        let (selectors, _) = partition_simple_selectors(parsed.selectors);
        let mut s = unsafe { STATE };
        let nfa = generate_nfa(&selectors, &mut dom.selector_manager, &mut s);
        unsafe {
            STATE = s;
        }
        let frames = parse_trace();
        let start = super::rdtsc();
        for frame in &frames {
            apply_frame(&mut dom, frame, &nfa);
        }
        std::hint::black_box(&dom);
        let end = super::rdtsc();
        end.wrapping_sub(start)
    }
}

mod rec_tri {
    include!("rec_tri.rs");
    pub fn bench_match_only() -> u64 {
        let mut dom = DOM::new();
        let website_name = super::website_name();
        let parsed = parse_css_with_pseudo(
            &std::fs::read_to_string(format!("css-gen-op/{0}/{0}.css", website_name)).unwrap(),
        );
        let (selectors, _) = partition_simple_selectors(parsed.selectors);
        let mut s = unsafe { STATE };
        let nfa = generate_nfa(&selectors, &mut dom.selector_manager, &mut s);
        unsafe {
            STATE = s;
        }
        let frames = parse_trace();
        let start = super::rdtsc();
        for frame in &frames {
            apply_frame(&mut dom, frame, &nfa);
        }
        std::hint::black_box(&dom);
        let end = super::rdtsc();
        end.wrapping_sub(start)
    }
}

mod quad {
    include!("quad.rs");
    pub fn bench_match_only() -> u64 {
        let mut dom = DOM::new();
        let website_name = super::website_name();
        let parsed = parse_css_with_pseudo(
            &std::fs::read_to_string(format!("css-gen-op/{0}/{0}.css", website_name)).unwrap(),
        );
        let (selectors, _) = partition_simple_selectors(parsed.selectors);
        let mut s = unsafe { STATE };
        let nfa = generate_nfa(&selectors, &mut dom.selector_manager, &mut s);
        unsafe {
            STATE = s;
        }
        let frames = parse_trace();
        let start = super::rdtsc();
        for frame in &frames {
            apply_frame(&mut dom, frame, &nfa);
        }
        std::hint::black_box(&dom);
        let end = super::rdtsc();
        end.wrapping_sub(start)
    }
}

mod naive {
    include!("naive.rs");
    pub fn bench_match_only() -> u64 {
        let mut dom = SimpleDom::default();
        let website_name = super::website_name();
        let (rules, _, _) = parse_css_rules(
            &std::fs::read_to_string(format!("css-gen-op/{0}/{0}.css", website_name,)).unwrap(),
        );
        let (css, _) = partition_rules(rules);
        let trace = parse_trace();
        let start = super::rdtsc();
        for frame in &trace {
            apply_frame_basic(&mut dom, frame);
        }
        std::hint::black_box(&dom);
        let end = super::rdtsc();
        let _ = css;
        end.wrapping_sub(start)
    }
}

fn parse_args() -> (Option<String>, Vec<String>) {
    let mut args = env::args().skip(1).peekable();
    let mut site = None;
    let mut targets = Vec::new();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--site" | "--website" | "-w" => {
                if let Some(value) = args.next() {
                    site = Some(value);
                } else {
                    eprintln!("--site requires a value");
                    std::process::exit(2);
                }
            }
            _ => targets.push(arg),
        }
    }

    if targets.is_empty() {
        return (
            site,
            vec!["bit", "tri", "rec_tri", "quad"]
                .into_iter()
                .map(String::from)
                .collect(),
        );
    }

    let mut expanded = Vec::new();
    for arg in targets {
        for name in arg.split(',') {
            let trimmed = name.trim();
            if !trimmed.is_empty() {
                expanded.push(trimmed.to_string());
            }
        }
    }

    (site, expanded)
}

fn website_name() -> String {
    env::var("WEBSITE_NAME").unwrap_or_else(|_| {
        eprintln!("WEBSITE_NAME not set. Example:");
        eprintln!("  WEBSITE_NAME=xxx cargo run -r --bin benchmark");
        eprintln!("  cargo run -r --bin benchmark -- --site xxx");
        std::process::exit(2);
    })
}

fn run_target(name: &str) -> Option<u64> {
    match name {
        "bit" => Some(bit::bench_match_only()),
        "tri" => Some(tri::bench_match_only()),
        "rec_tri" => Some(rec_tri::bench_match_only()),
        "quad" => Some(quad::bench_match_only()),
        "naive" => Some(naive::bench_match_only()),
        _ => None,
    }
}

fn main() {
    let (site_arg, targets) = parse_args();
    if let Some(site) = site_arg {
        unsafe {
            env::set_var("WEBSITE_NAME", site);
        }
    }
    let _ = website_name();
    if targets.is_empty() {
        eprintln!("no targets selected");
        return;
    }

    let mut results = Vec::new();
    for name in targets {
        match run_target(&name) {
            Some(cycles) => results.push((name, cycles)),
            None => {
                eprintln!("unknown target: {name}");
                eprintln!("valid targets: bit, tri, rec_tri, quad, naive");
                return;
            }
        }
    }

    println!("\nrdtsc cycles (lower is faster)");
    let base = results.clone()[0].1.max(1);
    let base_name = &results.clone()[0].0;
    for (name, cycles) in results {
        let ratio = cycles as f64 / base as f64;
        println!("{name:<8} {cycles:>12}  x{ratio:.2} vs {base_name}");
    }
}
