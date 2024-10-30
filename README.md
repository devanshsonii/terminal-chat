# Async TCP Chat Server with Tokio

This project is a simple asynchronous TCP chat server and client application built using [Tokio](https://tokio.rs/), a Rust asynchronous runtime. The server supports multiple clients connecting to it and provides features like broadcasting messages, listing connected users, and sending private messages.

## Features

- **Multi-client support**: Multiple clients can connect to the server and interact simultaneously.
- **Message broadcasting**: Each message sent by a client is broadcast to all other connected clients.
- **User listing**: Clients can view a list of currently connected users.
- **Private messaging**: Clients can send direct messages to specific users.
- **Asynchronous I/O**: Uses Tokio's async runtime for non-blocking I/O operations.

## Project Structure

- `main.rs`: Contains the main function to run the server.
- `lib.rs`: (to be created) A placeholder for library functions to modularize server and client code.
- `Cargo.toml`: Manages dependencies, including Tokio.

## Installation and Setup

1. **Clone the repository**:
   ```bash
   git clone https://github.com/devanshsonii/terminal-chat
   cd terminal-chat
   ```

2. **Add dependencies**: Ensure `tokio` is in the `[dependencies]` section of `Cargo.toml`.
   ```toml
   [dependencies]
   tokio = { version = "1", features = ["full"] }
   ```

3. **Build the project**:
   ```bash
   cargo build
   ```

## Usage

1. **Start the Server**: Run the server in one terminal window.
   ```bash
    cargo run 
   ```

2. **Start a Client**: In a separate terminal, start a client.
   ```bash
    cargo run --example user
   ```

3. **Commands**:
   - Send a message to all users by simply typing your message and pressing Enter.
   - List connected users by typing `/list` and pressing Enter.
   - Send a private message using the format: `/pvt <username> <message>`.
   - To disconnect, simply close the terminal or terminate the process.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue.
