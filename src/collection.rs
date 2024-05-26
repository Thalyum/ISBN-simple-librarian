use std::sync::Arc;

use crate::book::Book;

#[derive(Debug)]
pub struct Collection {
    id: i32,
    name: String,
    books: Vec<Arc<Book>>,
}

impl Collection {
    pub fn new(id: i32, name: String) -> Self {
        Self {
            id,
            name,
            books: vec![],
        }
    }

    #[allow(dead_code)]
    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name
    }

    #[allow(dead_code)]
    pub fn collection_size(&self) -> usize {
        self.books.len()
    }

    pub fn add_book(&mut self, new_book: Arc<Book>) {
        self.books.push(new_book)
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
