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
        let iter = buf.chunks(MSS_SIZE - 2);

        // for each segment
        for chunk in iter {
            // send segment
            let checksum = generate_checksum(chunk);
            let checksum_bytes = checksum.to_be_bytes();

            let res: Vec<u8> = [&checksum_bytes[..], chunk].concat();
            self.socket.send_to(&res, addr)?;
            let mut ack_buffer = [0; MSS_SIZE];
            let (_amt, src) = self.socket.recv_from(&mut ack_buffer)?;
            println!("{}{}", checksum, str::from_utf8(&ack_buffer).unwrap());
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
            let checksum = &bytes[..2];
            let number = be_u8_to_u16(checksum);
            println!("{}", validate_checksum(number, &bytes[2..]));
            println!("{:?}", &bytes);
            self.socket.send_to(b"ACK", src)?;
        }

        Ok(())
    }
}

fn be_u8_to_u16(checksum: &[u8]) -> u16 {
    ((checksum[0] as u16) << 8) | checksum[1] as u16
}

fn generate_checksum(buf: &[u8]) -> u16 {
    let mut sum: u32 = 0;
    let chunks = buf.chunks(2);
    for chunk in chunks {
        if chunk.len() > 1 {
            sum += u16::from_be_bytes([chunk[0], chunk[1]]) as u32;
        } else {
            sum += (chunk[0] as u32) << 8;
        }
    }
    while sum >> 16 != 0 {
        sum = (sum & 0xffff) + (sum >> 16);
    }
    let csum = !sum as u16;
    if csum == 0 { 0xffff } else { csum }
}

fn validate_checksum(checksum: u16, buf: &[u8]) -> bool {
    checksum == generate_checksum(buf)
}
