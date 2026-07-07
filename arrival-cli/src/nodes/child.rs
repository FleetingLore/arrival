use arrival_core::{Arg, Node, NodeResult, Path};

pub struct ChildNode;

impl Node for ChildNode {
    fn path(&self) -> Path {
        Path::from_str("root/child")
    }

    fn process(&self, arg: &dyn Arg) -> NodeResult {
        let s = arg.to_string();
        if s.contains("world") {
            NodeResult::Done(Box::new(crate::args::StringTarget {
                value: format!("child done: {}", s),
            }))
        } else {
            NodeResult::Next(
                Box::new(crate::args::StringArg {
                    raw: format!("child next: {}", s),
                }),
                Path::from_str("root/child"),
            )
        }
    }
}
