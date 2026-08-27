use crate::Node;
use serde::Deserialize;
use std::{
    error::Error,
    fmt::{self, Display},
    fs,
    path::Path,
};

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "name", rename_all = "snake_case")]
pub enum TraceCommand {
    Init {
        node: Node,
    },
    Add {
        path: Vec<usize>,
        node: Node,
    },
    Remove {
        path: Vec<usize>,
    },
    Replace {
        path: Vec<usize>,
        node: Node,
    },
    ReplaceValue {
        path: Vec<usize>,
        #[serde(rename = "type")]
        value_type: Option<String>,
        key: String,
        value: Option<serde_json::Value>,
        old_value: Option<serde_json::Value>,
    },
    InsertValue {
        path: Vec<usize>,
        #[serde(rename = "type")]
        value_type: Option<String>,
        key: String,
        value: Option<serde_json::Value>,
    },
    DeleteValue {
        path: Vec<usize>,
        #[serde(rename = "type")]
        value_type: Option<String>,
        key: String,
        old_value: Option<serde_json::Value>,
    },
    Recalculate,
}

impl TraceCommand {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Init { .. } => "init",
            Self::Add { .. } => "add",
            Self::Remove { .. } => "remove",
            Self::Replace { .. } => "replace",
            Self::ReplaceValue { .. } => "replace_value",
            Self::InsertValue { .. } => "insert_value",
            Self::DeleteValue { .. } => "delete_value",
            Self::Recalculate => "recalculate",
        }
    }
}

#[derive(Debug, Clone)]
pub struct TraceFrame {
    pub frame_id: usize,
    pub command: TraceCommand,
}

#[derive(Debug, Clone, Default)]
pub struct Trace {
    pub frames: Vec<TraceFrame>,
}

impl Trace {
    pub fn parse(path: &Path) -> Result<Self, RunError> {
        let content = fs::read_to_string(path)
            .map_err(|error| RunError::new(format!("cannot read {}: {error}", path.display())))?;
        let mut frames = Vec::new();
        for (frame_id, line) in content.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            let header: TraceHeader = serde_json::from_str(line).map_err(|error| {
                RunError::at(frame_id, format!("invalid command header: {error}"))
            })?;
            if header.name.starts_with("layout_") {
                continue;
            }
            let command = serde_json::from_str(line).map_err(|error| {
                RunError::at(
                    frame_id,
                    format!("invalid command `{}`: {error}", header.name),
                )
            })?;
            frames.push(TraceFrame { frame_id, command });
        }
        Ok(Self { frames })
    }
}

#[derive(Deserialize)]
struct TraceHeader {
    name: String,
}

#[derive(Debug, Clone)]
pub struct RunError {
    pub(crate) message: String,
}

impl RunError {
    pub(crate) fn new(message: String) -> Self {
        Self { message }
    }

    pub(crate) fn at(frame_id: usize, message: String) -> Self {
        Self::new(format!("frame {frame_id}: {message}"))
    }
}

impl Display for RunError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for RunError {}
