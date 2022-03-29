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
    println!("Pica warming up !");
    let uci_socket = SocketAddrV4::new(Ipv4Addr::LOCALHOST, UCI_PORT);
    let uci_listener = TcpListener::bind(uci_socket).await?;
    let mut mac_address = 0;
    println!("Pica: Listening on: {}", UCI_PORT);

    loop {
        let (socket, addr) = uci_listener.accept().await?;
        println!("Uwb host addr: {}", addr);
        mac_address += 1;

        tokio::spawn(async move {
            let mut device = Device::new(mac_address, socket);
            device.run().await
        });
    }
}
