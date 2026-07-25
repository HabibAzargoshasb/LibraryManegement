mod state;
use shared::{Book, Loan, Member, Request};
use state::LibraryState;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
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
                    let mut locked_state = state.lock().await;
                    let new_id = locked_state.next_book_id;
                    let new_book = Book::new(new_id, title, author, genre);
                    locked_state.books.push(new_book);
                    locked_state.next_book_id += 1;
                    println!("Book added with id {}", new_id);
                }
                Request::ListBooks => {
                    let locked_state = state.lock().await;
                    println!("Books: {:?}", locked_state.books);
                }
                Request::BorrowBook { book_id, member_id } => {
                    let mut locked_state = state.lock().await;
                    let mut found = false;
                    let mut successfully_borrowed = false;

                    for book in locked_state.books.iter_mut() {
                        if book.id == book_id {
                            found = true;
                            if !book.is_borrowed {
                                book.is_borrowed = true;
                                successfully_borrowed = true;
                                println!("Book {} borrowed by member {}", book_id, member_id);
                            } else {
                                println!("Book {} is already borrowed", book_id);
                            }
                        }
                    }

                    if !found {
                        println!("Book {} not found", book_id);
                    }

                    if successfully_borrowed {
                        let now = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs();
                        let due = now + (14 * 24 * 60 * 60);
                        let new_loan = Loan::new(book_id, member_id, now, due);
                        locked_state.loans.push(new_loan);
                    }
                }
                Request::ReturnBook { book_id, member_id } => {
                    let mut locked_state = state.lock().await;
                    let mut found = false;
                    let mut successfully_returned = false;
                    for book in locked_state.books.iter_mut() {
                        if book.id == book_id {
                            found = true;
                            if book.is_borrowed {
                                book.is_borrowed = false;
                                successfully_returned = true;
                                println!("Book {} returned by member {}", book_id, member_id);
                            } else {
                                println!("Book {} was not borrowed", book_id);
                            }
                        }
                    }
                    if !found {
                        println!(" Book {} not found", book_id);
                    }
                    if successfully_returned {
                        for loan in locked_state.loans.iter_mut() {
                            if loan.book_id == book_id
                                && loan.member_id == member_id
                                && !loan.returned
                            {
                                loan.returned = true;
                            }
                        }
                    }
                }
                Request::RemoveBook { book_id } => {
                    let mut locked_state = state.lock().await;
                    let mut index_to_remove: Option<usize> = None;
                    let mut current_index: usize = 0;
                    for book in locked_state.books.iter() {
                        if book.id == book_id {
                            index_to_remove = Some(current_index);
                        }
                        current_index += 1;
                    }
                    if let Some(index) = index_to_remove {
                        locked_state.books.remove(index);
                        println!("Book {} removed", book_id);
                    } else {
                        println!("Book {} not found.", book_id);
                    }
                }
                Request::EditBook {
                    book_id,
                    title,
                    author,
                    genre,
                } => {
                    let mut locked_state = state.lock().await;
                    let mut found = false;
                    for book in locked_state.books.iter_mut() {
                        if book.id == book_id {
                            found = true;
                            book.title = title.clone();
                            book.author = author.clone();
                            book.genre = genre.clone();
                        }
                    }
                    if found {
                        println!("Book {} updated", book_id);
                    } else {
                        println!("Book {} not found", book_id);
                    }
                }
                Request::SearchBook { query } => {
                    let locked_state = state.lock().await;
                    let mut results: Vec<Book> = Vec::new();
                    for book in locked_state.books.iter() {
                        if book.title.contains(&query) || book.author.contains(&query) {
                            results.push(book.clone());
                        }
                    }
                    println!("Search results {:?}", results);
                }
                Request::AddMember { name } => {
                    let mut locked_state = state.lock().await;
                    let new_id = locked_state.next_member_id;
                    let new_member = Member::new(new_id, name);
                    locked_state.members.push(new_member);
                    locked_state.next_member_id += 1;
                    println!("Member added with id {}", new_id);
                }
                Request::RemoveMember { member_id } => {
                    let mut locked_state = state.lock().await;
                    let mut index_to_remove: Option<usize> = None;
                    let mut current_index: usize = 0;
                    for member in locked_state.members.iter() {
                        if member.id == member_id {
                            index_to_remove = Some(current_index);
                        }
                        current_index += 1;
                    }
                    if let Some(index) = index_to_remove {
                        locked_state.members.remove(index);
                        println!("Member {} removed", member_id);
                    } else {
                        println!("Member {} not found", member_id);
                    }
                }
                Request::EditMember { member_id, name } => {
                    let mut locked_state = state.lock().await;
                    let mut found = false;
                    for member in locked_state.members.iter_mut() {
                        if member.id == member_id {
                            found = true;
                            member.name = name.clone();
                        }
                    }
                    if found {
                        println!("Member {} updated", member_id);
                    } else {
                        println!("Member {} not found", member_id);
                    }
                }
                Request::SearchMember { query } => {
                    let locked_state = state.lock().await;
                    let mut results: Vec<Member> = Vec::new();
                    for member in locked_state.members.iter() {
                        if member.name.contains(&query) {
                            results.push(member.clone());
                        }
                    }
                    println!("Search results {:?}", results);
                }
            }
        });
    }
}
