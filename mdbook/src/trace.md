# Trace -- 抽象路径

`Trace` 是 `arrival-trace` crate 提供的抽象路径类型, 用于在 Node 之间路由请求.
与 `std::path` 不同, 它不对路径的内部结构做具体约定 -- 各段 (segment) 的类型由使用
者决定.

## 基本结构

```rust
pub struct Trace {
    segments: Vec<Box<dyn Segment>>,
    left_open: bool,
    right_open: bool,
}
```

- `segments`: 路径的各个分段, 每个分段必须实现 `Segment` trait
- `left_open`: 路径左侧是否开放 (允许在前方继续拼接)
- `right_open`: 路径右侧是否开放 (允许在后方继续拼接)

## Display 格式

各段以 `::` 连接输出. 若 `left_open` 为真则在最前加 `::`, 若 `right_open` 为真
则在最后加 `::`.

```
Trace { segments: [a, b], left_open: true, right_open: false }  =>  ::a::b
Trace { segments: [a, b], left_open: false, right_open: true }  =>  a::b::
Trace { segments: [a, b], left_open: true, right_open: true }   =>  ::a::b::
Trace { segments: [a, b], left_open: false, right_open: false } =>  a::b
```

## 构造方法

### 从字符串解析

```rust
use arrival_trace::Trace;

// 普通路径
let t = Trace::from_str("root::child::leaf");

// 左侧开放
let t = Trace::from_str("::root::child");

// 右侧开放
let t = Trace::from_str("root::child::");
```

### 从空开始逐步构建

```rust
let mut t = Trace::new();
t.push_str("root");
t.push_str("child");
assert_eq!(t.to_string(), "root::child");
```

### 使用自定义 Segment

```rust
use arrival_trace::segment::Segment;

#[derive(Debug)]
struct MySegment(u32);

impl ToString for MySegment {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl Segment for MySegment {
    fn is_empty(&self) -> bool {
        false
    }
}

let mut t = Trace::new();
t.push(MySegment(42));
```

## API 参考

| 方法 | 说明 |
|------|------|
| `Trace::new()` | 创建空的闭合路径 |
| `Trace::from_str(s)` | 从 `::` 分隔的字符串解析 |
| `push(segment)` | 追加一个实现 `Segment` 的段 |
| `push_str(s)` | 以字符串形式追加一个段 |
| `segments_str()` | 返回所有段的字符串形式 `Vec<String>` |
| `is_empty()` | 判断路径是否为空 (无段且两端闭合) |
| `to_string()` | 输出为 `::` 分隔的字符串 |

## Segment trait

```rust
pub trait Segment: ToString + std::fmt::Debug {
    fn is_empty(&self) -> bool;
}
```

实现此 trait 即可作为 `Trace` 的一个段. 默认提供了 `StringSegment`, 它是 `String`
的简单封装.

## 与 Runtime 的关系

`Runtime` 在执行 descent 过程中会不断将当前路径追加到自身的 `path: Trace` 中,
形成一个完整的下降链路记录.

```rust
let mut runtime = Runtime::new();
// ... 添加 nodes ...
runtime.run(arg, Trace::from_str("root"));

// 查看完整路径
println!("{:?}", runtime.path().segments_str());
```
