use crate::library::Library;
use crate::book::Book;

pub fn exercise2() {
    println!("\n-- Exercise 2: Define `Library` struct, implement the `Display` trait for `Book` --");

    // Use the derived default constructor. You can derive `Default` trait for this.
    let mut library = Library::default();

    let book1 = Book::new("Rust Programming", "Rustacean", "1234");
    let book2 = Book::new("Advanced Rust", "Expert", "5678");

    // Adding books to the library by taking ownership of the books.
    library.add_book(book1);
    library.add_book(book2);

    // Print all books in the library, requires `Display` trait to be implemented for `Book`.
    library.print_books();

    println!("-- You should see all the books in the library (two books) printed above --\n");
}
