//! Comparison with the released parcel_selectors matcher on the shared DOM.
use super::*;
use lightningcss::{
    rules::CssRule,
    selector::{PseudoClass, Selector},
    stylesheet::{ParserOptions, PrinterOptions, StyleSheet},
    traits::ToCss,
};
use parcel_selectors::{
    Element, NthIndexCache, OpaqueElement, SelectorImpl,
    attr::{AttrSelectorOperation, CaseSensitivity, NamespaceConstraint},
    context::{MatchingContext, MatchingMode, QuirksMode},
    matching::{ElementSelectorFlags, matches_selector},
};
use std::time::Instant;

// Recover the public selector's implementation type without naming the private module.
trait Implementation<'i> {
    type Impl: SelectorImpl<'i>;
}
impl<'i, I: SelectorImpl<'i>> Implementation<'i> for parcel_selectors::parser::Selector<'i, I> {
    type Impl = I;
}
type CssImpl<'i> = <Selector<'i> as Implementation<'i>>::Impl;

#[derive(Clone, Copy, Debug)]
struct ElementRef<'a> {
    dom: &'a Dom,
    links: &'a [(Option<NodeId>, Option<NodeId>)],
    id: NodeId,
}

impl ElementRef<'_> {
    fn facts(&self) -> &NodeFacts {
        &self.dom.nodes[&self.id].facts
    }
    fn at(&self, id: NodeId) -> Self {
        Self {
            dom: self.dom,
            links: self.links,
            id,
        }
    }
}

impl<'i> Element<'i> for ElementRef<'_> {
    type Impl = CssImpl<'i>;
    fn opaque(&self) -> OpaqueElement {
        OpaqueElement::new(&self.dom.nodes[&self.id])
    }
    fn parent_element(&self) -> Option<Self> {
        self.dom.nodes[&self.id].parent.map(|id| self.at(id))
    }
    fn parent_node_is_shadow_root(&self) -> bool {
        false
    }
    fn containing_shadow_host(&self) -> Option<Self> {
        None
    }
    fn is_pseudo_element(&self) -> bool {
        false
    }
    fn prev_sibling_element(&self) -> Option<Self> {
        self.dom
            .nodes
            .slot(&self.id)
            .and_then(|slot| self.links.get(slot))
            .and_then(|pair| pair.0)
            .map(|id| self.at(id))
    }
    fn next_sibling_element(&self) -> Option<Self> {
        self.dom
            .nodes
            .slot(&self.id)
            .and_then(|slot| self.links.get(slot))
            .and_then(|pair| pair.1)
            .map(|id| self.at(id))
    }
    fn is_html_element_in_html_document(&self) -> bool {
        true
    }
    fn has_local_name(&self, name: &<Self::Impl as SelectorImpl<'i>>::BorrowedLocalName) -> bool {
        self.facts().tag == name.as_ref()
    }
    fn has_namespace(&self, ns: &<Self::Impl as SelectorImpl<'i>>::BorrowedNamespaceUrl) -> bool {
        ns.as_ref().is_empty() || ns.as_ref() == "http://www.w3.org/1999/xhtml"
    }
    fn is_same_type(&self, other: &Self) -> bool {
        self.facts().tag == other.facts().tag
    }
    fn attr_matches(
        &self,
        ns: &NamespaceConstraint<&<Self::Impl as SelectorImpl<'i>>::NamespaceUrl>,
        name: &<Self::Impl as SelectorImpl<'i>>::LocalName,
        operation: &AttrSelectorOperation<&<Self::Impl as SelectorImpl<'i>>::AttrValue>,
    ) -> bool {
        if matches!(ns, NamespaceConstraint::Specific(value) if !value.as_ref().is_empty()) {
            return false;
        }
        self.facts()
            .attributes
            .get_named(&self.dom.names, name.as_ref())
            .is_some_and(|value| operation.eval_str(value))
    }
    fn match_non_ts_pseudo_class<F>(
        &self,
        pc: &<Self::Impl as SelectorImpl<'i>>::NonTSPseudoClass,
        _: &mut MatchingContext<'_, 'i, Self::Impl>,
        _: &mut F,
    ) -> bool
    where
        F: FnMut(&Self, ElementSelectorFlags),
    {
        let pseudo = match pc {
            PseudoClass::Hover => DynamicPseudo::Hover,
            PseudoClass::Focus => DynamicPseudo::Focus,
            PseudoClass::FocusWithin => DynamicPseudo::FocusWithin,
            _ => return false,
        };
        self.facts().pseudos & pseudo_bit(pseudo) != 0
    }
    fn match_pseudo_element(
        &self,
        _: &<Self::Impl as SelectorImpl<'i>>::PseudoElement,
        _: &mut MatchingContext<'_, 'i, Self::Impl>,
    ) -> bool {
        false
    }
    fn is_link(&self) -> bool {
        false
    }
    fn is_html_slot_element(&self) -> bool {
        false
    }
    fn has_id(
        &self,
        id: &<Self::Impl as SelectorImpl<'i>>::Identifier,
        case: CaseSensitivity,
    ) -> bool {
        self.facts()
            .html_id
            .as_ref()
            .is_some_and(|value| case.eq(value.as_bytes(), id.as_ref().as_bytes()))
    }
    fn has_class(
        &self,
        class: &<Self::Impl as SelectorImpl<'i>>::Identifier,
        case: CaseSensitivity,
    ) -> bool {
        self.facts().classes.iter().any(|&id| {
            case.eq(
                self.dom.names.name(id).as_bytes(),
                class.as_ref().as_bytes(),
            )
        })
    }
    fn imported_part(
        &self,
        _: &<Self::Impl as SelectorImpl<'i>>::Identifier,
    ) -> Option<<Self::Impl as SelectorImpl<'i>>::Identifier> {
        None
    }
    fn is_part(&self, _: &<Self::Impl as SelectorImpl<'i>>::Identifier) -> bool {
        false
    }
    fn is_empty(&self) -> bool {
        self.dom.nodes[&self.id]
            .children
            .iter()
            .all(|id| !self.dom.nodes.contains_key(id))
    }
    fn is_root(&self) -> bool {
        self.dom.root == Some(self.id)
    }
}

