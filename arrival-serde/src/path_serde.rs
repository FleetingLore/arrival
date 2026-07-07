use arrival_core::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerdePath {
    pub nodes: Vec<String>,
}

impl SerdePath {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn from_path(path: &Path) -> Self {
        Self {
            nodes: path.nodes.clone(),
        }
    }

    pub fn to_path(&self) -> Path {
        Path {
            nodes: self.nodes.clone(),
        }
    }

    pub fn from_str(s: &str) -> Self {
        Self {
            nodes: s.split('/').map(|s| s.to_string()).collect(),
        }
    }

    pub fn to_string(&self) -> String {
        self.nodes.join("/")
    }

    pub fn push(&mut self, node: &str) {
        self.nodes.push(node.to_string());
    }

    pub fn matches(&self, path: &Path) -> bool {
        if self.nodes.len() != path.nodes.len() {
            return false;
        }
        self.nodes.iter().zip(&path.nodes).all(|(a, b)| a == b)
    }

    pub fn starts_with(&self, path: &Path) -> bool {
        if self.nodes.len() > path.nodes.len() {
            return false;
        }
        self.nodes.iter().zip(&path.nodes).all(|(a, b)| a == b)
    }
}

impl Default for SerdePath {
    fn default() -> Self {
        Self::new()
    }
}
