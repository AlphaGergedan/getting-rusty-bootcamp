use crate::library::Library;
use crate::book::Book;

pub fn exercise4() {
    println!("\n-- Exercise 4: Write `search_books` function to the `Library` struct using closures for filtering --");

    let mut library = Library::default();

    let book1 = Book::new("Rust Programming", "Rustacean", "1234");
    let book2 = Book::new("Advanced Rust", "Expert", "5678");
    let book3 = Book::new("JavaScript", "John Doe", "-1");

    library.add_book(book1);
    library.add_book(book2);
    library.add_book(book3);

    // `search_books` function accepts a closure
    let results = library.search_books(|b| b.title.contains("Rust"));
    for book in results {
        println!("{:?}", book);
    }

    println!("-- You should see all the books containing 'Rust' in their title above --\n");
}

