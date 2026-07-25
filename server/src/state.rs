use shared::{Book,Member,Loan,Reservation, Fine};
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
}
