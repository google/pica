extern crate bytes;
extern crate num_derive;
extern crate num_traits;
extern crate thiserror;

mod uci_packets;
mod uwb_subsystem;

use anyhow::Result;
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::net::TcpListener;

use uwb_subsystem::*;

mod position;
use position::Position;

const UCI_PORT: u16 = 7000;

#[tokio::main]
async fn main() -> Result<()> {
    let uci_socket = SocketAddrV4::new(Ipv4Addr::LOCALHOST, UCI_PORT);
    let uci_listener = TcpListener::bind(uci_socket).await?;
    println!("Pica: Listening on: {}", UCI_PORT);

    let mut pica = Pica::new();
    let tx = pica.tx();

    // Spawn and detach main Pica run task.
    tokio::spawn(async move { pica.run().await });

    loop {
        let (socket, addr) = uci_listener.accept().await?;
        println!("Uwb host addr: {}", addr);
        tx.send(PicaCommand::Connect(socket)).await?
    }
}
