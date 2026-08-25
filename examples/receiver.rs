use reliable_over_udp::ReliableSocket;
use std::io::prelude::*;
use std::net::SocketAddr;
use std::net::{TcpListener, UdpSocket};
use std::result::Result::Ok;

fn main() -> anyhow::Result<()> {
    let addr = "0.0.0.0:34254".parse()?;
    let socket = ReliableSocket::bind(addr)?;

    let mut buf = [0; 128];

    socket.recv_from(&mut buf)?;

    if let Ok(s) = str::from_utf8(&buf) {
        println!("{}", s);
    }

    Ok(())
}
