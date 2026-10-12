"""Build an isolated diagnostic copy. Keep the measured repository unchanged.

Usage: python3 dev/analyze-performance.py
The executable accepts --site, --samples, --output, or --profile-variant 0..4.
"""
from pathlib import Path
import argparse
import hashlib
import io
import json
import shutil
import subprocess
import tarfile

ROOT = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--storage', choices=['std', 'fx-internal', 'fx-all', 'vec-std', 'vec-fx'], default='std')
parser.add_argument('--revision', default='2e4f241', help='HashMap baseline revision')
args = parser.parse_args()
OUT = Path('/tmp/css-performance-analysis') / args.storage
SOURCE = OUT / 'baseline'
SOURCE.mkdir(parents=True, exist_ok=True)
revision = subprocess.check_output(['git','rev-parse',args.revision],cwd=ROOT,text=True).strip()
archive = subprocess.check_output(['git','archive',revision,'src','Cargo.toml','Cargo.lock',
    'dev/stylo-benchmark/Cargo.toml','dev/stylo-benchmark/Cargo.lock',
    'dev/stylo-benchmark/src/websites.rs'],cwd=ROOT)
with tarfile.open(fileobj=io.BytesIO(archive)) as bundle:
    bundle.extractall(SOURCE, filter='data')
CORE = OUT / 'core'
BENCH = OUT / 'bench'
REF = OUT / 'reference'
CORE.mkdir(parents=True, exist_ok=True)
BENCH.mkdir(parents=True, exist_ok=True)
REF.mkdir(parents=True, exist_ok=True)
shutil.copytree(SOURCE / 'src', REF / 'src', dirs_exist_ok=True)
reference_manifest = (SOURCE / 'Cargo.toml').read_text().replace('members = ["xtask"]', 'members = []')
(REF / 'Cargo.toml').write_text(reference_manifest.replace('name = "css-bitvector-compiler"', 'name = "css-reference-core"'))
shutil.copytree(SOURCE / 'src', CORE / 'src', dirs_exist_ok=True)
shutil.copy2(SOURCE / 'Cargo.lock', CORE / 'Cargo.lock')
s = (SOURCE / 'Cargo.toml').read_text().replace('members = ["xtask"]', 'members = []')
if args.storage not in ('std', 'vec-std'):
    s = s.replace('[dependencies]', '[dependencies]\nrustc-hash = "=2.1.3"')
(CORE / 'Cargo.toml').write_text(s)


def replace_once(s, old, new):
    assert s.count(old) == 1, old
    return s.replace(old, new)


p = CORE / 'src/clean/engine.rs'
s = p.read_text()
if args.storage not in ('std', 'vec-std'):
    s = s.replace('use std::collections::{BTreeMap, HashMap, HashSet};',
                  'use std::collections::{BTreeMap, HashMap, HashSet};\nuse rustc_hash::{FxHashMap, FxHashSet};')
    for old, new in [('nodes: HashMap<NodeId, DomNode>', 'nodes: FxHashMap<NodeId, DomNode>'),
                     ('trace_nodes: HashMap<u64, NodeId>', 'trace_nodes: FxHashMap<u64, NodeId>'),
                     ('HashSet<DynamicPseudo>', 'FxHashSet<DynamicPseudo>'),
                     ('pseudos: HashSet::new()', 'pseudos: FxHashSet::default()'),
                     ('let mut ids = HashSet::new()', 'let mut ids = FxHashSet::default()')]:
        s = s.replace(old, new)
if args.storage in ('fx-all', 'vec-fx'):
    s = s.replace('use std::collections::{BTreeMap, HashMap, HashSet};', 'use std::collections::{BTreeMap, HashMap};')
    s = s.replace('classes: HashSet<String>', 'classes: FxHashSet<String>')
    s = replace_once(s, '    attributes: HashMap<String, String>,', '    attributes: FxHashMap<String, String>,')
    s = replace_once(s, '        attributes,', '        attributes: attributes.into_iter().collect(),')
if args.storage in ('vec-std', 'vec-fx'):
    node_map = 'FxHashMap' if args.storage == 'vec-fx' else 'HashMap'
    s = replace_once(s, f'nodes: {node_map}<NodeId, DomNode>', 'nodes: NodeArena')
    s += (ROOT / 'dev/performance-arena.rs').read_text()
    cp = CORE / 'src/clean/engine/comparison.rs'
    cp.write_text(cp.read_text().replace('engine.dom.nodes.keys().copied()', 'engine.dom.nodes.keys()'))
s = replace_once(s, 'pub struct Engine {', 'pub struct Engine {\n    profile_flags: u8,')
s = replace_once(s, 'impl Engine {', '''impl Engine {
    pub fn profile_variant(&mut self, flags: u8) { self.profile_flags = flags; }
    pub fn profile_stats(&self) -> (usize, usize, usize, usize) {
        (self.stats.visited_nodes, self.stats.recomputed_nodes,
         self.stats.input_changes, self.stats.input_skips)
    }
''')
s = replace_once(s, '            stats: RunStats::default(),', '            stats: RunStats::default(),\n            profile_flags: 0,')
s = replace_once(s, '        let position = self.dom.position(id);', '''        if self.profile_flags & 1 != 0 && compound.nth.is_empty() { return true; }
        let position = self.dom.position(id);''')
