use shared::{Genre, Request, Response};
use std::io::{self, Write};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn read_u32(prompt: &str) -> u32 {
    loop {
        let input = read_input(prompt);
        match input.parse::<u32>() {
            Ok(value) => return value,
            Err(_) => println!("Invalid number, please try again."),
        }
    }
}

fn choose_genre() -> Genre {
    loop {
        let input = read_input("Genre (1=Novel, 2=Science, 3=History, 4=Technical, 5=Other): ");
        match input.as_str() {
            "1" => return Genre::Novel,
            "2" => return Genre::Science,
            "3" => return Genre::History,
            "4" => return Genre::Technical,
            "5" => return Genre::Other,
            _ => println!("Invalid choice, please try again."),
        }
    }
}

async fn send_request(request: &Request) -> Result<Response, Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    let json = serde_json::to_string(request)?;
    stream.write_all(json.as_bytes()).await?;
    stream.write_all(b"\n").await?;
    stream.flush().await?;

    let mut reader = BufReader::new(stream);
    let mut response_line = String::new();
    reader.read_line(&mut response_line).await?;
    let response: Response = serde_json::from_str(&response_line)?;

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        println!("\n=== Library Management System ===");
        println!("0. Exit");
        println!("1. Add Book");
        println!("2. List Books");
        println!("3. Borrow Book");
        println!("4. Return Book");
        println!("5. Remove Book");
        println!("6. Edit Book");
        println!("7. Search Book");
        println!("8. Add Member");
        println!("9. Remove Member");
        println!("10. Edit Member");
        println!("11. Search Member");
        println!("12. Reserve Book");

        let choice = read_input("Choice: ");

        if choice == "0" {
            println!("Exiting program.");
            break;
        }

        let request = match choice.as_str() {
            "1" => {
                let title = read_input("Title: ");
                let author = read_input("Author: ");
                let genre = choose_genre();
                Some(Request::AddBook {
                    title,
                    author,
                    genre,
                })
            }
            "2" => Some(Request::ListBooks),
            "3" => {
                let book_id = read_u32("Book ID: ");
                let member_id = read_u32("Member ID: ");
                Some(Request::BorrowBook { book_id, member_id })
            }
            "4" => {
                let book_id = read_u32("Book ID: ");
                let member_id = read_u32("Member ID: ");
                Some(Request::ReturnBook { book_id, member_id })
            }
            "5" => {
                let book_id = read_u32("Book ID: ");
                Some(Request::RemoveBook { book_id })
            }
            "6" => {
                let book_id = read_u32("Book ID: ");
                let title = read_input("New Title: ");
                let author = read_input("New Author: ");
                let genre = choose_genre();
                Some(Request::EditBook {
                    book_id,
                    title,
                    author,
                    genre,
                })
            }
            "7" => {
                let query = read_input("Search query: ");
                Some(Request::SearchBook { query })
            }
            "8" => {
                let name = read_input("Name: ");
                Some(Request::AddMember { name })
            }
            "9" => {
                let member_id = read_u32("Member ID: ");
                Some(Request::RemoveMember { member_id })
            }
            "10" => {
                let member_id = read_u32("Member ID: ");
                let name = read_input("New Name: ");
                Some(Request::EditMember { member_id, name })
            }
            "11" => {
                let query = read_input("Search query: ");
                Some(Request::SearchMember { query })
            }
            "12" => {
                let book_id = read_u32("Book ID: ");
                let member_id = read_u32("Member ID: ");
                Some(Request::ReserveBook { book_id, member_id })
            }
            _ => {
                println!("Invalid choice.");
                None
            }
        };

        if let Some(req) = request {
            match send_request(&req).await {
                Ok(response) => println!("Result: {:?}", response),
                Err(e) => println!("Connection error: {}", e),
            }
        }
    }

    Ok(())
}
