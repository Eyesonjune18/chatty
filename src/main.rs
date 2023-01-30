use std::io::{self, Write, BufRead, BufReader};
use std::net::{TcpStream, TcpListener};

fn main() {
    start_sender();
    start_receiver();
}

fn start_sender() {
    std::thread::spawn(|| {
        // Connect to the server at 192.168.137.191:9000
        let mut stream = TcpStream::connect("192.168.137.191:9015").unwrap();
        
        loop {
            // Read a line from stdin
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            
            // Write the line to the server
            stream.write(input.as_bytes()).unwrap();
        }
    });
}

fn start_receiver() {
    std::thread::spawn(|| {
        // Open connections on port 9001
        let listener = TcpListener::bind("192.168.137.102:9015").unwrap();

        // Accept a connection
        let (stream, _) = listener.accept().unwrap();
        // Read from the connection
        let mut reader = BufReader::new(stream);

        loop {
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            println!("R: {}", line);
        }
    });
}
