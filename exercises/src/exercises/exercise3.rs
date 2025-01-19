use crate::library::Library;
use crate::book::Book;

pub fn exercise3() {
    println!("\n-- Exercise 3: Define the `Library` struct, implement the `Display` trait for `Book` --");

    let mut library = Library::default();
    let book = Book::new("Rust Programming", "Rustacean", "1234");
    library.add_book(book);

    println!("Attempting to take an existing book...");
    match library.take_book("1234", "User1") {
        Ok(_) => println!("Take is successful!\n"),
        Err(e) => panic!("Error: {}", e),
    };

    println!("Attempting to take a non-existing book...");
    match library.take_book("9999", "User1") {
        Ok(_) => panic!("`library.take_book` function should return `Err` if book does not exist."),
        Err(e) => println!("Returned Error: {e} successfully!\n"),
    };

    println!("Attempting to take an existing book twice without returning...");
    match library.take_book("1234", "User2") {
        Ok(_) => panic!("`library.take_book` function should return `Err` if book is already taken."),
        Err(e) => println!("Returned Error: {e} successfully!\n"),
    };

    println!("Attempting to return a taken book...");
    match library.return_book("1234") {
        Ok(_) => println!("Returned successfully!\n"),
        Err(e) => panic!("Error: {}", e),
    };

    println!("Attempting to return the same book twice");
    match library.return_book("1234") {
        Ok(_) => panic!("`library.return_book` function should return `Err` if book is already returned."),
        Err(e) => println!("Returned Error: {e} successfully!\n"),
    }

    println!("Attempting to return a non-existing book");
    match library.return_book("9999") {
        Ok(_) => panic!("`library.return_book` function should return `Err` if book does not exist."),
        Err(e) => println!("Returned Error: {e} successfully!"),
    };

    println!("-- All the above operations should be successful without panicking --\n");
}
