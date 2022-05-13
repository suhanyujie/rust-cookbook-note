# zfz 源码阅读笔记（用 Rust 从零实现一个静态资源服务器）
关注 Rust 已经很久了，但是依然没能在工作中开始使用，有点遗憾的同时，也苦恼无法更进一步的了解 Rust。基于这个背景，我打算尝试阅读一些 Rust 项目的代码，并做一些笔记，以深入了解 Rust。

我选择了这个 [zfz](https://github.com/weihanglo/sfz/blob/master/src/main.rs)。之所以选择这个项目，是因为它足够小，并且不复杂。用 Rust 实现一些工具的功能，一定能带来很大的收获。我们开始吧！

## 错误处理
由于是一个命令行工具，所以作者对[错误处理](https://www.cnblogs.com/ishenghuo/p/15864482.html)采用了比较直接的方式： `Box<dyn std::error::Error>`，并且，错误的抛出是直接打印然后退出进程：

```rust
fn handle_err<T>(err: Box<dyn std::error::Error>) -> T {
    eprintln!("Server error: {}", err);
    std::process::exit(1);
}
```

-- todo


## 参考
* https://github.com/weihanglo/sfz