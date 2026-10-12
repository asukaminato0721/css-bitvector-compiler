module
public import Aeneas
@[expose] public section

/-- Container contracts used by the selected source proofs. Concrete storage,
    allocation, sorted keys, and sequential append are outside this model. -/
abbrev CssDom.clean.engine.arena.NodeArena (T : Type) := Aeneas.Std.U64 → Option T
abbrev CssDom.clean.engine.trace_index.TraceIndex := Aeneas.Std.U64 → Option Aeneas.Std.U64
