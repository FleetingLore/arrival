use arrival_core::Node;
use arrival_string::StringNode;

pub fn create_string_nodes() -> Vec<Box<dyn Node>> {
    vec![
        Box::new(StringNode::new("root", "root response")),
        Box::new(StringNode::new("root/child", "child response")),
    ]
}
