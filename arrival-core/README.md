# arrival-core

Arrival 的核心运行时 crate.

定义 Node 调度所需的核心 trait 和 `Runtime` 执行引擎.

## 核心 Trait

- `Node`: 处理请求的单个层, 返回 `NodeResult`
- `Arg`: 在层间传递的请求
- `Target`: 最终响应
- `Trace`: 节点路由路径 (来自 `arrival-trace`)

## 使用

```rust
use arrival_core::{Node, NodeResult, Arg, Target, Runtime, Trace};

// 实现自定义 Node
struct MyNode;

impl Node for MyNode {
    fn path(&self) -> Trace {
        Trace::from_str("greet")
    }

    fn process(&self, arg: &dyn Arg) -> NodeResult {
        NodeResult::Done(Box::new(MyTarget {
            value: format!("hello: {}", arg.to_string()),
        }))
    }
}

// 注册并运行
let mut runtime = Runtime::new();
runtime.add_node(Box::new(MyNode));
// ...
```
