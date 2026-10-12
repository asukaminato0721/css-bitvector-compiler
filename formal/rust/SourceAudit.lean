import Refinement
import CssProofs.Audit
import Lean

run_cmd do
  let env ← Lean.getEnv
  let mut checked := 0
  for (name, _) in env.constants.toList do
    if (`CssProofs.Rust).isPrefixOf name || (`CssRust).isPrefixOf name then
      for ax in ← Lean.collectAxioms name do
        unless #[`propext, `Quot.sound, `Classical.choice].contains ax do
          throwError "{name} depends on forbidden axiom {ax}"
      checked := checked + 1
  if checked == 0 then
    throwError "No source refinement declarations were checked"
  Lean.logInfo m!"Source axiom audit passed for {checked} declarations."
