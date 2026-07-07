use arrival_core::{Runtime, Path, Node};
use serde::{Deserialize, Serialize};
use crate::{SerdePath, SerdeNode, SerdeArg};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerdeRuntime {
    pub nodes: Vec<SerdeNode>,
}

impl SerdeRuntime {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, node: SerdeNode) {
        self.nodes.push(node);
    }

    pub fn to_runtime(&self) -> Runtime {
        let mut runtime = Runtime::new();
        for node in &self.nodes {
            runtime.add_node(Box::new(node.clone()));
        }
        runtime
    }

    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string(self)
    }

    pub fn from_toml(s: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(s)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }
}

impl Default for SerdeRuntime {
    fn default() -> Self {
        Self::new()
    }
}
