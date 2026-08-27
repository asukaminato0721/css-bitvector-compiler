use crate::context::Workspace;
use std::{error::Error, process::Command};

pub fn execute(workspace: &Workspace) -> Result<(), Box<dyn Error>> {
    let available = Command::new("tectonic")
        .arg("--version")
        .output()
        .is_ok_and(|output| output.status.success());
    if !available {
        return Err(
            "Tectonic is not installed; see https://tectonic-typesetting.github.io/".into(),
        );
    }
    let mut command = Command::new("tectonic");
    command
        .current_dir(workspace.root())
        .args(["docs/design.tex", "--outdir", "docs"]);
    workspace.run("design PDF", &mut command)
}
