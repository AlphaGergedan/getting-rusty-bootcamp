
#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
}

impl Book {
    pub fn new(title: &str, author: &str, isbn: &str) -> Self {
        Self {
            title: title.to_string(),
            author: author.to_string(),
            isbn: isbn.to_string(),
        }
    }
}

impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}' by {} (ISBN: {})", self.title, self.author, self.isbn)
    }
}

