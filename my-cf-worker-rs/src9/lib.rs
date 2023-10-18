use tokio::io::{AsyncReadExt, AsyncWriteExt};

use worker::*;


#[event(fetch)]
async fn main(_req: Request, _env: Env, _ctx: Context) -> worker::Result<Response> {
    let mut socket = worker::Socket::builder()
        .connect("(your ngrok hostname).ngrok-free.app", 443)?;

    console_log!("Connected to server!");

    // send all bytes
    socket.write_all(b"Hello, server!").await?;
    console_log!("Sent to server: Hello, server!");

    // receive
    let mut buffer = Vec::new();
    let bytes_read = socket.read_to_end(&mut buffer).await?;

    let received_data = String::from_utf8_lossy(&buffer[..bytes_read]);

    console_log!("Received from server: {}", received_data);

    Response::ok("done!")
}
