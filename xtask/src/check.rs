use crate::context::Workspace;
use std::error::Error;

pub fn execute(workspace: &Workspace) -> Result<(), Box<dyn Error>> {
    workspace.run(
        "cargo fmt",
        workspace.cargo().args(["fmt", "--all", "--", "--check"]),
    )?;
    workspace.run(
        "cargo clippy",
        workspace.cargo().args([
            "clippy",
            "--workspace",
            "--all-targets",
            "--all-features",
            "--",
            "-D",
            "warnings",
        ]),
    )?;
    workspace.run(
        "cargo test",
        workspace
            .cargo()
            .args(["test", "--workspace", "--all-targets"]),
    )
}
