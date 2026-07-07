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

    pub fn get_root(&self) -> Option<&str> {
        self.nodes.first().map(|s| s.as_str())
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
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

    pub fn get(&self, path: &Path) -> Option<&dyn Node> {
        let root = path.get_root()?;
        self.nodes.get(root).map(|b| &**b)
    }

    pub fn get_root(&self, path: &Path) -> Option<&dyn Node> {
        self.get(path)
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
    current_path: Path,
}

impl<'a> Runtime<'a> {
    pub fn new(registry: &'a Registry) -> Self {
        Self {
            registry,
            path: Path::new(),
            current_path: Path::new(),
        }
    }

    pub fn run(&mut self, initial_arg: Box<dyn Arg>) -> Option<Box<dyn Target>> {
        let mut current_arg = initial_arg;
        let mut current_path = Path::new();
        current_path.push("root");

        loop {
            self.path.push(current_path.get_root().unwrap_or("unknown"));

            let node = match self.registry.get_root(&current_path) {
                Some(n) => n,
                None => return None,
            };

            match node.process(&*current_arg) {
                NodeResult::Done(target) => return Some(target),
                NodeResult::Next(next_arg) => {
                    current_arg = next_arg;
                    let next_name = self.next_node_name(&*current_arg);
                    current_path.push(next_name);
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
        self.current_path = Path::new();
    }
}
