use shared::{Book,Member,Loan,Reservation, Fine};
pub struct LibraryState{
    pub books: Vec<Book>,
    pub members: Vec<Member>,
    pub loans: Vec<Loan>,
    pub reservations: Vec<Reservation>,
    pub fines: Vec<Fine>,
}
impl LibraryState{
    pub fn new() -> Self{
        LibraryState{
            books: Vec::new(),
            members: Vec::new(),
            loans: Vec::new(),
            reservations: Vec::new(),
            fines: Vec::new(),
        }
    }
}
