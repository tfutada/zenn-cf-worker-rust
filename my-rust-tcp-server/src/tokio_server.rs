use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

mod constants;

// TCP server using Tokio, echo server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(constants::SERVER_ADDR).await?;
    println!("Server is listening on {}", constants::SERVER_ADDR);

    while let Ok((mut socket, _)) = listener.accept().await {
        tokio::spawn(async move {
            if let Err(e) = handle_client(&mut socket).await {
                eprintln!("Error handling client: {}", e);
            }
        });
    }

    Ok(())
}

async fn handle_client(socket: &mut tokio::net::TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = vec![0; 1024];

    loop {
        match socket.read(&mut buf).await {
            // Return value of `Ok(0)` signifies that the remote has closed
            Ok(0) => return Ok(()),
            Ok(n) => {
                let received_data = String::from_utf8_lossy(&buf[..n]);
                println!("Server: a msg from client : {}", received_data);

                // Copy the data back to socket
                if let Err(e) = socket.write_all(&buf[..n]).await {
                    return Err(format!("Failed to write to socket: {}", e).into());
                }
                println!("Server: echo back to client: {}", received_data)
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
}
