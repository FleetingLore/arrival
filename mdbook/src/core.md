# Core

本章介绍 `arrival-core` 的核心类型和用法.

## 依赖

```toml
[dependencies]
arrival-core = "0.2"
```

## 核心 Trait

### Arg -- 请求

在节点间传递的参数.

```rust
pub trait Arg {
    fn to_string(&self) -> String;
}
```

### Target -- 结果

下降终点返回的响应.

```rust
pub trait Target {
    fn to_string(&self) -> String;
}
```

### Node -- 节点

框架的核心抽象. 每个节点有一个路径标识, 接收 Arg 并返回 NodeResult.

```rust
pub trait Node {
    fn path(&self) -> Trace;
    fn process(&self, arg: &dyn Arg) -> NodeResult;
}
```

### NodeResult -- 节点的决策

```rust
pub enum NodeResult {
    Next(Box<dyn Arg>, Trace),   // 继续下降到下一个节点
    Done(Box<dyn Target>),       // 停止, 返回结果
}
```

## Runtime -- 执行引擎

`Runtime` 管理所有注册的节点, 驱动下降流程.

```rust
pub struct Runtime {
    nodes: Vec<Box<dyn Node>>,
    path: Trace,
}
```

关键方法:

| 方法 | 说明 |
|------|------|
| `new()` | 创建空 Runtime |
| `add_node(node)` | 注册一个节点 |
| `get(path)` | 按路径查找节点 |
| `run(arg, start)` | 从起始路径开始执行下降 |
| `path()` | 查看当前下降链路的完整 Trace |
| `reset()` | 重置路径记录 |

## 完整示例

下面是一个包含根节点和子节点的完整例子:

```rust
use arrival_core::{Arg, Target, Node, NodeResult, Runtime, Trace};

// -- Arg --
struct MyArg {
    raw: String,
}

impl Arg for MyArg {
    fn to_string(&self) -> String {
        self.raw.clone()
    }
}

// -- Target --
struct MyTarget {
    value: String,
}

impl Target for MyTarget {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

// -- Node: root --
struct RootNode;

impl Node for RootNode {
    fn path(&self) -> Trace {
        Trace::from_str("root")
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
                Trace::from_str("root::child"),
            )
        }
    }
}

// -- Node: child --
struct ChildNode;

impl Node for ChildNode {
    fn path(&self) -> Trace {
        Trace::from_str("root::child")
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

    let result = runtime.run(arg, Trace::from_str("root"));
    println!("{}", result.unwrap().to_string());
}
```

## 下降流程

1. 用户创建初始 Arg
2. `Runtime::run` 从起始 Trace 开始
3. 按路径查找对应 Node, 调用其 `process`
4. 若返回 `Next(arg, next_trace)`, 则用新的 arg 和 trace 继续循环
5. 若返回 `Done(target)`, 则停止循环, 将 target 返回给用户
6. 整个过程记录的路径可通过 `runtime.path()` 查看
