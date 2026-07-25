use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Genre {
    Novel,
    Science,
    History,
    Technical,
    Other,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub genre: Genre,
    pub is_borrowed: bool,
    pub ratings: Vec<u8>,
}
impl Book {
    pub fn new(id: u32, title: String, author: String, genre: Genre) -> Self {
        Book {
            id: id,
            title: title,
            author: author,
            genre: genre,
            is_borrowed: false,
            ratings: Vec::new(),
        }
    }
    pub fn average_rating(&self) -> Option<f32> {
        if self.ratings.is_empty() {
            return None;
        }
        let length = self.ratings.len();
        let sum: u32 = self.ratings.iter().map(|r| *r as u32).sum();
        let result = (sum as f32) / (length as f32);
        return Some(result);
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Member {
   pub id: u32,
   pub name: String,
   pub borrowed_books: Vec<u32>,
}
impl Member {
    pub fn new(id: u32, name: String) -> Self {
        Member {
            id,
            name,
            borrowed_books : Vec::new(),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Loan {
    pub book_id: u32,
    pub member_id: u32,
    pub borrow_date: u64,
    pub due_date: u64,
    pub returned : bool,

}

impl Loan{
    pub fn new(book_id:u32, member_id:u32, borrow_date: u64, due_date:u64)->Self{
        Loan{
            book_id,
            member_id,
            borrow_date,
            due_date,
            returned:false,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Reservation{
    pub book_id: u32,
    pub member_id: u32,
    pub reservation_date: u64,
}
impl Reservation{
   pub fn new(book_id:u32, member_id:u32, reservation_date:u64) -> Self{
        Reservation{
            book_id,
            member_id,
            reservation_date,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Fine{
   pub member_id: u32,
   pub book_id: u32,
   pub overdue_days: u32,
   pub amount: u32,
   pub paid: bool,
}
impl Fine{
    pub fn new(member_id:u32, book_id:u32, overdue_days:u32, amount:u32) ->Self{
        Fine{
            member_id,
            book_id,
            overdue_days,
            amount,
            paid: false,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Rating{
    pub book_id: u32,
    pub member_id: u32,
    pub score: u8,
    pub comment: Option<String>,
}
impl Rating{
    pub fn new(book_id:u32, member_id:u32, score:u8, comment:Option<String>) -> Self{
        Rating{
            book_id,
            member_id,
            score,
            comment,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Request {
    AddBook {title: String, author: String, genre: Genre},
    ListBooks,
    BorrowBook {book_id: u32, member_id:u32},
    ReturnBook {book_id: u32, member_id: u32},
    RemoveBook {book_id : u32},
    EditBook { book_id: u32, title: String, author: String, genre: Genre},
    SearchBook {query : String},
    AddMember { name : String},
    RemoveMember {member_id: u32},
    EditMember { member_id: u32, name: String},
    SearchMember { query: String},
    ReserveBook {book_id : u32, member_id: u32},
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LibraryError{
    BookNotFound { book_id: u32 },
    MemberNotFound { member_id: u32 },
    BookAlreadyBorrowed,
    BookNotBorrowed,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
 pub enum Response{
    Success,
    Books(Vec<Book>),
    Members(Vec<Member>),
    BooksAdded {book_id: u32},
    Error(LibraryError),
 }


