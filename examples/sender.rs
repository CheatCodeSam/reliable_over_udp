use reliable_over_udp::ReliableSocket;

fn main() -> anyhow::Result<()> {
    let addr = "0.0.0.0:34255".parse()?;
    let socket = ReliableSocket::bind(addr)?;

    let buf = b"Hello, World";

    socket.send_to(buf, "0.0.0.0:34254".parse()?)?;

    Ok(())
}
