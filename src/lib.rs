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
    socket: UdpSocket,
    sending_state: SenderState,
}

impl ReliableSocket {
    pub fn bind(addr: SocketAddr) -> Result<Self> {
        let socket = UdpSocket::bind(addr)?;

        Ok(Self {
            socket,
            sending_state: SenderState::WaitForCall,
        })
    }

    pub fn send_to(&mut self, buf: &Bytes, addr: SocketAddr) -> Result<()> {
        let iter = buf.chunks(MSS_SIZE - 3);

        self.sending_state = SenderState::WaitForAck;
        let mut packet_number = 1;
        for chunk in iter {
            packet_number = packet_number ^ 1;
            println!("{packet_number}");

            self.send_chunk(addr, chunk, packet_number)?;

            loop {
                let mut ack_buffer = [0; MSS_SIZE];
                let (amt, _src) = self.socket.recv_from(&mut ack_buffer)?;

                let result = String::from_utf8_lossy(&ack_buffer[..amt]).to_string();
                println!("{result}");
                let expected_response = format!("ACK{packet_number}");
                if result == expected_response {
                    break;
                } else {
                    self.send_chunk(addr, chunk, packet_number)?;
                }
            }
        }

        self.sending_state = SenderState::WaitForCall;
        Ok(())
    }

    fn send_chunk(
        &self,
        addr: SocketAddr,
        chunk: &[u8],
        packet_number: u8,
    ) -> Result<(), anyhow::Error> {
        // 50% chance for data to get corrupted.
        let mut rng = rand::rng();
        let random_number = rng.random_range(0..=1);

        let checksum = generate_checksum(chunk);

        let checksum = checksum + random_number;

        let checksum_bytes = checksum.to_be_bytes();

        let res: Vec<u8> = [&checksum_bytes[..], &[packet_number], chunk].concat();
        self.socket.send_to(&res, addr)?;
        Ok(())
    }

    pub fn recv_from(&self, buf: &mut [u8]) -> Result<()> {
        let mut packet_number_waiting_for = 0;
        loop {
            let mut hold_buffer = [0; MSS_SIZE];
            let (amt, src) = self.socket.recv_from(&mut hold_buffer)?;
            let bytes = Bytes::copy_from_slice(&hold_buffer[..amt]);

            let data = &bytes[3..];
            let packet_number = &bytes[2];
            let checksum = &bytes[..2];

            let number = be_u8_to_u16(checksum);

            let valid_checksum = validate_checksum(number, data);
            let correct_packet_number = *packet_number == packet_number_waiting_for;
            if valid_checksum && correct_packet_number {
                let mut send = format!("ACK{packet_number}");

                // 50% chance for data to get corrupted.
                let mut rng = rand::rng();
                let random_number = rng.random_range(0..=1);
                if random_number == 0 {
                    send = "FES#".into();
                }

                self.socket.send_to(&send.into_bytes(), src)?;
                packet_number_waiting_for = packet_number_waiting_for ^ 1;
                println!("{:?}", str::from_utf8(data)?);
            } else {
                // ACK the last header that was received, by flipping packet waiting for.
                let send = format!("ACK{}", packet_number_waiting_for ^ 1);
                self.socket.send_to(&send.into_bytes(), src)?;
            }
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
