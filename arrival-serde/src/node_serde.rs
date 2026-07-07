use arrival_core::{Node, NodeResult, Arg, Target, Path};
use serde::{Deserialize, Serialize};
use crate::{SerdeArg, SerdeTarget, SerdePath};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerdeNode {
    pub path: SerdePath,
    #[serde(default)]
    pub next: Option<SerdePath>,
    #[serde(default)]
    pub result: Option<SerdeTarget>,
}

impl Node for SerdeNode {
    fn path(&self) -> Path {
        self.path.to_path()
    }

    fn process(&self, arg: &dyn Arg) -> NodeResult {
        let arg_str = arg.to_string();

        if let Some(result) = &self.result {
            NodeResult::Done(Box::new(result.clone()))
        } else if let Some(next) = &self.next {
            NodeResult::Next(
                Box::new(SerdeArg::String(format!("forward: {}", arg_str))),
                next.to_path(),
            )
        } else {
            NodeResult::Next(
                Box::new(SerdeArg::String(arg_str)),
                self.path.to_path(),
            )
        }
    }
}

impl SerdeNode {
    pub fn new(path: &str) -> Self {
        Self {
            path: SerdePath::from_str(path),
            next: None,
            result: None,
        }
    }

    pub fn with_next(mut self, next: &str) -> Self {
        self.next = Some(SerdePath::from_str(next));
        self
    }

    pub fn with_result(mut self, result: SerdeTarget) -> Self {
        self.result = Some(result);
        self
    }
}

impl Default for SerdeNode {
    fn default() -> Self {
        Self::new("")
    }
}
