#[derive(Debug)]
pub struct Book {
    isbn: String,
}

impl Book {
    pub fn new(isbn: String) -> Self {
        Self { isbn }
    }

    pub fn isbn(&self) -> &str {
        &self.isbn
    }
}
