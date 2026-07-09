# Arrival

A framework for abstract layer communication.

## Crates

- `arrival-core`: 核心运行时, 定义 `Node` / `Arg` / `Target` trait 和 `Runtime` 调度器
- `arrival-trace`: 抽象路径表示, 不依赖文件系统, 各段类型可自定义
- `arrival-string`: 基于字符串的 Node 实现
- `arrival-cli-return`: 调用命令行并返回结果的 Node 实现
- `arrival-serde`: TOML 配置中声明节点的序列化支持
- `arrival-toml`: TOML 配置解析入口
- `arrival-cli`: 命令行测试工具

## Overview

Arrival allows you to build systems where requests descend through layers of abstraction until an answer is reached.

## Quick Start

Add to Cargo.toml:

[dependencies]
arrival-core = "0.1"

## Built-in Nodes

- StringNode: Returns a fixed string
- CliReturnNode: Executes a CLI command
- SerdeNode: TOML-defined nodes
