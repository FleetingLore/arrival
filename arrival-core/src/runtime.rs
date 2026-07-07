use crate::{Arg, Target, Node, NodeResult, Path};

pub struct Runtime {
    nodes: Vec<Box<dyn Node>>,
    path: Path,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            path: Path::new(),
        }
    }

    pub fn add_node(&mut self, node: Box<dyn Node>) {
        self.nodes.push(node);
    }

    pub fn get(&self, path: &Path) -> Option<&dyn Node> {
        let key = path.to_string();
        self.nodes.iter().find(|n| n.path().to_string() == key).map(|n| &**n)
    }

    pub fn run(&mut self, initial_arg: Box<dyn Arg>, start_path: Path) -> Option<Box<dyn Target>> {
        let mut current_arg = initial_arg;
        let mut current_path = start_path;

        loop {
            self.path.push(&current_path.to_string());

            let node = match self.get(&current_path) {
                Some(n) => n,
                None => return None,
            };

            match node.process(&*current_arg) {
                NodeResult::Done(target) => return Some(target),
                NodeResult::Next(next_arg, next_path) => {
                    current_arg = next_arg;
                    current_path = next_path;
                }
            }
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn reset(&mut self) {
        self.path = Path::new();
    }

    pub fn iter_nodes(&self) -> impl Iterator<Item = &dyn Node> {
        self.nodes.iter().map(|n| &**n)
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}
