use crate::book::Book;

pub fn exercise1() {
    println!("\n-- Exercise 1: Define the `Book` struct --");

    // Example usage of the `new` constructor
    let book = Book::new("Rust programming", "Rustacean", "1234");

    // Debug print, you can derive the `Debug` trait for this
    println!("{:?}", book);

    println!("-- You should see a single book printed above --\n");
}
