use crate::context::{Workspace, value_after};
use std::error::Error;

pub fn execute(workspace: &Workspace, arguments: &[String]) -> Result<(), Box<dyn Error>> {
    let site = value_after(arguments, "--site").ok_or("benchmark requires --site <name>")?;
    let targets = arguments
        .iter()
        .rfind(|argument| !argument.starts_with('-') && argument.as_str() != site)
        .map(String::as_str)
        .unwrap_or("bit,tri,rec_tri");
    let mut command = workspace.cargo();
    command.args([
        "run",
        "--quiet",
        "--release",
        "--package",
        "css-bitvector-compiler",
        "--bin",
        "benchmark",
        "--",
        "--site",
        site,
        targets,
    ]);
    workspace.run(&format!("benchmark {site}"), &mut command)
}
