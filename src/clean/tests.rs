use super::engine::{facts_from_parts, matches_nth};
use super::*;
use crate::Node;
use std::collections::HashMap;

fn node(id: u64, tag: &str, class: Option<&str>, children: Vec<Node>) -> Node {
    let mut attributes = HashMap::new();
    if let Some(class) = class {
        attributes.insert("class".into(), serde_json::Value::String(class.into()));
    }
    Node {
        id,
        tag_name: tag.into(),
        node_type: Some("element".into()),
        attributes,
        children,
        extra: HashMap::new(),
    }
}

fn compile_rule(rule: &str) -> CompiledProgram {
    CompiledProgram::compile(&format!("{rule} {{ color: red }}")).unwrap()
}

#[test]
fn compiles_chain_in_dom_order() {
    let program = compile_rule("div > .item + span:hover");
    let selector = &program.selectors[0];
    assert_eq!(selector.compounds[0].tag.as_deref(), Some("div"));
    assert_eq!(selector.compounds[1].classes, ["item"]);
    assert_eq!(selector.compounds[2].tag.as_deref(), Some("span"));
    assert_eq!(
        selector.combinators,
        [Combinator::Child, Combinator::AdjacentSibling]
    );
}

#[test]
fn supports_forward_nth_and_rejects_last() {
    let program =
        CompiledProgram::compile("li:first-child {} li:nth-child(2n+1) {} li:last-child {}")
            .unwrap();
    assert_eq!(program.selectors.len(), 2);
    assert!(
        program
            .report
            .unsupported
            .iter()
            .any(|item| item.selector.contains("last-child"))
    );
}

#[test]
fn nth_formula_handles_positive_and_negative_steps() {
    assert!(matches_nth(
        5,
        Nth {
            a: 2,
            b: 1,
            of_type: false
        }
    ));
    assert!(!matches_nth(
        4,
        Nth {
            a: 2,
            b: 1,
            of_type: false
        }
    ));
    assert!(matches_nth(
        2,
        Nth {
            a: -1,
            b: 3,
            of_type: false
        }
    ));
    assert!(!matches_nth(
        4,
        Nth {
            a: -1,
            b: 3,
            of_type: false
        }
    ));
}

#[test]
fn hover_aliases_are_equivalent() {
    let first = facts_from_parts(
        "div",
        HashMap::from([("is_hover_root".into(), "true".into())]),
    );
    let second = facts_from_parts(
        "div",
        HashMap::from([("is_hovered_root".into(), "true".into())]),
    );
    assert!(first.hover_root);
    assert!(second.hover_root);
}

#[test]
fn engines_agree_on_sibling_and_nth_invalidation() {
    let program = compile_rule(".lead:first-child + .target");
    let trace = Trace {
        frames: vec![
            TraceFrame {
                frame_id: 0,
                command: TraceCommand::Init {
                    node: node(
                        1,
                        "main",
                        None,
                        vec![
                            node(2, "div", Some("lead"), vec![]),
                            node(3, "div", Some("target"), vec![]),
                        ],
                    ),
                },
            },
            TraceFrame {
                frame_id: 1,
                command: TraceCommand::Add {
                    path: vec![0],
                    node: node(4, "div", Some("prefix"), vec![]),
                },
            },
        ],
    };
    let naive = Engine::new(EngineKind::Naive, program.clone())
        .run(&trace)
        .unwrap();
    let mut tri_stats = None;
    for kind in [EngineKind::Bit, EngineKind::Tri, EngineKind::RecursiveTri] {
        let result = Engine::new(kind, program.clone()).run(&trace).unwrap();
        assert_eq!(result.matches, naive.matches, "engine {kind:?}");
        if kind == EngineKind::Tri {
            tri_stats = Some(result.stats);
        } else if kind == EngineKind::RecursiveTri {
            let tri = tri_stats.as_ref().unwrap();
            assert!(result.stats.visited_nodes <= tri.visited_nodes);
            assert!(result.stats.recomputed_nodes <= tri.recomputed_nodes);
        }
    }
    assert!(naive.matches.is_empty());
}

#[test]
fn checked_in_testcase_has_engine_parity() {
    let input = SiteInput::named("testcase");
    let (program, trace) = load_site(&input).unwrap();
    let naive = Engine::new(EngineKind::Naive, program.clone())
        .run(&trace)
        .unwrap();
    for kind in [EngineKind::Bit, EngineKind::Tri, EngineKind::RecursiveTri] {
        let result = Engine::new(kind, program.clone()).run(&trace).unwrap();
        assert_eq!(result.matches, naive.matches, "engine {kind:?}");
    }
}

#[test]
#[ignore = "full corpus regression; run explicitly before release"]
fn checked_in_corpus_has_engine_parity() {
    for site in [
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
    ] {
        eprintln!("checking corpus site {site}");
        let (program, trace) = load_site(&SiteInput::named(site)).unwrap();
        let naive = Engine::new(EngineKind::Naive, program.clone())
            .run(&trace)
            .unwrap();
        for kind in [EngineKind::Bit, EngineKind::Tri, EngineKind::RecursiveTri] {
            let result = Engine::new(kind, program.clone()).run(&trace).unwrap();
            assert_eq!(
                result.matches, naive.matches,
                "site {site}, engine {kind:?}"
            );
        }
    }
}
