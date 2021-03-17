use std::{convert::TryFrom, fs, io::Read, net::TcpListener};

use crate::http::{Method, ParseError, Request, Response, StatusCode};

const BUFFER_SIZE: usize = 16 * 1024;

#[derive(Debug)]
pub struct Server {
    pub port: u16,
    pub public_path: String,
}

impl Default for Server {
    fn default() -> Self {
        Server {
            port: 8080,
            public_path: String::from("./"),
        }
    }
}

impl Server {
    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        fs::read_to_string(path).ok()
    }

    /// The main request handler method.
    /// GET requests sent to "/" will read the contents from a index.html file located in the public directory
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(
                StatusCode::NotFound,
                Some(String::from(
                    "Could not find a index.html file in this dir...",
                )),
            ),
        }
    }

    fn handle_bad_request(&mut self, err: &ParseError) -> Response {
        println!("Failed to parse a request: {}", err);
        Response::new(
            StatusCode::BadRequest,
            Some("Nothing to see here...".to_string()),
        )
    }
}

impl Server {
    pub fn new(port: u16, public_path: String) -> Self {
        Server { port, public_path }
    }

    pub fn run(mut self) {
        let address = format!("127.0.0.1:{}", self.port);
        println!("Server running on http://{}", &address);
        let listener = TcpListener::bind(address).expect("Error binding TcpListener");

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; BUFFER_SIZE];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => self.handle_request(&request),
                                Err(err) => self.handle_bad_request(&err),
                            };

                            if let Err(err) = response.send(&mut stream) {
                                println!("Failed to send response, {}", err);
                            }
                        }
                        Err(err) => println!("Failed to read from connection: {}", err),
                    }
                }
                Err(err) => println!("Failed to establish a connection: {}", err),
            }
        }
    }
}
