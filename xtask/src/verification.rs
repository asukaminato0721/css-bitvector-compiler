use crate::context::Workspace;
use std::{error::Error, process::Command};

pub fn execute(workspace: &Workspace) -> Result<(), Box<dyn Error>> {
    let available = Command::new("cargo")
        .args(["kani", "--version"])
        .current_dir(workspace.root())
        .output()
        .is_ok_and(|output| output.status.success());
    if !available {
        return Err(
            "Kani is not installed; run `cargo install --locked kani-verifier` and `cargo kani setup`"
                .into(),
        );
    }
    let mut command = workspace.cargo();
    command
        .args(["kani", "--package", "css-bitvector-compiler"])
        // Compiler wrappers such as sccache/kache cannot wrap kani-compiler.
        .env("RUSTC_WRAPPER", "")
        .env("RUSTC_WORKSPACE_WRAPPER", "");
    workspace.run("Kani proofs", &mut command)
}
