use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error("No such collection ID: {0}")]
    NoSuchCollectionId(i32),
    #[error("Method not authorized: {0}")]
    MethodUnauthorized(tiny_http::Method),
}
