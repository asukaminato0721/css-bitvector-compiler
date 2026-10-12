//! Native, sequential incremental updates with a common per-node RGBA output.
use blitz_dom::{Attribute, BaseDocument, DocumentConfig, DocumentMutator, StyleThreading};
use blitz_traits::node_id::NodeId;
use css_bitvector_compiler::{
    Node,
    clean::{CompiledProgram, Engine, EngineKind, TraceCommand, TraceFrame},
};
use markup5ever::{QualName, ns};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    collections::BTreeMap, env, error::Error, fs, hint::black_box, process::Command, time::Instant,
};
use style::{color::AbsoluteColor, stylesheets::Origin};

type Result<T> = std::result::Result<T, Box<dyn Error>>;
type Colours = Vec<(u64, [u8; 4])>;

#[derive(Deserialize)]
struct Fixture {
    name: String,
    nodes: usize,
    css: String,
    frames: Vec<TraceCommand>,
}

trait Backend {
    fn update(&mut self, frame: &TraceFrame) -> Result<()>;
    fn colours(&self) -> Colours;
}

struct Project {
    engine: Engine,
    palette: Vec<[u8; 4]>,
}
impl Backend for Project {
    fn update(&mut self, frame: &TraceFrame) -> Result<()> {
        self.engine.comparison_update(frame)?;
        Ok(())
    }
    fn colours(&self) -> Colours {
        let mut result: Colours = self
            .engine
            .comparison_node_matches()
            .map(|(id, matches)| {
                let colour = matches
                    .iter()
                    .position(|&m| m)
                    .map_or([0; 4], |i| self.palette[i]);
                (id, colour)
            })
            .collect();
        result.sort_unstable_by_key(|&(id, _)| id);
        result
    }
}

struct Stylo {
    doc: BaseDocument,
    root: Option<NodeId>,
    ids: BTreeMap<u64, NodeId>,
}
impl Stylo {
    fn new(css: &str) -> Self {
        let mut doc = BaseDocument::new(DocumentConfig {
            ua_stylesheets: Some(vec![]),
            style_threading: StyleThreading::Sequential,
            ..Default::default()
        });
        // Parse the author sheet before the measured updates.
        let sheet = doc.make_stylesheet(css, Origin::Author);
        // The sheet owner is detached and is not part of the measured DOM.
        let owner = doc
            .mutate()
            .create_element(QualName::new(None, ns!(html), "style".into()), vec![]);
        doc.add_stylesheet_for_node(sheet, owner);
        Self {
            doc,
            root: None,
            ids: BTreeMap::new(),
        }
    }
    fn at(&self, path: &[usize]) -> NodeId {
        let mut id = self.root.expect("missing root");
        for &index in path {
            id = self.doc.get_node(id).unwrap().children[index];
        }
        id
    }
    fn build(
        mutation: &mut DocumentMutator<'_>,
        node: &Node,
        ids: &mut BTreeMap<u64, NodeId>,
    ) -> NodeId {
        let attrs = node
            .attributes
            .iter()
            .map(|(key, value)| Attribute {
                name: QualName::new(None, ns!(), key.as_str().into()),
                value: value
                    .as_str()
                    .expect("fixture attributes must be strings")
                    .into(),
            })
            .collect();
        let id = mutation.create_element(
            QualName::new(None, ns!(html), node.tag_name.as_str().into()),
            attrs,
        );
        assert!(ids.insert(node.id, id).is_none(), "duplicate node ID");
        for child in &node.children {
            let child = Self::build(mutation, child, ids);
            mutation.append_children(id, &[child]);
        }
        id
    }
}
impl Backend for Stylo {
    fn update(&mut self, frame: &TraceFrame) -> Result<()> {
        match &frame.command {
            TraceCommand::Init { node } => {
                assert!(self.root.is_none());
                let parent = self.doc.root_node().id;
                let mut mutation = self.doc.mutate();
                let root = Self::build(&mut mutation, node, &mut self.ids);
                mutation.append_children(parent, &[root]);
                self.root = Some(root);
            }
            TraceCommand::ReplaceValue {
                path,
                key,
                value,
                value_type,
                ..
            } => {
                assert_eq!(value_type.as_deref(), Some("attributes"));
                let id = self.at(path);
                self.doc.mutate().set_attribute(
                    id,
                    QualName::new(None, ns!(), key.as_str().into()),
                    value
                        .as_ref()
                        .and_then(|v| v.as_str())
                        .ok_or("invalid attribute value")?,
                );
            }
            TraceCommand::Add { path, node } => {
                let (&index, parent_path) = path.split_last().ok_or("empty insertion path")?;
                let parent = self.at(parent_path);
                let anchor = self
                    .doc
                    .get_node(parent)
                    .unwrap()
                    .children
                    .get(index)
                    .copied();
                let mut mutation = self.doc.mutate();
                let child = Self::build(&mut mutation, node, &mut self.ids);
                if let Some(anchor) = anchor {
                    mutation.insert_nodes_before(anchor, &[child]);
                } else {
                    mutation.append_children(parent, &[child]);
                }
            }
            TraceCommand::Remove { path } => {
                let id = self.at(path);
                // Fixtures remove one leaf. Reject wider inputs rather than keeping stale IDs.
                assert!(self.doc.get_node(id).unwrap().children.is_empty());
                self.ids.retain(|_, value| *value != id);
                self.doc.mutate().remove_and_drop_node(id);
            }
            _ => return Err("unsupported fixture command".into()),
        }
        // Public style-only entry point. Do not call layout or paint.
        self.doc.resolve_stylist(0.0);
        Ok(())
    }
    fn colours(&self) -> Colours {
        self.ids
            .iter()
            .map(|(&trace_id, &id)| {
                let node = self.doc.get_node(id).unwrap();
                let computed = node.primary_styles().expect("node has no computed style");
                let colour = computed
                    .get_background()
                    .background_color
                    .resolve_to_absolute(&AbsoluteColor::BLACK)
                    .into_srgb_legacy();
                (
                    trace_id,
                    colour.raw_components().map(|v| (v * 255.0).round() as u8),
                )
            })
            .collect()
    }
}

