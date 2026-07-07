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
    fn provide_arg(&self) -> Box<dyn Arg>;
    fn arrive(&self, arg: &dyn Arg) -> Option<Box<dyn Target>>;
    fn next_node(&self) -> Option<&dyn Node> {
        None
    }
}

pub struct CompositeNode {
    pub name: String,
    pub children: Vec<Box<dyn Node>>,
    pub current_child: usize,
}

impl CompositeNode {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            children: Vec::new(),
            current_child: 0,
        }
    }

    pub fn add_child(&mut self, child: Box<dyn Node>) {
        self.children.push(child);
    }
}

impl Node for CompositeNode {
    fn name(&self) -> &str {
        &self.name
    }

    fn provide_arg(&self) -> Box<dyn Arg> {
        if let Some(child) = self.children.get(self.current_child) {
            child.provide_arg()
        } else {
            Box::new(EmptyArg)
        }
    }

    fn arrive(&self, arg: &dyn Arg) -> Option<Box<dyn Target>> {
        for child in &self.children {
            if let Some(target) = child.arrive(arg) {
                return Some(target);
            }
        }
        None
    }

    fn next_node(&self) -> Option<&dyn Node> {
        self.children
            .get(self.current_child)
            .map(|b| &**b)
    }
}

struct EmptyArg;

impl Arg for EmptyArg {
    fn to_string(&self) -> String {
        String::new()
    }
}

pub struct Runtime<'a> {
    pub entry_node: &'a dyn Node,
    pub path: Path,
    pub current_depth: usize,
    pub max_depth: usize,
}

impl<'a> Runtime<'a> {
    pub fn new(entry_node: &'a dyn Node, max_depth: usize) -> Self {
        Self {
            entry_node,
            path: Path::new(),
            current_depth: 0,
            max_depth,
        }
    }

    pub fn run(&mut self, initial_arg: &dyn Arg) -> Option<Box<dyn Target>> {
        let mut current_node = self.entry_node;

        while self.current_depth < self.max_depth {
            self.path.push(current_node.name());

            let arg = current_node.provide_arg();
            if let Some(target) = current_node.arrive(&*arg) {
                return Some(target);
            }

            match current_node.next_node() {
                Some(next) => {
                    current_node = next;
                    self.current_depth += 1;
                }
                None => break,
            }
        }

        None
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn reset(&mut self) {
        self.path = Path::new();
        self.current_depth = 0;
    }
}
