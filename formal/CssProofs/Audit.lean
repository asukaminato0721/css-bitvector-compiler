import CssProofs.Requirements
import CssProofs.Tri
import CssProofs.Quad
import CssProofs.Selectors
import Lean

-- Reject proof gaps and extra axioms in every project declaration.
run_cmd do
  let env ← Lean.getEnv
  let mut checked := 0
  for (name, _) in env.constants.toList do
    if (`CssProofs).isPrefixOf name then
      let axioms ← Lean.collectAxioms name
      for ax in axioms do
        unless #[`propext, `Quot.sound, `Classical.choice].contains ax do
          throwError "{name} depends on forbidden axiom {ax}"
      checked := checked + 1
  if checked == 0 then
    throwError "No CssProofs declarations were checked"
  Lean.logInfo m!"Axiom audit passed for {checked} declarations."
