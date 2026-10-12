import MarkRefinement

open Aeneas Aeneas.Std Result
open CssDom.clean.compiler CssDom.clean.engine
open CssProofs.RustMark

namespace CssProofs.RustArena

/-- Every allocated key is below the next unused instance ID. -/
def Fresh (dom : Dom) : Prop :=
  ∀ id node, dom.nodes id = some node → id.val < dom.next_instance.val

/-- A new parent is absent or already allocated. -/
def ParentLive (dom : Dom) (node : DomNode) : Prop :=
  ∀ parent, node.parent = some parent → ∃ value, dom.nodes parent = some value

noncomputable def insertModel (dom : Dom) (node : DomNode) (next : Aeneas.Std.U64) : Dom := by
  classical
  exact { dom with
    nodes := Function.update dom.nodes dom.next_instance (some node)
    trace_nodes := Function.update dom.trace_nodes node.trace_id (some dom.next_instance)
    next_instance := next }

theorem insert_step (dom : Dom) (node : DomNode) :
    Dom.insert_instance dom node =
      match Aeneas.Std.U64.checked_add dom.next_instance 1#u64 with
      | none => ok (none, dom)
      | some next => ok (some dom.next_instance, insertModel dom node next) := by
  classical
  simp only [Dom.insert_instance]
  cases hc : Aeneas.Std.U64.checked_add dom.next_instance 1#u64 <;>
    simp [lift, std.collections.hash.map.HashMap.insert, insertModel]
  constructor <;> funext key <;> simp only [Function.update_apply] <;> split <;> simp_all

theorem insert_fresh (dom : Dom) (node : DomNode) (next : Aeneas.Std.U64)
    (hf : Fresh dom) (hn : next.val = dom.next_instance.val + 1) :
    Fresh (insertModel dom node next) := by
  classical
  intro id value hv
  by_cases same : id = dom.next_instance
  · subst id
    simp only [insertModel]
    omega
  · simp only [insertModel, Function.update_of_ne same] at hv
    have := hf id value hv
    simp only [insertModel]
    omega

theorem insert_descending (dom : Dom) (node : DomNode) (next : Aeneas.Std.U64)
    (hf : Fresh dom) (hd : Descending dom) (hp : ParentLive dom node) :
    Descending (insertModel dom node next) := by
  classical
  intro id value parent hv hparent
  by_cases same : id = dom.next_instance
  · subst id
    simp only [insertModel, Function.update_self, Option.some.injEq] at hv
    subst value
    obtain ⟨parentNode, hlookup⟩ := hp parent hparent
    exact hf parent parentNode hlookup
  · simp only [insertModel, Function.update_of_ne same] at hv
    exact hd id value parent hv hparent

/-- Allocation either succeeds with a fresh key or reports exhaustion.
    The source does not overwrite a live node. -/
theorem insert_correct (dom : Dom) (node : DomNode)
    (hf : Fresh dom) (hd : Descending dom) (hp : ParentLive dom node) :
    (dom.next_instance.val = Aeneas.Std.U64.max ∧
      Dom.insert_instance dom node = ok (none, dom)) ∨
    ∃ next, next.val = dom.next_instance.val + 1 ∧
      Dom.insert_instance dom node = ok (some dom.next_instance, insertModel dom node next) ∧
      dom.nodes dom.next_instance = none ∧
      Fresh (insertModel dom node next) ∧ Descending (insertModel dom node next) := by
  have spec := Aeneas.Std.U64.checked_add_bv_spec dom.next_instance 1#u64
  cases hc : Aeneas.Std.U64.checked_add dom.next_instance 1#u64 with
  | none =>
    simp only [hc] at spec
    left
    refine ⟨?_, ?_⟩
    · scalar_tac
    · simp [insert_step, hc]
  | some next =>
    simp only [hc] at spec
    have hn : next.val = dom.next_instance.val + 1 := by scalar_tac
    right
    refine ⟨next, hn, ?_, ?_, insert_fresh dom node next hf hn,
      insert_descending dom node next hf hd hp⟩
    · simp [insert_step, hc]
    · cases h : dom.nodes dom.next_instance with
      | none => rfl
      | some old => have := hf _ old h; omega

/-- Clearing the arena retains the allocation counter, so old IDs stay obsolete. -/
theorem clear_refines (dom : Dom) :
    Dom.clear dom = ok { dom with nodes := (fun _ => none), root := none, trace_nodes := (fun _ => none) } := by
  simp [Dom.clear, std.collections.hash.map.HashMap.clear]

theorem clear_invariants (dom : Dom) :
    ∃ out, Dom.clear dom = ok out ∧ out.next_instance = dom.next_instance ∧
      Fresh out ∧ Descending out := by
  refine ⟨_, clear_refines dom, rfl, ?_, ?_⟩
  · intro id node h; contradiction
  · intro id node parent h; contradiction

end CssProofs.RustArena
