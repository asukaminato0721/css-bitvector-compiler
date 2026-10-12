import CssProofs.Requirements

namespace CssProofs

/-- Node predicates are constants while only the input channels change. -/
inductive Expr (n : Nat) where
  | constant : Bool → Expr n
  | read : Address n → Expr n
  | andThen : Expr n → Expr n → Expr n
  | orEager : Expr n → Expr n → Expr n

def Expr.eval {n : Nat} : Expr n → Input n → Bool
  | .constant b, _ => b
  | .read a, i => i a
  | .andThen x y, i => x.eval i && y.eval i
  | .orEager x y, i => x.eval i || y.eval i

/-- The carry read occurs even when the raw transition is true. -/
def Expr.reads {n : Nat} : Expr n → Input n → List (Address n)
  | .constant _, _ => []
  | .read a, _ => [a]
  | .andThen x y, i => x.reads i ++ if x.eval i then y.reads i else []
  | .orEager x y, i => x.reads i ++ y.reads i

theorem tri_reuse {n : Nat} (e : Expr n) (old new : Input n)
    (h : Agrees (e.reads old) old new) : e.eval new = e.eval old := by
  induction e with
  | constant b => rfl
  | read a => exact h a (by simp [Expr.reads])
  | andThen x y ihx ihy =>
    have hs := (agrees_append _ _ old new).mp h
    have hx := ihx hs.1
    cases he : x.eval old with
    | false => simp [Expr.eval, hx, he]
    | true =>
      have hy := ihy (by simpa [he] using hs.2)
      simp [Expr.eval, hx, hy]
  | orEager x y ihx ihy =>
    have hs := (agrees_append _ _ old new).mp h
    simp [Expr.eval, ihx hs.1, ihy hs.2]

theorem tri_skip_sound {n : Nat} (e : Expr n) (old new : Input n)
    (h : Compatible (record (e.reads old) old) new) :
    e.eval new = e.eval old :=
  tri_reuse e old new ((compatible_record_iff _ _ _).mp h)

def programReads {n : Nat} (es : List (Expr n)) (i : Input n) : List (Address n) :=
  es.flatMap (fun e => e.reads i)

theorem tri_program_reuse {n : Nat} (es : List (Expr n)) (old new : Input n)
    (h : Compatible (record (programReads es old) old) new) :
    es.map (fun e => e.eval new) = es.map (fun e => e.eval old) := by
  have ha := (compatible_record_iff _ _ _).mp h
  apply List.map_congr_left
  intro e he
  apply tri_reuse e old new
  intro a hae
  exact ha a (List.mem_flatMap.mpr ⟨e, he, hae⟩)

/-- Recursive-tri can check the changed channel before it schedules a node. -/
theorem recursive_tri_admission {n : Nat} (e : Expr n) (old new : Input n)
    (changed : Channel)
    (hc : ∀ a ∈ e.reads old, a.1 = changed → new a = old a)
    (hu : ∀ a, a.1 ≠ changed → new a = old a) :
    e.eval new = e.eval old := by
  apply tri_reuse e old new
  intro a ha
  by_cases h : a.1 = changed
  · exact hc a ha h
  · exact hu a h

end CssProofs
