mod method;
mod req;
mod res;
mod status;

pub use method::Method;
pub use req::{ParseError, Request};
pub use res::Response;
pub use status::StatusCode;

/// Utility method for parsing the HTTP request
fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
}
