import CssProofs.Incremental
import CssProofs.Quad
import CssProofs.Selectors

namespace CssProofs.Algorithm

open Incremental

/-- The reference DOM has predicates and ordered children, but no caches. -/
inductive DOM (α : Type) where
  | nil
  | cons (eval : α → α → α) (children rest : DOM α)

inductive Output (α : Type) where
  | nil
  | cons (value : α) (children rest : Output α)
  deriving DecidableEq

def erase {α : Type} : Forest α → DOM α
  | .nil => .nil
  | .cons state children rest => .cons state.eval (erase children) (erase rest)

def observe {α : Type} : Forest α → Output α
  | .nil => .nil
  | .cons state children rest => .cons state.output (observe children) (observe rest)

/-- Full recomputation never reads a cached value or a dirty flag. -/
def reference {α : Type} (zero : α) : DOM α → α → α → Output α
  | .nil, _, _ => .nil
  | .cons eval children rest, p, s =>
    let value := eval p s
    .cons value (reference zero children value zero) (reference zero rest p value)

def load {α : Type} (zero : α) : DOM α → Forest α
  | .nil => .nil
  | .cons eval children rest =>
    .cons ⟨eval, zero, .nodeChanged, true⟩ (load zero children) (load zero rest)

@[simp] theorem erase_load {α : Type} (zero : α) (dom : DOM α) :
    erase (load zero dom) = dom := by
  induction dom <;> simp_all [load, erase]

theorem load_ready {α : Type} (zero : α) (dom : DOM α) (p s : α) :
    Ready zero (load zero dom) p s := by
  induction dom generalizing p s with
  | nil => trivial
  | cons eval children rest ihc ihr =>
    exact ⟨by simp, by simp, ihc _ _, ihr _ _⟩

theorem done_matches_reference {α : Type} (zero : α) (tree : Forest α) (p s : α)
    (hd : Done zero tree p s) : observe tree = reference zero (erase tree) p s := by
  induction tree generalizing p s with
  | nil => rfl
  | cons state children rest ihc ihr =>
    rcases hd with ⟨hv, _, _, hc, hr⟩
    simp only [observe, erase, reference]
    rw [ihc _ _ hc, ihr _ _ hr, hv]

@[simp] theorem erase_sweep {α : Type} [DecidableEq α] (zero : α)
    (tree : Forest α) (p s : α) (pc sc : Bool) :
    erase (sweep zero tree p s pc sc) = erase tree := by
  induction tree generalizing p s pc sc with
  | nil => rfl
  | cons state children rest ihc ihr =>
    simp only [sweep]
    split <;> simp only [erase, ihc, ihr]

@[simp] theorem erase_initDirty {α : Type} (tree : Forest α) :
    erase (initDirty tree) = erase tree := by
  induction tree <;> simp_all [initDirty, erase]

/-- Edits cannot inspect caches. Following predicates include updated positions. -/
inductive Edit (α : Type) where
  | facts (path : List Bool) (eval : α → α → α)
  | insert (path : List Bool) (eval : α → α → α) (children : DOM α)
      (following : (α → α → α) → α → α → α)
  | remove (path : List Bool) (following : (α → α → α) → α → α → α)

def mapFollowing {α : Type} (f : (α → α → α) → α → α → α) : DOM α → DOM α
  | .nil => .nil
  | .cons eval children rest => .cons (f eval) children (mapFollowing f rest)

def atPath {α : Type} : DOM α → List Bool → (DOM α → DOM α) → DOM α
  | dom, [], f => f dom
  | .nil, _ :: _, _ => .nil
  | .cons eval children rest, false :: path, f => .cons eval (atPath children path f) rest
  | .cons eval children rest, true :: path, f => .cons eval children (atPath rest path f)

def editReference {α : Type} (dom : DOM α) : Edit α → DOM α
  | .facts path eval => atPath dom path (fun t => match t with
      | .nil => .nil
      | .cons _ children rest => .cons eval children rest)
  | .insert path eval children f =>
      atPath dom path (fun rest => .cons eval children (mapFollowing f rest))
  | .remove path f => atPath dom path (fun t => match t with
      | .nil => .nil
      | .cons _ _ rest => mapFollowing f rest)

def mutation {α : Type} (zero : α) : Edit α → Mutation α
  | .facts path eval => .facts path eval
  | .insert path eval children f =>
      .insert path ⟨eval, zero, .nodeChanged, true⟩ (load zero children) (fun state => f state.eval)
  | .remove path f => .remove path (fun state => f state.eval)

@[simp] theorem erase_invalidate {α : Type} (tree : Forest α)
    (f : (α → α → α) → α → α → α) :
    erase (invalidateSuffix (fun state => f state.eval) tree) = mapFollowing f (erase tree) := by
  induction tree <;> simp_all [invalidateSuffix, erase, mapFollowing]

