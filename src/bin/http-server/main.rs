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

use anyhow::Result;
use clap::Parser;
use hyper::service::{make_service_fn, service_fn};
use hyper::{body, Body, Request, Response, Server, StatusCode as HttpStatusCode};
use serde::{Deserialize, Serialize};
use serde_json::error::Category as SerdeErrorCategory;
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::net::TcpListener;
use tokio::sync::{broadcast, mpsc, oneshot};
use tokio::try_join;
use tokio_stream::{wrappers::BroadcastStream, StreamExt};

use pica::{Category, MacAddress, Pica, PicaCommand, PicaCommandError, PicaEvent};

mod position;
use position::Position;

const DEFAULT_UCI_PORT: u16 = 7000;
const DEFAULT_WEB_PORT: u16 = 3000;

const STATIC_FILES: &[(&str, &str, &str)] = &[
    ("/", "text/html", include_str!("../../../static/index.html")),
    (
        "/openapi",
        "text/html",
        include_str!("../../../static/openapi.html"),
    ),
    (
        "/openapi.yaml",
        "text/yaml",
        include_str!("../../../static/openapi.yaml"),
    ),
    (
        "/src/components/Map.js",
        "application/javascript",
        include_str!("../../../static/src/components/Map.js"),
    ),
    (
        "/src/components/DeviceInfo.js",
        "application/javascript",
        include_str!("../../../static/src/components/DeviceInfo.js"),
    ),
    (
        "/src/components/Orientation.js",
        "application/javascript",
        include_str!("../../../static/src/components/Orientation.js"),
    ),
];

/// Record information about an active device.
#[derive(Debug, Serialize, Clone)]
struct DeviceInformation {
    category: pica::Category,
    mac_address: MacAddress,
    #[serde(flatten)]
    position: Position,
}

/// Record information about an active device.
#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum Event {
    DeviceAdded {
        category: Category,
        mac_address: MacAddress,
        #[serde(flatten)]
        position: Position,
    },
    DeviceRemoved {
        category: Category,
        mac_address: MacAddress,
    },
    DeviceUpdated {
        category: Category,
        mac_address: MacAddress,
        #[serde(flatten)]
        position: Position,
    },
    NeighborUpdated {
        source_category: Category,
        source_mac_address: MacAddress,
        destination_category: Category,
        destination_mac_address: MacAddress,
        distance: u16,
        azimuth: i16,
        elevation: i8,
    },
}

/// Record the position of active devices for reference by the
/// ranging estimator.
#[derive(Clone)]
struct Context {
    devices: Arc<Mutex<HashMap<pica::Handle, DeviceInformation>>>,
    events: broadcast::Sender<Event>,
}

impl Context {
    fn new() -> Self {
        let (events, _) = broadcast::channel(1024);
        Context {
            devices: Arc::new(Mutex::new(HashMap::new())),
            events,
        }
    }

    async fn handle_connection_events(
        self,
        mut events: broadcast::Receiver<PicaEvent>,
    ) -> Result<()> {
        loop {
            match events.recv().await {
                Ok(PicaEvent::Connected {
                    mac_address,
                    handle,
                }) => {
                    let mut devices = self.devices.lock().unwrap();
                    devices.insert(
                        handle,
                        DeviceInformation {
                            category: Category::Uci,
                            mac_address,
                            position: Default::default(),
                        },
                    );
                    self.events
                        .send(Event::DeviceAdded {
                            category: Category::Uci,
                            mac_address,
                            position: Default::default(),
                        })
                        .unwrap();
                }
                Ok(PicaEvent::Disconnected {
                    mac_address,
                    handle,
                }) => {
                    let mut devices = self.devices.lock().unwrap();
                    devices.remove(&handle);
                    self.events
                        .send(Event::DeviceRemoved {
                            category: Category::Uci,
                            mac_address,
                        })
                        .unwrap();
                }
                Err(err) => anyhow::bail!(err),
            }
        }
    }

    fn http_events(&self) -> Response<Body> {
        let stream = BroadcastStream::new(self.events.subscribe()).map(|result| {
            result.map(|event| {
                format!(
                    "event: {}\ndata: {}\n\n",
                    event.name(),
                    serde_json::to_string(&event).unwrap()
                )
            })
        });
        Response::builder()
            .header("content-type", "text/event-stream")
            .body(Body::wrap_stream(stream))
            .unwrap()
    }

