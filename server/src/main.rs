mod state;
use shared::{Book, Fine, LibraryError, Loan, Member, Request, Reservation, Response};
use state::LibraryState;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server is listening on 127.0.0.1:8080");
    let state = Arc::new(Mutex::new(LibraryState::load_from_file(
        "library_data.json",
    )));
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
                    locked_state.save_to_file("library_data.json");
                    drop(locked_state);

                    let response = Response::BooksAdded { book_id: new_id };
                    let response_json = serde_json::to_string(&response).unwrap();
                    reader
                        .get_mut()
                        .write_all(response_json.as_bytes())
                        .await
                        .unwrap();
                    reader.get_mut().write_all(b"\n").await.unwrap();
                }
                Request::ListBooks => {
                    let locked_state = state.lock().await;
                    let books_list = locked_state.books.clone();
                    drop(locked_state);

                    let response = Response::Books(books_list);
                    let response_json = serde_json::to_string(&response).unwrap();
                    reader
                        .get_mut()
                        .write_all(response_json.as_bytes())
                        .await
                        .unwrap();
                    reader.get_mut().write_all(b"\n").await.unwrap();
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
                            }
                        }
                    }

                    let response = if !found {
                        Response::Error(LibraryError::BookNotFound { book_id })
                    } else if !successfully_borrowed {
                        Response::Error(LibraryError::BookAlreadyBorrowed)
                    } else {
                        let now = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs();
                        let due = now + (14 * 24 * 60 * 60);
                        let new_loan = Loan::new(book_id, member_id, now, due);
                        locked_state.loans.push(new_loan);
                        locked_state.save_to_file("library_data.json");
                        Response::Success
                    };

                    drop(locked_state);
                    let response_json = serde_json::to_string(&response).unwrap();
                    reader
                        .get_mut()
                        .write_all(response_json.as_bytes())
                        .await
                        .unwrap();
                    reader.get_mut().write_all(b"\n").await.unwrap();
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
                            }
                        }
                    }

                    let response = if !found {
                        Response::Error(LibraryError::BookNotFound { book_id })
                    } else if !successfully_returned {
                        Response::Error(LibraryError::BookNotBorrowed)
                    } else {
                        for loan in locked_state.loans.iter_mut() {
                            if loan.book_id == book_id
                                && loan.member_id == member_id
                                && !loan.returned
                            {
                                loan.returned = true;
                            }
                        }

                        let now = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs();

                        let mut new_fines: Vec<Fine> = Vec::new();
                        for loan in locked_state.loans.iter() {
                            if loan.book_id == book_id
                                && loan.member_id == member_id
                                && loan.returned
                            {
                                if now > loan.due_date {
                                    let overdue_days =
                                        ((now - loan.due_date) / (24 * 60 * 60)) as u32;
                                    let amount = overdue_days * 1000;
                                    let new_fine =
                                        Fine::new(member_id, book_id, overdue_days, amount);
                                    new_fines.push(new_fine);
                                }
                            }
                        }

                        for fine in new_fines {
                            locked_state.fines.push(fine);
                        }

                        locked_state.save_to_file("library_data.json");
                        Response::Success
                    };

                    drop(locked_state);
                    let response_json = serde_json::to_string(&response).unwrap();
                    reader
                        .get_mut()
                        .write_all(response_json.as_bytes())
                        .await
                        .unwrap();
                    reader.get_mut().write_all(b"\n").await.unwrap();
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
                    let response = if let Some(index) = index_to_remove {
                        locked_state.books.remove(index);
                        locked_state.save_to_file("library_data.json");
                        Response::Success
                    } else {
                        Response::Error(LibraryError::BookNotFound { book_id })
                    };
                    drop(locked_state);
                    let response_json = serde_json::to_string(&response).unwrap();
                    reader
                        .get_mut()
                        .write_all(response_json.as_bytes())
                        .await
                        .unwrap();
                    reader.get_mut().write_all(b"\n").await.unwrap();
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
                    let response = if found {
                        locked_state.save_to_file("library_data.json");
                        Response::Success
                    } else {
                        Response::Error(LibraryError::BookNotFound { book_id })
                    };
                    drop(locked_state);
                    let response_json = serde_json::to_string(&response).unwrap();
                    reader
                        .get_mut()
                        .write_all(response_json.as_bytes())
                        .await
                        .unwrap();
                    reader.get_mut().write_all(b"\n").await.unwrap();
                }
                Request::SearchBook { query } => {
                    let locked_state = state.lock().await;
                    let mut results: Vec<Book> = Vec::new();
                    for book in locked_state.books.iter() {
                        if book.title.contains(&query) || book.author.contains(&query) {
                            results.push(book.clone());
                        }
                    }
                    drop(locked_state);

                    let response = Response::Books(results);
                    let response_json = serde_json::to_string(&response).unwrap();
                    reader
                        .get_mut()
                        .write_all(response_json.as_bytes())
                        .await
                        .unwrap();
                    reader.get_mut().write_all(b"\n").await.unwrap();
                }
                Request::AddMember { name } => {
                    let mut locked_state = state.lock().await;
                    let new_id = locked_state.next_member_id;
                    let new_member = Member::new(new_id, name);
                    locked_state.members.push(new_member);
                    locked_state.next_member_id += 1;
                    locked_state.save_to_file("library_data.json");
                    drop(locked_state);

                    let response = Response::MemberAdded { member_id: new_id };
                    let response_json = serde_json::to_string(&response).unwrap();
                    reader
                        .get_mut()
                        .write_all(response_json.as_bytes())
                        .await
                        .unwrap();
                    reader.get_mut().write_all(b"\n").await.unwrap();
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

                    let response = if let Some(index) = index_to_remove {
                        locked_state.members.remove(index);
                        locked_state.save_to_file("library_data.json");
                        Response::Success
                    } else {
                        Response::Error(LibraryError::MemberNotFound { member_id })
                    };

                    drop(locked_state);
                    let response_json = serde_json::to_string(&response).unwrap();
                    reader
                        .get_mut()
                        .write_all(response_json.as_bytes())
                        .await
                        .unwrap();
                    reader.get_mut().write_all(b"\n").await.unwrap();
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

                    let response = if found {
                        locked_state.save_to_file("library_data.json");
                        Response::Success
                    } else {
                        Response::Error(LibraryError::MemberNotFound { member_id })
                    };

                    drop(locked_state);
                    let response_json = serde_json::to_string(&response).unwrap();
                    reader
                        .get_mut()
                        .write_all(response_json.as_bytes())
                        .await
                        .unwrap();
                    reader.get_mut().write_all(b"\n").await.unwrap();
                }
                Request::SearchMember { query } => {
                    let locked_state = state.lock().await;
                    let mut results: Vec<Member> = Vec::new();
                    for member in locked_state.members.iter() {
                        if member.name.contains(&query) {
                            results.push(member.clone());
                        }
                    }
                    drop(locked_state);

                    let response = Response::Members(results);
                    let response_json = serde_json::to_string(&response).unwrap();
                    reader
                        .get_mut()
                        .write_all(response_json.as_bytes())
                        .await
                        .unwrap();
                    reader.get_mut().write_all(b"\n").await.unwrap();
                }
                Request::ReserveBook { book_id, member_id } => {
                    let mut locked_state = state.lock().await;
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    let new_reservation = Reservation::new(book_id, member_id, now);
                    locked_state.reservations.push(new_reservation);
                    locked_state.save_to_file("library_data.json");
                    drop(locked_state);

                    let response = Response::Success;
                    let response_json = serde_json::to_string(&response).unwrap();
                    reader
                        .get_mut()
                        .write_all(response_json.as_bytes())
                        .await
                        .unwrap();
                    reader.get_mut().write_all(b"\n").await.unwrap();
                }
            }
        });
    }
}
