use crate::library::Library;
use crate::book::Book;

pub fn exercise5() {
    println!("\n-- Exercise 5: Implement the `Display` trait for `Library` --");

    let mut library = Library::default();

    let book1 = Book::new("Rust Programming", "Rustacean", "1234");
    let book2 = Book::new("Advanced Rust", "Expert", "5678");
    let book3 = Book::new("Rust book", "Contributors", "9012");

    library.add_book(book1);
    library.add_book(book2);
    library.add_book(book3);

    // Requires `Display` trait to be implemented for `Library`
    print!("{}", library);

    println!("-- You should see the library nicely printed above --\n");
}

