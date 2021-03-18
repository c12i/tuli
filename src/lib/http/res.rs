use std::io::{Result as IoResult, Write};

use super::status::StatusCode;

#[derive(Debug)]
pub struct Response {
    status: StatusCode,
    body: Option<String>,
    chunks: Option<Vec<u8>>,
}

impl Response {
    pub fn new(status: StatusCode, body: Option<String>) -> Self {
        Response {
            status,
            body,
            chunks: None,
        }
    }

    #[allow(unused)]
    pub fn from_chunks(status: StatusCode, chunks: Option<Vec<u8>>) -> Self {
        Response {
            status,
            body: None,
            chunks,
        }
    }

    pub fn send<T: Write>(&self, stream: &mut T) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        // write to Tcp stream
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status,
            self.status.reason_phrase(),
            body
        )
    }
}
