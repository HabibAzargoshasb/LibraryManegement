use shared::{Genre, Request};
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
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
    let add_request_2 = Request::AddBook {
        title: "Brave New World".to_string(),
        author: "Aldous Huxley".to_string(),
        genre: Genre::Novel,
    };
    send_request(&add_request_2).await?;
    sleep(Duration::from_millis(500)).await;

    let borrow_request = Request::BorrowBook {
        book_id: 1,
        member_id: 1,
    };
    send_request(&borrow_request).await?;
    sleep(Duration::from_millis(500)).await;

    let list_request_1 = Request::ListBooks;
    send_request(&list_request_1).await?;
    sleep(Duration::from_millis(500)).await;

    let return_request = Request::ReturnBook {
        book_id: 1,
        member_id: 1,
    };
    send_request(&return_request).await?;
    sleep(Duration::from_millis(500)).await;

    let list_request_2 = Request::ListBooks;
    send_request(&list_request_2).await?;
    sleep(Duration::from_millis(500)).await;
    let search_request = Request::SearchBook {
        query: "Orwell".to_string(),
    };
    send_request(&search_request).await?;
    sleep(Duration::from_millis(500)).await;

    sleep(Duration::from_millis(500)).await;

    let edit_request = Request::EditBook {
        book_id: 1,
        title: "Animal Farm".to_string(),
        author: "George Orwell".to_string(),
        genre: Genre::Novel,
    };
    send_request(&edit_request).await?;
    sleep(Duration::from_millis(500)).await;

    let list_request_final = Request::ListBooks;
    send_request(&list_request_final).await?;

    let remove_request = Request::RemoveBook { book_id: 1 };
    send_request(&remove_request).await?;
    sleep(Duration::from_millis(500)).await;

    let list_request_3 = Request::ListBooks;
    send_request(&list_request_3).await?;

    Ok(())
}
