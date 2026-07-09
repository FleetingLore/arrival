# Introduction

Arrival 是一个抽象层通信框架. 请求逐层下降, 直到找到答案.

## Arrival 是什么?

Arrival 为分层系统提供了以下 trait:

- Arg: 在层间下降的请求
- Target: 返回的响应
- Node: 处理参数的单个层
- Trace: 节点之间的路由路径
- Runtime: 管理下降过程的执行引擎

## 使用场景

- 文件系统抽象
- 配置查找
- 协议协商
- 命令路由
- 多阶段处理

## 项目状态

Arrival 正在积极开发中. 核心 API 已稳定.