    fn http_set_position(&self, mac_address: MacAddress, position: Position) -> Response<Body> {
        log::info!("set-position({}, {})", mac_address, position);

        let mut devices = self.devices.lock().unwrap();
        let mut found_device = None;
        for (_, device) in devices.iter_mut() {
            if device.mac_address == mac_address {
                device.position = position;
                found_device = Some(device.clone());
                break;
            }
        }

        let Some(device) = found_device else {
            return Response::builder()
                .status(HttpStatusCode::NOT_FOUND)
                .body("".into())
                .unwrap();
        };

        self.events
            .send(Event::DeviceUpdated {
                category: device.category,
                mac_address,
                position,
            })
            .unwrap();

        for other in devices.values() {
            if other.mac_address != device.mac_address {
                let local = device
                    .position
                    .compute_range_azimuth_elevation(&other.position);
                let remote = other
                    .position
                    .compute_range_azimuth_elevation(&device.position);

                assert!(local.0 == remote.0);

                self.events
                    .send(Event::NeighborUpdated {
                        source_category: device.category,
                        source_mac_address: device.mac_address,
                        destination_category: other.category,
                        destination_mac_address: other.mac_address,
                        distance: local.0,
                        azimuth: local.1,
                        elevation: local.2,
                    })
                    .unwrap();

                let _ = self
                    .events
                    .send(Event::NeighborUpdated {
                        source_category: other.category,
                        source_mac_address: other.mac_address,
                        destination_category: device.category,
                        destination_mac_address: device.mac_address,
                        distance: remote.0,
                        azimuth: remote.1,
                        elevation: remote.2,
                    })
                    .unwrap();
            }
        }

        Response::builder()
            .status(HttpStatusCode::OK)
            .body("".into())
            .unwrap()
    }

    async fn http_create_anchor(
        &self,
        mac_address: MacAddress,
        position: Position,
        cmd_tx: mpsc::Sender<PicaCommand>,
    ) -> Response<Body> {
        log::info!("create-anchor({}, {})", mac_address, position);

        let (rsp_tx, rsp_rx) = oneshot::channel::<Result<pica::Handle, PicaCommandError>>();
        cmd_tx
            .send(PicaCommand::CreateAnchor(mac_address, rsp_tx))
            .await
            .unwrap();

        let status = match rsp_rx.await {
            Ok(Ok(handle)) => {
                let mut devices = self.devices.lock().unwrap();
                devices.insert(
                    handle,
                    DeviceInformation {
                        position,
                        mac_address,
                        category: Category::Anchor,
                    },
                );
                self.events
                    .send(Event::DeviceAdded {
                        category: Category::Anchor,
                        mac_address,
                        position,
                    })
                    .unwrap();
                HttpStatusCode::OK
            }
            Ok(Err(PicaCommandError::DeviceAlreadyExists(_))) => HttpStatusCode::CONFLICT,
            Ok(Err(PicaCommandError::DeviceNotFound(_))) => HttpStatusCode::NOT_FOUND,
            Err(_) => HttpStatusCode::INTERNAL_SERVER_ERROR,
        };

        Response::builder().status(status).body("".into()).unwrap()
    }

    async fn http_destroy_anchor(
        &self,
        mac_address: MacAddress,
        cmd_tx: mpsc::Sender<PicaCommand>,
    ) -> Response<Body> {
        log::info!("destroy-anchor({})", mac_address);

        let (rsp_tx, rsp_rx) = oneshot::channel::<Result<pica::Handle, PicaCommandError>>();
        cmd_tx
            .send(PicaCommand::DestroyAnchor(mac_address, rsp_tx))
            .await
            .unwrap();

        let status = match rsp_rx.await {
            Ok(Ok(handle)) => {
                let mut devices = self.devices.lock().unwrap();
                devices.remove(&handle);
                self.events
                    .send(Event::DeviceRemoved {
                        category: Category::Anchor,
                        mac_address,
                    })
                    .unwrap();
                HttpStatusCode::OK
            }
            Ok(Err(PicaCommandError::DeviceAlreadyExists(_))) => HttpStatusCode::CONFLICT,
            Ok(Err(PicaCommandError::DeviceNotFound(_))) => HttpStatusCode::NOT_FOUND,
            Err(_) => HttpStatusCode::INTERNAL_SERVER_ERROR,
        };

        Response::builder().status(status).body("".into()).unwrap()
    }

    fn http_get_state(&self) -> Response<Body> {
        log::info!("get-state()");

        #[derive(Serialize)]
        struct GetStateResponse {
            devices: Vec<DeviceInformation>,
        }

        let devices = self.devices.lock().unwrap();
        let response = GetStateResponse {
            devices: devices.values().cloned().collect::<Vec<_>>(),
        };
        let body = serde_json::to_string(&response).unwrap();
        Response::builder()
            .status(HttpStatusCode::OK)
            .body(body.into())
            .unwrap()
    }
}

