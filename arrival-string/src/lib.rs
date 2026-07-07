use arrival_core::{Arg, Target, Node, NodeResult, Path};

pub struct StringNode {
    path: Path,
    response: String,
}

impl StringNode {
    pub fn new(path: &str, response: &str) -> Self {
        Self {
            path: Path::from_str(path),
            response: response.to_string(),
        }
    }

    pub fn with_path(mut self, path: &str) -> Self {
        self.path = Path::from_str(path);
        self
    }

    pub fn with_response(mut self, response: &str) -> Self {
        self.response = response.to_string();
        self
    }
}

impl Node for StringNode {
    fn path(&self) -> Path {
        self.path.clone()
    }

    fn process(&self, arg: &dyn Arg) -> NodeResult {
        let _ = arg.to_string();
        NodeResult::Done(Box::new(StringTarget {
            value: self.response.clone(),
        }))
    }
}

pub struct StringTarget {
    value: String,
}

impl Target for StringTarget {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrival_core::Arg;

    struct TestArg {
        raw: String,
    }

    impl Arg for TestArg {
        fn to_string(&self) -> String {
            self.raw.clone()
        }
    }

    #[test]
    fn test_string_node() {
        let node = StringNode::new("root", "hello world");
        let arg = TestArg { raw: "test".to_string() };
        let result = node.process(&arg);
        match result {
            NodeResult::Done(target) => {
                assert_eq!(target.to_string(), "hello world");
            }
            _ => panic!("expected Done"),
        }
    }
}