/// Parses exactly the selector subset accepted by the project compiler.
/// Parsing happens before the measured interval.
pub struct ParcelProgram<'i> {
    selectors: Vec<Selector<'i>>,
}

#[derive(Debug, Clone)]
pub struct ComparisonSample {
    pub elapsed_ns: u128,
    pub matches: BTreeMap<String, Vec<u64>>,
}

impl<'i> ParcelProgram<'i> {
    pub fn parse(css: &'i str, program: &CompiledProgram) -> Result<Self, RunError> {
        let sheet = StyleSheet::parse(
            css,
            ParserOptions {
                error_recovery: true,
                ..Default::default()
            },
        )
        .map_err(|e| RunError::new(format!("parcel stylesheet parse: {e:?}")))?;
        let mut parsed = BTreeMap::new();
        for rule in sheet.rules.0 {
            if let CssRule::Style(style) = rule {
                for selector in style.selectors.0 {
                    let text = selector
                        .to_css_string(PrinterOptions::default())
                        .map_err(|e| {
                            RunError::new(format!("parcel selector serialization: {e:?}"))
                        })?;
                    parsed.insert(text, selector);
                }
            }
        }
        let selectors = program
            .selectors
            .iter()
            .map(|s| {
                parsed
                    .remove(&s.text)
                    .ok_or_else(|| RunError::new(format!("parcel selector missing: {}", s.text)))
            })
            .collect::<Result<_, _>>()?;
        Ok(Self { selectors })
    }

