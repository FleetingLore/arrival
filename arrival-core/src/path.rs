#[derive(Debug, Clone)]
pub struct Path {
    pub nodes: Vec<String>,
}

impl Path {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn from_str(s: &str) -> Self {
        Self {
            nodes: s.split('/').map(|s| s.to_string()).collect(),
        }
    }

    pub fn push(&mut self, node_name: &str) {
        self.nodes.push(node_name.to_string());
    }

    pub fn to_string(&self) -> String {
        self.nodes.join("/")
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

impl Default for Path {
    fn default() -> Self {
        Self::new()
    }
}
