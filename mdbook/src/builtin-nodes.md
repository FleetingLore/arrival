# Built-in Nodes

Arrival 提供了若干内置的 Node 实现.

## StringNode

无论输入如何, 始终返回固定字符串响应.

```rust
use arrival_string::StringNode;

let node = StringNode::new("root", "Hello, world!");
```

## CliReturnNode

执行命令行并返回其输出.

```rust
use arrival_cli_return::{CliReturnNode, CliCommand};

struct MyCommand;

impl CliCommand for MyCommand {
    fn execute(&self, input: &str) -> String {
        format!("processed: {}", input)
    }
}

let node = CliReturnNode::new("root", MyCommand);
```

## 自定义 Node

实现 Node trait 以实现自定义逻辑.

```rust
use arrival_core::{Node, NodeResult, Arg, Target, Trace};

struct MyNode;

impl Node for MyNode {
    fn path(&self) -> Trace {
        Trace::from_str("my::path")
    }

    fn process(&self, arg: &dyn Arg) -> NodeResult {
        NodeResult::Done(Box::new(MyTarget))
    }
}
```

## SerdeNode

通过 TOML 配置定义的节点. 详见配置章节.
