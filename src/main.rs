use mini_redis::client;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt, Result};
use std::collections::HashMap;
use std::vec;
use tokio::sync::Mutex;
use std::sync::Arc;
use tokio::net::tcp::OwnedWriteHalf;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6889").await?;
    let clients = Arc::new(Mutex::new(HashMap::new())); // Map of usernames to write halves

    loop {
        let (socket, _) = listener.accept().await?;
        let clients = Arc::clone(&clients); // Clone Arc for each task

        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, clients).await {
                eprintln!("Error: {:?}", e);
            }
        });
    }
}

async fn handle_client(socket: TcpStream, clients: Arc<Mutex<HashMap<String, Arc<Mutex<OwnedWriteHalf>>>>>) -> Result<()> {
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
        let mut clients_lock = clients.lock().await;
        clients_lock.insert(username.clone(), Arc::new(Mutex::new(writer)));
    }

    // Handle client messages
    loop {
        let n = reader.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        let msg = String::from_utf8_lossy(&buf[..n]);
        if msg.trim() == "/list" {
            print_users(&username, clients.clone()).await;
            echo_users(&username, clients.clone()).await?;
        } else {
            println!("{}: {}", username, msg.trim());
            echo_msg(&username, &msg, clients.clone()).await?;
        }
    }

    // Remove the client from the hashmap when they disconnect
    {
        let mut clients_lock = clients.lock().await; 
        clients_lock.remove(&username);
    }

    println!("{} disconnected", username);
    Ok(())
}

// echo all the users connected 
async fn echo_users(sender_username: &str, clients: Arc<Mutex<HashMap<String, Arc<Mutex<OwnedWriteHalf>>>>>) -> Result<()> {
    let clients_lock = clients.lock().await;
    let users: Vec<String> = clients_lock.keys()
        .filter(|&username| username != sender_username)
        .cloned()
        .collect();

    if let Some(writer) = clients_lock.get(sender_username) {
        let mut sender = writer.lock().await;
        let user_list = users.join("\n");
        let message = format!("Connected users:\n{}\n", user_list);
        sender.write_all(message.as_bytes()).await?;
    }
    Ok(())
}

// prints all the connected users
async fn print_users(current_username: &str, clients: Arc<Mutex<HashMap<String, Arc<Mutex<OwnedWriteHalf>>>>>) {
    let clients_lock = clients.lock().await;
    println!("Connected users:");
    for username in clients_lock.keys() {
        if username != current_username {
            println!("{}", username);
        }
    }
}

// sends the message to all the users
async fn echo_msg(username: &str, msg: &str, clients: Arc<Mutex<HashMap<String, Arc<Mutex<OwnedWriteHalf>>>>>) -> Result<()> {
    let clients_lock = clients.lock().await;
    for (usr, writer) in clients_lock.iter() {
        if usr != username {
            let mut sender = writer.lock().await;
            let message = format!("{}: {}\n", username, msg.trim());
            sender.write_all(message.as_bytes()).await?;
        }
    } 
    Ok(())
}