theorem erase_editAt {α : Type} (tree : Forest α) (path : List Bool)
    (f : Forest α → Forest α) (g : DOM α → DOM α)
    (h : ∀ t, erase (f t) = g (erase t)) :
    erase (editAt tree path f) = atPath (erase tree) path g := by
  induction path generalizing tree with
  | nil => cases tree <;> exact h _
  | cons direction path ih =>
    cases tree with
    | nil => rfl
    | cons state children rest => cases direction <;> simp [editAt, atPath, erase, ih]

theorem erase_editFacts {α : Type} (tree : Forest α) (path : List Bool)
    (eval : α → α → α) :
    erase (editFacts tree path eval) = editReference (erase tree) (.facts path eval) := by
  induction tree generalizing path with
  | nil => cases path <;> rfl
  | cons state children rest ihc ihr =>
    cases path with
    | nil => rfl
    | cons direction path =>
      cases direction <;> simp_all [editFacts, erase, editReference, atPath]

theorem mutation_refines_edit {α : Type} (zero : α) (tree : Forest α) (edit : Edit α) :
    erase (applyMutation tree (mutation zero edit)) = editReference (erase tree) edit := by
  cases edit with
  | facts path eval => exact erase_editFacts tree path eval
  | insert path eval children f =>
    apply erase_editAt
    intro t
    simp [insertFirst, erase]
  | remove path f =>
    apply erase_editAt
    intro t
    cases t <;> simp [removeFirst, erase]

def refresh {α : Type} [DecidableEq α] (zero : α) (tree : Forest α) : Forest α :=
  sweep zero tree zero zero false false

/-- Includes the initialized result and the result after every edit. -/
def replay {α : Type} [DecidableEq α] (zero : α) (tree : Forest α) :
    List (Edit α) → List (Output α)
  | [] => [observe tree]
  | edit :: rest => observe tree ::
      replay zero (refresh zero (applyMutation tree (mutation zero edit))) rest

def referenceTrace {α : Type} (zero : α) (dom : DOM α) : List (Edit α) → List (Output α)
  | [] => [reference zero dom zero zero]
  | edit :: rest => reference zero dom zero zero :: referenceTrace zero (editReference dom edit) rest

theorem replay_correct {α : Type} [DecidableEq α] (zero : α) (tree : Forest α)
    (edits : List (Edit α)) (hd : Done zero tree zero zero) :
    replay zero tree edits = referenceTrace zero (erase tree) edits := by
  induction edits generalizing tree with
  | nil => simp [replay, referenceTrace, done_matches_reference zero tree zero zero hd]
  | cons edit rest ih =>
    have hr := mutation_preserves_ready zero tree (mutation zero edit) zero zero
      (done_ready zero tree zero zero hd).1
    have hn := sweep_correct zero _ zero zero zero zero false false hr
      (fun _ => rfl) (fun _ => rfl)
    simp only [replay, referenceTrace, refresh]
    rw [done_matches_reference zero tree zero zero hd, ih _ hn]
    simp [mutation_refines_edit]

/-- No initial-cache correctness assumption is required. Every finite edit trace
    terminates and agrees with cache-free recomputation at every observation. -/
theorem algorithm_correct {α : Type} [DecidableEq α] (zero : α) (dom : DOM α)
    (edits : List (Edit α)) :
    replay zero (refresh zero (load zero dom)) edits = referenceTrace zero dom edits := by
  have hd := sweep_correct zero (load zero dom) zero zero zero zero false false
    (load_ready zero dom zero zero) (fun _ => rfl) (fun _ => rfl)
  simpa [refresh] using replay_correct zero (refresh zero (load zero dom)) edits hd

def acceptsReads {n : Nat} (reads : List (Address n)) (old new : Input n) : Bool :=
  reads.all (fun a => new a == old a)

theorem acceptsReads_iff {n : Nat} (reads : List (Address n)) (old new : Input n) :
    acceptsReads reads old new = true ↔ Compatible (record reads old) new := by
  rw [compatible_record_iff]
  simp [acceptsReads, Agrees, List.all_eq_true]

/-- Cache contents and their guard come from the same evaluation. -/
def triEval {n : Nat} (es : List (Expr n)) (old new : Input n) : List Bool :=
  if acceptsReads (programReads es old) old new
  then es.map (fun e => e.eval old) else es.map (fun e => e.eval new)

theorem triEval_correct {n : Nat} (es : List (Expr n)) (old new : Input n) :
    triEval es old new = es.map (fun e => e.eval new) := by
  unfold triEval
  split
  · rename_i h; exact (tri_program_reuse es old new ((acceptsReads_iff _ _ _).mp h)).symm
  · rfl

