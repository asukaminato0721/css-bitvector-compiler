import Lake
open Lake DSL

package CssRustProofs where
  leanOptions := #[⟨`autoImplicit, false⟩, ⟨`warningAsError, true⟩]

require CssProofs from ".."
require aeneas from git "https://github.com/AeneasVerif/aeneas.git" @
  "d119a474f852781f8ae88a85424f2f8ba8407b82" / "backends/lean"

lean_lib CssLogic
lean_lib Refinement
@[default_target] lean_lib SourceAudit
