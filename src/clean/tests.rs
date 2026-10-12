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
    for kind in [
        EngineKind::Bit,
        EngineKind::Tri,
        EngineKind::RecursiveTri,
        EngineKind::Quad,
    ] {
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
fn quad_composes_descendant_propagation_without_recompute() {
    let program = compile_rule(".a .b .c");
    let trace = Trace {
        frames: vec![
            TraceFrame {
                frame_id: 0,
                command: TraceCommand::Init {
                    node: node(
                        1,
                        "div",
                        Some("a"),
                        vec![node(
                            2,
                            "div",
                            Some("x"),
                            vec![node(3, "div", Some("c"), vec![])],
                        )],
                    ),
                },
            },
            TraceFrame {
                frame_id: 1,
                command: TraceCommand::ReplaceValue {
                    path: vec![],
                    value_type: Some("attributes".into()),
                    key: "class".into(),
                    value: Some(serde_json::Value::String("x".into())),
                    old_value: Some(serde_json::Value::String("a".into())),
                },
            },
        ],
    };
    let naive = Engine::new(EngineKind::Naive, program.clone())
        .run(&trace)
        .unwrap();
    let tri = Engine::new(EngineKind::Tri, program.clone())
        .run(&trace)
        .unwrap();
    let quad = Engine::new(EngineKind::Quad, program).run(&trace).unwrap();
    assert_eq!(quad.matches, naive.matches);
    assert_eq!(tri.matches, naive.matches);
    assert!(quad.stats.recomputed_nodes < tri.stats.recomputed_nodes);
    assert!(quad.stats.input_skips > 0);
}

#[test]
fn tri_and_recursive_tri_skip_unread_parent_input() {
    let program = compile_rule(".a > .b");
    let trace = Trace {
        frames: vec![
            TraceFrame {
                frame_id: 0,
                command: TraceCommand::Init {
                    node: node(
                        1,
                        "div",
                        Some("a"),
                        vec![node(2, "div", Some("not-b"), vec![])],
                    ),
                },
            },
            TraceFrame {
                frame_id: 1,
                command: TraceCommand::ReplaceValue {
                    path: vec![],
                    value_type: Some("attributes".into()),
                    key: "class".into(),
                    value: Some(serde_json::Value::String("x".into())),
                    old_value: Some(serde_json::Value::String("a".into())),
                },
            },
        ],
    };
    let bit = Engine::new(EngineKind::Bit, program.clone())
        .run(&trace)
        .unwrap();
    let tri = Engine::new(EngineKind::Tri, program.clone())
        .run(&trace)
        .unwrap();
    let recursive = Engine::new(EngineKind::RecursiveTri, program)
        .run(&trace)
        .unwrap();
    assert_eq!(bit.matches, tri.matches);
    assert_eq!(tri.matches, recursive.matches);
    assert!(tri.stats.recomputed_nodes < bit.stats.recomputed_nodes);
    assert!(recursive.stats.visited_nodes < tri.stats.visited_nodes);
}

#[test]
fn checked_in_testcase_has_engine_parity() {
    let input = SiteInput::named("testcase");
    let (program, trace) = load_site(&input).unwrap();
    let naive = Engine::new(EngineKind::Naive, program.clone())
        .run(&trace)
        .unwrap();
    let mut tri_misses = None;
    for kind in [
        EngineKind::Bit,
        EngineKind::Tri,
        EngineKind::RecursiveTri,
        EngineKind::Quad,
    ] {
        let result = Engine::new(kind, program.clone()).run(&trace).unwrap();
        assert_eq!(result.matches, naive.matches, "engine {kind:?}");
        if kind == EngineKind::Tri {
            tri_misses = Some(result.stats.recomputed_nodes);
        } else if kind == EngineKind::Quad {
            assert!(result.stats.recomputed_nodes <= tri_misses.unwrap());
        }
    }
}

#[test]
fn removing_old_move_reference_keeps_same_parent_destination() {
    let program = compile_rule("main > .moved");
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
                            node(2, "span", Some("moved"), vec![]),
                            node(3, "span", None, vec![]),
                        ],
                    ),
                },
            },
            TraceFrame {
                frame_id: 1,
                command: TraceCommand::Add {
                    path: vec![2],
                    node: node(2, "span", Some("moved"), vec![]),
                },
            },
            TraceFrame {
                frame_id: 2,
                command: TraceCommand::Remove { path: vec![0] },
            },
        ],
    };
    for kind in [
        EngineKind::Naive,
        EngineKind::Bit,
        EngineKind::Tri,
        EngineKind::RecursiveTri,
        EngineKind::Quad,
    ] {
        let result = Engine::new(kind, program.clone()).run(&trace).unwrap();
        assert_eq!(
            result.matches.get("main > .moved"),
            Some(&vec![2]),
            "{kind:?}"
        );
    }
}

