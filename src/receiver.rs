use std::io::{self, BufReader, BufRead};
use std::net::TcpListener;

#[allow(dead_code)]
pub fn start() -> io::Result<()> {
    let listener = TcpListener::bind("192.168.137.191:8095")?;
    let client = listener.accept()?.0;
    let mut reader = BufReader::new(client);
    
    loop {
        let mut message = String::new();
        let num_read = reader.read_line(&mut message)?;

        if num_read == 0 {
            return Ok(());
        }

        print!("Received message: {}", message);
    }
}
