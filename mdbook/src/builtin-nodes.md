# Built-in Nodes

Arrival provides several built-in node implementations.

## StringNode

Returns a fixed string response regardless of input.

use arrival_string::StringNode;

let node = StringNode::new("root", "Hello, world!");

## CliReturnNode

Executes a CLI command and returns its output.

use arrival_cli_return::{CliReturnNode, CliCommand};

struct MyCommand;

impl CliCommand for MyCommand {
    fn execute(&self, input: &str) -> String {
        format!("processed: {}", input)
    }
}

let node = CliReturnNode::new("root", MyCommand);

## CustomNode

Implement the Node trait for custom logic.

struct MyNode;

impl Node for MyNode {
    fn path(&self) -> Path {
        Path::from_str("my/path")
    }

    fn process(&self, arg: &dyn Arg) -> NodeResult {
        NodeResult::Done(Box::new(MyTarget))
    }
}

## SerdeNode

Nodes defined via TOML configuration. See the Configuration chapter.
