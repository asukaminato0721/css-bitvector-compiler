use lightningcss::{
    rules::CssRule,
    selector::{
        Combinator as CssCombinator, Component as CssComponent, PseudoClass,
        Selector as CssSelector,
    },
    stylesheet::{ParserOptions, PrinterOptions, StyleSheet},
    traits::ToCss,
};
use parcel_selectors::{attr::AttrSelectorOperator, parser::NthType};
use std::{
    collections::BTreeMap,
    error::Error,
    fmt::{self, Display},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StateId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineKind {
    Naive,
    Bit,
    Tri,
    RecursiveTri,
    ExperimentalQuad,
}

impl EngineKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::Naive => "naive",
            Self::Bit => "bit",
            Self::Tri => "tri",
            Self::RecursiveTri => "rec_tri",
            Self::ExperimentalQuad => "quad",
        }
    }

    pub(crate) fn uses_dependencies(self) -> bool {
        matches!(self, Self::Tri | Self::RecursiveTri)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Combinator {
    Descendant,
    Child,
    AdjacentSibling,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Nth {
    pub a: i32,
    pub b: i32,
    pub of_type: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DynamicPseudo {
    Hover,
    Focus,
    FocusWithin,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Compound {
    pub tag: Option<String>,
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub attributes: Vec<(String, String)>,
    pub pseudos: Vec<DynamicPseudo>,
    pub nth: Vec<Nth>,
}

impl Compound {
    fn is_empty(&self) -> bool {
        self.tag.is_none()
            && self.id.is_none()
            && self.classes.is_empty()
            && self.attributes.is_empty()
            && self.pseudos.is_empty()
            && self.nth.is_empty()
    }

    fn is_trivial_single(&self) -> bool {
        self.attributes.is_empty() && self.pseudos.is_empty() && self.nth.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectorChain {
    pub text: String,
    pub compounds: Vec<Compound>,
    /// `combinators[i]` connects compounds `i` and `i + 1`.
    pub combinators: Vec<Combinator>,
    state_offset: usize,
}

impl SelectorChain {
    pub(crate) fn state(&self, part: usize) -> StateId {
        StateId(self.state_offset + part)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsupportedSelector {
    pub selector: String,
    pub reason: String,
}

#[derive(Debug, Clone, Default)]
pub struct SelectorReport {
    pub skipped_simple: Vec<String>,
    pub unsupported: Vec<UnsupportedSelector>,
    pub unsupported_pseudos: BTreeMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Default)]
pub struct CompiledProgram {
    pub selectors: Vec<SelectorChain>,
    pub report: SelectorReport,
    pub state_count: usize,
}

impl CompiledProgram {
    pub fn compile(css: &str) -> Result<Self, CompileError> {
        let options = ParserOptions {
            error_recovery: true,
            ..ParserOptions::default()
        };
        let sheet = StyleSheet::parse(css, options)
            .map_err(|error| CompileError(format!("unable to parse stylesheet: {error:?}")))?;

        let mut parsed = Vec::new();
        let mut report = SelectorReport::default();
        for rule in sheet.rules.0 {
            let CssRule::Style(style) = rule else {
                continue;
            };
            for selector in style.selectors.0 {
                let text = selector_text(&selector);
                match parse_selector(&selector, text.clone()) {
                    Ok(chain)
                        if chain.combinators.is_empty()
                            && chain.compounds[0].is_trivial_single() =>
                    {
                        report.skipped_simple.push(text);
                    }
                    Ok(chain) => parsed.push(chain),
                    Err(reason) => {
                        if let Some(pseudo) = reason.strip_prefix("unsupported pseudo-class ") {
                            report
                                .unsupported_pseudos
                                .entry(pseudo.to_string())
                                .or_default()
                                .push(text.clone());
                        }
                        report.unsupported.push(UnsupportedSelector {
                            selector: text,
                            reason,
                        });
                    }
                }
            }
        }

        parsed.sort_by(|a, b| a.text.cmp(&b.text));
        parsed.dedup_by(|a, b| a.text == b.text);
        report.skipped_simple.sort();
        report.skipped_simple.dedup();
        report
            .unsupported
            .sort_by(|a, b| a.selector.cmp(&b.selector).then(a.reason.cmp(&b.reason)));
        report
            .unsupported
            .dedup_by(|a, b| a.selector == b.selector && a.reason == b.reason);
        for selectors in report.unsupported_pseudos.values_mut() {
            selectors.sort();
            selectors.dedup();
        }

        let mut state_count = 0;
        for selector in &mut parsed {
            selector.state_offset = state_count;
            state_count += selector.compounds.len();
        }
        Ok(Self {
            selectors: parsed,
            report,
            state_count,
        })
    }

    pub fn to_dot(&self) -> String {
        let mut dot = String::from("digraph SelectorProgram {\n  rankdir=LR;\n");
        for (rule_index, selector) in self.selectors.iter().enumerate() {
            for (part, compound) in selector.compounds.iter().enumerate() {
                let state = selector.state(part).0;
                let label = format!("{}:{} {:?}", rule_index, part, compound).replace('"', "\\\"");
                let shape = if part + 1 == selector.compounds.len() {
                    "doublecircle"
                } else {
                    "circle"
                };
                dot.push_str(&format!("  s{state} [shape={shape}, label=\"{label}\"];\n"));
                if part > 0 {
                    let previous = selector.state(part - 1).0;
                    dot.push_str(&format!(
                        "  s{previous} -> s{state} [label=\"{:?}\"];\n",
                        selector.combinators[part - 1]
                    ));
                }
            }
        }
        dot.push_str("}\n");
        dot
    }
}

#[derive(Debug, Clone)]
pub struct CompileError(pub String);

impl Display for CompileError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl Error for CompileError {}

fn selector_text(selector: &CssSelector) -> String {
    selector
        .to_css_string(PrinterOptions::default())
        .unwrap_or_else(|_| format!("{selector:?}"))
}

fn parse_selector(selector: &CssSelector, text: String) -> Result<SelectorChain, String> {
    let mut compounds = vec![Compound::default()];
    let mut combinators = Vec::new();

    for component in selector.iter_raw_parse_order_from(0) {
        match component {
            CssComponent::Combinator(css_combinator) => {
                if compounds.last().is_some_and(Compound::is_empty) {
                    continue;
                }
                let combinator = match css_combinator {
                    CssCombinator::Descendant => Combinator::Descendant,
                    CssCombinator::Child => Combinator::Child,
                    CssCombinator::NextSibling => Combinator::AdjacentSibling,
                    CssCombinator::LaterSibling => {
                        return Err("unsupported combinator general-sibling".into());
                    }
                    _ => return Err(format!("unsupported combinator {css_combinator:?}")),
                };
                combinators.push(combinator);
                compounds.push(Compound::default());
            }
            CssComponent::ExplicitUniversalType => {
                compounds.last_mut().unwrap().tag = Some("*".into());
            }
            CssComponent::LocalName(name) => {
                compounds.last_mut().unwrap().tag = Some(name.name.as_ref().to_ascii_lowercase());
            }
            CssComponent::ID(id) => compounds.last_mut().unwrap().id = Some(id.to_string()),
            CssComponent::Class(class) => {
                compounds
                    .last_mut()
                    .unwrap()
                    .classes
                    .push(class.to_string());
            }
            CssComponent::AttributeInNoNamespace {
                local_name,
                operator: AttrSelectorOperator::Equal,
                value,
                ..
            } => compounds
                .last_mut()
                .unwrap()
                .attributes
                .push((local_name.as_ref().to_ascii_lowercase(), value.to_string())),
            CssComponent::Nth(data) => {
                let of_type = match data.ty {
                    NthType::Child | NthType::OfType => data.ty == NthType::OfType,
                    NthType::LastChild | NthType::LastOfType => {
                        return Err("unsupported pseudo-class :last-*".into());
                    }
                    _ => return Err("unsupported pseudo-class :only-*".into()),
                };
                compounds.last_mut().unwrap().nth.push(Nth {
                    a: data.a,
                    b: data.b,
                    of_type,
                });
            }
            CssComponent::NonTSPseudoClass(pseudo) => {
                let pseudo = match pseudo {
                    PseudoClass::Hover => DynamicPseudo::Hover,
                    PseudoClass::Focus => DynamicPseudo::Focus,
                    PseudoClass::FocusWithin => DynamicPseudo::FocusWithin,
                    _ => return Err("unsupported pseudo-class <dynamic>".into()),
                };
                compounds.last_mut().unwrap().pseudos.push(pseudo);
            }
            CssComponent::NthOf(_) => return Err("unsupported pseudo-class :nth-*(of S)".into()),
            CssComponent::Negation(_) => return Err("unsupported pseudo-class :not".into()),
            CssComponent::Is(_) => return Err("unsupported pseudo-class :is".into()),
            CssComponent::Where(_) => return Err("unsupported pseudo-class :where".into()),
            CssComponent::Has(_) => return Err("unsupported pseudo-class :has".into()),
            CssComponent::PseudoElement(_) => {
                return Err("unsupported pseudo-element".into());
            }
            CssComponent::AttributeInNoNamespaceExists { .. }
            | CssComponent::AttributeInNoNamespace { .. }
            | CssComponent::AttributeOther(_) => {
                return Err("unsupported attribute operator".into());
            }
            other => return Err(format!("unsupported selector component {other:?}")),
        }
    }

    if compounds.last().is_some_and(Compound::is_empty) {
        compounds.pop();
    }
    if compounds.is_empty() || combinators.len() + 1 != compounds.len() {
        return Err("malformed selector chain".into());
    }
    for compound in &mut compounds {
        compound.classes.sort();
        compound.classes.dedup();
        compound.attributes.sort();
        compound.attributes.dedup();
        compound.pseudos.sort_by_key(|pseudo| *pseudo as u8);
        compound.pseudos.dedup();
    }
    Ok(SelectorChain {
        text,
        compounds,
        combinators,
        state_offset: 0,
    })
}
