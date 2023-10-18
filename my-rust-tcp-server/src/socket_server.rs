use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            let received_data = String::from_utf8_lossy(&buffer);
            println!("Received from client: {}", received_data);

            // Echo the data back to the client
            if let Err(e) = stream.write(&buffer[..bytes_read]) {
                eprintln!("Failed to send data to client: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to read from client: {}", e);
        }
    }
}

// telnet localhost 7878
fn main() {
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