s = replace_once(s, '        self.dom.refresh_pseudos();', '''        let needs_pseudos = match &frame.command {
            TraceCommand::Init {..} | TraceCommand::Add {..} |
            TraceCommand::Remove {..} | TraceCommand::Replace {..} => true,
            TraceCommand::Recalculate => false,
            TraceCommand::InsertValue {key,..} | TraceCommand::ReplaceValue {key,..} |
            TraceCommand::DeleteValue {key,..} => matches!(key.to_ascii_lowercase().as_str(),
                "is_hover_root" | "is_hovered_root" | "is_focus_root"),
        };
        if self.profile_flags & 2 == 0 || needs_pseudos { self.dom.refresh_pseudos(); }''')
p.write_text(s)

s = (SOURCE / 'dev/stylo-benchmark/Cargo.toml').read_text()
s = s.replace('path = "../.."', 'path = "../core"')
s = s.replace('[dependencies]', '[dependencies]\nreference-core = { package = "css-reference-core", path = "../reference" }')
s = s.replace('name = "css-stylo-websites"', 'name = "css-website-profile"')
s = s.replace('path = "src/websites.rs"', 'path = "profile.rs"')
(BENCH / 'Cargo.toml').write_text(s)
shutil.copy2(SOURCE / 'dev/stylo-benchmark/Cargo.lock', BENCH / 'Cargo.lock')
s = (SOURCE / 'dev/stylo-benchmark/src/websites.rs').read_text()
reference = s[:s.index('trait Backend {')].replace('use css_bitvector_compiler::', 'use reference_core::')
reference += '''
pub fn load(site: &str) -> Result<(Engine, Vec<TraceFrame>)> {
    let (original, trace) = load_site(&SiteInput::named(site))?;
    let projection = project(&original)?;
    let mut frames = trace.frames;
    for frame in &mut frames { normalize_frame(frame); }
    Ok((Engine::new(EngineKind::Bit, projection.program), frames))
}
'''
(BENCH / 'reference.rs').write_text('#![allow(unused)]\n'+reference)
s = replace_once(s, 'struct Project {', 'struct Project {\n    fast_output: bool,')
s = replace_once(s, '            engine: Engine::new(kind, projection.program.clone()),', '            fast_output: false,\n            engine: Engine::new(kind, projection.program.clone()),')
s = replace_once(s, '''                    self.priority
                        .iter()
                        .find(|&&i| matches[i])
                        .map_or([0; 4], |&i| colour(i)),''', '''                    if self.fast_output && !matches.contains(&true) { [0; 4] } else {
                        self.priority.iter().find(|&&i| matches[i]).map_or([0; 4], |&i| colour(i))
                    },''')
