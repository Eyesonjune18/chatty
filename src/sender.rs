use std::io::{self, Write};
use std::net::TcpStream;

#[allow(dead_code)]
pub fn start() -> io::Result<()> {
    let mut stream = TcpStream::connect("192.168.137.191:8095")?;

    loop {
        eprint!("Enter a message: ");

        let mut message = String::new();
        io::stdin().read_line(&mut message)?;

        stream.write_all(message.as_bytes())?;
    }
}
