import ArenaRefinement

open Aeneas Aeneas.Std Result
open CssDom.clean.compiler CssDom.clean.engine
open CssProofs.RustMark CssProofs.RustArena

namespace CssProofs.RustRemove


/-- Child slots, including obsolete slots, refer to newer instances. -/
def Forward (dom : Dom) : Prop :=
  ∀ id node child, dom.nodes id = some node → child ∈ node.children.val → id.val < child.val

noncomputable def eraseNode (dom : Dom) (id : NodeId) (node : DomNode) : Dom := by
  classical
  exact { dom with nodes := Function.update dom.nodes id none
                   trace_nodes := if dom.trace_nodes node.trace_id = some id
                     then Function.update dom.trace_nodes node.trace_id none else dom.trace_nodes }

theorem option_eq (a b : Option NodeId) :
    core.option.Option.Insts.CoreCmpPartialEqOption.eq
      (core.cmp.PartialEqShared NodeId.Insts.CoreCmpPartialEqNodeId) a b = ok (decide (a = b)) := by
  cases a <;> cases b <;>
    simp [core.option.Option.Insts.CoreCmpPartialEqOption.eq, NodeId.Insts.CoreCmpPartialEqNodeId.eq]

theorem remove_step (dom : Dom) (id : NodeId) :
    Dom.remove_subtree dom id =
      match dom.nodes id with
      | none => ok dom
      | some node => Dom.remove_subtree_loop node.children (eraseNode dom id node) := by
  classical
  rw [Dom.remove_subtree.eq_def]
  simp only [CssDom.clean.engine.trace_index.TraceIndex.remove, CssDom.clean.engine.arena.NodeArena.remove, CssDom.clean.engine.trace_index.TraceIndex.get,
    bind_ok]
  cases hn : dom.nodes id with
  | none =>
    simp
    congr 1
    funext key
    simp only [Function.update_apply]
    split <;> simp_all
  | some node =>
    simp only [option_eq, alloc.vec.IntoIteratorVec.into_iter, bind_ok]
    by_cases ht : dom.trace_nodes node.trace_id = some id <;>
      simp [ht, eraseNode]

/-- Removal never changes surviving node records. -/
def Submap (after before : Dom) : Prop :=
  ∀ id node, after.nodes id = some node → before.nodes id = some node

theorem submap_refl (dom : Dom) : Submap dom dom := fun _ _ h => h

theorem submap_trans {a b c : Dom} (hab : Submap a b) (hbc : Submap b c) : Submap a c :=
  fun id node h => hbc id node (hab id node h)

theorem erase_submap (dom : Dom) (id : NodeId) (node : DomNode) :
    Submap (eraseNode dom id node) dom := by
  classical
  intro key value hv
  by_cases same : key = id
  · subst key; simp [eraseNode] at hv
  · simpa only [eraseNode, Function.update_of_ne same] using hv

theorem submap_forward {after before : Dom} (hs : Submap after before) (hf : Forward before) :
    Forward after := fun id node child hn hc => hf id node child (hs id node hn) hc

noncomputable def removeFuel : Nat → Dom → NodeId → Dom
  | 0, dom, _ => dom
  | fuel + 1, dom, id => match dom.nodes id with
    | none => dom
    | some node => node.children.val.foldl (removeFuel fuel) (eraseNode dom id node)

theorem fold_submap (f : Dom → NodeId → Dom) (hf : ∀ dom id, Submap (f dom id) dom)
    (ids : List NodeId) (dom : Dom) : Submap (ids.foldl f dom) dom := by
  induction ids generalizing dom with
  | nil => exact submap_refl dom
  | cons id rest ih => exact submap_trans (ih (f dom id)) (hf dom id)

theorem remove_fuel_submap (fuel : Nat) (dom : Dom) (id : NodeId) :
    Submap (removeFuel fuel dom id) dom := by
  induction fuel generalizing dom id with
  | zero => exact submap_refl dom
  | succ fuel ih =>
    simp only [removeFuel]
    split
    · exact submap_refl dom
    · exact submap_trans (fold_submap (removeFuel fuel) ih _ _) (erase_submap dom id _)

