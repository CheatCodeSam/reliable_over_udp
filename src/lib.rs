use anyhow::Result;
use std::net::{SocketAddr, UdpSocket};

// TODO: Add real error handling

pub struct ReliableSocket {
    addr: SocketAddr,
    socket: UdpSocket,
}

impl ReliableSocket {
    pub fn bind(addr: SocketAddr) -> Result<Self> {
        let socket = UdpSocket::bind(addr)?;

        Ok(Self { addr, socket })
    }

    pub fn send_to(&self, buf: &[u8], addr: SocketAddr) -> Result<()> {
        self.socket.send_to(buf, addr)?;

        Ok(())
    }

    pub fn recv_from(&self, buf: &mut [u8]) -> Result<()> {
        let (_amt, _src) = self.socket.recv_from(buf)?;
        Ok(())
    }
}
