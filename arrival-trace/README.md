# arrival-trace

Lore 生态通用的抽象路径 crate.

与 `std::path` 不同, `Trace` 不对路径的内部结构做具体约定 -- 各段的类型由使用者决定.

## 核心类型

- `Trace`: 由若干段组成的抽象路径, 支持标记左右两端是否开放
- `Segment`: 路径段的 trait, 要求实现 `ToString + Debug`
- `StringSegment`: 基于 `String` 的默认 Segment 实现

## 使用

```rust
use arrival_trace::Trace;

// 从字符串构造
let t = Trace::from_str("root::child::leaf");

// 逐步构建
let mut t = Trace::new();
t.push_str("root");
t.push_str("child");
assert_eq!(t.to_string(), "root::child");

// 获取各段字符串
let segments = t.segments_str();
assert_eq!(segments, vec!["root", "child"]);
```

## Features

- `segment-string` (默认): 启用 `StringSegment` 和相关便捷方法
