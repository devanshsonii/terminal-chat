use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, Result};
use std::io;

#[tokio::main]
async fn main() -> Result<()>{
    let mut stream = TcpStream::connect("127.0.0.1:6889").await?;
    // the sender
    loop {
        let mut msg = String::new();
        io::stdin().read_line(&mut msg).expect("yeah yea");
        stream.write_all(msg.as_bytes()).await?; // send the message via the stream
        stream.flush().await?; // make sure everything is processed
    }

}