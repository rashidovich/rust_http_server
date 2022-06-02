use super::HttpMethod;
use super::ParseError;

use std::convert::{TryFrom};
use std::str;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: HttpMethod
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(value)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: HttpMethod = method.parse()?;
        unimplemented!()
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, chtr) in request.chars().enumerate() {
        if chtr == ' ' || chtr == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }

    }

    None
}