use crate::{Arg, Node, NodeResult, Target};
use arrival_trace::Trace;

pub struct Runtime {
    nodes: Vec<Box<dyn Node>>,
    path: Trace,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            path: Trace::new(),
        }
    }

    pub fn add_node(&mut self, node: Box<dyn Node>) {
        self.nodes.push(node);
    }

    pub fn get(&self, path: &Trace) -> Option<&dyn Node> {
        let key = path.to_string();
        self.nodes
            .iter()
            .find(|n| n.path().to_string() == key)
            .map(|n| &**n)
    }

    pub fn run(&mut self, initial_arg: Box<dyn Arg>, start_path: Trace) -> Option<Box<dyn Target>> {
        let mut current_arg = initial_arg;
        let mut current_path = start_path;

        loop {
            self.path.push_str(&current_path.to_string());

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

    pub fn path(&self) -> &Trace {
        &self.path
    }

    pub fn reset(&mut self) {
        self.path = Trace::new();
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
