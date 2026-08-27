use std::collections::HashMap;

pub mod clean;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Node {
    pub id: u64,
    #[serde(rename = "name")]
    pub tag_name: String,
    #[serde(rename = "type", default)]
    pub node_type: Option<String>,
    #[serde(default)]
    pub attributes: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub children: Vec<Node>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

pub fn attributes_to_string_map(
    attributes: &HashMap<String, serde_json::Value>,
) -> HashMap<String, String> {
    attributes
        .iter()
        .filter_map(|(name, value)| match value {
            serde_json::Value::String(value) => Some((name.to_ascii_lowercase(), value.clone())),
            serde_json::Value::Number(value) => {
                Some((name.to_ascii_lowercase(), value.to_string()))
            }
            serde_json::Value::Bool(value) => Some((name.to_ascii_lowercase(), value.to_string())),
            serde_json::Value::Null
            | serde_json::Value::Array(_)
            | serde_json::Value::Object(_) => None,
        })
        .collect()
}

#[inline(always)]
pub fn rdtsc() -> u64 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        std::arch::x86_64::_rdtsc()
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time is before the Unix epoch")
            .as_nanos() as u64
    }
}