    fn rematch(&self, engine: &mut Engine) {
        // Constant-time sibling access keeps the adapter from adding a linear
        // scan to every library traversal. Index construction is timed.
        let mut links = vec![(None, None); engine.dom.nodes.slot_count()];
        for node in engine.dom.nodes.values() {
            let mut previous = None;
            for &child in &node.children {
                if !engine.dom.nodes.contains_key(&child) {
                    continue;
                }
                links[engine.dom.nodes.slot(&child).unwrap()].0 = previous;
                if let Some(prev) = previous {
                    links[engine.dom.nodes.slot(&prev).unwrap()].1 = Some(child);
                }
                previous = Some(child);
            }
        }
        // Cache nth positions within this DOM version. Never retain it across a mutation.
        let mut nth = NthIndexCache::default();
        let mut ids = Vec::with_capacity(engine.dom.nodes.len());
        ids.extend(engine.dom.nodes.keys());
        for id in ids {
            let element = ElementRef {
                dom: &engine.dom,
                links: &links,
                id,
            };
            let mut context = MatchingContext::new(
                MatchingMode::Normal,
                None,
                Some(&mut nth),
                QuirksMode::NoQuirks,
            );
            let matches = self
                .selectors
                .iter()
                .map(|selector| {
                    matches_selector(selector, 0, None, &element, &mut context, &mut |_, _| {})
                })
                .collect();
            engine.dom.nodes.get_mut(&id).unwrap().state.matches = matches;
        }
    }

    pub fn run(
        &self,
        program: &CompiledProgram,
        trace: &Trace,
    ) -> Result<ComparisonSample, RunError> {
        let mut engine = Engine::new(EngineKind::Bit, program.clone());
        let start = Instant::now();
        for frame in &trace.frames {
            engine.apply_mutation(frame)?;
            self.rematch(&mut engine);
        }
        let elapsed_ns = start.elapsed().as_nanos();
        Ok(ComparisonSample {
            elapsed_ns,
            matches: engine.collect_matches(),
        })
    }

    /// Require exact match sets after every frame before accepting timing results.
    pub fn check(&self, program: &CompiledProgram, trace: &Trace) -> Result<(), RunError> {
        let mut external = Engine::new(EngineKind::Bit, program.clone());
        let mut engines: Vec<_> = [
            EngineKind::Naive,
            EngineKind::Bit,
            EngineKind::Tri,
            EngineKind::RecursiveTri,
            EngineKind::Quad,
        ]
        .into_iter()
        .map(|kind| Engine::new(kind, program.clone()))
        .collect();
        for frame in &trace.frames {
            external.apply_mutation(frame)?;
            self.rematch(&mut external);
            let expected = external.collect_matches();
            for engine in &mut engines {
                engine.apply(frame)?;
                let actual = engine.collect_matches();
                if actual != expected {
                    let selector = program
                        .selectors
                        .iter()
                        .find(|s| actual.get(&s.text) != expected.get(&s.text));
                    return Err(RunError::at(
                        frame.frame_id,
                        format!(
                            "{} differs from parcel_selectors for {:?}",
                            engine.kind.label(),
                            selector.map(|s| &s.text)
                        ),
                    ));
                }
            }
        }
        Ok(())
    }
}

impl Engine {
    /// Apply one update without exporting selector names or match sets.
    pub fn comparison_update(&mut self, frame: &TraceFrame) -> Result<(), RunError> {
        self.apply(frame)
    }

    /// Borrow final match bits for a benchmark's common output adapter.
    pub fn comparison_node_matches(&self) -> impl Iterator<Item = (u64, &[bool])> {
        self.dom
            .nodes
            .values()
            .map(|node| (node.trace_id, node.state.matches.as_slice()))
    }

    /// Export one frame for untimed differential checks with a browser.
    pub fn comparison_step(
        &mut self,
        frame: &TraceFrame,
    ) -> Result<BTreeMap<String, Vec<u64>>, RunError> {
        self.apply(frame)?;
        Ok(self.collect_matches())
    }

    pub fn comparison_run(mut self, trace: &Trace) -> Result<ComparisonSample, RunError> {
        let start = Instant::now();
        for frame in &trace.frames {
            self.apply(frame)?;
        }
        let elapsed_ns = start.elapsed().as_nanos();
        Ok(ComparisonSample {
            elapsed_ns,
            matches: self.collect_matches(),
        })
    }
}
