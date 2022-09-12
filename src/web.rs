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
use hyper::{body, Body, Request, Response, Server, StatusCode as HttpStatusCode};
use serde::{Deserialize, Serialize};
use serde_json::error::Category as SerdeErrorCategory;
use tokio::sync::{broadcast, mpsc, oneshot};
use tokio_stream::{wrappers::BroadcastStream, StreamExt};

use crate::position::Position;
use crate::{Anchor, MacAddress, PicaCommand, PicaCommandError, PicaCommandStatus, PicaEvent};
use PicaEvent::{DeviceAdded, DeviceRemoved, DeviceUpdated, NeighborUpdated};

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
];

#[derive(Deserialize)]
struct PositionBody {
    x: i16,
    y: i16,
    z: i16,
    yaw: i16,
    pitch: i8,
    roll: i16,
}

macro_rules! position {
    ($body: ident) => {
        position!($body, false)
    };
    ($body: ident, $mandatory: ident) => {
        match serde_json::from_slice::<PositionBody>(&$body) {
            Ok(body) => Position::new(body.x, body.y, body.z, body.yaw, body.pitch, body.roll),
            Err(err) => {
                if !$mandatory && err.classify() == SerdeErrorCategory::Eof {
                    Position::default()
                } else {
                    let reason = format!("Error while deserializing position: {}", err);
                    println!("{}", reason);
                    return Ok(Response::builder().status(406).body(reason.into()).unwrap());
                }
            }
        }
    };
}

macro_rules! mac_address {
    ($mac_address: ident) => {
        match MacAddress::new($mac_address.to_string()) {
            Ok(mac_address) => mac_address,
            Err(err) => {
                let reason = format!("Error mac_address: {}", err);
                println!("{}", reason);
                return Ok(Response::builder().status(406).body(reason.into()).unwrap());
            }
        }
    };
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Category {
    Uci,
    Anchor,
}

#[derive(Debug, Serialize, Clone)]
pub struct Device {
    pub category: Category,
    pub mac_address: String,
    #[serde(flatten)]
    pub position: Position,
}

impl Device {
    pub fn new(category: Category, mac_address: MacAddress, position: Position) -> Self {
        Self {
            category,
            mac_address: mac_address.to_string(),
            position,
        }
    }
}

impl From<Anchor> for Device {
    fn from(anchor: Anchor) -> Self {
        Self {
            category: Category::Anchor,
            mac_address: anchor.mac_address.to_string(),
            position: anchor.position,
        }
    }
}

fn event_name(event: &PicaEvent) -> &'static str {
    match event {
        DeviceAdded { .. } => "device-added",
        DeviceRemoved { .. } => "device-removed",
        DeviceUpdated { .. } => "device-updated",
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

    let body = body::to_bytes(req.body_mut()).await.unwrap();
    let (pica_cmd_rsp_tx, pica_cmd_rsp_rx) = oneshot::channel::<PicaCommandStatus>();

    let send_cmd = |pica_cmd| async {
        println!("PicaCommand: {}", pica_cmd);
        tx.send(pica_cmd).await.unwrap();
        let (status, description) = match pica_cmd_rsp_rx.await {
            Ok(Ok(_)) => (HttpStatusCode::OK, "success".into()),
            Ok(Err(err)) => (match err {
                PicaCommandError::DeviceAlreadyExists(_) => HttpStatusCode::CONFLICT,
                PicaCommandError::DeviceNotFound(_) => HttpStatusCode::NOT_FOUND,
            }, format!("{}", err)),
            Err(err) =>
                (HttpStatusCode::INTERNAL_SERVER_ERROR,
                    format!("Error getting command response: {}", err))
        };
        println!("  status: {}, {}", status, description);
        Response::builder().status(status).body(description.into()).unwrap()
    };

    match req
        .uri_mut()
        .path()
        .trim_start_matches('/')
        .split('/')
        .collect::<Vec<_>>()[..]
    {
        ["events"] => {
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
        ["init-uci-device", mac_address] => {
            return Ok(send_cmd(PicaCommand::InitUciDevice(
                mac_address!(mac_address),
                position!(body),
                pica_cmd_rsp_tx,
            ))
            .await);
        }
        ["set-position", mac_address] => {
            return Ok(send_cmd(PicaCommand::SetPosition(
                mac_address!(mac_address),
                position!(body),
                pica_cmd_rsp_tx,
            ))
            .await);
        }
        ["create-anchor", mac_address] => {
            return Ok(send_cmd(PicaCommand::CreateAnchor(
                mac_address!(mac_address),
                position!(body),
                pica_cmd_rsp_tx,
            ))
            .await);
        }
        ["destroy-anchor", mac_address] => {
            return Ok(send_cmd(PicaCommand::DestroyAnchor(
                mac_address!(mac_address),
                pica_cmd_rsp_tx,
            ))
            .await);
        }
        ["get-state"] => {
            #[derive(Serialize)]
            struct GetStateResponse {
                devices: Vec<Device>,
            }
            println!("PicaCommand: GetState");
            let (state_tx, state_rx) = oneshot::channel::<Vec<Device>>();
            tx.send(PicaCommand::GetState(state_tx)).await.unwrap();
            let devices = match state_rx.await {
                Ok(devices) => GetStateResponse { devices },
                Err(_) => GetStateResponse { devices: vec![] },
            };
            let body = serde_json::to_string(&devices).unwrap();
            return Ok(Response::builder().status(200).body(body.into()).unwrap());
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
