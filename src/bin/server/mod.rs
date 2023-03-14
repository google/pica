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

extern crate bytes;
extern crate num_derive;
extern crate num_traits;
extern crate thiserror;

#[cfg(feature = "web")]
mod web;

use anyhow::Result;
use clap::Parser;
use pica::{Pica, PicaCommand};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::PathBuf;
use tokio::net::TcpListener;
use tokio::sync::{broadcast, mpsc};
use tokio::try_join;

const DEFAULT_UCI_PORT: u16 = 7000;
const DEFAULT_WEB_PORT: u16 = 3000;

async fn accept_incoming(tx: mpsc::Sender<PicaCommand>, uci_port: u16) -> Result<()> {
    let uci_socket = SocketAddrV4::new(Ipv4Addr::LOCALHOST, uci_port);
    let uci_listener = TcpListener::bind(uci_socket).await?;
    println!("Pica: Listening on: {}", uci_port);

    loop {
        let (socket, addr) = uci_listener.accept().await?;
        println!("Uwb host addr: {}", addr);
        tx.send(PicaCommand::Connect(socket)).await?
    }
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
    /// Configure the HTTP port for the web interface.
    #[arg(short, long, value_name = "WEB_PORT", default_value_t = DEFAULT_WEB_PORT)]
    web_port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    assert_ne!(
        args.uci_port, args.web_port,
        "UCI port and Web port shall be different."
    );
    let (event_tx, _) = broadcast::channel(16);

    let mut pica = Pica::new(event_tx.clone(), args.pcapng_dir);
    let pica_tx = pica.tx();

    #[cfg(feature = "web")]
    try_join!(
        accept_incoming(pica_tx.clone(), args.uci_port),
        pica.run(),
        web::serve(pica_tx, event_tx, args.web_port)
    )?;

    #[cfg(not(feature = "web"))]
    try_join!(accept_incoming(pica_tx.clone(), args.uci_port), pica.run(),)?;

    Ok(())
}
