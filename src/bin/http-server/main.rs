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

#![cfg_attr(
    test,
    allow(
        clippy::indexing_slicing,
        clippy::panic,
        clippy::panic_in_result_fn,
        clippy::unwrap_used,
    )
)]

use clap::Parser;
use futures::TryFutureExt;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode as HttpStatusCode, body};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use serde_json::error::Category as SerdeErrorCategory;
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::PathBuf;
use std::sync::Arc;
use thiserror::Error;
use tokio::net::TcpListener;
use tokio::sync::{broadcast, mpsc, oneshot};
use tokio::try_join;
use tokio_stream::{StreamExt, wrappers::BroadcastStream};

use pica::{AlreadyExists, Handle, MacAddress, Pica, PicaCommand, PicaEvent};

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Hyper(#[from] hyper::Error),
    #[error(transparent)]
    EventRecv(#[from] tokio::sync::broadcast::error::RecvError),
    #[error("pica command stream closed")]
    CommandStreamClosed,
    #[error("UCI port and web port must be different")]
    PortConflict,
    #[error(transparent)]
    Pica(#[from] pica::PicaError),
}

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

#[derive(Clone, Copy, Debug, Serialize)]
enum Category {
    Uci,
    Anchor,
}

/// Record information about an active device.
#[derive(Debug, Serialize, Clone)]
struct DeviceInformation {
    handle: Handle,
    category: Category,
    #[serde(skip_serializing_if = "Option::is_none")]
    mac_address: Option<MacAddress>,
    #[serde(flatten)]
    position: Position,
}

/// Record information about an active device.
#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
enum Event {
    DeviceAdded {
        handle: Handle,
        category: Category,
        #[serde(skip_serializing_if = "Option::is_none")]
        mac_address: Option<MacAddress>,
        #[serde(flatten)]
        position: Position,
    },
    DeviceRemoved {
        handle: Handle,
        category: Category,
    },
    DeviceUpdated {
        handle: Handle,
        category: Category,
        #[serde(flatten)]
        position: Position,
    },
    NeighborUpdated {
        source_handle: Handle,
        source_category: Category,
        destination_handle: Handle,
        destination_category: Category,
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
    ) -> Result<(), Error> {
        loop {
            match events.recv().await {
                Ok(PicaEvent::Connected { handle }) => {
                    let mut devices = self.devices.lock();
                    devices.insert(
                        handle,
                        DeviceInformation {
                            handle,
                            category: Category::Uci,
                            mac_address: None,
                            position: Default::default(),
                        },
                    );
                    let _ = self.events.send(Event::DeviceAdded {
                        handle,
                        category: Category::Uci,
                        mac_address: None,
                        position: Default::default(),
                    });
                }
                Ok(PicaEvent::Disconnected { handle }) => {
                    let mut devices = self.devices.lock();
                    devices.remove(&handle);
                    let _ = self.events.send(Event::DeviceRemoved {
                        handle,
                        category: Category::Uci,
                    });
                }
                Err(err) => return Err(Error::EventRecv(err)),
            }
        }
    }

    fn http_events(&self) -> Response<Body> {
        let stream = BroadcastStream::new(self.events.subscribe()).map(|result| {
            result.map(|event| {
                let data = serde_json::to_string(&event).expect("failed to serialize JSON");
                format!("event: {}\ndata: {data}\n\n", event.name())
            })
        });
        Response::builder()
            .header("content-type", "text/event-stream")
            .body(Body::wrap_stream(stream))
            .expect("failed to build HTTP response")
    }

    fn http_set_position(&self, handle: Handle, position: Position) -> Response<Body> {
        tracing::info!("set-position({handle}, {position})");

        let mut devices = self.devices.lock();
        let Some(device) = devices.get_mut(&handle) else {
            return Response::builder()
                .status(HttpStatusCode::NOT_FOUND)
                .body(Body::empty())
                .expect("failed to build HTTP response");
        };
        device.position = position;
        let device = device.clone();

        let _ = self.events.send(Event::DeviceUpdated {
            handle,
            category: device.category,
            position,
        });

        for other in devices.values() {
            if other.handle != device.handle {
                let local = device
                    .position
                    .compute_range_azimuth_elevation(&other.position);
                let remote = other
                    .position
                    .compute_range_azimuth_elevation(&device.position);

                let _ = self.events.send(Event::NeighborUpdated {
                    source_handle: device.handle,
                    source_category: device.category,
                    destination_handle: other.handle,
                    destination_category: other.category,
                    distance: local.0,
                    azimuth: local.1,
                    elevation: local.2,
                });

                let _ = self.events.send(Event::NeighborUpdated {
                    source_handle: other.handle,
                    source_category: other.category,
                    destination_handle: device.handle,
                    destination_category: device.category,
                    distance: remote.0,
                    azimuth: remote.1,
                    elevation: remote.2,
                });
            }
        }

        Response::builder()
            .status(HttpStatusCode::OK)
            .body(Body::empty())
            .expect("failed to build HTTP response")
    }

    async fn http_create_anchor(
        &self,
        mac_address: MacAddress,
        position: Position,
        cmd_tx: mpsc::Sender<PicaCommand>,
    ) -> Response<Body> {
        tracing::info!("create-anchor({mac_address}, {position})");

        let (rsp_tx, rsp_rx) = oneshot::channel::<Result<Handle, AlreadyExists>>();
        if let Err(e) = cmd_tx
            .send(PicaCommand::CreateAnchor(mac_address, rsp_tx))
            .await
        {
            tracing::debug!("Channel closed: {e}");
            return Response::builder()
                .status(HttpStatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .expect("failed to build HTTP response");
        }

        match rsp_rx.await {
            Ok(Ok(handle)) => {
                let mut devices = self.devices.lock();
                devices.insert(
                    handle,
                    DeviceInformation {
                        handle,
                        category: Category::Anchor,
                        mac_address: Some(mac_address),
                        position,
                    },
                );
                let _ = self.events.send(Event::DeviceAdded {
                    handle,
                    category: Category::Anchor,
                    mac_address: Some(mac_address),
                    position,
                });
                Response::builder()
                    .status(HttpStatusCode::OK)
                    .header("content-type", "application/json")
                    .body(serde_json::json!({ "handle": handle }).to_string().into())
                    .expect("failed to build HTTP response")
            }
            Ok(Err(AlreadyExists(_))) => Response::builder()
                .status(HttpStatusCode::CONFLICT)
                .body(Body::empty())
                .expect("failed to build HTTP response"),
            Err(_) => Response::builder()
                .status(HttpStatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .expect("failed to build HTTP response"),
        }
    }

    async fn http_destroy_anchor(
        &self,
        handle: Handle,
        cmd_tx: mpsc::Sender<PicaCommand>,
    ) -> Response<Body> {
        tracing::info!("destroy-anchor({handle})");

        let (rsp_tx, rsp_rx) = oneshot::channel::<bool>();
        if let Err(e) = cmd_tx
            .send(PicaCommand::DestroyAnchor(handle, rsp_tx))
            .await
        {
            tracing::debug!("Channel closed: {e}");
            return Response::builder()
                .status(HttpStatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .expect("failed to build HTTP response");
        }

        let status = match rsp_rx.await {
            Ok(true) => {
                let mut devices = self.devices.lock();
                devices.remove(&handle);
                let _ = self.events.send(Event::DeviceRemoved {
                    handle,
                    category: Category::Anchor,
                });
                HttpStatusCode::OK
            }
            Ok(false) => HttpStatusCode::NOT_FOUND,
            Err(_) => HttpStatusCode::INTERNAL_SERVER_ERROR,
        };

        Response::builder()
            .status(status)
            .body(Body::empty())
            .expect("failed to build HTTP response")
    }

    fn http_get_state(&self) -> Response<Body> {
        tracing::info!("get-state()");

        #[derive(Serialize)]
        struct GetStateResponse {
            devices: Vec<DeviceInformation>,
        }

        let devices = self.devices.lock();
        let response = GetStateResponse {
            devices: devices.values().cloned().collect::<Vec<_>>(),
        };
        let body = serde_json::to_string(&response).expect("failed to serialize JSON");
        Response::builder()
            .status(HttpStatusCode::OK)
            .body(body.into())
            .expect("failed to build HTTP response")
    }
}

impl pica::RangingEstimator for Context {
    fn estimate(
        &self,
        left: &pica::Handle,
        right: &pica::Handle,
    ) -> Option<pica::RangingMeasurement> {
        let devices = self.devices.lock();
        let left_pos = devices.get(left)?.position;
        let right_pos = devices.get(right)?.position;
        let (range, azimuth, elevation) = left_pos.compute_range_azimuth_elevation(&right_pos);
        Some(pica::RangingMeasurement {
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
                    let reason = format!("Error while deserializing position: {err}");
                    tracing::error!("{reason}");
                    return Ok(Response::builder()
                        .status(HttpStatusCode::NOT_ACCEPTABLE)
                        .body(reason.into())
                        .expect("failed to build HTTP response"));
                }
            }
        }
    };
}

macro_rules! handle {
    ($handle: ident) => {
        match $handle.parse::<Handle>() {
            Ok(handle) => handle,
            Err(err) => {
                let reason = format!("Error handle: {err}");
                tracing::error!("{reason}");
                return Ok(Response::builder()
                    .status(HttpStatusCode::NOT_ACCEPTABLE)
                    .body(reason.into())
                    .expect("failed to build HTTP response"));
            }
        }
    };
}

macro_rules! mac_address {
    ($mac_address: ident) => {
        match MacAddress::new($mac_address.to_string()) {
            Ok(mac_address) => mac_address,
            Err(err) => {
                let reason = format!("Error mac_address: {err}");
                tracing::error!("{reason}");
                return Ok(Response::builder()
                    .status(HttpStatusCode::NOT_ACCEPTABLE)
                    .body(reason.into())
                    .expect("failed to build HTTP response"));
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
            .expect("failed to build HTTP response"));
    }

    let Ok(body) = body::to_bytes(req.body_mut()).await else {
        return Ok(Response::builder()
            .status(HttpStatusCode::BAD_REQUEST)
            .body(Body::empty())
            .expect("failed to build HTTP response"));
    };
    let path_segments = req
        .uri_mut()
        .path()
        .trim_start_matches('/')
        .split('/')
        .collect::<Vec<_>>();
    let response = match path_segments.as_slice() {
        ["events"] => context.http_events(),
        ["init-uci-device", handle] => context.http_set_position(handle!(handle), position!(body)),
        ["set-position", handle] => context.http_set_position(handle!(handle), position!(body)),
        ["create-anchor", mac_address] => {
            context
                .http_create_anchor(mac_address!(mac_address), position!(body), cmd_tx)
                .await
        }
        ["destroy-anchor", handle] => context.http_destroy_anchor(handle!(handle), cmd_tx).await,
        ["get-state"] => context.http_get_state(),

        _ => Response::builder()
            .status(HttpStatusCode::NOT_FOUND)
            .body(Body::empty())
            .expect("failed to build HTTP response"),
    };

    Ok(response)
}

async fn serve(
    context: Context,
    tx: mpsc::Sender<PicaCommand>,
    web_port: u16,
) -> Result<(), Error> {
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

    tracing::info!("Pica: Web server started on http://0.0.0.0:{web_port}");

    server.await?;
    Ok(())
}

async fn listen(tx: mpsc::Sender<PicaCommand>, uci_port: u16) -> Result<(), Error> {
    let uci_socket = SocketAddrV4::new(Ipv4Addr::LOCALHOST, uci_port);
    let uci_listener = TcpListener::bind(uci_socket).await?;
    tracing::info!("Pica: Listening on: {uci_port}");

    loop {
        let (socket, addr) = uci_listener.accept().await?;
        tracing::info!("Uwb host addr: {addr}");

        let (read_half, write_half) = socket.into_split();
        let stream = Box::pin(futures::stream::unfold(read_half, pica::packets::uci::read));
        let sink = Box::pin(futures::sink::unfold(write_half, pica::packets::uci::write));

        tx.send(PicaCommand::Connect(stream, sink))
            .await
            .map_err(|_| Error::CommandStreamClosed)?
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
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_ansi(std::io::IsTerminal::is_terminal(&std::io::stderr()))
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("debug")),
        )
        .init();

    let args = Args::parse();
    if args.uci_port == args.web_port {
        return Err(Error::PortConflict);
    }

    let context = Context::new();

    let pica = Pica::new(Box::new(context.clone()), args.pcapng_dir);
    let cmd_tx = pica.commands();
    let events_rx = pica.events();

    try_join!(
        pica.run().map_err(Error::from),
        listen(cmd_tx.clone(), args.uci_port),
        serve(context.clone(), cmd_tx.clone(), args.web_port),
        context.handle_connection_events(events_rx),
    )?;

    Ok(())
}
