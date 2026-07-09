# Core Concepts

## Arg

Arg 是表示请求或参数的 trait.

```rust
pub trait Arg {
    fn to_string(&self) -> String;
}
```

## Target

Target 是表示响应或结果的 trait.

```rust
pub trait Target {
    fn to_string(&self) -> String;
}
```

## Node

Node 是核心抽象. 每个节点有一个路径, 用于处理参数.

```rust
pub trait Node {
    fn path(&self) -> Trace;
    fn process(&self, arg: &dyn Arg) -> NodeResult;
}
```

## NodeResult

NodeResult 有两个变体:

```rust
pub enum NodeResult {
    Next(Box<dyn Arg>, Trace),
    Done(Box<dyn Target>),
}
```

`Next` 表示继续下降到下一个节点, 同时携带下一个路径.
`Done` 表示停止并返回 `Target`.

## Trace

Trace 是抽象路径, 由若干段组成, 各段间以 `::` 连接. 详见 [Trace](trace.md) 章节.

```rust
let path = Trace::from_str("root::child::leaf");
assert_eq!(path.to_string(), "root::child::leaf");
```

## Runtime

Runtime 管理整个下降过程.

```rust
pub struct Runtime {
    nodes: Vec<Box<dyn Node>>,
    path: Trace,
}
```

- `nodes`: 所有注册的节点
- `path`: 当前下降链路的完整记录

## 下降流程

1. 用户创建初始 Arg
2. Runtime 从起始路径开始
3. 根节点处理 Arg
4. 根节点返回 `Next`, 包含新的 Arg 和子路径
5. Runtime 查找子节点
6. 子节点处理 Arg
7. 子节点返回 `Done`, 携带 Target
8. Runtime 将 Target 返回给用户
