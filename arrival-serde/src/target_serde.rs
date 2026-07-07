use arrival_core::Target;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SerdeTarget {
    String(String),
    Number(i64),
    Float(f64),
    Boolean(bool),
    Null,
}

impl Target for SerdeTarget {
    fn to_string(&self) -> String {
        match self {
            SerdeTarget::String(s) => s.clone(),
            SerdeTarget::Number(n) => n.to_string(),
            SerdeTarget::Float(f) => f.to_string(),
            SerdeTarget::Boolean(b) => b.to_string(),
            SerdeTarget::Null => "null".to_string(),
        }
    }
}

impl From<String> for SerdeTarget {
    fn from(s: String) -> Self {
        SerdeTarget::String(s)
    }
}

impl From<&str> for SerdeTarget {
    fn from(s: &str) -> Self {
        SerdeTarget::String(s.to_string())
    }
}

impl From<i64> for SerdeTarget {
    fn from(n: i64) -> Self {
        SerdeTarget::Number(n)
    }
}

impl From<f64> for SerdeTarget {
    fn from(f: f64) -> Self {
        SerdeTarget::Float(f)
    }
}

impl From<bool> for SerdeTarget {
    fn from(b: bool) -> Self {
        SerdeTarget::Boolean(b)
    }
}
