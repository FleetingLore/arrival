use arrival_core::{Arg, Node, NodeResult, Path};

pub struct RootNode;

impl Node for RootNode {
    fn path(&self) -> Path {
        Path::from_str("root")
    }

    fn process(&self, arg: &dyn Arg) -> NodeResult {
        let s = arg.to_string();
        if s.contains("hello") {
            NodeResult::Done(Box::new(crate::args::StringTarget {
                value: format!("root done: {}", s),
            }))
        } else if s.contains("child") {
            NodeResult::Next(
                Box::new(crate::args::StringArg {
                    raw: format!("root forwarded: {}", s),
                }),
                Path::from_str("root/child"),
            )
        } else {
            NodeResult::Next(
                Box::new(crate::args::StringArg {
                    raw: format!("root next: {}", s),
                }),
                Path::from_str("root"),
            )
        }
    }
}
