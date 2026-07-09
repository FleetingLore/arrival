use arrival_core::Trace;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerdePath {
    pub nodes: Vec<String>,
}

impl SerdePath {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn from_trace(trace: &Trace) -> Self {
        Self {
            nodes: trace.segments_str(),
        }
    }

    pub fn to_trace(&self) -> Trace {
        let joined = self.nodes.join("::");
        Trace::from_str(&joined)
    }

    pub fn from_str(s: &str) -> Self {
        // SerdePath uses '/' as delimiter, Trace uses '::'
        // Keep the original behavior for SerdePath's own use
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

    pub fn matches(&self, trace: &Trace) -> bool {
        let trace_nodes = trace.segments_str();
        if self.nodes.len() != trace_nodes.len() {
            return false;
        }
        self.nodes.iter().zip(&trace_nodes).all(|(a, b)| a == b)
    }

    pub fn starts_with(&self, trace: &Trace) -> bool {
        let trace_nodes = trace.segments_str();
        if self.nodes.len() > trace_nodes.len() {
            return false;
        }
        self.nodes.iter().zip(&trace_nodes).all(|(a, b)| a == b)
    }
}

impl Default for SerdePath {
    fn default() -> Self {
        Self::new()
    }
}
