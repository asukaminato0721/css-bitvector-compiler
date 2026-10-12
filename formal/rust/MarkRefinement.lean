import CssMark.Funs
import CssProofs.Incremental

open Aeneas Aeneas.Std Result ControlFlow
open CssDom.clean.compiler CssDom.clean.engine CssDom.clean.logic

namespace CssProofs.RustMark

def merge : Dirty → Dirty → Dirty
  | .NodeChanged, _ | _, .NodeChanged => .NodeChanged
  | .InputChanged, _ | _, .InputChanged => .InputChanged
  | .Clean, .Clean => .Clean

theorem merge_eq (a b : Dirty) : Dirty.merge a b = ok (merge a b) := by
  cases a <;> cases b <;> rfl

def touch (node : DomNode) (d : Dirty) (first : Bool) : DomNode :=
  { node with state := { node.state with
      dirty := if first then merge node.state.dirty d else node.state.dirty,
      subtree_dirty := true } }

noncomputable def put (dom : Dom) (id : NodeId) (node : DomNode) : Dom := by
  classical
  exact { dom with nodes := Function.update dom.nodes id (some node) }

def Descending (dom : Dom) : Prop :=
  ∀ id node parent, dom.nodes id = some node → node.parent = some parent →
    parent.val < id.val

theorem put_preserves_descent (dom : Dom) (id : NodeId) (node : DomNode)
    (d : Dirty) (first : Bool) (hd : Descending dom) (hn : dom.nodes id = some node) :
    Descending (put dom id (touch node d first)) := by
  classical
  intro key value parent hv hp
  by_cases same : key = id
  · subst key
    simp only [put, Function.update_self, Option.some.injEq] at hv
    subst value
    exact hd id node parent hn hp
  · simp only [put, Function.update_of_ne same] at hv
    exact hd key value parent hv hp

/-- One source iteration updates the current entry and follows its parent. -/
theorem loop_step (dom : Dom) (current : Option NodeId) (d : Dirty) (first : Bool) :
    Dom.mark_loop current dom d first =
      match current with
      | none => ok dom
      | some id => match dom.nodes id with
        | none => ok dom
        | some node => Dom.mark_loop node.parent (put dom id (touch node d first)) d false := by
  classical
  rw [Dom.mark_loop.eq_def]
  cases current with
  | none => rfl
  | some id =>
    cases hn : dom.nodes id with
    | none =>
      simp [std.collections.hash.map.HashMap.get_mut, hn]
      congr 1
      funext key
      simp only [Function.update_apply]
      split <;> simp_all
    | some node =>
      cases first <;>
        simp [std.collections.hash.map.HashMap.get_mut, hn, merge_eq, touch, put]
        <;> (congr 1; congr 1; funext key; simp only [Function.update_apply]; split <;> simp_all)

noncomputable def markFuel : Nat → Dom → Option NodeId → Dirty → Bool → Dom
  | 0, dom, _, _, _ => dom
  | fuel + 1, dom, current, d, first =>
    match current with
    | none => dom
    | some id => match dom.nodes id with
      | none => dom
      | some node => markFuel fuel (put dom id (touch node d first)) node.parent d false

def Enough (fuel : Nat) : Option NodeId → Prop
  | none => True
  | some id => id.val < fuel

/-- The source loop terminates when each parent has a smaller instance ID.
    Its result is exactly the finite chain of map updates. -/
