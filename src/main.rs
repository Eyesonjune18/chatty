use std::io::{self, Write, BufRead, BufReader};
use std::net::{TcpStream, TcpListener};

fn main() {
    start_sender();
    start_receiver();

    // Don't let the main thread exit
    loop {}
}

fn start_sender() {
    std::thread::spawn(|| {
        fn connect() -> TcpStream {
            loop {
                // Connect to the server on port 9015
                match TcpStream::connect("192.168.137.191:9015") {
                    Ok(stream) => return stream,
                    Err(_) => continue,
                }
            }
        }

        println!("Waiting for a connection...");

        // Wait for the receiver to open a socket
        let mut stream = connect();

        println!("Connection successful. Type a message and press ENTER to send.");

        loop {
            // Prompt the user for a message to send
            print!("SEND: ");
            io::stdout().flush().unwrap();
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
        // Open connections on port 9015
        let listener = TcpListener::bind("192.168.137.102:9015").unwrap();

        // Accept a connection
        let (stream, _) = listener.accept().unwrap();
        // Read from the connection
        let mut reader = BufReader::new(stream);

        loop {
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            
            if line.len() > 0 {
                // Clear the SEND prompt line
                print!("\r\x1b[2K");
                // Print the received message
                print!("RECV: {}", line);
                // Re-print the send prompt
                print!("SEND: ");
                io::stdout().flush().unwrap();
            } else {
                println!("Connection closed.");
                std::process::exit(0);
            }
        }
    });
}