-- Permit tactic unification to unfold the library iterator type alias.
set_option backward.isDefEq.respectTransparency false in
theorem remove_loop_refines (fuel : Nat) (ids : List NodeId)
    (iter : alloc.vec.Vec NodeId) (hiter : iter.val = ids)
    (dom : Dom) (hf : Forward dom)
    (step : ∀ state id, Forward state → id ∈ ids →
      Dom.remove_subtree state id = ok (removeFuel fuel state id)) :
    Dom.remove_subtree_loop iter dom = ok (ids.foldl (removeFuel fuel) dom) := by
  induction ids generalizing iter dom with
  | nil =>
    rw [Dom.remove_subtree_loop.eq_def]
    unfold alloc.vec.into_iter.IteratorIntoIter.next
    split
    · simp
    · rename_i head tail he
      have : ([] : List NodeId) = head :: tail := hiter.symm.trans he
      contradiction
  | cons id rest ih =>
    rw [Dom.remove_subtree_loop.eq_def]
    unfold alloc.vec.into_iter.IteratorIntoIter.next
    split
    · rename_i he
      have : id :: rest = ([] : List NodeId) := hiter.symm.trans he
      contradiction
    · rename_i head tail he
      have eqs := List.cons.inj (hiter.symm.trans he)
      obtain ⟨rfl, rfl⟩ := eqs
      simp only [bind_ok]
      have hrest : rest.length ≤ Aeneas.Std.Usize.max := by
        have hsize := iter.property
        rw [hiter] at hsize
        simp only [List.length_cons] at hsize
        omega
      change (do
        let state ← Dom.remove_subtree dom id
        Dom.remove_subtree_loop (alloc.vec.Vec.from rest hrest) state) =
          ok ((id :: rest).foldl (removeFuel fuel) dom)
      rw [step dom id hf (by simp)]
      simp only [bind_ok, List.foldl_cons]
      apply ih _ (by simp) _ (submap_forward (remove_fuel_submap fuel dom id) hf)
      intro state child hstate hchild
      exact step state child hstate (by simp [hchild])

/-- Every recursive descent increases the instance ID. The finite scalar
    range supplies a termination measure even when child slots are obsolete. -/
theorem remove_refines (fuel : Nat) (dom : Dom) (id : NodeId)
    (hf : Forward dom) (enough : Aeneas.Std.U64.max - id.val < fuel) :
    Dom.remove_subtree dom id = ok (removeFuel fuel dom id) := by
  induction fuel generalizing dom id with
  | zero => omega
  | succ fuel ih =>
    rw [remove_step]
    cases hn : dom.nodes id with
    | none => simp [removeFuel, hn]
    | some node =>
      simp only [removeFuel, hn]
      apply remove_loop_refines fuel node.children.val node.children rfl
        (eraseNode dom id node) (submap_forward (erase_submap dom id node) hf)
      intro state child hstate hchild
      apply ih state child hstate
      have newer := hf id node child hn hchild
      scalar_tac

theorem remove_correct (dom : Dom) (id : NodeId) (hf : Forward dom) :
    ∃ out, Dom.remove_subtree dom id = ok out ∧ Submap out dom ∧ Forward out := by
  let fuel := Aeneas.Std.U64.max - id.val + 1
  refine ⟨removeFuel fuel dom id, remove_refines fuel dom id hf (by omega),
    remove_fuel_submap fuel dom id, ?_⟩
  exact submap_forward (remove_fuel_submap fuel dom id) hf

theorem remove_missing (dom : Dom) (id : NodeId) (h : dom.nodes id = none) :
    Dom.remove_subtree dom id = ok dom := by
  simp [remove_step, h]

theorem erase_keeps_replacement (dom : Dom) (id replacement : NodeId) (node : DomNode)
    (hlookup : dom.trace_nodes node.trace_id = some replacement) (hne : replacement ≠ id) :
    (eraseNode dom id node).trace_nodes node.trace_id = some replacement := by
  classical
  simp [eraseNode, hlookup, hne]

theorem fold_observe {α : Type} (f : Dom → NodeId → Dom) (observe : Dom → α)
    (hf : ∀ dom id, observe (f dom id) = observe dom) (ids : List NodeId) (dom : Dom) :
    observe (ids.foldl f dom) = observe dom := by
  induction ids generalizing dom with
  | nil => rfl
  | cons id rest ih => exact (ih (f dom id)).trans (hf dom id)

theorem remove_fuel_metadata (fuel : Nat) (dom : Dom) (id : NodeId) :
    ((removeFuel fuel dom id).root, (removeFuel fuel dom id).next_instance) =
      (dom.root, dom.next_instance) := by
  induction fuel generalizing dom id with
  | zero => rfl
  | succ fuel ih =>
    simp only [removeFuel]
    split
    · rfl
    · exact fold_observe (removeFuel fuel) (fun state => (state.root, state.next_instance)) ih _ _

