import Std

namespace CssProofs.Incremental

inductive Dirty where
  | clean | inputChanged | nodeChanged
  deriving DecidableEq

def Dirty.merge : Dirty → Dirty → Dirty
  | .nodeChanged, _ | _, .nodeChanged => .nodeChanged
  | .inputChanged, _ | _, .inputChanged => .inputChanged
  | .clean, .clean => .clean

theorem merge_preserves_node_change (d : Dirty) :
    Dirty.nodeChanged.merge d = .nodeChanged := by cases d <;> rfl

theorem merge_clean_iff (a b : Dirty) :
    a.merge b = .clean ↔ a = .clean ∧ b = .clean := by
  cases a <;> cases b <;> simp [Dirty.merge]

/-- The evaluator includes the current node facts. -/
structure State (α : Type) where
  eval : α → α → α
  output : α
  dirty : Dirty
  subtreeDirty : Bool

/-- First-child / next-sibling representation of an ordered forest. -/
inductive Forest (α : Type) where
  | nil
  | cons : State α → Forest α → Forest α → Forest α

def Quiet {α : Type} : Forest α → Prop
  | .nil => True
  | .cons state children rest =>
    state.dirty = .clean ∧ state.subtreeDirty = false ∧ Quiet children ∧ Quiet rest

/-- Clean caches use the current cached outputs of their direct providers.
    A false subtree flag certifies that no descendant has pending work. -/
def Ready {α : Type} (zero : α) : Forest α → α → α → Prop
  | .nil, _, _ => True
  | .cons state children rest, parent, sibling =>
    (state.dirty = .clean → state.output = state.eval parent sibling) ∧
    (state.subtreeDirty = false → state.dirty = .clean ∧ Quiet children) ∧
    Ready zero children state.output zero ∧ Ready zero rest parent state.output

def Done {α : Type} (zero : α) : Forest α → α → α → Prop
  | .nil, _, _ => True
  | .cons state children rest, parent, sibling =>
    state.output = state.eval parent sibling ∧
    state.dirty = .clean ∧ state.subtreeDirty = false ∧
    Done zero children state.output zero ∧ Done zero rest parent state.output

theorem ready_quiet_done {α : Type} (zero : α) (tree : Forest α) (p s : α)
    (hr : Ready zero tree p s) (hq : Quiet tree) : Done zero tree p s := by
  induction tree generalizing p s with
  | nil => trivial
  | cons state children rest ihc ihr =>
    rcases hr with ⟨hv, _, hc, ht⟩
    rcases hq with ⟨hd, hf, qc, qt⟩
    exact ⟨hv hd, hd, hf, ihc _ _ hc qc, ihr _ _ ht qt⟩

/-- Pending input changes are arguments rather than repeated writes to child
    records. The parent flag applies to every immediate child. The sibling
    flag applies only to the next sibling. -/
def sweep {α : Type} [DecidableEq α] (zero : α) :
    Forest α → α → α → Bool → Bool → Forest α
  | .nil, _, _, _, _ => .nil
  | .cons state children rest, p, s, parentChanged, siblingChanged =>
    if state.subtreeDirty || parentChanged || siblingChanged then
      let output := if state.dirty = .clean ∧ parentChanged = false ∧ siblingChanged = false
        then state.output else state.eval p s
      let changed := decide (output ≠ state.output)
      .cons { state with output, dirty := .clean, subtreeDirty := false }
        (sweep zero children output zero changed false)
        (sweep zero rest p output parentChanged changed)
    else
      .cons state children (sweep zero rest p state.output parentChanged false)

/-- A single preorder sweep terminates and repairs every pending dependency.
    It also preserves independently dirty descendants when a node output is unchanged. -/
