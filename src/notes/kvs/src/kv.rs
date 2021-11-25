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
>* 资料来源：https://github.com/pingcap/talent-plan/blob/master/courses/rust/projects/project-2/README.md#project-spec

### 部分 1：错误处理
在这个项目中，I/O 错误会导致代码执行失败。因此，在完全实现数据库之前，我们还需要确定一件
至关重要的事：错误处理策略。

Rust 的错误处理很强大，但需要以合适的方式使用多个样板文件，而对于这个项目，failure 库将提供便捷的错误处理工具。

failure 库的指南中描述了几种错误处理模式。

我们选择其中一种策略，然后在库中可以定义自己的错误类型，也可以导入其他 Error。这个策略对应的错误类型将会在项目中的 Result 中使用，
可以使用 `?` 操作符把其他库中的错误类型转换为自己库的错误类型。

这样，为 Result 定义一个含有错误类型的类型别名，编码时就不需要到处输入 Result<T, YourErrorType>，而可以简单的输入 Result。这是一种非常常见的 Rust 模式。

最后，使用 use 语句将这些类型导入到代码中，然后将 main 函数的签名的返回值部分修改为 `Result<()>`。

运行 cargo check 可以用编译器检查错误，然后修复这些错误。现在可以先使用 panic!() 来结束 main 函数，从而通过编译。

在前进之前，先确定好你的错误处理策略。

与之前的项目一样，你可以创建用于占位的数据结构和方法，以便跑通测试用例。现在你定义一个错误类型，这很简单。然后在所有需要编译测试用例的地方添加 panic（cargo test --no-run）。

注意：Rust 中的“错误处理”仍在发展和改进中。本课程目前使用 failure 库定义错误类型更容易。虽然 failure 设计不错，但它的使用不是最佳实践。Rust 专家可能会开发出更好的错误处理方式。
在后面的课程中有可能不会一直使用 failure。于此同时，它也是一个不错的选择，它能用于学习 Rust 错误处理的演进以及优化。

### 部分 2：log 的作用和原理
Now we are finally going to begin implementing the beginnings of a real database by reading and writing from disk. You will use serde to serialize the "set" and "rm" commands to a string, and the standard file I/O APIs to write it to disk.
> 现在无码终于要开始通过从磁盘读写来实现一个真正的数据库。我们将使用 serde 来把 "set" 和 "rm" 指令序列化为字符串，然后用标准的文件 I/O 接口来写到硬盘上。

下面这些是 kvs 最基本的日志行文：

* "set"
    * 用户调用 kvs 进行 set mykey myvalue
    * kvs 创建 set 指令包含的值，其中有 key 和 value
    * 然后，程序将指令序列化为字符串
    * 然后，把序列化的指令追加到日志文件中
    * 如果成功了，则以错误码 0 静默地退出
    * 如果失败了，就打印错误，并返回非 0 地错误代码并退出

* "get"
    * 用户调用 kvs 指令：get mykey
    * kvs reads the entire log, one command at a time, recording the affected key and file offset of the command to an in-memory key -> log pointer map
    * kvs 每次读取一个指令，将受影响地 key 和文件偏移量记录到内存的 map 中，即 key -> 日志指针
    * 然后，检查 map 中的日志指针
    * 如果失败，则打印“Key not found”，并以代码 0 退出
    * 如果成功
    * 它将指令日志反序列化得到最后的记录中的 key 和值
    * 然后将结果打印到标准输出，并以代码 0 退出

* "rm"
    * 用户调用 kvs 的指令 rm mykey
    * Same as the "get" command, kvs reads the entire log to build the in-memory index
    * 和 get 指令一样，kvs 读取整个日志来在内存中构建索引
    * 然后，它检查 map 中是否存在给定的 key
    * 如果不存在，就返回“Key not found”
    * 如果成功
    * 将会创建对应的 rm 指令，其中包含了 key
    * 然后将指令序列化追加到日志中
    * 如果成功，则以错误码 0 静默退出

The log is a record of the transactions committed to the database. By "replaying" the records in the log on startup we reconstruct the previous state of the database.
> 日志是提交到数据库的事务记录。通过在启动时，“重建”（replaying）日志中的记录，我们就可以重现数据库之前的状态。

In this iteration you may store the value of the keys directly in memory (and thus never read from the log after initial startup and log replay). In a future iteration you will store only "log pointers" (file offsets) into the log.
> 在这个迭代中，你可以将键的值直接存储在内存中（因此在重启或重建时是不会从日志中读取内容的）。在后面的迭代中，只需将日志指针（文件偏移量）存储到日志中。

### 部分 3：log 的写入
### 部分 4：log 的读取
### 部分 5：在索引中存储 log 的指针
### 部分 6：KvStore 的有状态和无状态
### 部分 7：log 的压缩

*/
