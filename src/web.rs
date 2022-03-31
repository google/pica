use std::convert::Infallible;
use std::net::{Ipv4Addr, SocketAddrV4};

use anyhow::{Context, Result};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use tokio::sync::{broadcast, mpsc};
use tokio_stream::{wrappers::BroadcastStream, StreamExt};

use crate::uwb_subsystem::{PicaCommand, PicaEvent};

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

fn event_name(event: &PicaEvent) -> &'static str {
    use PicaEvent::*;
    match event {
        AddDevice { .. } => "add-device",
        RemoveDevice { .. } => "remove-device",
        UpdatePosition { .. } => "update-position",
    }
}

async fn handle(
    req: Request<Body>,
    tx: mpsc::Sender<PicaCommand>,
    events: broadcast::Sender<PicaEvent>,
) -> Result<Response<Body>, Infallible> {
    let static_file = STATIC_FILES
        .iter()
        .find(|(path, _, _)| req.uri().path() == *path);

    if let Some((_, mime, content)) = static_file {
        return Ok(Response::builder()
            .header("content-type", *mime)
            .body((*content).into())
            .unwrap());
    }

    match req.uri().path() {
        "/events" => {
            let stream = BroadcastStream::new(events.subscribe()).map(|result| {
                result.map(|event| {
                    format!(
                        "event: {}\ndata: {}\n\n",
                        event_name(&event),
                        serde_json::to_string(&event).unwrap()
                    )
                })
            });
            return Ok(Response::builder()
                .header("content-type", "text/event-stream")
                .body(Body::wrap_stream(stream))
                .unwrap());
        }
        "/command" => {}
        _ => (),
    }

    Ok(Response::builder().status(404).body("".into()).unwrap())
}

pub async fn serve(
    tx: mpsc::Sender<PicaCommand>,
    events: broadcast::Sender<PicaEvent>,
) -> Result<()> {
    let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, WEB_PORT);

    let make_svc = make_service_fn(move |_conn| {
        let tx = tx.clone();
        let events = events.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                handle(req, tx.clone(), events.clone())
            }))
        }
    });

    let server = Server::bind(&addr.into()).serve(make_svc);

    println!("Pica: Web server started on http://0.0.0.0:{}", WEB_PORT);

    server.await.context("Web Server Error")
}
