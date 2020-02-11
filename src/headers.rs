#[derive(Clone, Debug)]
pub struct Headers {
    pub content_type: Option<String>,
    pub content_length: Option<usize>,
    pub x_data_version: Option<usize>,
}

impl Headers {
    pub fn new() -> Headers {
        Headers {
            content_type: None,
            content_length: None,
            x_data_version: None,
        }
    }

    // Parse each header line.
    // Improvement: Validate acceptable headers types.
    // Improvement: Parse all headers.
    pub fn parse_line(&mut self, line: &str) {
        if line.starts_with("Content-Type") {
            self.content_type = Headers::parse_content_type(line);
        };
        if line.starts_with("Content-Length") {
            self.content_length = Headers::parse_content_length(line);
        };
    }

    // Parse the value for the Content-Length header. Defaults to 0.
    // Improvement: Validate Content-Length for integer values.
    fn parse_content_length(content_length: &str) -> Option<usize> {
        let default = 0;

        Some(
            content_length
                .split_whitespace()
                .skip(1)
                .next()
                .unwrap_or("")
                .parse::<usize>()
                .unwrap_or(default),
        )
    }

    // Parse the value for the Content-Type header.  Defaults to "".
    // Improvement: Validate acceptable Content-Types.
    fn parse_content_type(content_type: &str) -> Option<String> {
        let default = "text/html";

        Some(
            content_type
                .split_whitespace()
                .skip(1)
                .next()
                .unwrap_or(default)
                .to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_content_length() {
        let mut headers = Headers::new();
        let length = 3;
        let line = format!("Content-Length: {}", length);
        headers.parse_line(&line);
        assert_eq!(length, headers.content_length.unwrap());
    }

    #[test]
    fn test_uses_default_content_lenght_when_empty() {
        let mut headers = Headers::new();
        headers.parse_line("Content-Length: ");
        assert_eq!(0, headers.content_length.unwrap());
    }

    #[test]
    fn test_parse_content_type() {
        let mut headers = Headers::new();
        let content_type = "application/json";
        let line = format!("Content-Type: {}", content_type);
        headers.parse_line(&line);
        assert_eq!(content_type, headers.content_type.unwrap());
    }

    #[test]
    fn test_uses_default_content_type_when_empty() {
        let mut headers = Headers::new();
        headers.parse_line("Content-Type: ");
        assert_eq!("text/html", headers.content_type.unwrap());
    }
}
