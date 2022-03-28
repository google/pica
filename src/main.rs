use anyhow::Result;
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::net::{TcpListener, TcpStream};

const UCI_PORT: u16 = 7000;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Pica warming up !");
    let uci_socket = SocketAddrV4::new(Ipv4Addr::LOCALHOST, UCI_PORT);
    let uci_listener = TcpListener::bind(uci_socket).await?;
    println!("Pica: Listening on: {}", UCI_PORT);

    loop {
        let (socket, addr) = uci_listener.accept().await?;
        println!("Uwb host addr: {}", addr);
        process_socket(socket).await;
    }
}

async fn process_socket(_socket: TcpStream) {
    println!("processing socket");
}
