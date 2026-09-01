use bytes::Bytes;
use reliable_over_udp::ReliableSocket;

fn main() -> anyhow::Result<()> {
    let addr = "0.0.0.0:34255".parse()?;
    let mut socket = ReliableSocket::bind(addr)?;

    let buf = std::fs::read("./alice.txt")?;

    let bytes = Bytes::from(buf);

    socket.send_to(&bytes, "0.0.0.0:34254".parse()?)?;

    Ok(())
}
