use std::io::prelude::*;
use std::net::{TcpListener, UdpSocket};

use std::result::Result::Ok;

fn main() -> anyhow::Result<()> {
    let socket = TcpListener::bind("0.0.0.0:34254")?;

    let (mut str, addr) = socket.accept()?;

    let mut buf = [0; 128];

    str.read(&mut buf)?;

    if let Ok(s) = str::from_utf8(&buf) {
        println!("{}", s);
    }

    Ok(())
}
