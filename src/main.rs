use tokio::net::{TcpListener, TcpStream};
use tokio::io::Result;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<()>{
    // create a listener 
    let listener = TcpListener::bind("127.0.0.1:6889").await?;
    // create the listener
    loop {
        let (socket, _) = listener.accept().await?;
        // waits for the message
        let _ = process(socket).await;
    }
}

async fn process(mut socket: TcpStream) -> Result<()>{
    let mut buf = vec![0; 1024]; 
    // creates the message
    loop {
        let n = socket.read(&mut buf).await?;
        if n == 0 {
            return Ok(())
        }
        let msg = String::from_utf8_lossy(&buf[..n]);
        println!("{msg}");
    }
}