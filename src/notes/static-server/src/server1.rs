use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;

/// hello world handle collection
/// 可以编写若干个 HC，每个 HC 中都有自己的路由和 handler 集合
pub async fn hello_world_hc(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    dbg!(req.uri().path());
    dbg!(req.method());
    // 通过 method 和 path 定义一个 handler  todo

    Ok(Response::new("Hello, World 111-1".into()))
}