theorem sweep_correct {α : Type} [DecidableEq α] (zero : α) (tree : Forest α)
    (oldP oldS p s : α) (pc sc : Bool)
    (hr : Ready zero tree oldP oldS)
    (hp : pc = false → p = oldP) (hs : sc = false → s = oldS) :
    Done zero (sweep zero tree p s pc sc) p s := by
  induction tree generalizing oldP oldS p s pc sc with
  | nil => trivial
  | cons state children rest ihc ihr =>
    rcases hr with ⟨hv, hf, hc, ht⟩
    by_cases visit : (state.subtreeDirty || pc || sc) = true
    · let output := if state.dirty = .clean ∧ pc = false ∧ sc = false
        then state.output else state.eval p s
      have correct : output = state.eval p s := by
        dsimp [output]
        split
        · rename_i h
          rw [hp h.2.1, hs h.2.2]
          exact hv h.1
        · rfl
      have unchanged : decide (output ≠ state.output) = false → output = state.output := by
        simp
      have childrenDone := ihc state.output zero output zero
        (decide (output ≠ state.output)) false hc unchanged (fun _ => rfl)
      have restDone := ihr oldP state.output p output pc
        (decide (output ≠ state.output)) ht hp unchanged
      simpa only [sweep, visit, ↓reduceIte, Done] using
        (show output = state.eval p s ∧ Dirty.clean = .clean ∧ false = false ∧
          Done zero (sweep zero children output zero (decide (output ≠ state.output)) false) output zero ∧
          Done zero (sweep zero rest p output pc (decide (output ≠ state.output))) p output from
          ⟨correct, rfl, rfl, childrenDone, restDone⟩)
    · have flags : state.subtreeDirty = false ∧ pc = false ∧ sc = false := by
        simpa [and_assoc] using visit
      rcases flags with ⟨hflag, hpc, hsc⟩
      rcases hf hflag with ⟨hclean, hquiet⟩
      have correct : state.output = state.eval p s := by
        rw [hp hpc, hs hsc]
        exact hv hclean
      have childrenDone := ready_quiet_done zero children state.output zero hc hquiet
      have restDone := ihr oldP state.output p state.output pc false ht hp (fun _ => rfl)
      simpa only [sweep, visit, Bool.false_eq_true, ↓reduceIte, Done] using
        (show state.output = state.eval p s ∧ state.dirty = .clean ∧ state.subtreeDirty = false ∧
          Done zero children state.output zero ∧
          Done zero (sweep zero rest p state.output pc false) p state.output from
          ⟨correct, hclean, hflag, childrenDone, restDone⟩)

/-- Updating node facts marks the node and every enclosing ancestor.
    A step to the next sibling does not make the preceding sibling dirty. -/
def editFacts {α : Type} : Forest α → List Bool → (α → α → α) → Forest α
  | .nil, _, _ => .nil
  | .cons state children rest, [], f =>
    .cons { state with eval := f, dirty := .nodeChanged, subtreeDirty := true } children rest
  | .cons state children rest, false :: path, f =>
    .cons { state with subtreeDirty := true } (editFacts children path f) rest
  | .cons state children rest, true :: path, f =>
    .cons state children (editFacts rest path f)

theorem edit_facts_preserves_ready {α : Type} (zero : α) (tree : Forest α)
    (path : List Bool) (f : α → α → α) (p s : α)
    (hr : Ready zero tree p s) : Ready zero (editFacts tree path f) p s := by
  induction tree generalizing path p s with
  | nil => trivial
  | cons state children rest ihc ihr =>
    rcases hr with ⟨hv, hf, hc, ht⟩
    cases path with
    | nil => exact ⟨by simp, by simp, hc, ht⟩
    | cons direction path =>
      cases direction with
      | false => exact ⟨hv, by simp, ihc path _ _ hc, ht⟩
      | true => exact ⟨hv, hf, hc, ihr path _ _ ht⟩

/-- Insertion and removal can change the predicate and sibling input of each
    following sibling. Their descendants retain their old cached providers. -/
def invalidateSuffix {α : Type} (facts : State α → α → α → α) : Forest α → Forest α
  | .nil => .nil
  | .cons state children rest =>
    .cons { state with eval := facts state, dirty := .nodeChanged, subtreeDirty := true }
      children (invalidateSuffix facts rest)

theorem invalidate_suffix_ready {α : Type} (zero : α) (tree : Forest α)
    (facts : State α → α → α → α) (oldP oldS p s : α)
    (hr : Ready zero tree oldP oldS) :
    Ready zero (invalidateSuffix facts tree) p s := by
  induction tree generalizing oldS s with
  | nil => trivial
  | cons state children rest _ ihr =>
    rcases hr with ⟨_, _, hc, ht⟩
    exact ⟨by simp, by simp, hc, ihr state.output state.output ht⟩

def initDirty {α : Type} : Forest α → Forest α
  | .nil => .nil
  | .cons state children rest =>
    .cons { state with dirty := .nodeChanged, subtreeDirty := true }
      (initDirty children) (initDirty rest)

theorem initDirty_ready {α : Type} (zero : α) (tree : Forest α) (p s : α) :
    Ready zero (initDirty tree) p s := by
  induction tree generalizing p s with
  | nil => trivial
  | cons state children rest ihc ihr =>
    exact ⟨by simp, by simp, ihc _ _, ihr _ _⟩

