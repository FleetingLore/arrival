# arrival-cli-return

调用命令行并返回结果的 Node 实现.

提供 `CliReturnNode` 和 `CliCommand` trait.

## 使用

```rust
use arrival_cli_return::{CliReturnNode, CliCommand};

struct EchoCommand;

impl CliCommand for EchoCommand {
    fn execute(&self, input: &str) -> String {
        format!("echo: {}", input)
    }
}

let node = CliReturnNode::new("echo", EchoCommand);
```
