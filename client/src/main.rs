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
    let add_member_1 = Request::AddMember {
        name: "Ali Rezaei".to_string(),
    };
    send_request(&add_member_1).await?;
    sleep(Duration::from_millis(500)).await;

    let add_member_2 = Request::AddMember {
        name: "Sara Ahmadi".to_string(),
    };
    send_request(&add_member_2).await?;
    sleep(Duration::from_millis(500)).await;

    let search_member = Request::SearchMember {
        query: "Ali".to_string(),
    };
    send_request(&search_member).await?;
    sleep(Duration::from_millis(500)).await;

    let edit_member = Request::EditMember {
        member_id: 1,
        name: "Ali Rezaei Jr.".to_string(),
    };
    send_request(&edit_member).await?;
    sleep(Duration::from_millis(500)).await;

    let remove_member = Request::RemoveMember { member_id: 2 };
    send_request(&remove_member).await?;
    sleep(Duration::from_millis(500)).await;

    let add_book = Request::AddBook {
        title: "1984".to_string(),
        author: "George Orwell".to_string(),
        genre: Genre::Novel,
    };
    send_request(&add_book).await?;
    sleep(Duration::from_millis(500)).await;

    let add_book_2 = Request::AddBook {
        title: "Brave New World".to_string(),
        author: "Aldous Huxley".to_string(),
        genre: Genre::Novel,
    };
    send_request(&add_book_2).await?;
    sleep(Duration::from_millis(500)).await;

    let borrow_book = Request::BorrowBook {
        book_id: 1,
        member_id: 1,
    };
    send_request(&borrow_book).await?;
    sleep(Duration::from_millis(500)).await;

    let reserve_book = Request::ReserveBook {
        book_id: 2,
        member_id: 1,
    };
    send_request(&reserve_book).await?;
    sleep(Duration::from_millis(500)).await;

    let list_books = Request::ListBooks;
    send_request(&list_books).await?;
    sleep(Duration::from_millis(500)).await;

    let return_book = Request::ReturnBook {
        book_id: 1,
        member_id: 1,
    };
    send_request(&return_book).await?;
    sleep(Duration::from_millis(500)).await;

    let list_books_final = Request::ListBooks;
    send_request(&list_books_final).await?;

    Ok(())
}