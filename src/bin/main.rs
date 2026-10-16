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

mod common;

use clap::Parser;
use common::{DeviceEvent, drain_packets, spawn_device, wait_for_deadline};
use pica::Pica;
use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::PathBuf;
use thiserror::Error;
use tokio::net::TcpListener;
use tokio::sync::mpsc;

const DEFAULT_UCI_PORT: u16 = 7000;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Parser, Debug)]
#[command(name = "pica", about = "Virtual UWB subsystem")]
struct Args {
    /// Output directory for storing .pcapng traces.
    /// If provided, .pcapng traces of client connections are automatically
    /// saved under the name `device-{handle}.pcapng`.
    #[arg(short, long, value_name = "PCAPNG_DIR")]
    pcapng_dir: Option<PathBuf>,
    /// Configure the TCP port for the UCI server.
    #[arg(short, long, value_name = "UCI_PORT", default_value_t = DEFAULT_UCI_PORT)]
    uci_port: u16,
}

struct MockRangingEstimator();

/// The position cannot be communicated to the pica environment when
/// using the default binary (HTTP interface not available).
/// Thus the ranging estimator cannot produce any result.
impl pica::RangingEstimator for MockRangingEstimator {
    fn estimate(
        &self,
        _left: &pica::Handle,
        _right: &pica::Handle,
    ) -> Option<pica::RangingMeasurement> {
        Some(Default::default())
    }
}

#[tokio::main(flavor = "current_thread")]
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

    let uci_socket = SocketAddrV4::new(Ipv4Addr::LOCALHOST, args.uci_port);
    let uci_listener = TcpListener::bind(uci_socket).await?;
    let uci_port = args.uci_port;
    tracing::info!("Pica: Listening on: {uci_port}");

    let (event_tx, mut event_rx) = mpsc::unbounded_channel::<DeviceEvent>();
    let mut device_sinks: HashMap<pica::Handle, mpsc::UnboundedSender<pica::UciPacket>> =
        HashMap::new();

    let mut pica = Pica::new(Box::new(MockRangingEstimator()));

    loop {
        let deadline = pica.next_deadline();

        tokio::select! {
            res = uci_listener.accept() => {
                let (socket, addr) = res?;
                tracing::info!("Uwb host addr: {addr}");

                let handle = pica.add_device();
                let sink = spawn_device(handle, socket, args.pcapng_dir.as_deref(), &event_tx);
                device_sinks.insert(handle, sink);
            }
            () = wait_for_deadline(deadline) => {
                pica.tick();
            }
            Some(event) = event_rx.recv() => {
                match event {
                    DeviceEvent::Packet(handle, packet) => {
                        pica.handle_packet(handle, &packet);
                    }
                    DeviceEvent::Disconnected(handle) => {
                        pica.remove_device(handle);
                        device_sinks.remove(&handle);
                    }
                }
            }
        }

        drain_packets(&mut pica, &device_sinks);
    }
}
