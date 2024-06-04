use std::sync::Arc;

use crate::{book::Book, collection::Collection, error::Error};

const FIRST_COLLECTION_ID: i32 = 1;

#[derive(Debug)]
pub struct Library {
    collections: Vec<Collection>,
    books: Vec<Arc<Book>>,
    next_id: i32,
}

impl Library {
    pub fn new() -> Self {
        Self {
            collections: vec![],
            books: vec![],
            next_id: FIRST_COLLECTION_ID,
        }
    }

    fn add_book(&mut self, book: Book) {
        let book = Arc::new(book);
        self.books.push(book)
    }

    fn add_book_with_collection(&mut self, book: Book, collection_id: i32) -> Result<(), Error> {
        let book = Arc::new(book);
        self.books.push(book.clone());
        self.collections
            .iter_mut()
            .find(|c| c.id() == collection_id)
            .ok_or(Error::NoSuchCollectionId(collection_id))
            .map(|c| c.add_book(book))
    }

    pub fn new_collection(&mut self, name: &str) -> i32 {
        let new_id = self.next_id;
        self.collections
            .push(Collection::new(new_id, name.to_string()));
        self.next_id += 1;
        new_id
    }

    pub fn find_collection_by_name(&self, name: &str) -> Option<i32> {
        self.collections
            .iter()
            .find(|c| c.name() == name)
            .map(|c| c.id())
    }

    pub fn register_book(&mut self, isbn: String, collection: Option<i32>) -> Result<(), Error> {
        let new_book = Book::new(isbn);
        match collection {
            Some(id) => {
                self.add_book_with_collection(new_book, id)?;
            }
            None => {
                self.add_book(new_book);
            }
        }
        Ok(())
    }
}
