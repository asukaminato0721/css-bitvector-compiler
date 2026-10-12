import Refinement
import MarkRefinement
import GuardRefinement
import SiblingRefinement
import CssProofs.Audit
import Lean

run_cmd do
  let env ← Lean.getEnv
  let names := env.constants.toList.map Prod.fst |>.filter fun declName =>
    (`CssProofs).isPrefixOf declName || (`CssDom).isPrefixOf declName
  names.forM fun declName => do
    let axioms ← Lean.collectAxioms declName
    axioms.forM fun ax => do
      unless #[`propext, `Quot.sound, `Classical.choice].contains ax do
        throwError "{declName} depends on forbidden axiom {ax}"
  if names.isEmpty then
    throwError "No source refinement declarations were checked"
  Lean.logInfo m!"Source axiom audit passed for {names.length} declarations."
