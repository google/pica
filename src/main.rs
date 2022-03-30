extern crate bytes;
extern crate num_derive;
extern crate num_traits;
extern crate thiserror;

mod position;
mod uci_packets;
mod uwb_subsystem;

use anyhow::Result;
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::net::TcpListener;
use tokio::sync::mpsc::Sender;
use tokio::{pin, try_join};

use uwb_subsystem::*;

const UCI_PORT: u16 = 7000;

async fn accept_incoming(tx: Sender<PicaCommand>) -> Result<()> {
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
    let mut pica = Pica::new();
    let tx = pica.tx();

    try_join!(accept_incoming(tx), pica.run())?;

    Ok(())
}
