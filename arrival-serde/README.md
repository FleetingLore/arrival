# arrival-serde

Arrival 的序列化支持.

提供 `SerdeNode`, `SerdePath`, `SerdeTarget` 等可序列化类型,
用于从 TOML/JSON 配置中定义节点.

## 类型

- `SerdeNode`: 可序列化的 Node
- `SerdePath`: 可序列化的路径 (内部 `nodes: Vec<String>`)
- `SerdeTarget`: 可序列化的 Target (支持 String/Number/Boolean/Null)
- `SerdeRuntime`: 可序列化的 Runtime 容器

## 使用

```rust
use arrival_serde::{SerdeNode, SerdePath};

let node = SerdeNode::new("root")
    .with_next("root::child");
```
