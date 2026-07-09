# Configuration

Arrival 支持 TOML 配置文件.

## 基本结构

```toml
[[nodes]]
path = { nodes = ["root"] }
next = { nodes = ["root", "child"] }

[[nodes]]
path = { nodes = ["root", "child"] }
result = "Hello, world!"
```

节点的 `path` 和 `next` 字段使用 `SerdePath` 的 `{ nodes = [...] }` 格式.
`result` 字段可以是字符串、数字、布尔值等.

## 从字符串加载

```rust
use arrival_toml::from_str;

let toml = r#"
[[nodes]]
path = { nodes = ["root"] }
next = { nodes = ["root", "child"] }

[[nodes]]
path = { nodes = ["root", "child"] }
result = "Hello"
"#;

let runtime = from_str(toml).unwrap();
```

## 从文件加载

```rust
use arrival_toml::from_file;

let runtime = from_file("config.toml").unwrap();
```

## 内置节点 vs 配置节点

配置文件中定义的节点通过 `arrival-serde` 的 `SerdeNode` 实现, 解析后自动注册到
`Runtime`. 它们可以和内置节点 (如 `StringNode`) 混合使用.
