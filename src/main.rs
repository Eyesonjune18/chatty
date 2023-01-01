use std::{
    error::Error,
    net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
    str::FromStr,
    thread,
    time::Duration,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Bind the socket to a local IP and port
    let local_ip = IpAddr::V4(Ipv4Addr::from_str("192.168.137.102")?);
    let local_port = 12345;
    let local_addr = SocketAddr::new(local_ip, local_port);
    let socket = UdpSocket::bind(local_addr)?;

    // Set the socket to non-blocking mode
    socket.set_nonblocking(true)?;

    // Send messages to the other client
    let other_ip = IpAddr::V4(Ipv4Addr::from_str("192.168.137.101")?);
    let other_port = 12346;
    let other_addr = SocketAddr::new(other_ip, other_port);

    loop {
        let message = b"Hello from the other side!";
        socket.send_to(message, other_addr)?;
        println!("Sent message to other client: {}", String::from_utf8_lossy(message));

        thread::sleep(Duration::from_secs(1));
    }

}
