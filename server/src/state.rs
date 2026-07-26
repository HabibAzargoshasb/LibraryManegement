use serde::{Deserialize, Serialize};
use shared::{Book, Fine, Loan, Member, Reservation};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LibraryState {
    pub books: Vec<Book>,
    pub members: Vec<Member>,
    pub loans: Vec<Loan>,
    pub reservations: Vec<Reservation>,
    pub fines: Vec<Fine>,
    pub next_book_id: u32,
    pub next_member_id: u32,
}

impl LibraryState {
    pub fn new() -> Self {
        LibraryState {
            books: Vec::new(),
            members: Vec::new(),
            loans: Vec::new(),
            reservations: Vec::new(),
            fines: Vec::new(),
            next_book_id: 1,
            next_member_id: 1,
        }
    }
    pub fn save_to_file(&self, path: &str) {
        let json = serde_json::to_string(self).unwrap();
        std::fs::write(path, json).unwrap();
    }
    pub fn load_from_file(path: &str) -> Self {
        match std::fs::read_to_string(path) {
            Ok(content) => match serde_json::from_str(&content) {
                Ok(state) => state,
                Err(_) => LibraryState::new(),
            },
            Err(_) => LibraryState::new(),
        }
    }
}
