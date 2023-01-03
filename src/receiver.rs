use std::io::{self, Read};
use std::net::TcpListener;

#[allow(dead_code)]
pub fn start() -> io::Result<()> {
    let listener = TcpListener::bind("192.168.137.191:8095")?;
    let mut client = listener.accept()?.0;
    
    let mut buffer = [0; 1024];

    loop {
        let num_read = client.read(&mut buffer)?;

        if num_read == 0 {
            return Ok(());
        }

        let message = String::from_utf8_lossy(&buffer[..num_read]);
        print!("Received message: {}", message);
    }
}
