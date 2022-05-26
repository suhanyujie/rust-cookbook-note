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

所以在请求到来时，我们需要将获取到的 `/mydir1` 参数值拼接上 basePath —— `./someDir`，得到 `./someDir/mydir1` 的路径，进而获取该路径下的文件列表（前提是该路径是合法的目录）。send_dir() 就是获取文件列表的实现：

```rust
/// Send a HTML page of all files under the path.
///
/// # Parameters
///
/// * `dir_path` - Directory to be listed files.
/// * `base_path` - The base path resolving all filepaths under `dir_path`.
/// * `show_all` - Whether to show hidden and 'dot' files.
/// * `with_ignore` - Whether to respet gitignore files.
/// * `path_prefix` - The url path prefix optionally defined
pub fn send_dir<P1: AsRef<Path>, P2: AsRef<Path>>(
    dir_path: P1,
    base_path: P2,
    show_all: bool,
    with_ignore: bool,
    path_prefix: Option<&str>,
) -> io::Result<Vec<u8>> {
    // ...
}
```

可是 `./someDir/mydir1` 是相对路径，我们需要将其转换为更加可读的路径，除此之外，我们还要防止非法的输入，比如 `./../../../user/somePath`。sfz 的做法如下：

```rust
/// Parse path.
fn parse_path<P: AsRef<Path>>(path: P) -> BoxResult<PathBuf> {
    let path = path.as_ref();
    if !path.exists() {
        bail!("error: path \"{}\" doesn't exist", path.display());
    }

    env::current_dir()
        .and_then(|mut p| {
            p.push(path); // If path is absolute, it replaces the current path.
            canonicalize(p)
        })
        .or_else(|err| {
            bail!(
                "error: failed to access path \"{}\": {}",
                path.display(),
                err,
            )
        })
}
```

通过 `env::current_dir()` 获取到当前路径下的绝对路径，再拼接上用户输入的局部路径，然后进行一次路径转换 —— `canonicalize()`，得到一个完整的路径。

根据这个路径，读取其下面的文件和目录。sfz 作者使用了[另一个 crate](https://crates.io/crates/ignore) 可以方便地按照 gitignore 规范，读取目录下的文件：

```rust
/// Walking inside a directory recursively
fn get_dir_contents<P: AsRef<Path>>(
    dir_path: P,
    with_ignore: bool,
    show_all: bool,
    depth: Option<usize>,
) -> ignore::Walk {
    WalkBuilder::new(dir_path)
        .standard_filters(false) // Disable all standard filters.
        .git_ignore(with_ignore)
        .hidden(!show_all) // Filter out hidden entries on demand.
        .max_depth(depth) // Do not traverse subpaths.
        .build()
}
```