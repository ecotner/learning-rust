use std::io::{Write, Result as IoResult};
use super::status_code::StatusCode;

pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response {status_code, body}
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(string) => string,
            None => "",
        };
        write!(
            stream,
            "HTTP/1.1 {} {}\n\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )       
    }
}
