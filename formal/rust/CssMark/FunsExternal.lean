module
public import CssMark.Types
@[expose] public section
open Aeneas Aeneas.Std Result
open CssDom
noncomputable section
set_option linter.unusedVariables false

def core.borrow.Borrow.Blanket.borrow {T : Type} (x : T) : Result T := ok x

-- Hash values are unobservable in the extensional map model.
def U64.Insts.CoreHashHash.hash {H : Type} (h : core.hash.Hasher H)
    (x : Aeneas.Std.U64) (state : H) : Result H := ok state

/-- Only equal key types are used by `Dom::mark`. Heterogeneous lookup is not
    specified. The backward function writes exactly the borrowed entry. -/
def std.collections.hash.map.HashMap.get_mut
    {K V S A Q H : Type} (eqK : core.cmp.Eq K) (hashK : core.hash.Hash K)
    (build : core.hash.BuildHasher S H) (borrow : core.borrow.Borrow K Q)
    (hashQ : core.hash.Hash Q) (eqQ : core.cmp.Eq Q)
    (map : std.collections.hash.map.HashMap K V S A) (q : Q) :
    Result (Option V × (Option V → std.collections.hash.map.HashMap K V S A)) := by
  classical
  exact if h : Q = K then
    let key : K := h ▸ q
    ok (map key, fun value => Function.update map key value)
  else fail .panic

def std.hash.random.DefaultHasher.Insts.CoreHashHasher.finish
    (_ : std.hash.random.DefaultHasher) : Result Aeneas.Std.U64 := ok 0#u64

def std.hash.random.DefaultHasher.Insts.CoreHashHasher.write
    (_ : std.hash.random.DefaultHasher) (_ : Slice Aeneas.Std.U8) :
    Result std.hash.random.DefaultHasher := ok ()

def std.hash.random.RandomState.Insts.CoreHashBuildHasherDefaultHasher.build_hasher
    (_ : std.hash.random.RandomState) : Result std.hash.random.DefaultHasher := ok ()
