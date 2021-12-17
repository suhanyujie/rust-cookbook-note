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

运行 `cargo check` 可以用编译器检查错误，然后修复这些错误。现在可以先使用 `panic!()` 来结束 `main` 函数，从而通过编译。

在前进之前，先确定好你的错误处理策略。

与之前的项目一样，你可以创建用于占位的数据结构和方法，以便跑通测试用例。现在你定义一个错误类型，这很简单。然后在所有需要编译测试用例的地方添加 panic（`cargo test --no-run`）。

注意：Rust 中的“错误处理”仍在发展和改进中。本课程目前使用 [`failure`](https://docs.rs/failure/0.1.5/failure/) 库定义错误类型更容易。虽然 `failure` 设计不错，但它的使用[不是最佳实践](https://github.com/rust-lang-nursery/rust-cookbook/issues/502#issue-387418261)。Rust 专家可能会开发出更好的错误处理方式。
在后面的课程中有可能不会一直使用 `failure`。于此同时，它也是一个不错的选择，它能用于学习 Rust 错误处理的演进以及优化。

### 部分 2：log 的作用和原理
Now we are finally going to begin implementing the beginnings of a real database by reading and writing from disk. You will use serde to serialize the "set" and "rm" commands to a string, and the standard file I/O APIs to write it to disk.
> 现在无码终于要开始通过从磁盘读写来实现一个真正的数据库。我们将使用 [serde](https://serde.rs/) 来把 "set" 和 "rm" 指令序列化为字符串，然后用标准的文件 I/O 接口来写到硬盘上。

下面这些是 `kvs` 最基本的日志行文：

* "set"
    * 用户调用 `kvs set mykey myvalue`
    * `kvs` 创建 set 指令包含的值，其中有 key 和 value
    * 然后，程序将指令序列化为 `String`
    * 然后，把序列化的指令追加到日志文件中
    * 如果成功了，则以错误码 0 静默地退出
    * 如果失败了，就打印错误，并返回非 0 地错误代码并退出

* "get"
    * 用户调用指令：`kvs get mykey`
    * kvs reads the entire log, one command at a time, recording the affected key and file offset of the command to an in-memory key -> log pointer map
    * kvs 每次读取一个指令，将受影响地 key 和文件偏移量记录到内存的 map 中，即 key -> 日志指针
    * 然后，检查 map 中的日志指针
    * 如果失败，则打印“Key not found”，并以代码 0 退出
    * 如果成功
    * 它将指令日志反序列化得到最后的记录中的 key 和值
    * 然后将结果打印到标准输出，并以代码 0 退出

* "rm"
    * 用户调用指令 `kvs rm mykey`
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
You will start by implementing the "set" flow. There are a number of steps here. Most of them are straightforward to implement and you can verify you've done so by running the appropriate `cli_*` test cases.
> 我们将从 set 开始。接下来将会有很多步骤。但大部分都比较容易实现，你可以通过运行 `cli_*` 测试用例来验证你的实现。

`serde` is a large library, with many options, and supporting many serialization formats. Basic serialization and deserialization only requires annotating your data structure correctly, and calling a function to write it either to a `String` or a stream implementing `Write`.
> `serde` 是一个大型库，有许多功能选项，支持多种序列化格式。基本的序列化和反序列化只需要对结构体进行合适的注解，然后调用一个函数将序列化后的内容写入 `String` 或者 `Write` 流。

You need to pick a serialization format. Think about the properties you want in your serialization format — do you want to prioritize performance? Do you want to be able to read the content of the log in plain text? It's your choice, but maybe you should include a comment in the code explaining it.
> 你需要选择一种序列化格式。并确定你需要的属性 —— 你是否需要性能优先？你你希望以纯文本形式读取日志内容吗？这都在于你如何配置，但你记得在代码中写好注释。

Other things to consider include: where is the system performing buffering and where do you need buffering? What is the impact of buffering on subsequent reads? When should you open and close file handles? For each command? For the lifetime of the `KvStore`?
> 还有其他因素要考虑一下：系统在哪设置缓冲，以及哪些地方需要？缓冲后续的影响是什么？何时打开和关闭文件句柄？有哪些支持的命令？`KvStore` 的生命周期是什么？

Some of the APIs you will call may fail, and return a `Result` of some error type. Make sure that your calling functions return a `Result` of your own error type, and that you convert between the two with `?`.
> 你调用的一些 api 可能会失败，并返回错误类型的 `Result`。你需要确保调用函数会返回你自己设定的错误类型的 `Result`，并用 `?` 向上传递。

It is similar to implementing the "rm" command, but you should additionally check if the key exists before writing the command to the log. As we have two different commands that must be distinguished, you may use variants of a single enum type to represent each command. `serde` just works perfectly with enums.
> 类似于 rm 命令，我们希望在把命令写入日志之前，还要检查 key 是否存在。因为两种情况下的命令需要区分开，所以可以使用 enum 类型的变体来统一所有命令。`serde` 可以完美地与枚举一起使用。

You may implement the "set" and "rm" commands now, focusing on the `set` / `rm` test cases, or you can proceed to the next section to read about the "get" command. It may help to keep both in mind, or to implement them both simultaneously. It is your choice.
> 你现在可以实现 set 和 rm 命令了，重点放在 set / rm 对应的测试用例上，也可以阅读下一节的 get 命令实现。记住这两个命令并加以实现，会对你很有帮助。选择权在你。

### 部分 4：log 的读取
Now it's time to implement "get". In this part, you don't need to store log pointers in the index, we will leave the work to the next part. Instead, just read each command in the log on startup, executing them to save every key and value in the memory. Then read from the memory.
> 现在该实现 get 了。在这一部分中，你不需要把日志指针存储在索引中，而将其放到下一节进行实现。这一节我们只需在启动时，读取日志中的所有命令，执行它们将每个键值对保存在内存中。然后根据需要从内存中读取。

Should you read all records in the log into memory at once and then replay them into your map type; or should you read them one at a time while replaying them into your map? Should you read into a buffer before deserializing or deserialize from a file stream? Think about the memory usage of your approach. Think about the way reading from I/O streams interacts with the kernel.
> 应该一次性把日志内容全部读取到内存中替换已存在的 map 吗？；应该在某个时候读取一条从而重现 map 中的某条数据？应该在序列化、反序列化之前将其从文件系统中读取到 buffer 中吗？想想你使用内存的方式。考虑一下与内核交互是从 I/O 流 读取数据。

Remember that "get" may not find a value and that case has to be handled specially. Here, our API returns `None` and our command line client prints a particular message and exits with a zero exit code.
> 记住，"get" 可能获取不到值，这种情况下，需要特殊处理。这里，我们的 API 返回 `None`，然后客户端打印一个特定的消息，并以零代码退出。

There's one complication to reading the log, and you may have already considered it while writing the "set" code: how do you distinguish between each record in the log? That is, how do you know when to stop reading one record, and start reading the next? Do you even need to? Maybe serde will deserialize a record directly from an I/O stream and stop reading when it's done, leaving the file cursor in the correct place to read subsequent records. Maybe serde will report an error when it sees two records back-to-back. Maybe you need to insert additional information to distinguish the length of each record. Maybe not.
> 读取日志比较复杂，你在编写 set 时，可能已经想到了：如何区分日志中的记录？也就是说，如何终止读取，何时开始读取下一条记录？需要这样实现吗？也许 serde 将直接从 I/O 流中发序列化一条记录，并在操作完后停止读取，将游标停留在正确的位置，以便读取后续的记录。也许 serde 在检查到两条背靠背（back-to-back）时会报错。也许你需要插入额外的信息来区分每个记录的长度，也有可能有其他方式。

_Implement "get" now._

### 部分 5：在索引中存储 log 的指针
At this point most, if not all (besides the compaction test), other test suite should all pass. The changes introduced in the next steps are simple optimizations, necessary for fast performance and reduced storage. As you implement them, pay attention to what exactly they are optimizing for.
> 此时，处理压缩数据相关的测试以外，其他测试应该都是通过的。接下来的步骤是一些性能优化和存储优化。当你实现它们时，需要注意它们的意义是什么？

As we've described, the database you are building maintains an in-memory index of all keys in the database. That index maps from string keys to log pointers, not the values themselves.

This change introduces the need to perform reads from the log at arbitrary offsets. Consider how that might impact the way you manage file handles.

If, in the previous steps, you elected to store the string values directly in memory, now is the time to update your code to store log pointers instead, loading from disk on demand.

### 部分 6：KvStore 的有状态和无状态
Remember that our project is both a library and a command-line program. They have sligtly different requirements: the kvs CLI commits a single change to disk, then exits (it is stateless); the KvStore type commits changes to disk, then stays resident in memory to service future queries (it is stateful).
> 请记住，我们的项目不仅是一个库，也可作为命令行程序。它们有些不一样：kvs 命令行程序向磁盘提交一个更改，然后就退出了（无状态）；KvStore 会将更改提交到磁盘，然后常驻内存以服务后续的查询（有状态）。

Is your KvStore stateful or stateless?
> 你的 KvStore 是有状态还是无状态呢？

Make your KvStore retain the index in memory so it doesn't need to re-evaluate it for every call to get.
> 可以让你的 KvStore 的索引常驻内存中，这样就无需在每次调用时重新执行所有的日志指令。

### 部分 7：log 的压缩
At this point the database works just fine, but the log grows indefinitely. That is appropriate for some databases, but not the one we're building — we want to minimize disk usage as much as we can.
> 到这里，数据库运行是正常的，但日志会无限增长。这对其他数据库可能没啥影响，但对于我们正在构建的数据库 —— 我们需要尽量减少磁盘的占用。

So the final step in creating your database is to compact the log. Consider that as the log grows that multiple entries may set the value of a given key. Consider also that only the most recent command that modified a given key has any effect on the current value of that key:
> 因此，最后一步就是压缩日志了。需要考虑到随着日志的增长，可能有多个指令日志对同一个键操作。还要考虑到，对于同一个键，最近一次的日志的更改才对其有影响：

idx | command 
|:---- |:--- |
| 0 | ~~Command::Set("key-1", "value-1a")~~  | 
| 20 | Command::Set("key-2", "value-2") | 
|   |   ... | 
| 100  | Command::Set("key-1", "value-1b") | 

In this example obviously the command at index 0 is redundant, so it doesn't need to be stored. Log compaction then is about rebuilding the log to remove redundancy:
> 在这个例子中，索引 0 的日志很明显是冗余的，因此不需要对其存储。日志压缩其实就是重新构建日志并且消除冗余：

idx | command 
|:---- |:--- |
| 0 | Command::Set("key-2", "value-2")  | 
|   |    ...  | 
| 99  |  Command::Set("key-1", "value-1b") | 

Here's the basic algorithm you will use:
> 这是基本算法的使用：

How you re-build the log is up to you. Consider questions like: what is the naive solution? How much memory do you need? What is the minimum amount of copying necessary to compact the log? Can the compaction be done in-place? How do you maintain data-integrity if compaction fails?
> 如何重建日志取决于你。考虑一下这个问题：最直接的方法是什么？需要多少内存？压缩日志所需的最小拷贝量是多少？能实时压缩吗？如果压缩失败，怎样保证数据完整性？

So far we've been refering to "the log", but in actuallity it is common for a database to store many logs, in different files. You may find it easier to compact the log if you split your log across files.
> 到目前为止，我们一直致力于“日志”，但实际上，数据库的数据存储在多个日志文件中是很常见的。如果你将日志拆分到多个文件中，你可能会发现压缩日志更容易。

Implement log compaction for your database.
> 给数据库实现日志压缩。

Congratulations! You have written a fully-functional database.
> 恭喜！你已经编写了一个功能齐全的数据库了。

If you are curious, now is a good time to start comparing the performance of your key/value store to others, like sled, bitcask, badger, or RocksDB. You might enjoy investigating their architectures, thinking about how theirs compare to yours, and how architecture affects performance. The next few projects will give you opportunities to optimize.
> 如果你很好奇，你可以将你实现的数据库的性能与其他数据库（如 sled、bitcask、badger 或 RicksDB）进行性能对比。你可能喜欢研究它们实现的架构，将其与你自己的架构对比，以及架构的不同对性能有何影响。接下来的几个项目将为你提供优化的机会。

Nice coding, friend. Enjoy a nice break.
> 写的很棒，朋友。可以休息一下了。

*/