impl pica::RangingEstimator for Context {
    fn estimate(
        &self,
        left: &pica::Handle,
        right: &pica::Handle,
    ) -> anyhow::Result<pica::RangingMeasurement> {
        let devices = self
            .devices
            .lock()
            .map_err(|_| anyhow::anyhow!("cannot take lock"))?;
        let left_pos = devices
            .get(left)
            .ok_or(anyhow::anyhow!("unknown position"))?
            .position;
        let right_pos = devices
            .get(right)
            .ok_or(anyhow::anyhow!("unknown position"))?
            .position;
        let (range, azimuth, elevation) = left_pos.compute_range_azimuth_elevation(&right_pos);
        Ok(pica::RangingMeasurement {
            range,
            azimuth,
            elevation,
        })
    }
}

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
                    log::error!("{}", reason);
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
                log::error!("{}", reason);
                return Ok(Response::builder().status(406).body(reason.into()).unwrap());
            }
        }
    };
}

impl Event {
    fn name(&self) -> &'static str {
        match self {
            Event::DeviceAdded { .. } => "device-added",
            Event::DeviceRemoved { .. } => "device-removed",
            Event::DeviceUpdated { .. } => "device-updated",
            Event::NeighborUpdated { .. } => "neighbor-updated",
        }
    }
}

async fn http_request(
    mut req: Request<Body>,
    cmd_tx: mpsc::Sender<PicaCommand>,
    context: Context,
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
    let response = match req
        .uri_mut()
        .path()
        .trim_start_matches('/')
        .split('/')
        .collect::<Vec<_>>()[..]
    {
        ["events"] => context.http_events(),
        ["init-uci-device", mac_address] => {
            context.http_set_position(mac_address!(mac_address), position!(body))
        }
        ["set-position", mac_address] => {
            context.http_set_position(mac_address!(mac_address), position!(body))
        }
        ["create-anchor", mac_address] => {
            context
                .http_create_anchor(mac_address!(mac_address), position!(body), cmd_tx)
                .await
        }
        ["destroy-anchor", mac_address] => {
            context
                .http_destroy_anchor(mac_address!(mac_address), cmd_tx)
                .await
        }
        ["get-state"] => context.http_get_state(),

        _ => Response::builder()
            .status(HttpStatusCode::NOT_FOUND)
            .body("".into())
            .unwrap(),
    };

    Ok(response)
}

async fn serve(context: Context, tx: mpsc::Sender<PicaCommand>, web_port: u16) -> Result<()> {
    let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, web_port);
    let make_svc = make_service_fn(move |_conn| {
        let tx = tx.clone();
        let local_context = context.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                http_request(req, tx.clone(), local_context.clone())
            }))
        }
    });

    let server = Server::bind(&addr.into()).serve(make_svc);

    log::info!("Pica: Web server started on http://0.0.0.0:{}", web_port);

    server.await?;
    Ok(())
}

async fn listen(tx: mpsc::Sender<PicaCommand>, uci_port: u16) -> Result<()> {
    let uci_socket = SocketAddrV4::new(Ipv4Addr::LOCALHOST, uci_port);
    let uci_listener = TcpListener::bind(uci_socket).await?;
    log::info!("Pica: Listening on: {}", uci_port);

    loop {
        let (socket, addr) = uci_listener.accept().await?;
        log::info!("Uwb host addr: {}", addr);

        let (read_half, write_half) = socket.into_split();
        let stream = Box::pin(futures::stream::unfold(read_half, pica::packets::uci::read));
        let sink = Box::pin(futures::sink::unfold(write_half, pica::packets::uci::write));

        tx.send(PicaCommand::Connect(stream, sink))
            .await
            .map_err(|_| anyhow::anyhow!("pica command stream closed"))?
    }
}

#[derive(Parser, Debug)]
#[command(name = "pica", about = "Virtual UWB subsystem")]
struct Args {
    /// Output directory for storing .pcapng traces.
    /// If provided, .pcapng traces of client connections are automatically
    /// saved under the name `device-{handle}.pcapng`.
    #[arg(short, long, value_name = "DIR")]
    pcapng_dir: Option<PathBuf>,
    /// Configure the TCP port for the UCI server.
    #[arg(short, long, value_name = "PORT", default_value_t = DEFAULT_UCI_PORT)]
    uci_port: u16,
    /// Configure the HTTP port for the web interface.
    #[arg(short, long, value_name = "PORT", default_value_t = DEFAULT_WEB_PORT)]
    web_port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    assert_ne!(
        args.uci_port, args.web_port,
        "UCI port and WEB port must be different."
    );

    let context = Context::new();

    let pica = Pica::new(Box::new(context.clone()), args.pcapng_dir);
    let cmd_tx = pica.commands();
    let events_rx = pica.events();

    try_join!(
        pica.run(),
        listen(cmd_tx.clone(), args.uci_port),
        serve(context.clone(), cmd_tx.clone(), args.web_port),
        context.handle_connection_events(events_rx),
    )?;

    Ok(())
}
