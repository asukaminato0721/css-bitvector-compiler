use crate::context::Workspace;
use std::error::Error;

pub fn execute(workspace: &Workspace) -> Result<(), Box<dyn Error>> {
    let mut command = workspace.cargo();
    command.args([
        "test",
        "--release",
        "--package",
        "css-bitvector-compiler",
        "clean::tests::checked_in_corpus_has_engine_parity",
        "--",
        "--ignored",
        "--nocapture",
    ]);
    workspace.run("full corpus parity", &mut command)
}
