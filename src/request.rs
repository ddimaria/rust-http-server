use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::{prelude::*, Result};
use std::net::TcpStream;

use crate::headers::Headers;

const BUFFER_SIZE: usize = 512;

#[derive(Clone, Debug, PartialEq)]
pub enum Method {
    Delete,
    Get,
    Post,
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Method::Delete => write!(f, "DELETE"),
            Method::Get => write!(f, "GET"),
            Method::Post => write!(f, "POST"),
        }
    }
}

pub struct Request {
    pub method: Method,
    pub uri: String,
    pub query: HashMap<String, String>,
    pub headers: Headers,
    pub body: String,
    pub stream: TcpStream,
}

impl Request {
    pub fn new(mut stream: TcpStream) -> Result<Request> {
        let mut buffer = [0; BUFFER_SIZE];
        let length = stream.read(&mut buffer)?;

        let full = String::from_utf8_lossy(&buffer[0..length]);
        let mut lines = full.lines();

        let mut headers = Headers::new();

        let first_line = lines.by_ref().next();
        let (method, uri, query) = Request::parse_first_line(first_line);
        let query = query.unwrap_or(HashMap::new());
        let empty_line = |line: &&str| line != &"";

        // Parse the headers.  Starting at the second line, read until a line
        // with only a line break is found.
        lines
            .by_ref()
            .take_while(empty_line)
            .for_each(|line| headers.parse_line(line));

        // Parse the body portion, order is important here.
        let body = lines
            .take_while(empty_line)
            .collect::<Vec<&str>>()
            .join("\n");

        info!(
            "REQUEST: {} {} Query {:?} {:?}",
            method, uri, query, headers
        );

        Ok(Request {
            method,
            uri,
            query,
            headers,
            body,
            stream: stream,
        })
    }

    // The first line contains the "method", "uri", and "query params".
    // Ignore http version for now, we're assuming 1.1.
    // Improvement: Respond with a 400 for versions other than 1.1.
    // Improvement: Accept different http versions.
    fn parse_first_line(first_line: Option<&str>) -> (Method, String, Option<HashMap<String, String>>) {
        let mut split = first_line.unwrap_or("GET /").split_whitespace();
        let method = Request::parse_method(split.next());
        let (uri, query) = Request::parse_uri(split.next());

        (method, uri.into(), query)
    }

    // Parse the "method" portion, default to GET if absent.
    fn parse_method(method: Option<&str>) -> Method {
        match method {
            Some("DELETE") => Method::Delete,
            Some("GET") => Method::Get,
            Some("POST") => Method::Post,
            _ => Method::Get,
        }
    }

    // Parse the URI, separating out the uri from query string
    fn parse_uri(uri: Option<&str>) -> (&str, Option<HashMap<String, String>>) {
        let mut safe_uri = uri.unwrap_or("/");
        let uri_with_query = safe_uri.split("?").collect::<Vec<&str>>();
        let has_query_string = uri_with_query.len() == 2;

        let query: Option<HashMap<String, String>> = match has_query_string {
            true => {
                safe_uri = uri_with_query[0];
                Request::parse_query(uri_with_query[1])
            }
            false => None,
        };

        // Remove trailing slash to avoid duplicate entries
        if safe_uri.ends_with("/") && safe_uri != "/" {
            safe_uri = &safe_uri[0..safe_uri.len() - 1];
        }
        (safe_uri, query)
    }

    // Parse the query string portion of the URI
    fn parse_query(query: &str) -> Option<HashMap<String, String>> {
        let mut hashmap: HashMap<String, String> = HashMap::new();
        let items = query
            .to_string()
            .split("&")
            .for_each(|x| {
                let pair = x.split("=").collect::<Vec<&str>>();                
                if pair.len() == 2 {
                    hashmap.insert(pair[0].into(), pair[1].into());
                }
            });

        Some(hashmap)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_first_line_without_version() {
        let first_line = "POST /foo HTTP/1.1";
        let (method, uri, version) = Request::parse_first_line(Some(first_line));
        assert_eq!(method, Method::Post);
        assert_eq!(uri, "/foo".to_string());
        assert_eq!(version, None);
    }

    #[test]
    fn test_parse_first_line_with_version() {
        let first_line = "POST /foo?a=b HTTP/1.1";
        let (method, uri, query) = Request::parse_first_line(Some(first_line));
        assert_eq!(method, Method::Post);
        assert_eq!(uri, "/foo".to_string());
        assert_eq!(query.unwrap().get("a"), Some(&"b".to_string()));
    }

    #[test]
    fn test_parse_method() {
        let method = "POST";
        let parsed = Request::parse_method(Some(method));
        assert_eq!(parsed, Method::Post);
    }

    #[test]
    fn test_parse_uri() {
        let uri = "/foo";
        let parsed = Request::parse_uri(Some(uri));
        assert_eq!(parsed.0, uri);
    }

    #[test]
    fn test_parse_uri_removes_trailing_slash() {
        let uri = "/foo/";
        let parsed = Request::parse_uri(Some(uri));
        assert_eq!(parsed.0, "/foo");
    }

    #[test]
    fn test_parse_uri_with_query_string() {
        let uri = "/foo?a=b";
        let parsed = Request::parse_uri(Some(uri));
        assert_eq!(parsed.0, "/foo");
        assert_eq!(parsed.1.unwrap().get("a"), Some(&"b".to_string()));
    }

    #[test]
    fn test_parse_query() {
        let query = "a=b&c=d";
        let parsed = Request::parse_query(query).unwrap();
        assert_eq!(parsed.get("a"), Some(&"b".to_string()));
    }

    #[test]
    fn test_parse_complex_query() {
        let query = "a=b&c=d";
        let parsed = Request::parse_query(query).unwrap();
        assert_eq!(parsed.get("a"), Some(&"b".to_string()));
        assert_eq!(parsed.get("c"), Some(&"d".to_string()));
    }

    #[test]
    fn test_parse_query_skips_unmatched_pairs() {
        let query = "a=b&c&d&e";
        let parsed = Request::parse_query(query).unwrap();
        assert_eq!(parsed.keys().len(), 1);
    }
}