def quadEval {n : Nat} (ss : List (Step n)) (old new : Input n) : List Bool :=
  if acceptsReads (quadReads ss old) old new
  then ss.map (fun s => (s.specialize old).1.eval new)
  else ss.map (fun s => (s.specialize new).1.eval new)

theorem quadEval_correct {n : Nat} (ss : List (Step n)) (old new : Input n) :
    quadEval ss old new = ss.map (fun s => s.expr.eval new) := by
  unfold quadEval
  split
  · rename_i h; exact quad_program_reuse ss old new ((acceptsReads_iff _ _ _).mp h)
  · exact quad_initial_correct ss new

def recursiveTriEval {n : Nat} (es : List (Expr n)) (old new : Input n)
    (changed : Channel) : List Bool :=
  if acceptsReads ((programReads es old).filter (fun a => a.1 == changed)) old new
  then es.map (fun e => e.eval old) else es.map (fun e => e.eval new)

theorem recursiveTriEval_correct {n : Nat} (es : List (Expr n)) (old new : Input n)
    (changed : Channel) (otherUnchanged : ∀ a, a.1 ≠ changed → new a = old a) :
    recursiveTriEval es old new changed = es.map (fun e => e.eval new) := by
  unfold recursiveTriEval
  split
  · rename_i h
    apply (tri_program_reuse es old new ?_).symm
    apply (compatible_record_iff _ _ _).mpr
    intro a ha
    by_cases hc : a.1 = changed
    · have agree := (compatible_record_iff _ _ _).mp ((acceptsReads_iff _ _ _).mp h)
      exact agree a (by simp [List.mem_filter, ha, hc])
    · exact otherUnchanged a hc
  · rfl

/-- Node predicates are supplied by the supported CSS compound semantics. -/
inductive FactsDOM where
  | nil
  | cons (facts : Nat → Bool) (children rest : FactsDOM)

abbrev Bits := Selectors.Selector → Bool × Bool

def selectorEval (facts : Nat → Bool) (parent sibling : Bits) : Bits :=
  fun selector =>
    let raw := Selectors.transition facts parent sibling selector
    (raw, raw || (parent selector).2)

def compileDOM : FactsDOM → DOM Bits
  | .nil => .nil
  | .cons facts children rest => .cons (selectorEval facts) (compileDOM children) (compileDOM rest)

def mapOutput {α β : Type} (f : α → β) : Output α → Output β
  | .nil => .nil
  | .cons value children rest => .cons (f value) (mapOutput f children) (mapOutput f rest)

def oracleForest : FactsDOM → Selectors.Context → Selectors.Context → Output (Selectors.Selector → Bool)
  | .nil, _, _ => .nil
  | .cons facts children rest, parent, sibling =>
    let context := Selectors.Context.node facts parent sibling
    .cons (fun selector => Selectors.oracle selector context)
      (oracleForest children context .empty) (oracleForest rest parent context)

/-- The cache-free reference itself agrees with direct selector interpretation
    at every node of a finite forest, including descendant and sibling chains. -/
theorem reference_matches_selector_oracle (dom : FactsDOM) (parent sibling : Selectors.Context) :
    mapOutput (fun bits selector => (bits selector).1)
      (reference (Selectors.compiled .empty) (compileDOM dom)
        (Selectors.compiled parent) (Selectors.compiled sibling)) =
      oracleForest dom parent sibling := by
  induction dom generalizing parent sibling with
  | nil => rfl
  | cons facts children rest ihc ihr =>
    have localCorrect :
        (fun selector => (selectorEval facts (Selectors.compiled parent)
          (Selectors.compiled sibling) selector).1) =
        (fun selector => Selectors.oracle selector (.node facts parent sibling)) := by
      funext selector
      exact Selectors.accept_correct (.node facts parent sibling) selector
    change Output.cons _ _ _ = Output.cons _ _ _
    have hc := ihc (.node facts parent sibling) .empty
    have hr := ihr parent (.node facts parent sibling)
    exact congr (congr (congrArg Output.cons localCorrect) hc) hr

/-- A correct incremental state of a compiled DOM has direct CSS-selector results. -/
theorem done_matches_selector_oracle (tree : Forest Bits) (dom : FactsDOM)
    (shape : erase tree = compileDOM dom)
    (hd : Done (Selectors.compiled .empty) tree (Selectors.compiled .empty)
      (Selectors.compiled .empty)) :
    mapOutput (fun bits selector => (bits selector).1) (observe tree) =
      oracleForest dom .empty .empty := by
  rw [done_matches_reference _ tree _ _ hd, shape]
  exact reference_matches_selector_oracle dom .empty .empty

end CssProofs.Algorithm