// Accept one non-inherited, constant colour per selector. Validation also proves
// that at most one rule matches each node at every fixture frame. Under these
// conditions, colours encode the complete selector match relation.
fn palette(fixture: &Fixture, program: &CompiledProgram) -> Result<Vec<[u8; 4]>> {
    let mut rules = BTreeMap::new();
    for line in fixture.css.lines() {
        let (selector, colour) = line
            .split_once(" { background-color: rgb(")
            .ok_or("unsupported rule")?;
        let rgb: Vec<u8> = colour
            .strip_suffix("); }")
            .ok_or("unsupported declaration")?
            .split(',')
            .map(|s| s.trim().parse())
            .collect::<std::result::Result<_, _>>()?;
        let [r, g, b]: [u8; 3] = rgb.try_into().map_err(|_| "invalid RGB")?;
        let parsed = CompiledProgram::compile(&format!("{selector} {{}}"))?;
        if parsed.selectors.len() != 1 {
            return Err("expected one accepted selector".into());
        }
        if rules
            .insert(parsed.selectors[0].text.clone(), [r, g, b, 255])
            .is_some()
        {
            return Err("duplicate selector".into());
        }
    }
    if rules.len() != program.selectors.len() {
        return Err("selectors were excluded".into());
    }
    let result: Vec<_> = program.selectors.iter().map(|s| rules[&s.text]).collect();
    let unique: std::collections::BTreeSet<_> = result.iter().collect();
    if unique.len() != result.len() {
        return Err("colours must identify selectors uniquely".into());
    }
    Ok(result)
}

