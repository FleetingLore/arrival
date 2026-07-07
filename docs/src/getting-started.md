# Getting Started

Add Arrival to Cargo.toml:

[dependencies]
arrival-core = "0.1"

## Basic Example

use arrival_core::{Arg, Target, Node, NodeResult, Runtime, Path};

struct MyArg {
    raw: String,
}

impl Arg for MyArg {
    fn to_string(&self) -> String {
        self.raw.clone()
    }
}

struct MyTarget {
    value: String,
}

impl Target for MyTarget {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

struct RootNode;

impl Node for RootNode {
    fn path(&self) -> Path {
        Path::from_str("root")
    }

    fn process(&self, arg: &dyn Arg) -> NodeResult {
        if arg.to_string().contains("hello") {
            NodeResult::Done(Box::new(MyTarget {
                value: "hello from root".to_string(),
            }))
        } else {
            NodeResult::Next(
                Box::new(MyArg {
                    raw: format!("forwarded: {}", arg.to_string()),
                }),
                Path::from_str("root/child"),
            )
        }
    }
}

struct ChildNode;

impl Node for ChildNode {
    fn path(&self) -> Path {
        Path::from_str("root/child")
    }

    fn process(&self, arg: &dyn Arg) -> NodeResult {
        NodeResult::Done(Box::new(MyTarget {
            value: format!("child processed: {}", arg.to_string()),
        }))
    }
}

fn main() {
    let mut runtime = Runtime::new();
    runtime.add_node(Box::new(RootNode));
    runtime.add_node(Box::new(ChildNode));

    let arg = Box::new(MyArg {
        raw: "hello".to_string(),
    });

    let result = runtime.run(arg, Path::from_str("root"));
    println!("{}", result.unwrap().to_string());
}
