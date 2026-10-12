import CssProofs.Tri

namespace CssProofs

inductive Quad (n : Nat) where
  | zero | one
  | fromInput : Address n → Quad n

def Quad.eval {n : Nat} : Quad n → Input n → Bool
  | .zero, _ => false
  | .one, _ => true
  | .fromInput a, i => i a

def Quad.reads {n : Nat} : Quad n → List (Address n)
  | .zero | .one => []
  | .fromInput a => [a]

def Quad.concrete {n : Nat} (b : Bool) : Quad n := if b then .one else .zero

@[simp] theorem concrete_eval {n : Nat} (b : Bool) (i : Input n) :
    (Quad.concrete b).eval i = b := by
  cases b <;> rfl

theorem decision_preserves_value {n : Nat} (q : Quad n) (old new : Input n)
    (h : Agrees q.reads old new) : q.eval new = q.eval old := by
  cases q with
  | zero => rfl
  | one => rfl
  | fromInput a => exact h a (by simp [Quad.reads])

def specializeOr {n : Nat} (raw carry : Quad n) (old : Input n) :
    Quad n × List (Address n) :=
  (if raw.eval old then raw else carry, raw.reads)

def specializeConcrete {n : Nat} (q : Quad n) (old : Input n) :
    Quad n × List (Address n) :=
  (.concrete (q.eval old), q.reads)

theorem specialized_or_sound {n : Nat} (raw carry : Quad n) (old new : Input n)
    (h : Agrees (specializeOr raw carry old).2 old new) :
    (specializeOr raw carry old).1.eval new = (raw.eval new || carry.eval new) := by
  have hr := decision_preserves_value raw old new h
  cases he : raw.eval old <;> simp [specializeOr, he, hr]

/-- The recorded guard is sufficient, but it need not be necessary. -/
theorem identical_or_needs_no_guard {n : Nat} (q : Quad n) (old new : Input n) :
    (specializeOr q q old).1.eval new = (q.eval new || q.eval new) := by
  simp [specializeOr]

theorem concrete_accept_sound {n : Nat} (q : Quad n) (old new : Input n)
    (h : Agrees (specializeConcrete q old).2 old new) :
    (specializeConcrete q old).1.eval new = q.eval new := by
  simpa [specializeConcrete] using (decision_preserves_value q old new h).symm

/-- One compiled state. Node facts determine localMatch before input reads. -/
structure Step (n : Nat) where
  localMatch : Bool
  predecessor : Option (Address n)
  carry : Option (Address n)
  accept : Bool

def Step.raw {n : Nat} (s : Step n) : Quad n :=
  if s.localMatch then
    match s.predecessor with
    | none => .one
    | some a => .fromInput a
  else .zero

def Step.rawExpr {n : Nat} (s : Step n) : Expr n :=
  .andThen (.constant s.localMatch)
    (match s.predecessor with
    | none => .constant true
    | some a => .read a)

theorem raw_correct {n : Nat} (s : Step n) (i : Input n) :
    s.raw.eval i = s.rawExpr.eval i := by
  cases s with
  | mk localBit pred carry accept =>
    cases localBit <;> cases pred <;> rfl

/-- Accept states have no outgoing descendant edge in a valid selector. -/
def Step.expr {n : Nat} (s : Step n) : Expr n :=
  if s.accept then s.rawExpr else
    match s.carry with
    | none => s.rawExpr
    | some a => .orEager s.rawExpr (.read a)

def Step.specialize {n : Nat} (s : Step n) (old : Input n) :
    Quad n × List (Address n) :=
  if s.accept then specializeConcrete s.raw old else
    match s.carry with
    | none => (s.raw, [])
    | some a => specializeOr s.raw (.fromInput a) old

theorem quad_step_sound {n : Nat} (s : Step n) (old new : Input n)
    (h : Agrees (s.specialize old).2 old new) :
    (s.specialize old).1.eval new = s.expr.eval new := by
  by_cases ht : s.accept = true
  · simp only [Step.specialize, Step.expr, ht, if_true] at *
    exact (concrete_accept_sound _ _ _ h).trans (raw_correct s new)
  · cases hc : s.carry with
    | none => simpa [Step.specialize, Step.expr, ht, hc] using raw_correct s new
    | some a =>
      simp only [Step.specialize, Step.expr, ht, hc] at *
      have hs := specialized_or_sound s.raw (.fromInput a) old new h
      rw [raw_correct] at hs
      exact hs

def quadReads {n : Nat} (ss : List (Step n)) (old : Input n) : List (Address n) :=
  ss.flatMap (fun s => (s.specialize old).2)

/-- Reuse includes materialization with the new inputs. -/
theorem quad_program_reuse {n : Nat} (ss : List (Step n)) (old new : Input n)
    (h : Compatible (record (quadReads ss old) old) new) :
    ss.map (fun s => (s.specialize old).1.eval new) =
      ss.map (fun s => s.expr.eval new) := by
  have ha := (compatible_record_iff _ _ _).mp h
  apply List.map_congr_left
  intro s hs
  apply quad_step_sound s old new
  intro a hread
  exact ha a (List.mem_flatMap.mpr ⟨s, hs, hread⟩)

/-- The same input always satisfies its own recorded requirements. -/
theorem record_accepts_original {n : Nat} (reads : List (Address n)) (old : Input n) :
    Compatible (record reads old) old := by
  apply (compatible_record_iff _ _ _).mpr
  intro a _
  rfl

theorem quad_initial_correct {n : Nat} (ss : List (Step n)) (old : Input n) :
    ss.map (fun s => (s.specialize old).1.eval old) =
      ss.map (fun s => s.expr.eval old) :=
  quad_program_reuse ss old old (record_accepts_original _ old)

end CssProofs
