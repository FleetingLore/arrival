pub trait Arg {
    fn to_string(&self) -> String;
}

pub trait Target {
    fn to_string(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct Path {
    pub nodes: Vec<String>,
}

impl Path {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn push(&mut self, node_name: &str) {
        self.nodes.push(node_name.to_string());
    }
}

pub trait Node {
    fn name(&self) -> &str;
    fn process(&self, arg: &dyn Arg) -> NodeResult;
}

pub enum NodeResult {
    Next(Box<dyn Arg>),
    Done(Box<dyn Target>),
}

pub struct Registry {
    nodes: std::collections::HashMap<String, Box<dyn Node>>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            nodes: std::collections::HashMap::new(),
        }
    }

    pub fn register(&mut self, node: Box<dyn Node>) {
        let name = node.name().to_string();
        self.nodes.insert(name, node);
    }

    pub fn get(&self, name: &str) -> Option<&dyn Node> {
        self.nodes.get(name).map(|b| &**b)
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Runtime<'a> {
    registry: &'a Registry,
    path: Path,
}

impl<'a> Runtime<'a> {
    pub fn new(registry: &'a Registry) -> Self {
        Self {
            registry,
            path: Path::new(),
        }
    }

    pub fn run(&mut self, initial_arg: Box<dyn Arg>) -> Option<Box<dyn Target>> {
        let mut current_arg = initial_arg;
        let mut current_node_name = "root".to_string();

        loop {
            let node_name = current_node_name.clone();
            self.path.push(&node_name);

            let node = match self.registry.get(&node_name) {
                Some(n) => n,
                None => return None,
            };

            match node.process(&*current_arg) {
                NodeResult::Done(target) => return Some(target),
                NodeResult::Next(next_arg) => {
                    current_arg = next_arg;
                    current_node_name = self.next_node_name(&*current_arg).to_string();
                }
            }
        }
    }

    fn next_node_name(&self, arg: &dyn Arg) -> &str {
        if arg.to_string().contains("child") {
            "child"
        } else {
            "root"
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn reset(&mut self) {
        self.path = Path::new();
    }
}
