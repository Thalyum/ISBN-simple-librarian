use anyhow::Result;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tiny_http::{Request, Response};

use crate::{
    error::{self, Error},
    library::Library,
};

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
}

impl TryFrom<&tiny_http::Method> for Method {
    type Error = error::Error;

    fn try_from(value: &tiny_http::Method) -> Result<Self, Self::Error> {
        match value {
            tiny_http::Method::Get => Ok(Method::GET),
            tiny_http::Method::Post => Ok(Method::POST),
            _ => Err(Error::MethodUnauthorized(value.clone())),
        }
    }
}

impl Method {
    pub fn handle_request(&self, rq: Request, library: &mut Arc<Mutex<Library>>) -> Result<()> {
        let url = rq.url();
        println!(
            "'{:?}' query '{}' from '{}'",
            &self,
            url,
            rq.remote_addr().unwrap()
        );
        let ParsedUrl(endpoint, queries) = parse_url(url);

        // if any query key has not parameter => bad format: error 400
        if queries.iter().any(|(_, v)| v.is_none()) {
            let response = Response::empty(400);
            rq.respond(response)?;
            return Ok(());
        }

        match endpoint.as_str() {
            "/book" => {
                match self {
                    Method::GET => {
                        let response = Response::empty(405);
                        rq.respond(response)?;
                    }
                    Method::POST => Self::register_book(rq, queries, library)?,
                };
            }
            "/collection" => {
                match self {
                    Method::GET => Self::get_or_create_collection(rq, queries, library)?,
                    Method::POST => {
                        let response = Response::empty(405);
                        rq.respond(response)?;
                    }
                };
            }
            _ => {
                let response = Response::empty(400);
                rq.respond(response)?;
            }
        };
        Ok(())
    }

    fn register_book(
        rq: Request,
        queries: HashMap<String, Option<String>>,
        library: &mut Arc<Mutex<Library>>,
    ) -> Result<()> {
        match queries.get("isbn") {
            // Query 'isbn' is mandatory, and must have an argument
            Some(Some(isbn)) => {
                // Do not authorize empty isbn: Error 400
                if isbn.is_empty() {
                    let response = Response::empty(400);
                    rq.respond(response)?;
                } else {
                    let collection = queries
                        .get("collection_id")
                        .map(|o| Into::<Option<&String>>::into(o))
                        .flatten()
                        .map(|s| s.parse::<i32>())
                        .transpose()?;
                    library
                        .lock()
                        .unwrap()
                        .register_book(isbn.clone(), collection)?;
                }
            }
            _ => {
                let response = Response::empty(400);
                rq.respond(response)?;
            }
        }
        Ok(())
    }

    fn get_or_create_collection(
        rq: Request,
        queries: HashMap<String, Option<String>>,
        library: &mut Arc<Mutex<Library>>,
    ) -> Result<()> {
        match queries.get("name") {
            // Query 'name' is mandatory, and must have an argument
            Some(Some(name)) => {
                // Do not authorize empty name: Error 400
                if name.is_empty() {
                    let response = Response::empty(400);
                    rq.respond(response)?;
                } else {
                    let found = library.lock().unwrap().find_collection_by_name(name);
                    match found {
                        Some(id) => {
                            let response = Response::from_string(format!("{}", id));
                            rq.respond(response)?;
                        }
                        None => {
                            if matches!(queries.get("create"), Some(&Some(ref a)) if a == "true") {
                                // create new collection with name
                                let new_id = library.lock().unwrap().new_collection(name);
                                let response = Response::from_string(format!("{}", new_id));
                                rq.respond(response)?;
                            } else {
                                let err_msg = format!("Collection named '{name}' does not exist, and will not create it");
                                println!("{}", err_msg);
                                // collection does not exist: Error 404
                                let response = Response::from_string(err_msg).with_status_code(404);
                                rq.respond(response)?;
                            }
                        }
                    }
                }
            }
            _ => {
                let response = Response::empty(400);
                rq.respond(response)?;
            }
        }
        Ok(())
    }
}

type QueryMap = HashMap<String, Option<String>>;

struct ParsedUrl(String, QueryMap);

fn parse_url(url: &str) -> ParsedUrl {
    let s: Vec<&str> = url.splitn(2, '?').collect();
    let endpoint = s[0].to_string();
    let queries = s
        .get(1)
        .unwrap_or(&"")
        .split('&')
        .map(|q| q.splitn(2, '=').collect::<Vec<&str>>())
        .fold(QueryMap::new(), |mut acc, query| {
            let key = query[0].to_owned();
            let value = query.get(1).map(|p| p.to_string());
            acc.insert(key, value);
            acc
        });
    ParsedUrl(endpoint, queries)
}
