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
        let iter = buf.chunks(MSS_SIZE);

        // for each segment
        for chunk in iter {
            // send segment
            self.socket.send_to(chunk, addr)?;
            let mut ack_buffer = [0; MSS_SIZE];
            let (_amt, src) = self.socket.recv_from(&mut ack_buffer)?;
            println!("{}", str::from_utf8(&ack_buffer).unwrap());
            // wait for ack
        }

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

        loop {
            let mut hold_buffer = [0; MSS_SIZE];
            let (amt, src) = self.socket.recv_from(&mut hold_buffer)?;
            let bytes = Bytes::copy_from_slice(&hold_buffer[..amt]);
            println!("{:?}", &bytes);
            self.socket.send_to(b"ACK", src)?;
        }

        Ok(())
    }
}
