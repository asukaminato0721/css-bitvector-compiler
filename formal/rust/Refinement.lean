import CssMark.Funs
import CssProofs.Quad
import CssProofs.Incremental

open Aeneas Aeneas.Std Aeneas.Std.Result

namespace CssProofs.Rust

def dirty : CssDom.clean.logic.Dirty → Incremental.Dirty
  | .Clean => .clean
  | .InputChanged => .inputChanged
  | .NodeChanged => .nodeChanged

theorem dirty_merge_refines (a b : CssDom.clean.logic.Dirty) :
    ∃ out, CssDom.clean.logic.Dirty.merge a b = ok out ∧ dirty out = (dirty a).merge (dirty b) := by
  cases a <;> cases b <;> exact ⟨_, rfl, rfl⟩

def requirement : CssDom.clean.logic.Requirement → Requirement
  | .Unused => .unused
  | .Zero => .zero
  | .One => .one

theorem from_bit_refines (b : Bool) :
    ∃ r, CssDom.clean.logic.Requirement.from_bit b = ok r ∧ requirement r = .fromBit b := by
  cases b <;> exact ⟨_, rfl, rfl⟩

theorem accepts_refines (r : CssDom.clean.logic.Requirement) (b : Bool) :
    ∃ accepted, CssDom.clean.logic.Requirement.accepts r b = ok accepted ∧
      (accepted = true ↔ (requirement r).accepts b) := by
  cases r <;> cases b <;>
    simp [CssDom.clean.logic.Requirement.accepts, requirement, Requirement.accepts]

/-- The relation includes the bounds of every source projection. -/
inductive Represents {n : Nat} : CssDom.clean.logic.QuadValue → Quad n → Prop where
  | zero : Represents .Zero .zero
  | one : Represents .One .one
  | parent (i : Aeneas.Std.Usize) (k : Fin n) (same : i.val = k.val) :
      Represents (.FromParent i) (.fromInput (.parent, k))
  | sibling (i : Aeneas.Std.Usize) (k : Fin n) (same : i.val = k.val) :
      Represents (.FromSibling i) (.fromInput (.sibling, k))

def InputRep {n : Nat} (p s : Slice Bool) (input : Input n) : Prop :=
  (∀ (i : Aeneas.Std.Usize) (k : Fin n), i.val = k.val →
    p.index_usize i = ok (input (.parent, k))) ∧
  (∀ (i : Aeneas.Std.Usize) (k : Fin n), i.val = k.val →
    s.index_usize i = ok (input (.sibling, k)))

/-- Equal lengths and equal entries establish the input relation. -/
theorem input_rep_of_arrays {n : Nat} (p s : Slice Bool) (input : Input n)
    (hp : p.val.length = n) (hs : s.val.length = n)
    (vp : ∀ k : Fin n, p.val[k.val]'(hp ▸ k.isLt) = input (.parent, k))
    (vs : ∀ k : Fin n, s.val[k.val]'(hs ▸ k.isLt) = input (.sibling, k)) :
    InputRep p s input := by
  constructor
  · intro i k same
    have bound : i.val < p.val.length := by omega
    simp only [Slice.index_usize, Slice.getElem?_Usize_eq,
      List.getElem?_eq_getElem bound]
    simpa only [same] using congrArg ok (vp k)
  · intro i k same
    have bound : i.val < s.val.length := by omega
    simp only [Slice.index_usize, Slice.getElem?_Usize_eq,
      List.getElem?_eq_getElem bound]
    simpa only [same] using congrArg ok (vs k)

theorem materialize_refines {n : Nat} {q : CssDom.clean.logic.QuadValue} {m : Quad n}
    (h : Represents q m) (p s : Slice Bool) (input : Input n)
    (hi : InputRep p s input) :
    CssDom.clean.logic.materialize_quad_value q p s = ok (m.eval input) := by
  cases h with
  | zero => rfl
  | one => rfl
  | parent i k same => exact hi.1 i k same
  | sibling i k same => exact hi.2 i k same

inductive DecisionRep {n : Nat} (input : Input n) :
    Option CssDom.clean.logic.Decision → List (Address n) → Prop where
  | none : DecisionRep input none []
  | read (source : CssDom.clean.logic.QuadValue) (a : Address n)
      (h : Represents source (.fromInput a)) :
      DecisionRep input (some ⟨source, input a⟩) [a]

theorem decision_refines {n : Nat} {q : CssDom.clean.logic.QuadValue} {m : Quad n}
    (h : Represents q m) (p s : Slice Bool) (input : Input n)
    (hi : InputRep p s input) :
    ∃ d, CssDom.clean.logic.decision_for q p s = ok d ∧ DecisionRep input d m.reads := by
  cases h with
  | zero => exact ⟨none, rfl, .none⟩
  | one => exact ⟨none, rfl, .none⟩
  | parent i k same =>
    exact ⟨_, by simp [CssDom.clean.logic.decision_for, hi.1 i k same], .read _ _ (.parent i k same)⟩
  | sibling i k same =>
    exact ⟨_, by simp [CssDom.clean.logic.decision_for, hi.2 i k same], .read _ _ (.sibling i k same)⟩

theorem specialize_or_refines {n : Nat} {q c : CssDom.clean.logic.QuadValue} {m carry : Quad n}
    (hq : Represents q m) (hc : Represents c carry)
    (p s : Slice Bool) (input : Input n) (hi : InputRep p s input) :
    ∃ out d, CssDom.clean.logic.specialize_or q c p s = ok (out, d) ∧
      Represents out (specializeOr m carry input).1 ∧
      DecisionRep input d (specializeOr m carry input).2 := by
  obtain ⟨d, hd, hr⟩ := decision_refines hq p s input hi
  have hv := materialize_refines hq p s input hi
  cases hb : m.eval input with
  | false => exact ⟨c, d, by simp [CssDom.clean.logic.specialize_or, hv, hb, hd], by simpa [specializeOr, hb] using hc, hr⟩
  | true => exact ⟨q, d, by simp [CssDom.clean.logic.specialize_or, hv, hb, hd], by simpa [specializeOr, hb] using hq, hr⟩

theorem specialize_concrete_refines {n : Nat} {q : CssDom.clean.logic.QuadValue} {m : Quad n}
    (hq : Represents q m) (p s : Slice Bool) (input : Input n) (hi : InputRep p s input) :
    ∃ out d, CssDom.clean.logic.specialize_concrete q p s = ok (out, d) ∧
      Represents out (specializeConcrete m input).1 ∧
      DecisionRep input d (specializeConcrete m input).2 := by
  obtain ⟨d, hd, hr⟩ := decision_refines hq p s input hi
  have hv := materialize_refines hq p s input hi
  cases hb : m.eval input with
  | false => exact ⟨.Zero, d, by simp [CssDom.clean.logic.specialize_concrete, hv, hb, hd], by simpa [specializeConcrete, Quad.concrete, hb] using Represents.zero (n := n), hr⟩
  | true => exact ⟨.One, d, by simp [CssDom.clean.logic.specialize_concrete, hv, hb, hd], by simpa [specializeConcrete, Quad.concrete, hb] using Represents.one (n := n), hr⟩

end CssProofs.Rust
