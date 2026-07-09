# arrival-cli

Arrival 命令行测试工具.

## 安装

```
cargo install --path arrival-cli
```

## 用法

```
# 使用内置节点
arrival builtin hello --verbose

# 使用 TOML 配置
arrival toml hello --config config.toml

# 列出所有可用节点
arrival list
```

## 日志

```
RUST_LOG=debug arrival builtin hello
```
