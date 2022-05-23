## http 服务器
[sfz](https://github.com/weihanglo/sfz) 的服务是基于 [hyper](https://github.com/hyperium/hyper) 的。为了能更加熟悉使用 hyper，特地开启一个 mod 用于熟悉 hyper 的使用。

官方文档已经给出了比较简单的[示例](https://hyper.rs/guides/server/hello-world/)。

一个静态文件服务器实现的主要流程有：
    - serve 请求，解析出请求的参数
    - 根据请求参数中的路径，读取对应路径的文件列表
    - 根据文件列表，渲染出结果字符串
    - 返回

## 读取文件列表
根据 sfz 源码，处理请求的 handler 位于 handle_request 函数（src/server/serve.rs）中，通过 `req.uri().query()` 即可拿到路径相关参数，在没有特殊参数 action 的前提下，直接执行默认的处理逻辑 `default_action` —— `Action::ListDir`。

首先执行 sfz 服务的命令是形如 `sfz ./someDir`。也就是说，所有 http 请求查询路径下文件时，都是基于这个 `./someDir` 目录的。

比如，通过 `localhost:3001/mydir1` 查询路径下的文件列表，其实就是查询 `./someDir/mydir1` 下的文件列表。

所以在请求到来时，我们需要将获取到的 `/mydir1` 参数值拼接上 basePath —— `./someDir`

    