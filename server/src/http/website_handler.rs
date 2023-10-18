use crate::server::Handler;
use crate::http::{Request, Response};
use crate::http::status_code::StatusCode;

pub struct WebsiteHandler;

impl WebsiteHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        Response::new(StatusCode::Ok, Some("<h1>TEST</h1>".to_string()))
    }
}