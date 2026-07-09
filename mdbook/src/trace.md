# Trace -- 抽象路径

`Trace` 是 `arrival-trace` crate 提供的抽象路径类型, 用于在 Node 之间路由请求.
各段的类型由使用者决定, 不局限于文件系统路径.

## 基本结构

```rust
pub struct Trace {
    segments: Vec<Box<dyn Segment>>,
    left_open: bool,
    right_open: bool,
}
```

- `segments`: 路径的各个分段
- `left_open`: 左侧是否开放 (允许在前方继续拼接)
- `right_open`: 右侧是否开放 (允许在后方继续拼接)

## Display 格式

各段以 `::` 连接. 开放端额外附加 `::`.

```
[a, b] 左右闭合  =>  a::b
[a, b] 左侧开放  =>  ::a::b
[a, b] 右侧开放  =>  a::b::
[a, b] 两端开放  =>  ::a::b::
```

## 构造

### 从字符串解析

```rust
use arrival_trace::Trace;

let t = Trace::from_str("root::child::leaf");
let open = Trace::from_str("::root::child::");
```

前缀 `::` 表示 left_open, 后缀 `::` 表示 right_open.

### 逐步构建

```rust
let mut t = Trace::new();
t.push_str("root");
t.push_str("child");
assert_eq!(t.to_string(), "root::child");
```

### 自定义 Segment

```rust
use arrival_trace::segment::Segment;

#[derive(Debug)]
struct IdSegment(u32);

impl ToString for IdSegment {
    fn to_string(&self) -> String { self.0.to_string() }
}

impl Segment for IdSegment {
    fn is_empty(&self) -> bool { false }
}

let mut t = Trace::new();
t.push(IdSegment(42));
```

## API 参考

| 方法 | 说明 |
|------|------|
| `new()` | 创建空路径 |
| `from_str(s)` | 从 `::` 分隔的字符串解析 |
| `push(segment)` | 追加任意 Segment |
| `push_str(s)` | 以字符串追加一段 |
| `segments_str()` | 返回各段的 `Vec<String>` |
| `is_empty()` | 路径是否为空 |
| `to_string()` (Display) | 输出 `::` 分隔的字符串 |

## Segment trait

```rust
pub trait Segment: ToString + std::fmt::Debug {
    fn is_empty(&self) -> bool;
}
```

默认提供 `StringSegment`, 封装 `String`. 启用 `segment-string` feature (默认) 即可使用.

## 在 Runtime 中的角色

每次下降时 Runtime 将当前路径追加入内部的 `Trace`, 形成完整的链路记录:

```rust
let mut runtime = Runtime::new();
runtime.run(arg, Trace::from_str("root"));

// 查看完整下降路径
println!("{:?}", runtime.path().segments_str());
```