#[test]
fn moved_slots_do_not_count_as_css_siblings() {
    let program = CompiledProgram::compile(
        ".lead:first-child + .moved:nth-child(2) {} \
         .moved:nth-of-type(2) {} .moved > .inner {}",
    )
    .unwrap();
    // Move within one parent, in both directions. Check the intermediate frame,
    // before the old trace slot is removed, against explicit CSS results.
    for (children, insertion, obsolete) in [
        (
            vec![
                node(
                    2,
                    "span",
                    Some("moved"),
                    vec![node(4, "b", Some("inner"), vec![])],
                ),
                node(3, "span", Some("lead"), vec![]),
            ],
            2,
            0,
        ),
        (
            vec![
                node(3, "span", Some("lead"), vec![]),
                node(
                    2,
                    "span",
                    Some("moved"),
                    vec![node(4, "b", Some("inner"), vec![])],
                ),
            ],
            1,
            2,
        ),
    ] {
        let mut trace = Trace {
            frames: vec![
                TraceFrame {
                    frame_id: 0,
                    command: TraceCommand::Init {
                        node: node(1, "main", None, children),
                    },
                },
                TraceFrame {
                    frame_id: 1,
                    command: TraceCommand::Add {
                        path: vec![insertion],
                        node: node(
                            2,
                            "span",
                            Some("moved"),
                            vec![node(4, "b", Some("inner"), vec![])],
                        ),
                    },
                },
            ],
        };
        for after_removal in [false, true] {
            if after_removal {
                trace.frames.push(TraceFrame {
                    frame_id: 2,
                    command: TraceCommand::Remove {
                        path: vec![obsolete],
                    },
                });
            }
            for kind in [
                EngineKind::Naive,
                EngineKind::Bit,
                EngineKind::Tri,
                EngineKind::RecursiveTri,
                EngineKind::Quad,
            ] {
                let result = Engine::new(kind, program.clone()).run(&trace).unwrap();
                assert_eq!(result.matches.len(), 3, "{kind:?}, removed={after_removal}");
                assert_eq!(
                    result.matches[".lead:first-child + .moved:nth-child(2)"],
                    [2]
                );
                assert_eq!(result.matches[".moved:nth-of-type(2)"], [2]);
                assert_eq!(result.matches[".moved > .inner"], [4]);
            }
        }
    }
}

#[test]
fn invalid_node_ownership_is_rejected() {
    let program = compile_rule("*");
    let duplicate = Trace {
        frames: vec![TraceFrame {
            frame_id: 0,
            command: TraceCommand::Init {
                node: node(1, "main", None, vec![node(1, "span", None, vec![])]),
            },
        }],
    };
    assert!(
        Engine::new(EngineKind::Quad, program.clone())
            .run(&duplicate)
            .is_err()
    );
    let cycle = Trace {
        frames: vec![
            TraceFrame {
                frame_id: 0,
                command: TraceCommand::Init {
                    node: node(1, "main", None, vec![node(2, "div", None, vec![])]),
                },
            },
            TraceFrame {
                frame_id: 1,
                command: TraceCommand::Add {
                    path: vec![0, 0],
                    node: node(1, "main", None, vec![]),
                },
            },
        ],
    };
    assert!(Engine::new(EngineKind::Quad, program).run(&cycle).is_err());
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
        let mut tri_misses = None;
        for kind in [
            EngineKind::Bit,
            EngineKind::Tri,
            EngineKind::RecursiveTri,
            EngineKind::Quad,
        ] {
            let result = Engine::new(kind, program.clone()).run(&trace).unwrap();
            assert_eq!(
                result.matches, naive.matches,
                "site {site}, engine {kind:?}"
            );
            if kind == EngineKind::Tri {
                tri_misses = Some(result.stats.recomputed_nodes);
            } else if kind == EngineKind::Quad {
                assert!(
                    result.stats.recomputed_nodes <= tri_misses.unwrap(),
                    "site {site}: quad recomputed more nodes than tri"
                );
            }
        }
    }
}
