import CssLogic
import CssProofs.Quad

open Aeneas Aeneas.Std Aeneas.Std.Result

namespace CssProofs.Rust

def requirement : CssRust.Requirement → Requirement
  | .Unused => .unused
  | .Zero => .zero
  | .One => .one

theorem from_bit_refines (b : Bool) :
    ∃ r, CssRust.Requirement.from_bit b = ok r ∧ requirement r = .fromBit b := by
  cases b <;> exact ⟨_, rfl, rfl⟩

theorem accepts_refines (r : CssRust.Requirement) (b : Bool) :
    ∃ accepted, CssRust.Requirement.accepts r b = ok accepted ∧
      (accepted = true ↔ (requirement r).accepts b) := by
  cases r <;> cases b <;>
    simp [CssRust.Requirement.accepts, requirement, Requirement.accepts]

/-- The relation includes the bounds of every source projection. -/
inductive Represents {n : Nat} : CssRust.QuadValue → Quad n → Prop where
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
    simp [Slice.index_usize, List.getElem?_eq_getElem bound, same, vp]
  · intro i k same
    have bound : i.val < s.val.length := by omega
    simp [Slice.index_usize, List.getElem?_eq_getElem bound, same, vs]

theorem materialize_refines {n : Nat} {q : CssRust.QuadValue} {m : Quad n}
    (h : Represents q m) (p s : Slice Bool) (input : Input n)
    (hi : InputRep p s input) :
    CssRust.materialize_quad_value q p s = ok (m.eval input) := by
  cases h with
  | zero => rfl
  | one => rfl
  | parent i k same => exact hi.1 i k same
  | sibling i k same => exact hi.2 i k same

inductive DecisionRep {n : Nat} (input : Input n) :
    Option CssRust.Decision → List (Address n) → Prop where
  | none : DecisionRep input none []
  | read (source : CssRust.QuadValue) (a : Address n)
      (h : Represents source (.fromInput a)) :
      DecisionRep input (some ⟨source, input a⟩) [a]

theorem decision_refines {n : Nat} {q : CssRust.QuadValue} {m : Quad n}
    (h : Represents q m) (p s : Slice Bool) (input : Input n)
    (hi : InputRep p s input) :
    ∃ d, CssRust.decision_for q p s = ok d ∧ DecisionRep input d m.reads := by
  cases h with
  | zero => exact ⟨none, rfl, .none⟩
  | one => exact ⟨none, rfl, .none⟩
  | parent i k same =>
    exact ⟨_, by simp [CssRust.decision_for, hi.1 i k same], .read _ _ (.parent i k same)⟩
  | sibling i k same =>
    exact ⟨_, by simp [CssRust.decision_for, hi.2 i k same], .read _ _ (.sibling i k same)⟩

theorem specialize_or_refines {n : Nat} {q c : CssRust.QuadValue} {m carry : Quad n}
    (hq : Represents q m) (hc : Represents c carry)
    (p s : Slice Bool) (input : Input n) (hi : InputRep p s input) :
    ∃ out d, CssRust.specialize_or q c p s = ok (out, d) ∧
      Represents out (specializeOr m carry input).1 ∧
      DecisionRep input d (specializeOr m carry input).2 := by
  obtain ⟨d, hd, hr⟩ := decision_refines hq p s input hi
  have hv := materialize_refines hq p s input hi
  cases hb : m.eval input with
  | false => exact ⟨c, d, by simp [CssRust.specialize_or, hv, hb, hd], by simpa [specializeOr, hb] using hc, hr⟩
  | true => exact ⟨q, d, by simp [CssRust.specialize_or, hv, hb, hd], by simpa [specializeOr, hb] using hq, hr⟩

theorem specialize_concrete_refines {n : Nat} {q : CssRust.QuadValue} {m : Quad n}
    (hq : Represents q m) (p s : Slice Bool) (input : Input n) (hi : InputRep p s input) :
    ∃ out d, CssRust.specialize_concrete q p s = ok (out, d) ∧
      Represents out (specializeConcrete m input).1 ∧
      DecisionRep input d (specializeConcrete m input).2 := by
  obtain ⟨d, hd, hr⟩ := decision_refines hq p s input hi
  have hv := materialize_refines hq p s input hi
  cases hb : m.eval input with
  | false => exact ⟨.Zero, d, by simp [CssRust.specialize_concrete, hv, hb, hd], by simpa [specializeConcrete, Quad.concrete, hb] using Represents.zero (n := n), hr⟩
  | true => exact ⟨.One, d, by simp [CssRust.specialize_concrete, hv, hb, hd], by simpa [specializeConcrete, Quad.concrete, hb] using Represents.one (n := n), hr⟩

end CssProofs.Rust
