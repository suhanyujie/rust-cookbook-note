>* 原文作者：[suhanyujie](https://github.com/suhanyujie/rust-cookbook-note)
>* 永久链接：https://github.com/suhanyujie/rust-cookbook-note
>* 博客链接：https://ishenghuo.cnblogs.com

# 自定义类型的数据在线程间的安全传递
这本是 [README](../README.md) 文件中的一个问题，但是我觉得可以将其拿出来单独作为一个 topic 进行详细说明。

在阅读这篇笔记之前，还请先读完 Rust 权威指南的[相关章节](https://github.com/rust-lang/book/blob/master/src/ch16-04-extensible-concurrency-sync-and-send.md) [中文版](https://kaisery.github.io/trpl-zh-cn/ch16-04-extensible-concurrency-sync-and-send.html)。

## 为什么需要在线程间传递
在编写服务端程序或者 cli 等应用时，你可能会用到多线程。我们会通常需要将一个数据从一个线程中，传递到另一个线程中进行处理。此时，我们就需要将数据在线程间进行传递。而要想数据在线程间传递，需要数据类型实现了 `Send` trait 和 `Sync` trait。

## Send 和 Sync trait
Send 和 Sync trait 是 Rust 中的标记 trait，位于标准库的 `std::marker` crate 下。

## 实践

## 结论

## reference
