use arrival_core::{Arg, Node, NodeResult, Trace};

pub struct CustomNode {
    path_str: String,
}

impl CustomNode {
    pub fn new(path: &str) -> Self {
        Self {
            path_str: path.to_string(),
        }
    }
}

impl Node for CustomNode {
    fn path(&self) -> Trace {
        Trace::from_str(&self.path_str)
    }

    fn process(&self, arg: &dyn Arg) -> NodeResult {
        let s = arg.to_string();
        if s.contains("hello") {
            NodeResult::Done(Box::new(CustomTarget {
                value: format!("custom done: {}", s),
            }))
        } else if s.contains("next") {
            NodeResult::Next(
                Box::new(CustomArg {
                    raw: format!("custom forwarded: {}", s),
                }),
                Trace::from_str("root/child"),
            )
        } else {
            NodeResult::Next(
                Box::new(CustomArg {
                    raw: format!("custom next: {}", s),
                }),
                Trace::from_str("root"),
            )
        }
    }
}

pub struct CustomArg {
    pub raw: String,
}

impl Arg for CustomArg {
    fn to_string(&self) -> String {
        self.raw.clone()
    }
}

pub struct CustomTarget {
    pub value: String,
}

impl arrival_core::Target for CustomTarget {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}
