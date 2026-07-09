# arrival-toml

TOML 配置解析入口.

将 TOML 配置字符串或文件解析为可运行的 `Runtime`.

## 使用

```rust
use arrival_toml::from_str;

let toml = r#"
[[nodes]]
path = { nodes = ["root"] }
next = { nodes = ["root", "child"] }

[[nodes]]
path = { nodes = ["root", "child"] }
result = "Hello, world!"
"#;

let runtime = from_str(toml).unwrap();
```

## API

- `from_str(content: &str) -> Result<Runtime, TomlError>`
- `from_file(path: &str) -> Result<Runtime, TomlError>`
