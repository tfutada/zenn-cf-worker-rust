use tokio::io::{self, AsyncReadExt};
use tokio::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    // let mut buffer = Vec::new();
    let mut buffer = [0; 5]; // read 5 bytes at a time but it is 7 bytes

    // read the whole file to a vec
    //
    let n = f.read_to_end(&mut buffer).await?;

    println!("The bytes: {} {:?}", n, &buffer[..n]);

    Ok(())
}