use std::{
    error::Error,
    path::{Path, PathBuf},
    process::{Command, Output},
};

pub struct Workspace {
    root: PathBuf,
}

impl Workspace {
    pub fn discover() -> Result<Self, Box<dyn Error>> {
        let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let root = manifest
            .parent()
            .ok_or("xtask manifest has no parent directory")?
            .to_path_buf();
        Ok(Self { root })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn cargo(&self) -> Command {
        let mut command = Command::new("cargo");
        command.current_dir(&self.root);
        command
    }

    pub fn run(&self, label: &str, command: &mut Command) -> Result<(), Box<dyn Error>> {
        println!("==> {label}");
        let status = command.status()?;
        if status.success() {
            Ok(())
        } else {
            Err(format!("{label} exited with {status}").into())
        }
    }

    pub fn capture(&self, label: &str, command: &mut Command) -> Result<Output, Box<dyn Error>> {
        let output = command.output()?;
        if output.status.success() {
            Ok(output)
        } else {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!(
                "{label} exited with {}\nstdout:\n{}\nstderr:\n{}",
                output.status, stdout, stderr
            )
            .into())
        }
    }
}

pub fn value_after<'a>(arguments: &'a [String], flag: &str) -> Option<&'a str> {
    arguments
        .windows(2)
        .find(|window| window[0] == flag)
        .map(|window| window[1].as_str())
}

pub fn has_flag(arguments: &[String], flag: &str) -> bool {
    arguments.iter().any(|argument| argument == flag)
}
