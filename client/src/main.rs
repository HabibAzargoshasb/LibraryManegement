use shared::{Request, Genre};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use std::time::Duration;
use tokio::time::sleep;

async fn send_request(request: &Request) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    let json = serde_json::to_string(request)?;
    stream.write_all(json.as_bytes()).await?;
    stream.write_all(b"\n").await?;
    stream.flush().await?;
    println!("Sent request: {:?}", request);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let add_request = Request::AddBook {
        title: "1984".to_string(),
        author: "George Orwell".to_string(),
        genre: Genre::Novel,
    };
    send_request(&add_request).await?;
    sleep(Duration::from_millis(500)).await;
    
    let list_request = Request::ListBooks;
    send_request(&list_request).await?;

    Ok(())
}