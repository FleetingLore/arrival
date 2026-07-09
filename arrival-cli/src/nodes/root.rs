use arrival_core::{Arg, Node, NodeResult, Trace};

pub struct RootNode;

impl Node for RootNode {
    fn path(&self) -> Trace {
        Trace::from_str("root")
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
                Trace::from_str("root/child"),
            )
        } else {
            NodeResult::Next(
                Box::new(crate::args::StringArg {
                    raw: format!("root next: {}", s),
                }),
                Trace::from_str("root"),
            )
        }
    }
}
