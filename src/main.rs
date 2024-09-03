use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt, Result};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::tcp::OwnedWriteHalf;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6889").await?;
    let clients = Arc::new(Mutex::new(HashMap::new())); // Map of usernames to write halves

    loop {
        let (socket, _) = listener.accept().await?;
        let clients = Arc::clone(&clients);
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, clients).await {
                eprintln!("Error: {e}");
            }
        });
    }
}

async fn handle_client(socket: TcpStream, clients: Arc<Mutex<HashMap<String, Arc<Mutex<OwnedWriteHalf>>>>> ) -> Result<()> {
    let (mut reader, writer) = socket.into_split();
    let mut buf = vec![0; 1024];

    // Read username from the client
    let n = reader.read(&mut buf).await?;
    if n == 0 {
        return Ok(());
    }
    let username = String::from_utf8_lossy(&buf[..n]).trim().to_string();
    println!("{} connected", username);

    // Store the client's write half in the hashmap
    {
        let mut clients_lock = clients.lock().unwrap();
        clients_lock.insert(username.clone(), Arc::new(Mutex::new(writer)));
    }

    // Handle client messages
    loop {
        let n = reader.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        let msg = String::from_utf8_lossy(&buf[..n]);
        println!("Received from {}: {}", username, msg);
    }

    // Remove the client from the hashmap when they disconnect
    {
        let mut clients_lock = clients.lock().unwrap();
        clients_lock.remove(&username);
    }

    println!("{} disconnected", username);
    Ok(())
}