use crate::server::Handler;
use crate::http::{Request, Response};
use crate::http::status_code::StatusCode;
use std::fs;
use super::Method;

pub struct WebsiteHandler {
    public_path: String
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self {public_path}
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        // need to verify that the client isn't requesting files outside the public path.
        // compare "canonical" (aka "absolute") path and confirm that it starts with the
        // public path. otherwise return 404
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory traversal attack attempted: {}", file_path);
                    None
                }
            }
            Err(_) => None
        }
    }
}

impl Handler for WebsiteHandler {
    /// Handles a request from client and returns appropriate response
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                path => match self.read_file(path){
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                }
            }
            _ => Response::new(StatusCode::NotFound, None)
        }
    }
}