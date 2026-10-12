module
public import CssMark.Types
@[expose] public section
open Aeneas Aeneas.Std Result
open CssDom
noncomputable section
set_option linter.unusedVariables false

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

namespace CssDom.clean.engine.trace_index.TraceIndex
def get (map : TraceIndex) (key : Aeneas.Std.U64) : Result (Option Aeneas.Std.U64) :=
  ok (map key)
def insert (map : TraceIndex) (key value : Aeneas.Std.U64) :
    Result (Option Aeneas.Std.U64 × TraceIndex) := by
  classical
  exact ok (map key, Function.update map key (some value))
def remove (map : TraceIndex) (key : Aeneas.Std.U64) :
    Result (Option Aeneas.Std.U64 × TraceIndex) := by
  classical
  exact ok (map key, Function.update map key none)
def clear (_ : TraceIndex) : Result TraceIndex := ok (fun _ => none)
end CssDom.clean.engine.trace_index.TraceIndex

def core.option.Option.Insts.CoreCmpPartialEqOption.eq
    {T : Type} (eqT : core.cmp.PartialEq T T) (a b : Option T) : Result Bool :=
  match a, b with
  | none, none => ok true
  | some x, some y => eqT.eq x y
  | _, _ => ok false
