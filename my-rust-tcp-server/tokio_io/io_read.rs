use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("./foo.txt").await?;
    let mut buffer = [0; 5]; // read 5 bytes at a time but it is 7 bytes

    // read up to 10 bytes
    let n = f.read(&mut buffer[..]).await?;

    println!("The bytes: {} {:?}", n, &buffer[..n]);
    Ok(())
}