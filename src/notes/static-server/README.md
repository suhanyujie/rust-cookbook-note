# zfz 源码阅读笔记（用 Rust 从零实现一个静态资源服务器）
关注 Rust 已经很久了，但是依然没能在工作中开始使用，有点遗憾的同时，也苦恼无法更进一步的了解 Rust。基于这个背景，我打算尝试阅读一些 Rust 项目的代码，并做一些笔记，以深入了解 Rust。

我选择了这个 [zfz](https://github.com/weihanglo/sfz/blob/master/src/main.rs)。之所以选择这个项目，是因为它足够小，并且不复杂。用 Rust 实现一些工具的功能，一定能带来很大的收获。我们开始吧！

在了解 sfz 之前，我们应该先了解它的用法。终端输入 `sfz --help` 看看都有什么帮助信息。

```shell
$ sfz --help
sfz 0.7.0
Weihang Lo <me@weihanglo.tw>

A simple static file serving command-line tool.

USAGE:
    sfz [OPTIONS] [path]

ARGS:
    <path>    Path to a directory for serving files [default: .]

OPTIONS:
    -a, --all                   Serve hidden and dot (.) files
    -b, --bind <address>        Specify bind address [default: 127.0.0.1]
    -c, --cache <seconds>       Specify max-age of HTTP caching in seconds [default: 0]
    -C, --cors                  Enable Cross-Origin Resource Sharing from any origin (*)
        --coi                   Enable Cross-Origin isolation
    -h, --help                  Print help information
    -I, --no-ignore             Don't respect gitignore file
    -L, --follow-links          Follow symlinks outside current serving base path
        --no-log                Don't log any request/response information.
    -p, --port <port>           Specify port to listen on [default: 5000]
        --path-prefix <path>    Specify an url path prefix, helpful when running behing a reverse proxy
    -r, --render-index          Render existing index.html when requesting a directory.
    -V, --version               Print version information
    -Z, --unzipped              Disable HTTP compression

```

前面几行主要是应用的名字、版本、作者、描述等信息。我们着重看下 OPTIONS 下对应的参数及其用法。

```
-a, --all                   服务点（`.`）前缀的隐藏文件在内的所有文件。
-b, --bind <address>        指定 bind 的服务地址 [默认: 127.0.0.1]
-c, --cache <seconds>       指定 http 缓存的最大秒数，默认 0
-C, --cors                  启用跨域资源访问，任意请求来源
    --coi                   启用跨域 isolation
-h, --help                  打印帮助信息
-I, --no-ignore             忽略 gitignore 文件
-L, --follow-links          跟随链接符号所指向的路径 Follow symlinks outside current serving base path
    --no-log                不打印请求、响应日志信息
-p, --port <port>           指定端口号 [默认: 5000]
    --path-prefix <path>    指定一个 url 路径前缀，在反向代理的场景中会很有帮助
-r, --render-index          请求一个路径时，按照 index.html 文件渲染返回
-V, --version               打印版本信息
-Z, --unzipped              禁用 http 压缩
```

## 主要思路
虽然该工具具备一些“周边”功能，但我们不能脱离主题 —— 围绕实现一个静态资源服务器来分析源代码，因此我们需要有自己的思路，带着自己的想法和问题，去看源代码。下面就是一个参考的思路：

* cli 程序的结构
* 服务器的启动和实现
* 目录和文件的读取
* 渲染
* 响应请求
* 压缩

所以这篇笔记的思路就按照上方的几个重点来开展。

### cli 程序的结构
和很多命令行程序一样，sfz 也是基于 [clap](https://crates.io/crates/clap) 开发的。作者使用了一种很好的方式，将应用的定义、参数、解析分开在不同的 mod 中，这样看起来不那么零乱了。

#### 参数模式
首先是 app 的定义，位于 src/cli/app.rs 文件中。由于 app 的生命周期贯穿整个 sfz 的使用，因此有如下定义：

```rust
fn app() -> clap::Command<'static>
```

函数体中是一些参数的模式：

```rust
let arg_port = Arg::new("port")
    .short('p')
    .long("port")
    .default_value("5000")
    .help("Specify port to listen on")
    .value_name("port");
```

参数的匹配独立到 app 函数中，有利于后期的管理和维护，如果要增加或更新，直接修改 app 函数即可。

#### 参数的解析
参数解析位于 src/cli/args.rs 文件，命令行参数虽然是看似零乱的标记，但通过匹配拿到后，可以将其放在一个特定的结构中，sfz 就是如此：

```rust
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Args {
    pub address: String,
    pub port: u16,
    pub cache: u64,
    pub cors: bool,
    pub coi: bool,
    pub compress: bool,
    pub path: PathBuf,
    pub all: bool,
    pub ignore: bool,
    pub follow_links: bool,
    pub render_index: bool,
    pub log: bool,
    pub path_prefix: Option<String>,
}
```

-- todo


### 服务器的启动和实现


### 目录和文件的读取
### 渲染
### 渲染
### 压缩
### 响应请求

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
