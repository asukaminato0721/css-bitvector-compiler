import Std

namespace CssProofs.Selectors

inductive Combinator where
  | descendant | child | adjacent

/-- Predicates identify compounds. Their internal CSS semantics are parameters. -/
inductive Selector where
  | atom : Nat → Selector
  | extend : Selector → Combinator → Nat → Selector

/-- A finite unfolding of parent and previous-sibling links. -/
inductive Context where
  | empty
  | node : (Nat → Bool) → Context → Context → Context

def ancestorAny (p : Context → Bool) : Context → Bool
  | .empty => false
  | .node facts parent sibling => p (.node facts parent sibling) || ancestorAny p parent

/-- Direct selector interpretation. Descendants search the ancestor chain. -/
def oracle : Selector → Context → Bool
  | .atom p, .node facts _ _ => facts p
  | .extend s .child p, .node facts parent _ => facts p && oracle s parent
  | .extend s .adjacent p, .node facts _ sibling => facts p && oracle s sibling
  | .extend s .descendant p, .node facts parent _ => facts p && ancestorAny (oracle s) parent
  | _, .empty => false

/-- Each prefix has a local bit and an inclusive descendant carry bit. -/
def transition (facts : Nat → Bool) (parent sibling : Selector → Bool × Bool) :
    Selector → Bool
  | .atom p => facts p
  | .extend s .child p => facts p && (parent s).1
  | .extend s .adjacent p => facts p && (sibling s).1
  | .extend s .descendant p => facts p && (parent s).2

/-- A state uses the carry bit only if its outgoing edge is descendant. -/
def compiled : Context → Selector → Bool × Bool
  | .empty, _ => (false, false)
  | .node facts parent sibling, s =>
    let raw := transition facts (compiled parent) (compiled sibling) s
    (raw, raw || (compiled parent s).2)

/-- No bound on selector length, ancestor depth, or sibling depth is required. -/
theorem compiled_correct (ctx : Context) : ∀ s : Selector,
    (compiled ctx s).1 = oracle s ctx ∧
    (compiled ctx s).2 = ancestorAny (oracle s) ctx := by
  induction ctx with
  | empty =>
    intro s
    cases s <;> simp [compiled, oracle, ancestorAny]
  | node facts parent sibling ihp ihs =>
    intro s
    have hraw : transition facts (compiled parent) (compiled sibling) s =
        oracle s (.node facts parent sibling) := by
      cases s with
      | atom p => rfl
      | extend s comb p =>
        cases comb <;> simp [transition, oracle, (ihp s).1, (ihp s).2, (ihs s).1]
    constructor
    · exact hraw
    · change (transition facts (compiled parent) (compiled sibling) s ||
        (compiled parent s).2) = _
      rw [hraw, (ihp s).2]
      rfl

/-- Accept states observe the local bit, so ancestor matches cannot leak. -/
theorem accept_correct (ctx : Context) (s : Selector) :
    (compiled ctx s).1 = oracle s ctx := (compiled_correct ctx s).1

end CssProofs.Selectors
