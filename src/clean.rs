mod compiler;
mod engine;
mod runner;
mod trace;

pub use compiler::{
    Combinator, CompileError, CompiledProgram, Compound, DynamicPseudo, EngineKind, NodeId, Nth,
    SelectorChain, SelectorReport, StateId, UnsupportedSelector,
};
pub use engine::{Dom, Engine, FrameStats, RunResult, RunStats};
pub use runner::{
    SiteInput, binary_main, env_flag, load_site, report_selectors, run_site,
    run_site_with_frame_stats,
};
pub use trace::{RunError, Trace, TraceCommand, TraceFrame};

#[cfg(test)]
mod tests;
