module
public import CssMark.Types
@[expose] public section
open Aeneas Aeneas.Std Result
open CssDom
noncomputable section
set_option linter.unusedVariables false

-- FxHash only selects buckets. The extensional map model does not observe them.
def CssDom.rustc_hash.FxHasher.Insts.CoreHashHasher.finish
    (_ : CssDom.rustc_hash.FxHasher) : Result Aeneas.Std.U64 := ok 0#u64
def CssDom.rustc_hash.FxHasher.Insts.CoreHashHasher.write
    (_ : CssDom.rustc_hash.FxHasher) (_ : Slice Aeneas.Std.U8) :
    Result CssDom.rustc_hash.FxHasher := ok ()
def CssDom.rustc_hash.FxBuildHasher.Insts.CoreHashBuildHasherFxHasher.build_hasher
    (_ : CssDom.rustc_hash.FxBuildHasher) : Result CssDom.rustc_hash.FxHasher := ok ()

-- These are the storage API contracts, not an extraction of the Vec methods.
namespace CssDom.clean.engine.arena.NodeArena
def get_mut {T : Type} (map : NodeArena T) (key : Aeneas.Std.U64) :
    Result (Option T × (Option T → NodeArena T)) := by
  classical
  exact ok (map key, fun value => Function.update map key value)
def get {T : Type} (map : NodeArena T) (key : Aeneas.Std.U64) : Result (Option T) :=
  ok (map key)
def contains_key {T : Type} (map : NodeArena T) (key : Aeneas.Std.U64) : Result Bool :=
  ok (map key).isSome
def insert {T : Type} (map : NodeArena T) (key : Aeneas.Std.U64) (value : T) :
    Result (NodeArena T) := by
  classical
  exact ok (Function.update map key (some value))
def remove {T : Type} (map : NodeArena T) (key : Aeneas.Std.U64) :
    Result (Option T × NodeArena T) := by
  classical
  exact ok (map key, Function.update map key none)
def clear {T : Type} (_ : NodeArena T) : Result (NodeArena T) := ok (fun _ => none)
end CssDom.clean.engine.arena.NodeArena

def core.borrow.Borrow.Blanket.borrow {T : Type} (x : T) : Result T := ok x

-- Hash values are unobservable in the extensional map model.
def U64.Insts.CoreHashHash.hash {H : Type} (h : core.hash.Hasher H)
    (x : Aeneas.Std.U64) (state : H) : Result H := ok state

/-- The selected source functions use equal key types. Heterogeneous lookup is not
    specified. The backward function writes exactly the borrowed entry. -/
def std.collections.hash.map.HashMap.get_mut
    {K V S A Q H : Type} (eqK : core.cmp.Eq K) (hashK : core.hash.Hash K)
    (build : core.hash.BuildHasher S H) (borrow : core.borrow.Borrow K Q)
    (hashQ : core.hash.Hash Q) (eqQ : core.cmp.Eq Q)
    (map : std.collections.hash.map.HashMap K V S A) (q : Q) :
    Result (Option V × (Option V → std.collections.hash.map.HashMap K V S A)) := by
  classical
  exact if h : Q = K then
    let key : K := h ▸ q
    ok (map key, fun value => Function.update map key value)
  else fail .panic

def std.hash.random.DefaultHasher.Insts.CoreHashHasher.finish
    (_ : std.hash.random.DefaultHasher) : Result Aeneas.Std.U64 := ok 0#u64

def std.hash.random.DefaultHasher.Insts.CoreHashHasher.write
    (_ : std.hash.random.DefaultHasher) (_ : Slice Aeneas.Std.U8) :
    Result std.hash.random.DefaultHasher := ok ()

def std.hash.random.RandomState.Insts.CoreHashBuildHasherDefaultHasher.build_hasher
    (_ : std.hash.random.RandomState) : Result std.hash.random.DefaultHasher := ok ()

def std.collections.hash.map.HashMap.get
    {K V S A Q H : Type} (eqK : core.cmp.Eq K) (hashK : core.hash.Hash K)
    (build : core.hash.BuildHasher S H) (borrow : core.borrow.Borrow K Q)
    (hashQ : core.hash.Hash Q) (eqQ : core.cmp.Eq Q)
    (map : std.collections.hash.map.HashMap K V S A) (q : Q) : Result (Option V) := do
  let (value, _) ← std.collections.hash.map.HashMap.get_mut eqK hashK build borrow hashQ eqQ map q
  ok value

def std.collections.hash.map.HashMap.contains_key
    {K V S A Q H : Type} (eqK : core.cmp.Eq K) (hashK : core.hash.Hash K)
    (build : core.hash.BuildHasher S H) (borrow : core.borrow.Borrow K Q)
    (hashQ : core.hash.Hash Q) (eqQ : core.cmp.Eq Q)
    (map : std.collections.hash.map.HashMap K V S A) (q : Q) : Result Bool := do
  let value ← std.collections.hash.map.HashMap.get eqK hashK build borrow hashQ eqQ map q
  ok value.isSome

def std.collections.hash.map.HashMap.remove
    {K V S A Q H : Type} (eqK : core.cmp.Eq K) (hashK : core.hash.Hash K)
    (build : core.hash.BuildHasher S H) (borrow : core.borrow.Borrow K Q)
    (hashQ : core.hash.Hash Q) (eqQ : core.cmp.Eq Q)
    (map : std.collections.hash.map.HashMap K V S A) (q : Q) :
    Result (Option V × std.collections.hash.map.HashMap K V S A) := do
  let (value, back) ← std.collections.hash.map.HashMap.get_mut eqK hashK build borrow hashQ eqQ map q
  ok (value, back none)

def std.collections.hash.map.HashMap.insert
    {K V S A H : Type} (eqK : core.cmp.Eq K) (hashK : core.hash.Hash K)
    (build : core.hash.BuildHasher S H)
    (map : std.collections.hash.map.HashMap K V S A) (key : K) (value : V) :
    Result (Option V × std.collections.hash.map.HashMap K V S A) := by
  classical
  exact ok (map key, Function.update map key (some value))

def std.collections.hash.map.HashMap.clear
    {K V S A : Type} (_ : std.collections.hash.map.HashMap K V S A) :
    Result (std.collections.hash.map.HashMap K V S A) := ok (fun _ => none)

def core.option.Option.Insts.CoreCmpPartialEqOption.eq
    {T : Type} (eqT : core.cmp.PartialEq T T) (a b : Option T) : Result Bool :=
  match a, b with
  | none, none => ok true
  | some x, some y => eqT.eq x y
  | _, _ => ok false
