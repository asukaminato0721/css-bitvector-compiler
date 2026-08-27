mod compiler;
mod engine;
mod runner;
mod trace;

pub use compiler::{
    Combinator, CompileError, CompiledProgram, Compound, DynamicPseudo, EngineKind, NodeId, Nth,
    SelectorChain, SelectorReport, StateId, UnsupportedSelector,
};
pub use engine::{Dom, Engine, RunResult, RunStats};
pub use runner::{SiteInput, load_site};
pub use trace::{RunError, Trace, TraceCommand, TraceFrame};

#[cfg(test)]
mod tests;
