module
public import Aeneas
@[expose] public section

/-- Extensional map model. Capacity, buckets, allocation, and hashing are outside
    this model. Source refinement uses the Rust standard-library map contract. -/
abbrev std.collections.hash.map.HashMap (K V _S _A : Type) := K → Option V
abbrev std.collections.hash.set.HashSet (T _S _A : Type) := T → Bool
abbrev std.hash.random.RandomState := Unit
abbrev std.hash.random.DefaultHasher := Unit

/-- Algorithm proofs use the container contract. The concrete Vec representation,
    sequential append invariant, and allocation behavior are outside this model. -/
abbrev CssDom.clean.engine.arena.NodeArena (T : Type) := Aeneas.Std.U64 → Option T
abbrev CssDom.rustc_hash.FxHasher := Unit