s = replace_once(s, 'fn main() -> Result<()> {', '#[allow(dead_code)]\nfn original_main() -> Result<()> {')
s += r'''
#[allow(unused)]
mod reference;
// Diagnostic variants share one binary. Flags are per engine instance.
const VARIANTS: [&str; 5] = ["baseline", "lazy_position", "event_pseudos", "both", "both_fast_output"];
fn candidate(projection: &Projection, kind: EngineKind, variant: usize) -> Project {
    let mut p = Project::new(kind, projection);
    p.engine.profile_variant([0,1,2,3,3][variant]);
    p.fast_output = variant == 4;
    p
}
#[inline(never)]
fn measured_update(p: &mut Project, frame: &TraceFrame) -> Result<()> { p.update(frame) }
#[inline(never)]
fn measured_output(p: &Project) -> Colours { p.colours() }
fn main() -> Result<()> {
    let site = option("--site").ok_or("missing --site")?;
    let (original,trace) = load_site(&SiteInput::named(&site))?;
    let projection = project(&original)?;
    let mut frames = trace.frames;
    for f in &mut frames { normalize_frame(f); }
    if let Some(variant) = option("--profile-variant") {
        let variant: usize = variant.parse()?;
        let mut p = candidate(&projection,EngineKind::Bit,variant);
        p.update(&frames[0])?;
        let mut update_ns = 0;
        let mut output_ns = 0;
        for f in &frames[1..] {
            let now = Instant::now(); measured_update(&mut p,f)?; update_ns += now.elapsed().as_nanos();
            let now = Instant::now(); let output = black_box(measured_output(&p)); output_ns += now.elapsed().as_nanos();
            drop(output);
        }
        println!("{}",json!({"site":site,"variant":VARIANTS[variant],"update_ns":update_ns,"output_ns":output_ns,"stats":p.engine.profile_stats()}));
        return Ok(());
    }
    // Validate every match bit, not only the winning colour. Check all four engines.
    let (mut baseline, reference_frames) = reference::load(&site)?;
    let mut candidates: Vec<_> = [0,4].into_iter().flat_map(|variant|
        KINDS.iter().map(move |&kind| (variant,kind)))
        .map(|(variant,kind)| candidate(&projection,kind,variant)).collect();
    let mut expected = Vec::new();
    for (frame, reference_frame) in frames.iter().zip(&reference_frames) {
        baseline.comparison_update(reference_frame)?;
        let wanted: BTreeMap<_,_> = baseline.comparison_node_matches().collect();
        let colours: Colours = wanted.iter().map(|(&id,bits)| (id,
            projection.priority.iter().find(|&&i| bits[i]).map_or([0;4], |&i| colour(i)))).collect();
        for p in &mut candidates {
            p.update(frame)?;
            let actual: BTreeMap<_,_> = p.engine.comparison_node_matches().collect();
            assert_eq!(actual,wanted,"match bits differ at frame {}",frame.frame_id);
            check(&p.colours(), &colours, frame)?;
        }
        expected.push(colours);
    }
    let stats: Vec<_> = candidates.iter().map(|p| p.engine.profile_stats()).collect();
    eprintln!("{site}: all four candidate engines passed full match-bit and RGBA checks");
    drop(baseline); drop(candidates);
    if env::args().any(|s| s == "--validate-only") {
        println!("{}",json!({"site":site,"frames":frames.len(),"stats":stats,"full_match_bits_validated":true}));
        return Ok(());
    }
    let samples: usize = option("--samples").unwrap_or("3".into()).parse()?;
    assert!(samples > 0 && samples % 2 == 1);
    let selected: Vec<usize> = option("--variants").unwrap_or("0,4".into()).split(',').map(|s|s.parse().unwrap()).collect();
    assert!(!selected.is_empty() && selected.iter().all(|&i| i < 5));
    let mut measurements = vec![Vec::new();5];
    for round in 0..=samples {
        for index in (0..selected.len()).map(|offset| selected[(round+offset)%selected.len()]) {
            let sample = run(&mut candidate(&projection,EngineKind::Bit,index),&frames,&expected)?;
            eprintln!("{site} {round}/{samples} {}: update {:.3} ms, output {:.3} ms",VARIANTS[index],sample.update_ns as f64/1e6,sample.output_ns as f64/1e6);
            if round > 0 { measurements[index].push(sample); }
        }
    }
    let mut results = Vec::new();
    for (i,values) in measurements.into_iter().enumerate() {
        if values.is_empty() { continue; }
        let summary = |field: fn(&Sample)->u128| {
            let mut sorted: Vec<_> = values.iter().map(field).collect(); sorted.sort_unstable();
            json!({"median_ns":sorted[samples/2],"min_ns":sorted[0],"max_ns":sorted[samples-1]})
        };
        results.push(json!({"variant":VARIANTS[i],"update":summary(|s|s.update_ns),"output":summary(|s|s.output_ns),"total":summary(|s|s.total_ns),"samples":values}));
    }
    let report = json!({"site":site,"frames":frames.len(),"samples":samples,"warmups":1,"engine":"bit","full_match_bits_validated_kinds":["bit","tri","rec_tri","quad"],"candidate_stats":stats,"results":results});
    fs::write(option("--output").ok_or("missing --output")?,serde_json::to_string_pretty(&report)?+"\n")?;
    Ok(())
}
'''
(BENCH / 'profile.rs').write_text(s)
source_files = [*sorted((SOURCE/'src').rglob('*.rs')), SOURCE/'Cargo.toml', SOURCE/'Cargo.lock', SOURCE/'dev/stylo-benchmark/Cargo.toml', SOURCE/'dev/stylo-benchmark/Cargo.lock', SOURCE/'dev/stylo-benchmark/src/websites.rs', ROOT/'dev/performance-arena.rs', Path(__file__).resolve()]
subprocess.run(['cargo','build','--release','--offline','--manifest-path',str(BENCH/'Cargo.toml'),'--bin','css-website-profile','--target-dir',str(ROOT/'dev/stylo-benchmark/target')],check=True,cwd=ROOT)
shutil.copy2(ROOT/'dev/stylo-benchmark/target/release/css-website-profile', OUT/'profile')
generated = [*sorted((CORE/'src').rglob('*.rs')), CORE/'Cargo.toml', BENCH/'Cargo.toml', BENCH/'Cargo.lock', BENCH/'profile.rs', BENCH/'reference.rs']
manifest = {'storage':args.storage, 'baseline_revision':revision, 'sources': {str(p.relative_to(SOURCE if p.is_relative_to(SOURCE) else ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in source_files}, 'generated': {str(p.relative_to(OUT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in generated}, 'binary_sha256':hashlib.sha256((OUT/'profile').read_bytes()).hexdigest()}
(OUT/'source-manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
print(OUT/'profile')
