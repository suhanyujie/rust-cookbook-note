# 构建 HA 的 KV 数据库
最近打算用 Rust 做个练习 —— 从零实现一个 KV 数据库。深度参考 [mum](https://github.com/dyxushuai/mum)，而 mum 又是 [talent-plan](https://github.com/pingcap/talent-plan/blob/master/courses/dss/raft/README.md) 的一个课程的实现。

## todo
* [*] 实现简单的 KV 存储
* [ ] 日志
* [ ] 通过接口实现 KV 数据库的操作
* [ ] 多节点和分布式
* [ ] 持久化

## run
* `cargo run`

## 参考
* WAL 介绍 https://www.cnblogs.com/xuwc/p/14037750.html
* 深度参考 mum https://github.com/dyxushuai/mum
* tikv 的设计实现 https://www.cnblogs.com/qcloud1001/p/7865246.html
* talent-plan Log-structured file I/O https://github.com/pingcap/talent-plan/tree/master/courses/rust/projects/project-2#user-content-introduction
* Rust Project 2: Log-structured file I/O 解读 http://blog.yanick.site/2020/10/16/rust/talent-plan/project-2/
* BuffWriterWithPos https://www.cnblogs.com/pdev/p/11452529.html
* talent plan project https://github.com/pingcap/talent-plan/blob/master/courses/rust/README.md
* 从零开始写KV数据库：基于哈希索引 https://zhuanlan.zhihu.com/p/351897096
