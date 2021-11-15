/*
## 数据的持久化
Rust 中把数据持久化的一种方式是通过 [bincode crate](https://docs.rs/bincode)

从 mum 仓库中的数据持久化[代码](https://github.com/dyxushuai/mum/blob/master/src/snap.rs#L24)可以看出它的使用方式是通过 bincode 的序列化和反序列化功能。


## 参考资料
* https://github.com/dyxushuai/mum

*/

mod kv;
mod util;
mod storage;
