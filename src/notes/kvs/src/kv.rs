//! 通过 [indexmap](https://github.com/bluss/indexmap) 实现简单的 KV 数据库
//! 为了防止 data race，将 IndexMap 用 Arc 进行包装
//! 具体实现可以参考：https://github.com/pingcap/talent-plan/blob/master/courses/rust/projects/project-2/src/kv.rs

use super::util::HandyRwLock;
use indexmap::IndexMap;
use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};

/// 键值对存储在日志文件中 todo
struct KVStore {
    /// 当将键设置为值时，kvs 将 set 命令写入硬盘中的有序日志中，
    /// 然后将该日志对应的指针(文件偏移量)指向键和内容，并存储在内存索引中。
    /// 类似地，当删除一个键时，kvs 将 rm 命令写入日志，然后从内存索引中删除该键。
    /// 当使用 get 命令检索键的值时，它检索索引，如果找到了，就从对应的日志指针上加载命令，执行命令并返回结果。
    ///
    /// kvs 启动时，就会按从旧到新的顺序从日志中遍历并执行命令，内存索引也会对应的重建。
    ///
    /// 当日志条数达到给定阈值时，kvs 会其压缩为一个新日志，删除冗余日志以回收磁盘空间。
    ///
    /// 注意，kvs 项目既是一个无状态命令行程序，也是一个包含有状态 KVStore 类型的库：
    /// 对于 CLI，使用 KVStore 类型将加载索引，执行命令，然后退出；对于库使用，它将加载索引，然后执行多个命令，维护索引状态，直到它被删除。
    /// ref: https://github.com/pingcap/talent-plan/blob/master/courses/rust/projects/project-2/README.md#project-spec
    path: PathBuf,
    inner: Arc<RwLock<IndexMap<Vec<u8>, Vec<u8>>>>,
}

impl KVStore {
    fn new() -> Self {
        KVStore::from_map(IndexMap::new())
    }

    fn from_map(m: IndexMap<Vec<u8>, Vec<u8>>) -> Self {
        KVStore {
            inner: Arc::new(RwLock::new(m)),
        }
    }

    fn set(&mut self, k: Vec<u8>, v: Vec<u8>) -> Option<Vec<u8>> {
        self.inner.wl().insert(k, v)
    }

    fn get(&self, k: &[u8]) -> Option<Vec<u8>> {
        self.inner.rl().get(k).map(|v| v.clone())
    }

    fn delete(&mut self, k: &[u8]) -> Option<Vec<u8>> {
        self.inner.wl().remove(k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store1() {
        let mut st = KVStore::new();
        let cache_key: Vec<u8> = "org_1001_info".as_bytes().into();
        st.set(cache_key.clone(), "hello org".as_bytes().into());
        assert_eq!(st.get(&cache_key), Some("hello org".as_bytes().into()));
        assert!(false);
    }

    #[test]
    fn test_store_delete() {
        let mut st = KVStore::new();
        let cache_key: Vec<u8> = "org_1001_info".as_bytes().into();
        st.set(cache_key.clone(), "hello org".as_bytes().into());
        assert_eq!(st.delete(&cache_key), Some("hello org".as_bytes().into()));
        assert_eq!(st.get(&cache_key), None);
    }
}

/*
### 错误处理
In this project it will be possible for the code to fail due to I/O errors.
So before we get started implementing a database we need to do one more thing that is crucial to Rust projects:
 decide on an error handling strategy.
>在这个项目中，I/O 错误会导致代码执行失败。因此，在完全实现数据库之前，我们还需要做一件对这个 Rust 项目
至关重要的事：错误处理策略。

Rust's error handling is powerful, but involves a lot of boilerplate to use correctly.
 For this project the failure crate will provide the tools to easily handle errors of all kinds.
>Rust 的错误处理很强大，但需要以合适的方式使用多个样板文件，而对于这个项目，failure 库将提供便捷的错误处理工具。

The failure guide describes several error handling patterns.
>failure 库的指南中描述了几种错误处理模式。

Pick one of those strategies and, in your library, either define your own error type or import failures Error.
This is the error type you will use in all of your Results, converting error types from other crates to your own with the ? operator.
>我们选择其中一种策略，然后在库中可以定义自己的错误类型，也可以导入其他 Error。这个策略对应的错误类型将会在项目中的 Result 中使用，
可以使用 `?` 操作符把其他库中的错误类型转换为自己库的错误类型。

After that, define a type alias for Result that includes your concrete error type, so that you don't need to type Result<T, YourErrorType> everywhere, but can simply type Result<T>. This is a common Rust pattern.
>这样，为 Result 定义一个含有错误类型的类型别名，编码时就不需要到处输入 Result<T, YourErrorType>，而可以简单的输入 Result。这是一种非常常见的 Rust 模式。

Finally, import those types into your executable with use statements, and change mains function signature to return Result<()>. All functions in your library that may fail will pass these Results back down the stack all the way to main, and then to the Rust runtime, which will print an error.
>最后，使用 use 语句将这些类型导入到代码中，然后将 main 函数的签名的返回值部分修改为 `Result<()>`。

Run cargo check to look for compiler errors, then fix them. For now it's ok to end main with panic!() to make the project build.
>运行 cargo check 可以用编译器检查错误，然后修复这些错误。现在可以先使用 panic!() 来结束 main 函数，从而编译项目。

Set up your error handling strategy before continuing.
>在前进之前，先确定好你的错误处理策略。

As with the previous project, you'll want to create placeholder data structures and methods so that the tests compile. Now that you have defined an error type this should be straightforward. Add panics anywhere necessary to get the test suite to compile (cargo test --no-run).
>与之前的项目一样，你可以创建用于占位的数据结构和方法，以便跑通测试用例。现在你定义一个错误类型，这很简单。在所有需要编译测试用例的地方添加 panic（cargo test --no-run）。

Note: Error-handling practices in Rust are still evolving. This course currently uses the failure crate to make defining error types easier. While failure has a good design, its use is arguably not a best practice. It may not continue to be viewed favorably by Rust experts. Future iterations of the course will likely not use failure. In the meantime, it is fine, and presents an opportunity to learn more of the history and nuance of Rust error handling.
> 注意：Rust 中的“错误处理”仍在发展和改进中。本课程目前使用 failure 库定义错误类型更容易。虽然 failure 设计不错，但它使用起来不是最佳实践。Rust 专家可能会开发出更好的错误处理方式。在后面的课程中有可能不会使用 failure。于此同时，它也是一个不错的选择，可提供一个学习 Rust 错误处理的演进以及改进的契机。


*/