theorem remove_fuel_absent (fuel : Nat) (dom : Dom) (id : NodeId) :
    (removeFuel (fuel + 1) dom id).nodes id = none := by
  classical
  cases hn : dom.nodes id with
  | none => simp [removeFuel, hn]
  | some node =>
    simp only [removeFuel, hn]
    have hs := fold_submap (removeFuel fuel) (remove_fuel_submap fuel)
      node.children.val (eraseNode dom id node)
    cases hout : (node.children.val.foldl (removeFuel fuel) (eraseNode dom id node)).nodes id with
    | none => rfl
    | some value =>
      have := hs id value hout
      simp [eraseNode] at this

/-- Deletion terminates, removes the target, and preserves the ID invariants.
    A missing obsolete slot is a no-op, including its trace-ID mapping. -/
theorem remove_invariants (dom : Dom) (id : NodeId)
    (hw : Forward dom) (hf : Fresh dom) (hd : Descending dom) :
    ∃ out, Dom.remove_subtree dom id = ok out ∧ out.nodes id = none ∧
      Submap out dom ∧ out.next_instance = dom.next_instance ∧
      Forward out ∧ Fresh out ∧ Descending out := by
  let fuel := Aeneas.Std.U64.max - id.val
  let out := removeFuel (fuel + 1) dom id
  have hs : Submap out dom := remove_fuel_submap _ _ _
  have hc : out.next_instance = dom.next_instance :=
    congrArg Prod.snd (remove_fuel_metadata _ _ _)
  refine ⟨out, remove_refines _ _ _ hw (by omega), remove_fuel_absent _ _ _, hs, hc,
    submap_forward hs hw, ?_, ?_⟩
  · intro key node hn
    rw [hc]
    exact hf key node (hs key node hn)
  · intro key node parent hn hp
    exact hd key node parent (hs key node hn) hp

/-- A binding to a surviving instance remains unchanged. -/
def KeepsBindings (after before : Dom) : Prop :=
  ∀ key target node, after.nodes target = some node →
    before.trace_nodes key = some target → after.trace_nodes key = some target

theorem erase_keeps_bindings (dom : Dom) (id : NodeId) (node : DomNode) :
    KeepsBindings (eraseNode dom id node) dom := by
  classical
  intro key target value halive hlookup
  by_cases same : key = node.trace_id
  · subst key
    by_cases ht : dom.trace_nodes node.trace_id = some id
    · have eq : target = id := Option.some.inj (hlookup.symm.trans ht)
      subst target
      simp [eraseNode] at halive
    · simpa [eraseNode, ht] using hlookup
  · by_cases ht : dom.trace_nodes node.trace_id = some id <;>
      simpa [eraseNode, ht, Function.update_of_ne same] using hlookup

theorem keeps_bindings_trans {a b c : Dom} (hs : Submap a b)
    (hab : KeepsBindings a b) (hbc : KeepsBindings b c) : KeepsBindings a c := by
  intro key target node hn ht
  exact hab key target node hn (hbc key target node (hs target node hn) ht)

theorem fold_keeps_bindings (f : Dom → NodeId → Dom)
    (hs : ∀ dom id, Submap (f dom id) dom)
    (hb : ∀ dom id, KeepsBindings (f dom id) dom)
    (ids : List NodeId) (dom : Dom) : KeepsBindings (ids.foldl f dom) dom := by
  induction ids generalizing dom with
  | nil => exact fun _ _ _ _ h => h
  | cons id rest ih =>
    exact keeps_bindings_trans (fold_submap f hs rest (f dom id)) (ih (f dom id)) (hb dom id)

theorem remove_fuel_keeps_bindings (fuel : Nat) (dom : Dom) (id : NodeId) :
    KeepsBindings (removeFuel fuel dom id) dom := by
  induction fuel generalizing dom id with
  | zero => exact fun _ _ _ _ h => h
  | succ fuel ih =>
    simp only [removeFuel]
    split
    · exact fun _ _ _ _ h => h
    · exact keeps_bindings_trans
        (fold_submap (removeFuel fuel) (remove_fuel_submap fuel) _ _)
        (fold_keeps_bindings (removeFuel fuel) (remove_fuel_submap fuel) ih _ _)
        (erase_keeps_bindings dom id _)

/-- Recursive removal cannot clear a trace binding to a surviving replacement. -/
theorem remove_keeps_bindings (dom : Dom) (id : NodeId) (hf : Forward dom) :
    ∃ out, Dom.remove_subtree dom id = ok out ∧ KeepsBindings out dom := by
  let fuel := Aeneas.Std.U64.max - id.val + 1
  exact ⟨removeFuel fuel dom id, remove_refines fuel dom id hf (by omega),
    remove_fuel_keeps_bindings fuel dom id⟩

end CssProofs.RustRemove
