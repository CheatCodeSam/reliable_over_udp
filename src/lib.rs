use anyhow::Result;
use bytes::Bytes;
use std::net::{SocketAddr, UdpSocket};

// TODO: Add real error handling

const MSS_SIZE: usize = 1460;

enum SenderState {
    WaitForCall,
    WaitForAck,
}

pub struct ReliableSocket {
    addr: SocketAddr,
    socket: UdpSocket,
}

impl ReliableSocket {
    pub fn bind(addr: SocketAddr) -> Result<Self> {
        let socket = UdpSocket::bind(addr)?;

        Ok(Self { addr, socket })
    }

    pub fn send_to(&self, buf: &Bytes, addr: SocketAddr) -> Result<()> {
        // Chop into MSS
        let mut iter = buf.chunks(MSS_SIZE);

        for chunk in iter {
            self.socket.send_to(chunk, addr)?;
        }

        // for each segment
        // send segment
        // wait for ack

        Ok(())
    }

    pub fn recv_from(&self, buf: &mut [u8]) -> Result<()> {
        // while true
        // receive segment
        // if segment matches checksum
        //      ack
        //      add to array
        // if segment matches checksum
        //      nack

        let (_amt, _src) = self.socket.recv_from(buf)?;
        Ok(())
    }
}
