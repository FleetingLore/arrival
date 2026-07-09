# CLI Reference

Arrival CLI 提供命令行接口.

## Commands

### builtin

使用内置节点运行.

```
arrival builtin <RAW> [OPTIONS]
```

选项:
- `--verbose`, `-v`: 显示详细输出
- `--node-type <TYPE>`: custom, string, 或 cli-return

示例:
```
arrival builtin hello --verbose
arrival builtin test --node-type string
```

### toml

使用 TOML 配置运行.

```
arrival toml <RAW> [OPTIONS]
```

选项:
- `--verbose`, `-v`: 显示详细输出
- `--config <PATH>`: TOML 配置文件路径

示例:
```
arrival toml hello --verbose --config config.toml
```

### list

列出所有可用节点.

```
arrival list
```

## 日志

设置 `RUST_LOG` 环境变量控制日志级别.

```
RUST_LOG=debug arrival builtin hello
```
