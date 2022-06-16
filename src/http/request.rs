use super::HttpMethod;
use super::ParseError;
use super::{QueryString};

//use std::convert::{TryFrom};
use std::str;

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: HttpMethod
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(value: &'buf [u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(value)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: HttpMethod = method.parse()?;
        let mut query_string:Option<QueryString<'buf>> = None;

        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self{
            path,
            query_string,
            method
        })
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