import Std

namespace CssProofs

inductive Channel where
  | parent | sibling
  deriving DecidableEq

abbrev Address (n : Nat) := Channel × Fin n
abbrev Input (n : Nat) := Address n → Bool

inductive Requirement where
  | unused | zero | one
  deriving DecidableEq

def Requirement.fromBit (b : Bool) : Requirement := if b then .one else .zero

def Requirement.accepts : Requirement → Bool → Prop
  | .unused, _ => True
  | .zero, b => b = false
  | .one, b => b = true

theorem fromBit_accepts_iff (old new : Bool) :
    (Requirement.fromBit old).accepts new ↔ new = old := by
  cases old <;> simp [Requirement.fromBit, Requirement.accepts]

/-- Each read records the value from the same evaluation input. -/
def record {n : Nat} (reads : List (Address n)) (old : Input n) :
    Address n → Requirement :=
  fun a => if a ∈ reads then .fromBit (old a) else .unused

def Compatible {n : Nat} (r : Address n → Requirement) (new : Input n) : Prop :=
  ∀ a, (r a).accepts (new a)

def Agrees {n : Nat} (reads : List (Address n)) (old new : Input n) : Prop :=
  ∀ a, a ∈ reads → new a = old a

theorem compatible_record_iff {n : Nat} (reads : List (Address n)) (old new : Input n) :
    Compatible (record reads old) new ↔ Agrees reads old new := by
  constructor
  · intro h a ha
    have h' := h a
    simpa [record, ha, fromBit_accepts_iff] using h'
  · intro h a
    by_cases ha : a ∈ reads
    · simpa [record, ha, fromBit_accepts_iff] using h a ha
    · simp [record, ha, Requirement.accepts]

theorem agrees_append {n : Nat} (xs ys : List (Address n)) (old new : Input n) :
    Agrees (xs ++ ys) old new ↔ Agrees xs old new ∧ Agrees ys old new := by
  simp only [Agrees, List.mem_append]
  constructor
  · intro h
    exact ⟨fun a ha => h a (Or.inl ha), fun a ha => h a (Or.inr ha)⟩
  · rintro ⟨hx, hy⟩ a (ha | ha)
    · exact hx a ha
    · exact hy a ha

/-- A repeated read cannot conflict with an earlier read of the same input. -/
theorem record_cons {n : Nat} (a : Address n) (reads : List (Address n)) (old : Input n) :
    record (a :: reads) old =
      fun b => if b = a then .fromBit (old a) else record reads old b := by
  funext b
  by_cases h : b = a
  · subst b
    simp [record]
  · simp [record, h]

end CssProofs
