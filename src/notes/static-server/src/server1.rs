use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::env;
use std::fs::canonicalize;
use std::path::{Path, PathBuf};

pub type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

macro_rules! bail {
    ($($tt:tt)*) => {
        return Err(From::from(format!($($tt)*)))
    }
}

/// hello world handle collection
/// 可以编写若干个 HC，每个 HC 中都有自己的路由和 handler 集合
pub async fn hello_world_hc(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    dbg!(req.uri().path());
    dbg!(req.method());
    // 通过 method 和 path 定义一个 handler  todo
    let res = parse_path("./");
    dbg!(res);

    Ok(Response::new("Hello, World 111-1".into()))
}

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
