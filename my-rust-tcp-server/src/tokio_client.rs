use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

mod constants;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the server
    let mut stream = TcpStream::connect(constants::SERVER_ADDR).await?;

    // Data to send to the server
    let data_to_send = "Hello, server!";
    stream.write_all(data_to_send.as_bytes()).await?;

    // Buffer to store the received data
    let mut buffer = vec![0; 512];
    let bytes_read = stream.read(&mut buffer).await?;

    // Convert the received data to a string and print it
    let received_data = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Client: echo back from the server: {}", received_data);

    Ok(())
}
