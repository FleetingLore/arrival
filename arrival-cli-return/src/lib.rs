use arrival_core::{Arg, Target, Node, NodeResult, Path};

pub struct CliReturnNode {
    path: Path,
    cli: Box<dyn CliCommand>,
}

impl CliReturnNode {
    pub fn new<C: CliCommand + 'static>(path: &str, cli: C) -> Self {
        Self {
            path: Path::from_str(path),
            cli: Box::new(cli),
        }
    }

    pub fn with_path(mut self, path: &str) -> Self {
        self.path = Path::from_str(path);
        self
    }
}

impl Node for CliReturnNode {
    fn path(&self) -> Path {
        self.path.clone()
    }

    fn process(&self, arg: &dyn Arg) -> NodeResult {
        let input = arg.to_string();
        let output = self.cli.execute(&input);
        NodeResult::Done(Box::new(CliTarget { value: output }))
    }
}

pub trait CliCommand {
    fn execute(&self, input: &str) -> String;
}

pub struct CliTarget {
    value: String,
}

impl Target for CliTarget {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

impl From<String> for CliTarget {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct EchoCommand;

    impl CliCommand for EchoCommand {
        fn execute(&self, input: &str) -> String {
            format!("echo: {}", input)
        }
    }

    #[test]
    fn test_cli_return_node() {
        let node = CliReturnNode::new("root", EchoCommand);
        let arg = TestArg { raw: "hello".to_string() };
        let result = node.process(&arg);
        match result {
            NodeResult::Done(target) => {
                assert_eq!(target.to_string(), "echo: hello");
            }
            _ => panic!("expected Done"),
        }
    }

    struct TestArg {
        raw: String,
    }

    impl Arg for TestArg {
        fn to_string(&self) -> String {
            self.raw.clone()
        }
    }
}
