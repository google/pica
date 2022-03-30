use std::convert::Infallible;
use std::net::{Ipv4Addr, SocketAddrV4};

use anyhow::{Context, Result};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

const WEB_PORT: u16 = 3000;

const STATIC_FILES: &'static [(&'static str, &'static str, &'static str)] = &[
    ("/", "text/html", include_str!("../static/index.html")),
    (
        "/src/components/Map.js",
        "application/javascript",
        include_str!("../static/src/components/Map.js"),
    ),
    (
        "/src/components/DeviceInfo.js",
        "application/javascript",
        include_str!("../static/src/components/DeviceInfo.js"),
    ),
    (
        "/src/components/Orientation.js",
        "application/javascript",
        include_str!("../static/src/components/Orientation.js"),
    ),
];

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let static_file = STATIC_FILES
        .iter()
        .find(|(path, _, _)| req.uri().path() == *path);

    if let Some((path, mime, content)) = static_file {
        return Ok(Response::builder()
            .header("content-type", *mime)
            .body((*content).into())
            .unwrap());
    }

    Ok(Response::builder().status(404).body("".into()).unwrap())
}

pub async fn serve() -> Result<()> {
    let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, WEB_PORT);

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    let server = Server::bind(&addr.into()).serve(make_svc);

    println!("Pica: Web server started on http://0.0.0.0:{}", WEB_PORT);

    server.await.context("Web Server Error")
}