def insertFirst {α : Type} (state : State α) (children rest : Forest α)
    (facts : State α → α → α → α) : Forest α :=
  .cons { state with dirty := .nodeChanged, subtreeDirty := true }
    (initDirty children) (invalidateSuffix facts rest)

theorem insert_preserves_ready {α : Type} (zero : α) (state : State α)
    (children rest : Forest α) (facts : State α → α → α → α) (p s : α)
    (hr : Ready zero rest p s) :
    Ready zero (insertFirst state children rest facts) p s := by
  exact ⟨by simp, by simp, initDirty_ready _ _ _ _,
    invalidate_suffix_ready zero rest facts p s p state.output hr⟩

def removeFirst {α : Type} (facts : State α → α → α → α) : Forest α → Forest α
  | .nil => .nil
  | .cons _ _ rest => invalidateSuffix facts rest

theorem remove_preserves_ready {α : Type} (zero : α) (tree : Forest α)
    (facts : State α → α → α → α) (p s : α) (hr : Ready zero tree p s) :
    Ready zero (removeFirst facts tree) p s := by
  cases tree with
  | nil => trivial
  | cons state children rest =>
    exact invalidate_suffix_ready zero rest facts p state.output p s hr.2.2.2

/-- Lift a forest edit to any child/sibling path. Only enclosing ancestors
    need their subtree flags set. The edited forest keeps its external inputs. -/
def editAt {α : Type} : Forest α → List Bool → (Forest α → Forest α) → Forest α
  | tree, [], change => change tree
  | .nil, _ :: _, _ => .nil
  | .cons state children rest, false :: path, change =>
    .cons { state with subtreeDirty := true } (editAt children path change) rest
  | .cons state children rest, true :: path, change =>
    .cons state children (editAt rest path change)

theorem edit_at_preserves_ready {α : Type} (zero : α) (tree : Forest α)
    (path : List Bool) (change : Forest α → Forest α)
    (safe : ∀ t p s, Ready zero t p s → Ready zero (change t) p s)
    (p s : α) (hr : Ready zero tree p s) : Ready zero (editAt tree path change) p s := by
  induction path generalizing tree p s with
  | nil => cases tree <;> exact safe _ p s hr
  | cons direction path ih =>
    cases tree with
    | nil => trivial
    | cons state children rest =>
      rcases hr with ⟨hv, hf, hc, ht⟩
      cases direction with
      | false => exact ⟨hv, by simp, ih children _ _ hc, ht⟩
      | true => exact ⟨hv, hf, hc, ih rest _ _ ht⟩

theorem done_ready {α : Type} (zero : α) (tree : Forest α) (p s : α)
    (hd : Done zero tree p s) : Ready zero tree p s ∧ Quiet tree := by
  induction tree generalizing p s with
  | nil => exact ⟨trivial, trivial⟩
  | cons state children rest ihc ihr =>
    rcases hd with ⟨hv, hc, hf, hchildren, hrest⟩
    rcases ihc _ _ hchildren with ⟨rc, qc⟩
    rcases ihr _ _ hrest with ⟨rt, qt⟩
    exact ⟨⟨fun _ => hv, fun _ => ⟨hc, qc⟩, rc, rt⟩, hc, hf, qc, qt⟩

/-- Attribute and derived-pseudo updates can occur at any finite set of nodes. -/
def editMany {α : Type} (tree : Forest α) :
    List (List Bool × (α → α → α)) → Forest α
  | [] => tree
  | (path, facts) :: updates => editMany (editFacts tree path facts) updates

theorem edit_many_ready {α : Type} (zero : α) (tree : Forest α)
    (updates : List (List Bool × (α → α → α))) (p s : α)
    (hr : Ready zero tree p s) : Ready zero (editMany tree updates) p s := by
  induction updates generalizing tree with
  | nil => exact hr
  | cons update updates ih =>
    exact ih _ (edit_facts_preserves_ready zero tree update.1 update.2 p s hr)

theorem mutation_then_sweep {α : Type} [DecidableEq α] (zero : α)
    (tree : Forest α) (updates : List (List Bool × (α → α → α))) (p s : α)
    (hr : Ready zero tree p s) :
    Done zero (sweep zero (editMany tree updates) p s false false) p s :=
  sweep_correct zero _ p s p s false false
    (edit_many_ready zero tree updates p s hr) (fun _ => rfl) (fun _ => rfl)

end CssProofs.Incremental
