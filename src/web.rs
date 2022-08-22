// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::convert::Infallible;
use std::net::{Ipv4Addr, SocketAddrV4};

use anyhow::{Context, Result};
use hyper::service::{make_service_fn, service_fn};
use hyper::{body, Body, Request, Response, Server};
use serde::Deserialize;
use tokio::sync::{broadcast, mpsc};
use tokio_stream::{wrappers::BroadcastStream, StreamExt};

use crate::position::Position;
use crate::{MacAddress, PicaCommand, PicaEvent};
use PicaEvent::{AddDevice, RemoveDevice, UpdateNeighbor, UpdatePosition};

const WEB_PORT: u16 = 3000;

const STATIC_FILES: &[(&str, &str, &str)] = &[
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
    (
        "/openapi",
        "text/html",
        include_str!("../static/openapi.html"),
    ),
    (
        "/openapi.yaml",
        "text/yaml",
        include_str!("../static/openapi.yaml"),
    ),
];

fn event_name(event: &PicaEvent) -> &'static str {
    match event {
        AddDevice { .. } => "add-device",
        RemoveDevice { .. } => "remove-device",
        UpdatePosition { .. } => "update-position",
        UpdateNeighbor { .. } => "update-neighbor",
    }
}

async fn handle(
    mut req: Request<Body>,
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
        "/set_position" => {
            #[derive(Deserialize)]
            struct SetPositionBody {
                mac_address: MacAddress,
                x: i16,
                y: i16,
                z: i16,
                yaw: i16,
                pitch: i8,
                roll: i16,
            }

            let body = body::to_bytes(req.body_mut()).await.unwrap();
            let body: SetPositionBody = serde_json::from_slice(&body).unwrap();

            let position = Position::new(body.x, body.y, body.z, body.yaw, body.pitch, body.roll);

            tx.send(PicaCommand::SetPosition(body.mac_address, position))
                .await
                .unwrap();

            return Ok(Response::builder().status(200).body("".into()).unwrap());
        }
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
