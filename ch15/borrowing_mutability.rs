#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    author: &'static str,
    title: &'static str,
    year: u32, 
}

fn borrow_book(book: &Book) {
    println!("I immutably borrowed {} - {} edition", book.title, book.year);
}

fn new_edition(book: &mut Book) {
    book.year = 2024;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn main() {
    let immutabook = Book {
        author: "lng2020",
        title: "a funny joke",
        year: 2023,
    };
    let mut mutabook = immutabook;
    borrow_book(&immutabook);
    borrow_book(&mutabook);
    new_edition(&mut mutabook);
    // new_edition(&mut immutabook);
}