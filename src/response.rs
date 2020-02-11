use std::io::prelude::*;
use std::net::TcpStream;

use crate::headers::Headers;

const LINE_BREAK: &str = "\r\n";
const EMPTY: String = String::new();

#[derive(Clone)]
pub enum HttpStatusCode {
    OK,
    NotFound,
    ServerError,
}

// Write the response to the TcpStream.
// Return an INTERNAL SERVER ERROR if issues occur.
// Improvement: Chunk response data.
// Improvement: gzip response data.
pub fn respond(
    mut stream: TcpStream,
    status: HttpStatusCode,
    headers: Option<Headers>,
    body: Option<String>,
) {
    let response = build_response(status, headers, body);
    // make this a closure so that it's only invoked for error cases
    let internal_server_error = || build_response(HttpStatusCode::ServerError, None, None);

    // Write the response to the TcpStream
    let _write = stream
        .write(response.as_bytes())
        .map_err(|_| internal_server_error());

    // Flush the TcpStream
    let _flush = stream.flush().map_err(|_| internal_server_error());
}

// Build the response string
fn build_response(
    status: HttpStatusCode,
    headers: Option<Headers>,
    body: Option<String>,
) -> String {
    let first_line = build_first_line(status);
    let headers = build_headers(headers).unwrap_or(EMPTY);
    let body = build_response_body(body).unwrap_or(EMPTY);
    let response = first_line + &headers + &body;

    info!("RESPONSE: {:?}", response);

    response
}

// Build the first line response string.
// Improvement: Handle all HTTP Status Codes.
// Improvement: Handle different HTTP versions.
fn build_first_line(status: HttpStatusCode) -> String {
    let first_line = match status {
        HttpStatusCode::OK => "200 OK",
        HttpStatusCode::NotFound => "404 Not Found",
        HttpStatusCode::ServerError => "500 Internal Server Error",
    };
    format!("HTTP/1.1 {}{}", first_line, LINE_BREAK)
}

// Build the headers portion of the response string
fn build_headers(headers: Option<Headers>) -> Option<String> {
    if let Some(headers) = headers {
        return Some(format!(
            "X-Data-Version: {}{}",
            headers.x_data_version.unwrap_or(0),
            LINE_BREAK,
        ));
    }

    None
}

// Build the response body portion string.
// Include Content-Type and Content-Length.
fn build_response_body(body: Option<String>) -> Option<String> {
    if let Some(body) = body {
        return Some(format!(
            "Content-Length: {}{}{}{}{}",
            body.len(),
            LINE_BREAK,
            LINE_BREAK,
            body,
            LINE_BREAK
        ));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_response() {
        let response = build_response(HttpStatusCode::ServerError, None, None);
        assert_eq!(response, "HTTP/1.1 500 Internal Server Error\r\n");
    }

    #[test]
    fn test_build_first_line() {
        let response = build_first_line(HttpStatusCode::OK);
        assert_eq!(response, "HTTP/1.1 200 OK\r\n");
    }

    #[test]
    fn test_build_headers() {
        let mut headers = Headers::new();
        headers.x_data_version = Some(2);
        let response = build_headers(Some(headers)).unwrap();
        assert_eq!(response, "X-Data-Version: 2\r\n");
    }

    #[test]
    fn test_build_headers_with_none() {
        let response = build_headers(None);
        assert!(response.is_none());
    }

    #[test]
    fn test_build_response_body() {
        let response_body = "body".to_string();
        let response = build_response_body(Some(response_body)).unwrap();
        assert_eq!(
            response,
            "Content-Length: 4\r\n\r\nbody\r\n"
        );
    }

    #[test]
    fn test_build_response_body_with_none() {
        let response = build_response_body(None);
        assert!(response.is_none());
    }
}