#[derive(Clone, Serialize)]
struct Sample {
    update_ns: u128,
    output_ns: u128,
    total_ns: u128,
}
fn run(backend: &mut dyn Backend, frames: &[TraceFrame], expected: &[Colours]) -> Result<Sample> {
    // DOM construction, first styling and first output are outside both timers.
    backend.update(&frames[0])?;
    assert_eq!(backend.colours(), expected[0], "initial output mismatch");
    let mut update_ns = 0;
    let mut output_ns = 0;
    for (frame, wanted) in frames[1..].iter().zip(&expected[1..]) {
        let start = Instant::now();
        backend.update(black_box(frame))?;
        update_ns += start.elapsed().as_nanos();
        let start = Instant::now();
        let actual = black_box(backend.colours());
        output_ns += start.elapsed().as_nanos();
        // Compare all live IDs and RGBA channels outside the timers.
        assert_eq!(
            &actual, wanted,
            "output mismatch at frame {}",
            frame.frame_id
        );
    }
    Ok(Sample {
        update_ns,
        output_ns,
        total_ns: update_ns + output_ns,
    })
}
fn option(name: &str) -> Option<String> {
    let args: Vec<_> = env::args().collect();
    args.iter()
        .position(|s| s == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}
fn command_output(cmd: &str, args: &[&str]) -> String {
    let output = Command::new(cmd).args(args).output().unwrap();
    assert!(output.status.success());
    String::from_utf8(output.stdout).unwrap()
}
fn main() -> Result<()> {
    if cfg!(debug_assertions) {
        return Err("use a release build".into());
    }
    let input = option("--input").ok_or("missing --input")?;
    let output = option("--output").ok_or("missing --output")?;
    let samples: usize = option("--samples").unwrap_or("7".into()).parse()?;
    if samples == 0 || samples.is_multiple_of(2) {
        return Err("samples must be positive and odd".into());
    }
    let fixtures: Vec<Fixture> = serde_json::from_str(&fs::read_to_string(&input)?)?;
    let mut report = json!({
        "protocol": "native-incremental-rgba-v1", "samples": samples, "warmups": 1,
        "stylo": "0.22.0", "blitz_dom": "0.3.0-beta.3", "threading": "sequential",
        "timer": "std::time::Instant; sums of per-frame update and output intervals",
        "output": "all live (trace ID, RGBA8) pairs, sorted by trace ID, after every update",
        "initialization_timed": false, "validation_timed": false,
        "captured_at_utc": command_output("date", &["-u", "+%FT%TZ"]).trim(),
        "rustc": command_output("rustc", &["-Vv"]),
        "cpu": fs::read_to_string("/proc/cpuinfo")?.lines().find(|s| s.starts_with("model name")).unwrap_or("unknown"),
        "cpu4_governor": fs::read_to_string("/sys/devices/system/cpu/cpu4/cpufreq/scaling_governor").ok().map(|s|s.trim().to_owned()),
        "cpu_affinity": fs::read_to_string("/proc/self/status")?.lines().find(|s| s.starts_with("Cpus_allowed_list:")).unwrap_or("unknown"),
        "source_hashes": command_output("sha256sum", &[
            "src/clean/engine.rs", "src/clean/engine/arena.rs", "src/clean/engine/names.rs", "src/clean/engine/trace_index.rs", "Cargo.toml", "Cargo.lock", "src/clean/engine/comparison.rs", "src/clean/compiler.rs", "src/clean/logic.rs",
            "dev/stylo-benchmark/src/main.rs", "dev/stylo-benchmark/Cargo.toml", "dev/stylo-benchmark/Cargo.lock", &input]),
        "results": []
    });
    let kinds = [
        EngineKind::Bit,
        EngineKind::Tri,
        EngineKind::RecursiveTri,
        EngineKind::Quad,
    ];
    for fixture in fixtures {
        if option("--case").is_some_and(|name| fixture.name != name) {
            continue;
        }
        let program = CompiledProgram::compile(&fixture.css)?;
        if !program.report.unsupported.is_empty() || !program.report.skipped_simple.is_empty() {
            return Err("fixture includes unsupported selectors".into());
        }
        let colours = palette(&fixture, &program)?;
        let frames: Vec<_> = fixture
            .frames
            .into_iter()
            .enumerate()
            .map(|(frame_id, command)| TraceFrame { frame_id, command })
            .collect();
        let mut reference = Project {
            engine: Engine::new(EngineKind::Naive, program.clone()),
            palette: colours.clone(),
        };
        let mut expected = Vec::new();
        for frame in &frames {
            reference.update(frame)?;
            for (_, matches) in reference.engine.comparison_node_matches() {
                assert!(
                    matches.iter().filter(|&&m| m).count() <= 1,
                    "overlapping fixture rules"
                );
            }
            expected.push(reference.colours());
        }
        assert_eq!(expected[0].len(), fixture.nodes);
        let mut measurements = vec![Vec::new(); 5];
        let mut order = Vec::new();
        for round in 0..=samples {
            // Rotate both engine families in one process on the same CPU.
            let sequence: Vec<_> = (0..5).map(|offset| (round + offset) % 5).collect();
            for &index in &sequence {
                let mut backend: Box<dyn Backend> = if index == 4 {
                    Box::new(Stylo::new(&fixture.css))
                } else {
                    Box::new(Project {
                        engine: Engine::new(kinds[index], program.clone()),
                        palette: colours.clone(),
                    })
                };
                let result = run(&mut *backend, &frames, &expected)?;
                if round != 0 {
                    measurements[index].push(result);
                }
            }
            order.push(sequence);
            eprintln!(
                "{}: round {round}/{samples}, all outputs passed",
                fixture.name
            );
        }
        let measurements: Vec<_> = measurements.into_iter().enumerate().map(|(index, values)| {
            let summary = |field: fn(&Sample) -> u128| {
                let mut sorted: Vec<_> = values.iter().map(field).collect();
                sorted.sort_unstable();
                json!({"median_ns": sorted[samples / 2], "min_ns": sorted[0], "max_ns": sorted[samples - 1]})
            };
            json!({"engine": if index == 4 { "stylo_blitz" } else { kinds[index].label() },
                "update": summary(|s| s.update_ns), "output": summary(|s| s.output_ns),
                "total": summary(|s| s.total_ns), "samples": values})
        }).collect();
        report["results"].as_array_mut().unwrap().push(json!({
            "name": fixture.name, "nodes": fixture.nodes, "selectors": program.selectors.len(),
            "updates": frames.len() - 1, "checked_frames_per_run": frames.len(),
            "execution_order": order, "measurements": measurements
        }));
        fs::write(&output, serde_json::to_string_pretty(&report)? + "\n")?;
    }
    if report["results"].as_array().unwrap().is_empty() {
        return Err("no fixtures selected".into());
    }
    Ok(())
}
