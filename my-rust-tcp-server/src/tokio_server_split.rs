use tokio::net::TcpListener;

mod constants; // pub const SERVER_ADDR: &str = "localhost:5000";

// TCP server using Tokio, echo server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(constants::SERVER_ADDR).await?;
    println!("Server is listening on {}", constants::SERVER_ADDR);

    while let Ok((mut socket, _)) = listener.accept().await {
        tokio::spawn(async move {
            // split socket
            let (mut rd, mut wr) = socket.split();
            // copy data from rd to wr
            tokio::io::copy(&mut rd, &mut wr).await.expect("TODO: panic message");
        });
    }

    Ok(())
}
