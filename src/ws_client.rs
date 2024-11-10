use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{SinkExt, StreamExt};
use tokio::io::{self, AsyncBufReadExt};

#[tokio::main]
async fn main() {
    const EXIT_WS: &str = "x";

    let (mut socket, response) = connect_async("ws://127.0.0.1:8081/ws").await.expect("Failed to connect");

    println!("Connected to WebSocket server [{}]. Type messages to send.", response.status());

    let stdin = io::BufReader::new(io::stdin());
    let mut lines = stdin.lines();

    loop {
        tokio::select! {
            result = lines.next_line() => {
                match result {
                    Ok(Some(line)) => {
                        match line.as_str() {
                            EXIT_WS => break,
                            _ => {
                                socket.send(Message::Text(line)).await.expect("Failed to send message");
                            }
                        }
                    }
                    Ok(None) => break,
                    Err(e) => {
                        eprintln!("Error reading line: {}", e);
                        break;
                    }
                }
            },
            Some(Ok(message)) = socket.next() => {
                if let Message::Text(text) = message {
                    println!("Received: {text}");
                }
            },
            else => break,
        }
    }

    println!("Disconnected from WebSocket server.");
}
