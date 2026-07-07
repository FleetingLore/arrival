use arrival_core::Node;
use arrival_cli_return::{CliReturnNode, CliCommand};

pub struct EchoCommand;

impl CliCommand for EchoCommand {
    fn execute(&self, input: &str) -> String {
        format!("echo: {}", input)
    }
}

pub fn create_cli_return_nodes() -> Vec<Box<dyn Node>> {
    vec![
        Box::new(CliReturnNode::new("root", EchoCommand)),
        Box::new(CliReturnNode::new("root/child", EchoCommand)),
    ]
}
