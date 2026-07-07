use arrival_core::Arg;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SerdeArg {
    String(String),
    Number(i64),
    Float(f64),
    Boolean(bool),
    Null,
}

impl Arg for SerdeArg {
    fn to_string(&self) -> String {
        match self {
            SerdeArg::String(s) => s.clone(),
            SerdeArg::Number(n) => n.to_string(),
            SerdeArg::Float(f) => f.to_string(),
            SerdeArg::Boolean(b) => b.to_string(),
            SerdeArg::Null => "null".to_string(),
        }
    }
}

impl From<String> for SerdeArg {
    fn from(s: String) -> Self {
        SerdeArg::String(s)
    }
}

impl From<&str> for SerdeArg {
    fn from(s: &str) -> Self {
        SerdeArg::String(s.to_string())
    }
}

impl From<i64> for SerdeArg {
    fn from(n: i64) -> Self {
        SerdeArg::Number(n)
    }
}

impl From<f64> for SerdeArg {
    fn from(f: f64) -> Self {
        SerdeArg::Float(f)
    }
}

impl From<bool> for SerdeArg {
    fn from(b: bool) -> Self {
        SerdeArg::Boolean(b)
    }
}
