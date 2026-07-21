use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, BufReader};
use shared::Request;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server is listening on 127.0.0.1:8080");

    loop {
        let (socket, addr) = listener.accept().await?;

        tokio::spawn(async move {
            println!("New client connected: {}", addr);
            let mut reader = BufReader::new(socket);
            let mut line = String::new();
            reader.read_line(&mut line).await.unwrap();
            let request : Request = serde_json::from_str(&line).unwrap();
            println!("Received: {:?}", request);
            
        });
    }
}