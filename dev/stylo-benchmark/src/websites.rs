//! Replay captured website data with a shared tree and RGBA output contract.
use blitz_dom::{Attribute, BaseDocument, DocumentConfig, StyleThreading};
use blitz_traits::node_id::NodeId;
use css_bitvector_compiler::{
    Node, attributes_to_string_map,
    clean::{
        Combinator, CompiledProgram, DynamicPseudo, Engine, EngineKind, SelectorChain, SiteInput,
        TraceCommand, TraceFrame, load_site,
    },
};
use lightningcss::{
    rules::CssRule,
    stylesheet::{ParserOptions, StyleSheet},
};
use markup5ever::{QualName, ns};
use serde::Serialize;
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, HashMap},
    env,
    error::Error,
    fmt::Write as _,
    fs,
    hint::black_box,
    process::Command,
    time::Instant,
};
use style::{color::AbsoluteColor, stylesheets::Origin};
use stylo_dom::ElementState;
type Result<T> = std::result::Result<T, Box<dyn Error>>;
type Colours = Vec<(u64, [u8; 4])>;
const SITES: &[&str] = &[
    "amazon",
    "bilibili",
    "bing",
    "bootstrap",
    "google",
    "netflix",
    "tiktok",
    "whatsapp",
    "wikipedia",
    "yahoo",
    "youtube",
];
const KINDS: [EngineKind; 4] = [
    EngineKind::Bit,
    EngineKind::Tri,
    EngineKind::RecursiveTri,
    EngineKind::Quad,
];
fn tag(name: &str) -> String {
    let mut out = "bench-".to_string();
    for byte in name.to_ascii_lowercase().bytes() {
        write!(out, "{byte:02x}").unwrap();
    }
    out
}
fn attr(name: &str) -> String {
    let key = name.to_ascii_lowercase();
    match key.as_str() {
        "id" | "class" | "is_hover_root" | "is_hovered_root" | "is_focus_root" => key,
        _ => format!("data-{}", tag(&key)),
    }
}
fn escape(value: &str) -> String {
    let mut out = String::new();
    for c in value.chars() {
        assert_ne!(c, '\0');
        write!(out, "\\{:x} ", c as u32).unwrap();
    }
    out
}
fn selector(chain: &SelectorChain, native: bool) -> String {
    let mut out = String::new();
    for (i, part) in chain.compounds.iter().enumerate() {
        if i != 0 {
            out.push_str(match chain.combinators[i - 1] {
                Combinator::Descendant => " ",
                Combinator::Child => " > ",
                Combinator::AdjacentSibling => " + ",
            });
        }
        if let Some(name) = &part.tag {
            out.push_str(&if name == "*" { "*".into() } else { tag(name) });
        }
        if let Some(id) = &part.id {
            write!(out, "#{}", escape(id)).unwrap();
        }
        for class in &part.classes {
            write!(out, ".{}", escape(class)).unwrap();
        }
        for (key, value) in &part.attributes {
            write!(out, "[{}=\"{}\"]", escape(&attr(key)), escape(value)).unwrap();
        }
        for pseudo in &part.pseudos {
            out.push_str(match pseudo {
                DynamicPseudo::Hover => ":hover",
                DynamicPseudo::Focus => ":focus",
                DynamicPseudo::FocusWithin => {
                    if native {
                        ":active"
                    } else {
                        ":focus-within"
                    }
                }
            });
        }
        for nth in &part.nth {
            write!(
                out,
                ":nth-{}({}n{:+})",
                if nth.of_type { "of-type" } else { "child" },
                nth.a,
                nth.b
            )
            .unwrap();
        }
        // The algorithm root has no sibling position. A native root is a child of Document.
        if native && !part.nth.is_empty() {
            out.push_str(":where(:not(:root))");
        }
        if part.tag.is_none()
            && part.id.is_none()
            && part.classes.is_empty()
            && part.attributes.is_empty()
            && part.pseudos.is_empty()
            && part.nth.is_empty()
        {
            out.push('*');
        }
    }
    out
}
fn normalize_node(node: &mut Node) {
    node.tag_name = tag(&node.tag_name);
    node.attributes = attributes_to_string_map(&node.attributes)
        .into_iter()
        .map(|(k, v)| (attr(&k), Value::String(v)))
        .collect();
    for child in &mut node.children {
        normalize_node(child);
    }
}
fn normalize_frame(frame: &mut TraceFrame) {
    match &mut frame.command {
        TraceCommand::Init { node }
        | TraceCommand::Add { node, .. }
        | TraceCommand::Replace { node, .. } => normalize_node(node),
        TraceCommand::ReplaceValue { key, .. }
        | TraceCommand::InsertValue { key, .. }
        | TraceCommand::DeleteValue { key, .. } => *key = attr(key),
        _ => {}
    }
}
fn colour(index: usize) -> [u8; 4] {
    let v = index + 1;
    assert!(v < 1 << 24);
    [v as u8, (v >> 8) as u8, (v >> 16) as u8, 255]
}
struct Projection {
    program: CompiledProgram,
    css: String,
    priority: Vec<usize>,
    // Index in the normalized program -> index in the original program.
    original_indices: Vec<usize>,
}
fn project(original: &CompiledProgram) -> Result<Projection> {
    // Preserve each selector and each state, including equivalent conditions.
    let mut program = original.clone();
    let mut css = String::new();
    let mut priorities = Vec::new();
    let original_indices = (0..original.selectors.len()).collect();
    for (i, (s, raw)) in program
        .selectors
        .iter_mut()
        .zip(&original.selectors)
        .enumerate()
    {
        let native = selector(raw, true);
        s.text = selector(raw, false);
        for part in &mut s.compounds {
            if let Some(t) = &mut part.tag
                && t != "*"
            {
                *t = tag(t);
            }
            for (key, _) in &mut part.attributes {
                *key = attr(key);
            }
        }
        let input = format!("{} {{}}", s.text);
        let parsed =
            StyleSheet::parse(&input, ParserOptions::default()).map_err(|e| e.to_string())?;
        let CssRule::Style(rule) = &parsed.rules.0[0] else {
            unreachable!()
        };
        priorities.push((rule.selectors.0[0].specificity(), i));
        let [r, g, b, _] = colour(i);
        writeln!(css, "{native} {{ background-color: rgb({r},{g},{b}); }}").unwrap();
    }
    priorities.sort_unstable_by(|a, b| b.cmp(a));
    Ok(Projection {
        program,
        css,
        priority: priorities.into_iter().map(|(_, i)| i).collect(),
        original_indices,
    })
}
trait Backend {
    fn update(&mut self, frame: &TraceFrame) -> Result<()>;
    fn colours(&self) -> Colours;
}
struct Project {
    engine: Engine,
    priority: Vec<usize>,
}
impl Project {
    fn new(kind: EngineKind, projection: &Projection) -> Self {
        Self {
            engine: Engine::new(kind, projection.program.clone()),
            priority: projection.priority.clone(),
        }
    }
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
                (
                    id,
                    self.priority
                        .iter()
                        .find(|&&i| matches[i])
                        .map_or([0; 4], |&i| colour(i)),
                )
            })
            .collect();
        result.sort_unstable_by_key(|&(id, _)| id);
        result
    }
}
struct LogicalNode {
    trace_id: u64,
    children: Vec<NodeId>,
    hover: bool,
    focus: bool,
}
struct Stylo {
    doc: BaseDocument,
    root: Option<NodeId>,
    ids: BTreeMap<u64, NodeId>,
    nodes: HashMap<NodeId, LogicalNode>,
}
fn name(key: &str) -> QualName {
    QualName::new(None, ns!(), key.into())
}
impl Stylo {
    fn new(css: &str) -> Self {
        let mut doc = BaseDocument::new(DocumentConfig {
            ua_stylesheets: Some(vec![]),
            style_threading: StyleThreading::Sequential,
            ..Default::default()
        });
        let sheet = doc.make_stylesheet(css, Origin::Author);
        let owner = doc
            .mutate()
            .create_element(QualName::new(None, ns!(html), "style".into()), vec![]);
        doc.add_stylesheet_for_node(sheet, owner);
        Self {
            doc,
            root: None,
            ids: BTreeMap::new(),
            nodes: HashMap::new(),
        }
    }
    fn at(&self, path: &[usize]) -> NodeId {
        let mut id = self.root.unwrap();
        for &index in path {
            id = self.nodes[&id].children[index];
        }
        assert!(self.nodes.contains_key(&id), "obsolete path {path:?}");
        id
    }
    fn forget(&mut self, id: NodeId) {
        if let Some(node) = self.nodes.remove(&id) {
            if self.ids.get(&node.trace_id) == Some(&id) {
                self.ids.remove(&node.trace_id);
            }
            for child in node.children {
                self.forget(child);
            }
        }
    }
    fn drop_tree(&mut self, id: NodeId) {
        if self.nodes.contains_key(&id) {
            self.forget(id);
            self.doc.mutate().remove_and_drop_node(id);
        }
    }
    fn build(&mut self, node: &Node) -> NodeId {
        if let Some(&old) = self.ids.get(&node.id) {
            self.drop_tree(old);
        }
        let attrs = attributes_to_string_map(&node.attributes);
        let flag = |key| {
            attrs
                .get(key)
                .is_some_and(|v| v.eq_ignore_ascii_case("true"))
        };
        let id = self.doc.mutate().create_element(
            QualName::new(None, ns!(html), node.tag_name.as_str().into()),
            attrs
                .iter()
                .map(|(k, v)| Attribute {
                    name: name(k),
                    value: v.as_str().into(),
                })
                .collect(),
        );
        self.ids.insert(node.id, id);
        self.nodes.insert(
            id,
            LogicalNode {
                trace_id: node.id,
                children: vec![],
                hover: flag("is_hover_root") || flag("is_hovered_root"),
                focus: flag("is_focus_root"),
            },
        );
        for child in &node.children {
            let child = self.build(child);
            self.doc.mutate().append_children(id, &[child]);
            self.nodes.get_mut(&id).unwrap().children.push(child);
        }
        id
    }
    fn add(&mut self, path: &[usize], node: &Node) {
        let (&index, parent) = path.split_last().unwrap();
        let parent = self.at(parent);
        assert!(index <= self.nodes[&parent].children.len());
        let child = self.build(node);
        let anchor = self.nodes[&parent].children[index..]
            .iter()
            .find(|id| self.nodes.contains_key(id))
            .copied();
        if let Some(anchor) = anchor {
            self.doc.mutate().insert_nodes_before(anchor, &[child]);
        } else {
            self.doc.mutate().append_children(parent, &[child]);
        }
        self.nodes
            .get_mut(&parent)
            .unwrap()
            .children
            .insert(index, child);
    }
    fn remove(&mut self, path: &[usize]) {
        let (&index, parent) = path.split_last().unwrap();
        let parent = self.at(parent);
        let id = self.nodes.get_mut(&parent).unwrap().children.remove(index);
        self.drop_tree(id);
    }
    fn attribute(
        &mut self,
        path: &[usize],
        key: &str,
        value: Option<&Value>,
        value_type: Option<&str>,
    ) -> Result<()> {
        if value_type != Some("attributes") {
            return Err("unsupported value type".into());
        }
        let id = self.at(path);
        let value = value
            .map(|v| match v {
                Value::String(s) => Ok(s.clone()),
                Value::Bool(_) | Value::Number(_) => Ok(v.to_string()),
                _ => Err("non-scalar attribute"),
            })
            .transpose()?;
        if let Some(v) = &value {
            self.doc.mutate().set_attribute(id, name(key), v);
        } else {
            self.doc.mutate().clear_attribute(id, name(key));
        }
        let flag = value.is_some_and(|s| s.eq_ignore_ascii_case("true"));
        match key {
            "is_hover_root" | "is_hovered_root" => self.nodes.get_mut(&id).unwrap().hover = flag,
            "is_focus_root" => self.nodes.get_mut(&id).unwrap().focus = flag,
            _ => {}
        }
        Ok(())
    }
    fn states(&mut self, id: NodeId, parent_hover: bool) -> bool {
        let Some(node) = self.nodes.get(&id) else {
            return false;
        };
        let hover = parent_hover || node.hover;
        let focus = node.focus;
        let children = node.children.clone();
        let mut within = focus;
        for child in children {
            within |= self.states(child, hover);
        }
        let mut state = ElementState::empty();
        state.set(ElementState::HOVER, hover);
        state.set(ElementState::FOCUS, focus);
        // Blitz 0.3.0-beta.3 returns false for FocusWithin. ACTIVE is unused
        // by accepted source selectors and carries the same derived boolean.
        state.set(ElementState::ACTIVE, within);
        let mask = ElementState::HOVER | ElementState::FOCUS | ElementState::ACTIVE;
        let old = self
            .doc
            .get_node(id)
            .unwrap()
            .element_data()
            .unwrap()
            .element_state
            & mask;
        if old != state {
            self.doc.snapshot_node_and(id, old ^ state, |node| {
                let element = node.element_data_mut().unwrap();
                element.element_state.remove(mask);
                element.element_state.insert(state);
                node.mark_ancestors_dirty();
            });
        }
        within
    }
}
impl Backend for Stylo {
    fn update(&mut self, frame: &TraceFrame) -> Result<()> {
        match &frame.command {
            TraceCommand::Init { node } => {
                assert!(self.root.is_none());
                let root = self.build(node);
                let doc = self.doc.root_node().id;
                self.doc.mutate().append_children(doc, &[root]);
                self.root = Some(root);
            }
            TraceCommand::Add { path, node } => self.add(path, node),
            TraceCommand::Remove { path } => self.remove(path),
            TraceCommand::Replace { path, node } => {
                self.remove(path);
                self.add(path, node);
            }
            TraceCommand::ReplaceValue {
                path,
                key,
                value,
                value_type,
                ..
            }
            | TraceCommand::InsertValue {
                path,
                key,
                value,
                value_type,
            } => self.attribute(path, key, value.as_ref(), value_type.as_deref())?,
            TraceCommand::DeleteValue {
                path,
                key,
                value_type,
                ..
            } => self.attribute(path, key, None, value_type.as_deref())?,
            TraceCommand::Recalculate => {}
        }
        self.states(self.root.unwrap(), false);
        self.doc.resolve_stylist(0.0);
        Ok(())
    }
    fn colours(&self) -> Colours {
        self.ids
            .iter()
            .map(|(&trace_id, &id)| {
                let computed = self
                    .doc
                    .get_node(id)
                    .unwrap()
                    .primary_styles()
                    .unwrap_or_else(|| panic!("missing computed style: trace node {trace_id}, native {id:?}, parent {:?}", self.doc.get_node(id).unwrap().parent));
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
#[derive(Clone, Serialize)]
struct Sample {
    update_ns: u128,
    output_ns: u128,
    total_ns: u128,
}
fn check(actual: &Colours, wanted: &Colours, frame: &TraceFrame) -> Result<()> {
    if actual != wanted {
        let differences: Vec<_> = actual
            .iter()
            .zip(wanted)
            .filter(|(a, b)| a != b)
            .take(12)
            .collect();
        return Err(format!(
            "frame {}: RGBA mismatch, sizes {} / {}, first differences {:?}",
            frame.frame_id,
            actual.len(),
            wanted.len(),
            differences
        )
        .into());
    }
    Ok(())
}
fn run(backend: &mut dyn Backend, frames: &[TraceFrame], expected: &[Colours]) -> Result<Sample> {
    backend.update(&frames[0])?;
    check(&backend.colours(), &expected[0], &frames[0])?;
    let mut update_ns = 0;
    let mut output_ns = 0;
    for (frame, wanted) in frames[1..].iter().zip(&expected[1..]) {
        let start = Instant::now();
        backend.update(black_box(frame))?;
        update_ns += start.elapsed().as_nanos();
        let start = Instant::now();
        let actual = black_box(backend.colours());
        output_ns += start.elapsed().as_nanos();
        check(&actual, wanted, frame)?;
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
fn command(cmd: &str, args: &[&str]) -> String {
    let out = Command::new(cmd).args(args).output().unwrap();
    assert!(out.status.success());
    String::from_utf8(out.stdout).unwrap()
}
fn main() -> Result<()> {
    if cfg!(debug_assertions) {
        return Err("use a release build".into());
    }
    let validate = env::args().any(|s| s == "--validate-only");
    let samples: usize = option("--samples").unwrap_or("7".into()).parse()?;
    if samples == 0 || samples.is_multiple_of(2) {
        return Err("samples must be positive and odd".into());
    }
    let output = option("--output").unwrap_or("docs/website-results.json".into());
    let mut report = json!({"protocol":"website-incremental-rgba-v1","samples":samples,"warmups":1,"validation_only":validate,
        "stylo":"0.22.0","blitz_dom":"0.3.0-beta.3","threading":"sequential",
        "initialization_timed":false,"validation_timed":false,"output":"all live (trace ID, RGBA8) pairs after each frame, sorted by trace ID",
        "normalization":"all recorded nodes are generic HTML elements; type selectors use encoded tags; attributes except class, id, and trace state flags are encoded and inert; dynamic state follows the project tree model; synthetic unique background colours; specificity then original compiled selector order; native :active represents derived focus-within because Blitz does not implement FocusWithin matching",
        "captured_at_utc":command("date",&["-u","+%FT%TZ"]).trim(),"rustc":command("rustc",&["-Vv"]),
        "cpu":fs::read_to_string("/proc/cpuinfo")?.lines().find(|s| s.starts_with("model name")).unwrap_or("unknown"),
        "cpu_affinity":fs::read_to_string("/proc/self/status")?.lines().find(|s| s.starts_with("Cpus_allowed_list:")).unwrap_or("unknown"),
        "source_hashes":command("sha256sum",&["src/clean/engine.rs","src/clean/engine/comparison.rs","src/clean/compiler.rs","src/clean/logic.rs","dev/stylo-benchmark/src/websites.rs","dev/stylo-benchmark/Cargo.toml","dev/stylo-benchmark/Cargo.lock"]),"results":[]});
    for site in SITES {
        if option("--site").is_some_and(|s| s != *site) {
            continue;
        }
        let input = SiteInput::named(*site);
        let (original, trace) = load_site(&input)?;
        let projection = project(&original)?;
        let mut frames = trace.frames.clone();
        for frame in &mut frames {
            normalize_frame(frame);
        }
        eprintln!(
            "{site}: validate {} selectors, {} frames",
            projection.program.selectors.len(),
            frames.len()
        );
        let mut reference = Project::new(EngineKind::Bit, &projection);
        let mut original_engine = Engine::new(EngineKind::Bit, original.clone());
        let mut expected = Vec::new();
        let mut overlapping_node_frames = 0usize;
        let mut matched_node_frames = 0usize;
        for (frame, raw) in frames.iter().zip(&trace.frames) {
            reference.update(frame)?;
            original_engine.comparison_update(raw)?;
            let raw_matches: BTreeMap<_, _> = original_engine.comparison_node_matches().collect();
            for (id, matches) in reference.engine.comparison_node_matches() {
                let old = raw_matches[&id];
                assert_eq!(matches.len(), old.len());
                for (i, &hit) in matches.iter().enumerate() {
                    assert_eq!(
                        hit, old[projection.original_indices[i]],
                        "projection changed a match: {site}, frame {}, node {id}, selector {i}",
                        frame.frame_id
                    );
                }
                let count = matches.iter().filter(|&&hit| hit).count();
                overlapping_node_frames += usize::from(count > 1);
                matched_node_frames += usize::from(count > 0);
            }
            assert_eq!(
                raw_matches.len(),
                reference.engine.comparison_node_matches().count()
            );
            expected.push(reference.colours());
        }
        eprintln!("{site}: projection check passed");
        let mut command_counts: BTreeMap<&str, usize> = BTreeMap::new();
        for frame in &frames {
            let label = match frame.command {
                TraceCommand::Init { .. } => "init",
                TraceCommand::Add { .. } => "add",
                TraceCommand::Remove { .. } => "remove",
                TraceCommand::Replace { .. } => "replace",
                TraceCommand::ReplaceValue { .. } => "replace_value",
                TraceCommand::InsertValue { .. } => "insert_value",
                TraceCommand::DeleteValue { .. } => "delete_value",
                TraceCommand::Recalculate => "recalculate",
            };
            *command_counts.entry(label).or_default() += 1;
        }
        let mut measurements = vec![Vec::new(); 5];
        let mut order = Vec::new();
        for round in 0..=if validate { 0 } else { samples } {
            let sequence: Vec<_> = (0..5).map(|offset| (round + offset) % 5).collect();
            for &index in &sequence {
                let mut backend: Box<dyn Backend> = if index == 4 {
                    Box::new(Stylo::new(&projection.css))
                } else {
                    Box::new(Project::new(KINDS[index], &projection))
                };
                let sample = run(&mut *backend, &frames, &expected)
                    .map_err(|e| format!("{site}, engine {index}, round {round}: {e}"))?;
                eprintln!(
                    "{site}: round {round}, {}: {:.3} ms, outputs passed",
                    if index == 4 {
                        "stylo_blitz"
                    } else {
                        KINDS[index].label()
                    },
                    sample.total_ns as f64 / 1e6
                );
                if round > 0 {
                    measurements[index].push(sample);
                }
            }
            order.push(sequence);
            eprintln!(
                "{site}: round {round}/{}, all outputs passed",
                if validate { 0 } else { samples }
            );
        }
        let measurements: Vec<_> = measurements.into_iter().enumerate().map(|(i,values)| {
            let summary = |field: fn(&Sample)->u128| { let mut sorted: Vec<_> = values.iter().map(field).collect(); sorted.sort_unstable(); if sorted.is_empty() {Value::Null} else {json!({"median_ns":sorted[sorted.len()/2],"min_ns":sorted[0],"max_ns":sorted[sorted.len()-1]})} };
            json!({"engine":if i==4 {"stylo_blitz"} else {KINDS[i].label()},"update":summary(|s|s.update_ns),"output":summary(|s|s.output_ns),"total":summary(|s|s.total_ns),"samples":values})
        }).collect();
        report["results"].as_array_mut().unwrap().push(json!({"name":site,"command_counts":command_counts,"selectors":original.selectors.len(),"unsupported":original.report.unsupported.len(),"skipped_simple":original.report.skipped_simple.len(),"updates":frames.len()-1,"checked_frames_per_run":frames.len(),"initial_nodes":expected[0].len(),"peak_nodes":expected.iter().map(Vec::len).max(),"node_frames":expected.iter().map(Vec::len).sum::<usize>(),"matched_node_frames":matched_node_frames,"overlapping_node_frames":overlapping_node_frames,"input_hashes":command("sha256sum",&[input.css_path.to_str().unwrap(),input.trace_path.to_str().unwrap()]),"execution_order":order,"measurements":measurements}));
        fs::write(&output, serde_json::to_string_pretty(&report)? + "\n")?;
    }
    if report["results"].as_array().unwrap().is_empty() {
        return Err("no sites selected".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frames(commands: Vec<Value>) -> Vec<TraceFrame> {
        commands
            .into_iter()
            .enumerate()
            .map(|(frame_id, command)| TraceFrame {
                frame_id,
                command: serde_json::from_value(command).unwrap(),
            })
            .collect()
    }
    fn parity(css: &str, mut frames: Vec<TraceFrame>) -> Vec<Colours> {
        let original = CompiledProgram::compile(css).unwrap();
        let projection = project(&original).unwrap();
        for frame in &mut frames {
            normalize_frame(frame);
        }
        let mut oracle = Project::new(EngineKind::Naive, &projection);
        let expected: Vec<_> = frames
            .iter()
            .map(|f| {
                oracle.update(f).unwrap();
                oracle.colours()
            })
            .collect();
        for kind in KINDS {
            run(&mut Project::new(kind, &projection), &frames, &expected).unwrap();
        }
        run(&mut Stylo::new(&projection.css), &frames, &expected).unwrap();
        expected
    }

    #[test]
    fn overlapping_rules_and_inert_inline_style() {
        let css = "div[class=\"a b\"] {} .b[class=\"a b\"] {} #winner[style=\"display:none\"] {} [style=\"display:none\"] {}";
        let original = CompiledProgram::compile(css).unwrap();
        let winner = original
            .selectors
            .iter()
            .position(|s| s.text.contains("#winner"))
            .unwrap();
        let result = parity(
            css,
            frames(vec![
                json!({"name":"init","node":{"id":1,"name":"DIV","attributes":{"id":"winner","class":"a b","style":"display:none"}}}),
                json!({"name":"delete_value","path":[],"type":"attributes","key":"id","old_value":"winner"}),
            ]),
        );
        assert_eq!(result[0], vec![(1, colour(winner))]);
        assert_ne!(result[0], result[1]);
    }

    #[test]
    fn moved_ids_keep_obsolete_slots_and_position() {
        let node = json!({"id":2,"name":"span","attributes":{"class":"a"}});
        let result = parity(
            "div > span {} span:nth-child(2) {} .a + span {}",
            frames(vec![
                json!({"name":"init","node":{"id":1,"name":"div","children":[node,{"id":3,"name":"span"}]}}),
                json!({"name":"add","path":[2],"node":node}),
                json!({"name":"remove","path":[0]}),
                json!({"name":"replace","path":[0],"node":{"id":4,"name":"span"}}),
            ]),
        );
        assert_eq!(result[1], result[2]);
        assert_eq!(
            result[3].iter().map(|x| x.0).collect::<Vec<_>>(),
            vec![1, 2, 4]
        );
    }

    #[test]
    fn dynamic_states_and_root_position() {
        let result = parity(
            "div:hover {} span:hover {} div:focus-within {} span:focus {} *:nth-child(1) {}",
            frames(vec![
                json!({"name":"init","node":{"id":1,"name":"div","children":[{"id":2,"name":"span"}]}}),
                json!({"name":"insert_value","path":[],"type":"attributes","key":"is_hover_root","value":true}),
                json!({"name":"insert_value","path":[0],"type":"attributes","key":"is_focus_root","value":true}),
                json!({"name":"delete_value","path":[],"type":"attributes","key":"is_hover_root","old_value":true}),
                json!({"name":"delete_value","path":[0],"type":"attributes","key":"is_focus_root","old_value":true}),
                json!({"name":"recalculate"}),
            ]),
        );
        assert_eq!(result[0][0], (1, [0; 4]));
        assert_ne!(result[0], result[1]);
        assert_ne!(result[1], result[3]);
        assert_eq!(result[0], result[4]);
        assert_eq!(result[4], result[5]);
    }

    #[test]
    fn equal_conditions_keep_separate_rules() {
        let original = CompiledProgram::compile("div .a.b {} div .b.a {} * > div {}").unwrap();
        let projection = project(&original).unwrap();
        assert_eq!(original.selectors.len(), projection.program.selectors.len());
        assert_eq!(original.state_count, projection.program.state_count);
    }
}
