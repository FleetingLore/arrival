# arrival-string

基于字符串的内置 Node 实现.

提供 `StringNode`, 无论输入如何始终返回固定字符串响应.

## 使用

```rust
use arrival_string::StringNode;

let node = StringNode::new("greet", "Hello, world!");
```

## 运行测试

```
cargo test -p arrival-string
```
