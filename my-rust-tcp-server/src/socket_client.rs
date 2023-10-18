use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;

    // Data to send to the server
    let data_to_send = "12345678abcdefgh!";
    stream.write_all(data_to_send.as_bytes())?;

    // Vector to store the received data
    let mut buffer = Vec::new();
    let mut temp_buffer = [0; 8];

    loop {
        let bytes_read = stream.read(&mut temp_buffer)?;
        println!("bytes_read: {} {:?}", bytes_read, temp_buffer);

        if bytes_read == 0 {
            // No more data to read
            break;
        }

        // Question: is buffer similar to HTTP response body with streaming?
        buffer.extend_from_slice(&temp_buffer[..bytes_read]);
    }

    // Convert the received data to a string and print it
    let received_data = String::from_utf8_lossy(&buffer);
    println!("Received from server: {}", received_data);

    Ok(())
}
