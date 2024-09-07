use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt, Result};
use std::fmt::format;
use std::io;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<()> {
    let stream = TcpStream::connect("127.0.0.1:6889").await?;

    // Split the TcpStream into a read half and a write half
    let (mut reader, mut writer) = stream.into_split();

    // Input username
    let mut username = String::new();
    print!("Enter your username: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).expect("Failed to read username");
    let username = username.trim().to_string(); 
    writer.write_all(username.trim().as_bytes()).await?; // Send username to the server
    // Spawn a task to listen for messages from the server
    tokio::spawn(async move {
        let mut buf = vec![0; 1024];
        loop {
            let n = reader.read(&mut buf).await.unwrap_or(0);
            if n == 0 {
                break;
            }
            let msg = String::from_utf8_lossy(&buf[..n]);
            io::stdout().flush().unwrap();

            print!("{}\n", msg.trim().to_string());
            io::stdout().flush().unwrap();
        }
    });
    
    // The sender loop
    loop {
        let mut msg = String::new();
        io::stdin().read_line(&mut msg).expect("Failed to read message");
        let message = format!("{}\n", msg);
        writer.write_all(message.as_bytes()).await?; // Send the message via the stream
    }
}
