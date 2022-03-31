extern crate bytes;
extern crate num_derive;
extern crate num_traits;
extern crate thiserror;

mod position;
mod uci_packets;
mod uwb_subsystem;
mod web;

use anyhow::Result;
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::net::TcpListener;
use tokio::sync::{broadcast, mpsc};
use tokio::try_join;

use uwb_subsystem::*;

const UCI_PORT: u16 = 7000;

async fn accept_incoming(tx: mpsc::Sender<PicaCommand>) -> Result<()> {
    let uci_socket = SocketAddrV4::new(Ipv4Addr::LOCALHOST, UCI_PORT);
    let uci_listener = TcpListener::bind(uci_socket).await?;
    println!("Pica: Listening on: {}", UCI_PORT);

    loop {
        let (socket, addr) = uci_listener.accept().await?;
        println!("Uwb host addr: {}", addr);
        tx.send(PicaCommand::Connect(socket)).await?
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let (event_tx, _) = broadcast::channel(16);

    let mut pica = Pica::new(event_tx.clone());
    let tx = pica.tx();

    try_join!(
        accept_incoming(tx.clone()),
        pica.run(),
        web::serve(tx, event_tx)
    )?;

    Ok(())
}
