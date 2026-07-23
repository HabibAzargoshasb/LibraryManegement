mod state;
use shared::Book;
use shared::Request;
use state::LibraryState;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server is listening on 127.0.0.1:8080");
    let state = Arc::new(Mutex::new(LibraryState::new()));
    loop {
        let (socket, addr) = listener.accept().await?;
        let state = Arc::clone(&state);

        tokio::spawn(async move {
            println!("New client connected: {}", addr);
            let mut reader = BufReader::new(socket);
            let mut line = String::new();
            reader.read_line(&mut line).await.unwrap();
            let request: Request = serde_json::from_str(&line).unwrap();
            match request {
                Request::AddBook {
                    title,
                    author,
                    genre,
                } => {
                    let new_book = Book::new(1, title, author, genre);
                    let mut locked_state = state.lock().await;
                    locked_state.books.push(new_book);
                    println!("Book added!");
                }
                Request::ListBooks => {
                    let locked_state = state.lock().await;
                    println!("Books: {:?}", locked_state.books);
                }
                Request::BorrowBook { book_id, member_id } => {
                    println!("BorrowBook received (not implemented yet)");
                }
                Request::ReturnBook { book_id, member_id } => {
                    println!("ReturnBook received (not implemented yet)");
                }
            }
        });
    }
}
