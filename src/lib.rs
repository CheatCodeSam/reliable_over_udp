use anyhow::Result;
use bytes::Bytes;
use rand::prelude::*;
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
    sending_state: SenderState,
}

impl ReliableSocket {
    pub fn bind(addr: SocketAddr) -> Result<Self> {
        let socket = UdpSocket::bind(addr)?;

        Ok(Self {
            addr,
            socket,
            sending_state: SenderState::WaitForCall,
        })
    }

    pub fn send_to(&mut self, buf: &Bytes, addr: SocketAddr) -> Result<()> {
        let iter = buf.chunks(MSS_SIZE - 2);

        self.sending_state = SenderState::WaitForAck;
        for chunk in iter {
            self.send_chunk(addr, chunk)?;

            loop {
                let mut ack_buffer = [0; MSS_SIZE];
                let (amt, _src) = self.socket.recv_from(&mut ack_buffer)?;

                let result = String::from_utf8_lossy(&ack_buffer[..amt]).to_string();
                println!("{result}");
                if result == "ACK" {
                    break;
                } else {
                    self.send_chunk(addr, chunk)?;
                }
            }
        }

        self.sending_state = SenderState::WaitForCall;
        Ok(())
    }

    fn send_chunk(&self, addr: SocketAddr, chunk: &[u8]) -> Result<(), anyhow::Error> {\
        // 50% chance for data to get corrupted.
        let mut rng = rand::rng();
        let random_number = rng.random_range(0..=1);

        let checksum = generate_checksum(chunk);

        let checksum = checksum + random_number;

        let checksum_bytes = checksum.to_be_bytes();

        let res: Vec<u8> = [&checksum_bytes[..], chunk].concat();
        self.socket.send_to(&res, addr)?;
        Ok(())
    }

    pub fn recv_from(&self, buf: &mut [u8]) -> Result<()> {
        loop {
            let mut hold_buffer = [0; MSS_SIZE];
            let (amt, src) = self.socket.recv_from(&mut hold_buffer)?;
            let bytes = Bytes::copy_from_slice(&hold_buffer[..amt]);

            let data = &bytes[2..];
            let checksum = &bytes[..2];

            let number = be_u8_to_u16(checksum);

            let valid_checksum = validate_checksum(number, data);
            if valid_checksum {
                self.socket.send_to(b"ACK", src)?;
            } else {
                self.socket.send_to(b"NACK", src)?;
            }
            println!("{}", validate_checksum(number, data));
            println!("{:?}", &bytes);
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
