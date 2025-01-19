use std::collections::HashMap;
use crate::book::Book;

#[derive(Default)]
pub struct Library {
    pub books: Vec<Book>,
    // ISBN -> User
    pub borrowed: HashMap<String, Option<String>>,
}

impl Library {
    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
        let book_ref = self.books.last().expect("Failed to get latest added book");
        self.borrowed.insert(book_ref.isbn.clone(), None);
    }

    pub fn print_books(&self) {
        for book in &self.books {
            println!("{}", book);
        }
    }

    pub fn take_book(&mut self, isbn: &str, user: &str) -> Result<(), String> {
        match self.borrowed.get_mut(isbn) {
            Some(borrow_status) if borrow_status.is_none() => {
                *borrow_status = Some(user.to_string());
                Ok(())
            },
            Some(_) => Err("Book is already borrowed.".into()).into(),
            None => Err("Book not found.".into()),
        }
    }

    pub fn return_book(&mut self, isbn: &str) -> Result<(), String> {
        match self.borrowed.get_mut(isbn) {
            Some(borrow_status) if borrow_status.is_some() => {
                *borrow_status = None;
                Ok(())
            },
            Some(_) => Err("Book was not borrowed.".into()),
            None => Err("Book not found.".into())
        }
    }

    pub fn search_books<F>(&self, filter_closure: F) -> Vec<&Book>
    where
        F: Fn(&Book) -> bool,
    {
        self.books.iter().filter(|&book| filter_closure(book)).collect()
    }
}

impl std::fmt::Display for Library {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        writeln!(f, "Library Contents:")?;
        for book in &self.books {
            let status = match self.borrowed.get(&book.isbn) {
                Some(Some(user)) => format!("Borrowed by: {}", user),
                Some(None) => String::from("Available"),
                None => String::from("Status unknown"),
            };
            writeln!(f, "{} - {}", book, status)?;
        }

        Ok(())
    }
}
