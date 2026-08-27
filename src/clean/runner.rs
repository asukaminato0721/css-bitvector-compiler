use super::{CompiledProgram, RunError, Trace};
use std::{fs, path::PathBuf};

#[derive(Debug, Clone)]
pub struct SiteInput {
    pub name: String,
    pub css_path: PathBuf,
    pub trace_path: PathBuf,
}

impl SiteInput {
    pub fn named(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            css_path: PathBuf::from(format!("css-gen-op/{name}/{name}.css")),
            trace_path: PathBuf::from(format!("css-gen-op/{name}/command.json")),
            name,
        }
    }
}

pub fn load_site(input: &SiteInput) -> Result<(CompiledProgram, Trace), RunError> {
    let css = fs::read_to_string(&input.css_path).map_err(|error| {
        RunError::new(format!("cannot read {}: {error}", input.css_path.display()))
    })?;
    let program = CompiledProgram::compile(&css)
        .map_err(|error| RunError::new(format!("{}: {error}", input.css_path.display())))?;
    let trace = Trace::parse(&input.trace_path)?;
    Ok((program, trace))
}
