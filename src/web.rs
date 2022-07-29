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

use std::collections::HashMap;
use std::convert::Infallible;
use std::net::{Ipv4Addr, SocketAddrV4};

use anyhow::{Context, Result};
use hyper::service::{make_service_fn, service_fn};
use hyper::{body, Body, Request, Response, Server};
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, mpsc, oneshot};
use tokio_stream::{wrappers::BroadcastStream, StreamExt};

use bincode;

use crate::position::Position;
use crate::{Anchor, MacAddress, PicaCommand, PicaEvent};
use PicaEvent::{DeviceAdded, DeviceRemoved, NeighborUpdated, PositionUpdated};

const WEB_PORT: u16 = 3000;

const STATIC_FILES: &[(&str, &str, &str)] = &[
    ("/", "text/html", include_str!("../static/index.html")),
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Category {
    Uci,
    Anchor,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebDevice {
    pub category: Category,
    pub mac_address: MacAddress,
    pub position: Position,
}

impl WebDevice {
    pub fn new(category: Category, mac_address: MacAddress, position: Position) -> Self {
        Self {
            category,
            mac_address,
            position,
        }
    }
}

impl From<Anchor> for WebDevice {
    fn from(anchor: Anchor) -> Self {
        Self {
            category: Category::Anchor,
            mac_address: anchor.mac_address,
            position: anchor.position,
        }
    }
}

fn event_name(event: &PicaEvent) -> &'static str {
    match event {
        DeviceAdded { .. } => "device-added",
        DeviceRemoved { .. } => "device-removed",
        PositionUpdated { .. } => "position-updated",
        NeighborUpdated { .. } => "neighbor-updated",
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

    let query_params = req
        .uri()
        .query()
        .into_iter()
        .flat_map(|query| query.split("&"))
        .map(|param| param.split_once("=").unwrap_or((param, "")))
        .collect::<HashMap<_, _>>();
    match req
        .uri()
        .path()
        .trim_start_matches("/")
        .split("/")
        .collect::<Vec<_>>()[..]
    {
        ["/events"] => {
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
        ["/init-uci-device", mac_address] => {
            let position = match query_params.get("position") {
                Some(position) => position,
                None => unimplemented!(),
            };

            // tx.send(PicaCommand::InitUciDevice(body.mac_address, position))
            //     .await
            //     .unwrap();

            return Ok(Response::builder().status(200).body("".into()).unwrap());
        }
        ["/set_position", mac_address] => {
            #[derive(Deserialize)]
            struct SetPositionBody {
                mac_address: MacAddress,
                position: Position,
            }

            let body = body::to_bytes(req.body_mut()).await.unwrap();
            let body: SetPositionBody = serde_json::from_slice(&body).unwrap();

            tx.send(PicaCommand::SetPosition(body.mac_address, body.position))
                .await
                .unwrap();

            return Ok(Response::builder().status(200).body("".into()).unwrap());
        }
        ["/create-anchor", mac_address] => {
            let position = match query_params.get("position") {
                Some(position) => position,
                None => unimplemented!(),
            };

            // tx.send(PicaCommand::CreateAnchor(body.mac_address, position))
            //     .await
            //     .unwrap();
            println!("create anchor position: {}", position);

            return Ok(Response::builder().status(200).body("".into()).unwrap());
        }
        ["/destroy-anchor", mac_address] => {
            #[derive(Deserialize)]
            struct DestroyAnchorBody {
                mac_address: MacAddress,
            }

            let body = body::to_bytes(req.body_mut()).await.unwrap();
            let body: DestroyAnchorBody = serde_json::from_slice(&body).unwrap();
            tx.send(PicaCommand::DestroyAnchor(body.mac_address))
                .await
                .unwrap();

            return Ok(Response::builder().status(200).body("".into()).unwrap());
        }
        ["/get-state"] => {
            #[derive(Serialize)]
            struct GetStateBody {
                devices: Vec<WebDevice>,
            }
            let (state_tx, state_rx) = oneshot::channel::<Vec<WebDevice>>();
            tx.send(PicaCommand::GetState(state_tx)).await.unwrap();
            let devices = match state_rx.await {
                Ok(devices) => devices,
                Err(_) => Vec::new(),
            };

            let get_state_body = GetStateBody { devices };
            let serialize_body = bincode::serialize(&get_state_body).unwrap();
            return Ok(Response::builder()
                .status(200)
                .body(serialize_body.into())
                .unwrap());
        }
        _ => (),
    }

    println!("404 error: {}", req.uri().path());
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
