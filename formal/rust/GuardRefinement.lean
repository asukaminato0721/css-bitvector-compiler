import Refinement

open Aeneas Aeneas.Std Result ControlFlow WP
open CssDom.clean.logic

namespace CssProofs.Rust

theorem requirements_length_mismatch (r : Slice CssDom.clean.logic.Requirement)
    (input : Slice Bool) (h : r.length ≠ input.length) :
    requirements_accept r input = ok false := by
  have hn : Slice.len r ≠ Slice.len input := by
    intro heq
    exact h (congrArg UScalar.val heq)
  simp [requirements_accept, hn]

def Entry (r : Slice CssDom.clean.logic.Requirement) (input : Slice Bool) (j : Nat) : Prop :=
  (requirement (r.val[j]?.getD .Unused)).accepts (input.val[j]?.getD false)

def Prefix (r : Slice CssDom.clean.logic.Requirement) (input : Slice Bool) (k : Nat) : Prop :=
  ∀ j, j < k → Entry r input j

theorem prefix_succ (r : Slice CssDom.clean.logic.Requirement) (input : Slice Bool) (k : Nat) :
    Prefix r input (k + 1) ↔ Prefix r input k ∧ Entry r input k := by
  constructor
  · intro h
    exact ⟨fun j hj => h j (by omega), h k (by omega)⟩
  · rintro ⟨h, hk⟩ j hj
    by_cases hlt : j < k
    · exact h j hlt
    · have heq : j = k := by omega
      simpa only [heq] using hk

@[step] theorem requirement_accepts_spec (r : CssDom.clean.logic.Requirement) (b : Bool) :
    CssDom.clean.logic.Requirement.accepts r b ⦃ out =>
      out = true ↔ (requirement r).accepts b ⦄ := by
  obtain ⟨out, h, hp⟩ := accepts_refines r b
  simpa only [h, spec_ok] using hp

def GuardInv (r : Slice CssDom.clean.logic.Requirement) (input : Slice Bool)
    (state : Bool × Aeneas.Std.Usize) : Prop :=
  state.2.val ≤ r.length ∧ (state.1 = true ↔ Prefix r input state.2.val)

theorem guard_loop_spec (r : Slice CssDom.clean.logic.Requirement) (input : Slice Bool)
    (hlen : r.length = input.length) (accepted : Bool) (index : Aeneas.Std.Usize)
    (hindex : index.val ≤ r.length)
    (hprefix : accepted = true ↔ Prefix r input index.val) :
    requirements_accept_loop r input accepted index ⦃ out =>
      out = true ↔ Prefix r input r.length ⦄ := by
  rw [requirements_accept_loop.eq_def]
  by_cases hlt : index < Slice.len r
  · simp only [hlt, ↓reduceIte]
    have ib : index.val < r.length := by scalar_tac
    cases ha : accepted with
    | false =>
      simp only [Bool.false_eq_true, ↓reduceIte, spec_ok]
      constructor
      · intro h; contradiction
      · intro hall
        have hp : Prefix r input index.val := fun j hj => hall j (by omega)
        have := hprefix.mpr hp
        simp [ha] at this
    | true =>
      simp only [↓reduceIte]
      have bound : index.val + 1 ≤ Aeneas.Std.Usize.max :=
        Nat.le_trans (by simpa only [Slice.length] using Nat.succ_le_of_lt ib) (Slice.length_ineq r)
      step as ⟨value, hv⟩
      step as ⟨bit, hb⟩
      step as ⟨accepted1, haccept⟩
      step as ⟨index1, hi1⟩
      have he : (requirement value).accepts bit ↔ Entry r input index.val := by
        simp only [Entry, List.getElem?_eq_getElem ib, hv, hb, Option.getD_some]
        have ii : index.val < input.length := by omega
        simp only [List.getElem?_eq_getElem ii, Option.getD_some]
      apply guard_loop_spec r input hlen accepted1 index1 (by scalar_tac)
      rw [hi1, prefix_succ]
      have hp := hprefix.mp ha
      simpa only [hp, true_and] using haccept.trans he
  · simp only [hlt, ↓reduceIte]
    have eqend : index.val = r.length := by scalar_tac
    simpa only [spec_ok, eqend] using hprefix
termination_by r.length - index.val
decreasing_by scalar_tac

theorem requirements_accept_correct (r : Slice CssDom.clean.logic.Requirement)
    (input : Slice Bool) :
    requirements_accept r input ⦃ accepted =>
      accepted = true ↔ r.length = input.length ∧ Prefix r input r.length ⦄ := by
  by_cases hlen : r.length = input.length
  · have hscalar : Slice.len r = Slice.len input := by scalar_tac
    simp only [requirements_accept, hscalar, bne_self_eq_false, Bool.false_eq_true, ↓reduceIte]
    have hloop : requirements_accept_loop r input true 0#usize ⦃ accepted =>
        accepted = true ↔ Prefix r input r.length ⦄ := by
      apply guard_loop_spec r input hlen true 0#usize
      · simp
      · simp [Prefix]
    simpa only [hlen, true_and] using hloop
  · rw [requirements_length_mismatch r input hlen]
    simp [hlen]

end CssProofs.Rust
