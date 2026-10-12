import RemoveRefinement

open Aeneas Aeneas.Std Result WP
open CssDom.clean.compiler CssDom.clean.engine

namespace CssProofs.RustSibling

def previousLive (nodes : NodeId → Option DomNode) (target : NodeId) :
    List NodeId → Option NodeId → Option NodeId
  | [], _ => none
  | child :: rest, previous =>
    if child = target then previous
    else previousLive nodes target rest (if (nodes child).isSome then some child else previous)

theorem skip_obsolete (nodes : NodeId → Option DomNode) (target child : NodeId)
    (rest : List NodeId) (previous : Option NodeId)
    (missing : nodes child = none) (different : child ≠ target) :
    previousLive nodes target (child :: rest) previous = previousLive nodes target rest previous := by
  simp [previousLive, missing, different]

theorem previous_loop_spec (nodes : NodeId → Option DomNode) (target : NodeId)
    (siblings : alloc.vec.Vec NodeId) (previous : Option NodeId) (index : Aeneas.Std.Usize)
    (hindex : index.val ≤ siblings.val.length) :
    Dom.previous_sibling_loop nodes target siblings previous index ⦃ out =>
      out = previousLive nodes target (siblings.val.drop index.val) previous ⦄ := by
  rw [Dom.previous_sibling_loop.eq_def]
  by_cases hlt : index < alloc.vec.Vec.len siblings
  · simp only [hlt, ↓reduceIte]
    have hb : index.val < siblings.val.length := by scalar_tac
    step as ⟨child, hc⟩
    have hs : previousLive nodes target (siblings.val.drop index.val) previous =
        if child = target then previous else
          previousLive nodes target (siblings.val.drop (index.val + 1))
            (if (nodes child).isSome then some child else previous) := by
      rw [List.drop_eq_getElem_cons hb]
      simp only [previousLive, hc]
    rw [hs]
    simp only [NodeId.Insts.CoreCmpPartialEqNodeId.eq, bind_ok]
    by_cases same : child = target
    · simp [same]
    · simp only [same, decide_false, Bool.false_eq_true, ↓reduceIte]
      simp [CssDom.clean.engine.arena.NodeArena.contains_key, bind_ok]
      have bound : index.val + 1 ≤ Aeneas.Std.Usize.max :=
        Nat.le_trans (by omega) siblings.property
      cases hlive : (nodes child).isSome <;> simp only [Bool.false_eq_true, ↓reduceIte, bind_ok]
      all_goals
        step as ⟨index1, hi⟩
        have nextProof := previous_loop_spec nodes target siblings
          (if (nodes child).isSome then some child else previous) index1 (by scalar_tac)
        simpa [hi, hlive] using nextProof
  · have he : index.val = siblings.val.length := by scalar_tac
    simp [hlt, he, previousLive]
termination_by siblings.val.length - index.val
decreasing_by all_goals scalar_tac

def previousModel (dom : Dom) (target : NodeId) : Option NodeId :=
  match dom.nodes target with
  | none => none
  | some node => match node.parent with
    | none => none
    | some parent => match dom.nodes parent with
      | none => none
      | some parentNode => previousLive dom.nodes target parentNode.children.val none

/-- The source returns the preceding live slot and ignores obsolete slots.
    No well-formed-tree precondition is needed. -/
theorem previous_correct (dom : Dom) (target : NodeId) :
    Dom.previous_sibling dom target ⦃ out => out = previousModel dom target ⦄ := by
  simp only [Dom.previous_sibling, CssDom.clean.engine.arena.NodeArena.get, bind_ok]
  cases hn : dom.nodes target with
  | none => simp [previousModel, hn]
  | some node =>
    cases hp : node.parent with
    | none => simp [previousModel, hn, hp]
    | some parent =>
      cases hparent : dom.nodes parent with
      | none => simp [previousModel, hn, hp, hparent]
      | some parentNode =>
        simp [previousModel, hn, hp, hparent]
        simpa using previous_loop_spec dom.nodes target parentNode.children none 0#usize (by simp)

end CssProofs.RustSibling
