use std::{
    io::Write,
    net::{TcpListener, TcpStream, UdpSocket},
};

fn main() -> anyhow::Result<()> {
    let mut socket = TcpStream::connect("0.0.0.0:34254")?;

    let buf = b"Hello, World";

    socket.write(&buf[..])?;

    Ok(())
}