theorem loop_refines (fuel : Nat) (dom : Dom) (current : Option NodeId)
    (d : Dirty) (first : Bool) (hd : Descending dom) (enough : Enough fuel current) :
    Dom.mark_loop current dom d first = ok (markFuel fuel dom current d first) := by
  induction fuel generalizing dom current first with
  | zero =>
    cases current with
    | none =>
      rw [loop_step]
      rfl
    | some id => simp [Enough] at enough
  | succ fuel ih =>
    cases current with
    | none =>
      rw [loop_step]
      rfl
    | some id =>
      cases hn : dom.nodes id with
      | none =>
        rw [loop_step]
        simp only [hn, markFuel]
      | some node =>
        have nextEnough : Enough fuel node.parent := by
          cases hp : node.parent with
          | none => trivial
          | some parent =>
            have bound := hd id node parent hn hp
            simp only [Enough] at enough ⊢
            omega
        have next := ih (put dom id (touch node d first)) node.parent false
          (put_preserves_descent dom id node d first hd hn) nextEnough
        rw [loop_step]
        simp only [hn, markFuel]
        exact next

theorem mark_refines (dom : Dom) (id : NodeId) (d : Dirty) (hd : Descending dom) :
    Dom.mark dom id d = ok (markFuel (id.val + 1) dom (some id) d true) :=
  loop_refines (id.val + 1) dom (some id) d true hd (by simp [Enough])

/-- Remove only the two flags. Equality of these values preserves every
    other node field, including links, facts, inputs, and cached outputs. -/
def withoutFlags (node : DomNode) : DomNode :=
  { node with state := { node.state with dirty := .Clean, subtree_dirty := false } }

theorem touch_without_flags (node : DomNode) (d : Dirty) (first : Bool) :
    withoutFlags (touch node d first) = withoutFlags node := by
  rfl

theorem mark_fuel_frame (fuel : Nat) (dom : Dom) (current : Option NodeId)
    (d : Dirty) (first : Bool) :
    (markFuel fuel dom current d first).root = dom.root ∧
    (markFuel fuel dom current d first).trace_nodes = dom.trace_nodes ∧
    (markFuel fuel dom current d first).next_instance = dom.next_instance ∧
    ∀ key, ((markFuel fuel dom current d first).nodes key).map withoutFlags =
      (dom.nodes key).map withoutFlags := by
  classical
  induction fuel generalizing dom current first with
  | zero => exact ⟨rfl, rfl, rfl, fun _ => rfl⟩
  | succ fuel ih =>
    cases current with
    | none => exact ⟨rfl, rfl, rfl, fun _ => rfl⟩
    | some id =>
      cases hn : dom.nodes id with
      | none => simp [markFuel, hn]
      | some node =>
        simp only [markFuel, hn]
        rcases ih (put dom id (touch node d first)) node.parent false with ⟨hr, ht, hi, hm⟩
        refine ⟨hr, ht, hi, ?_⟩
        intro key
        rw [hm]
        by_cases same : key = id
        · subst key
          simp [put, hn, touch_without_flags]
        · simp [put, Function.update_of_ne same]

theorem mark_fuel_preserves_descent (fuel : Nat) (dom : Dom) (current : Option NodeId)
    (d : Dirty) (first : Bool) (hd : Descending dom) :
    Descending (markFuel fuel dom current d first) := by
  induction fuel generalizing dom current first with
  | zero => exact hd
  | succ fuel ih =>
    cases current with
    | none => exact hd
    | some id =>
      cases hn : dom.nodes id with
      | none => simpa [markFuel, hn] using hd
      | some node =>
        simp only [markFuel, hn]
        exact ih _ _ _ (put_preserves_descent dom id node d first hd hn)

/-- Source marking terminates, preserves all non-flag data, and preserves
    the parent-order invariant needed by the next marking operation. -/
theorem mark_preserves (dom : Dom) (id : NodeId) (d : Dirty) (hd : Descending dom) :
    ∃ out, Dom.mark dom id d = ok out ∧ Descending out ∧
      out.root = dom.root ∧ out.trace_nodes = dom.trace_nodes ∧
      out.next_instance = dom.next_instance ∧
      ∀ key, (out.nodes key).map withoutFlags = (dom.nodes key).map withoutFlags := by
  exact ⟨_, mark_refines dom id d hd,
    mark_fuel_preserves_descent _ _ _ _ _ hd,
    mark_fuel_frame _ _ _ _ _⟩

end CssProofs.RustMark
