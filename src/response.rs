use crate::http::StatusCodes;

#[derive(Debug, PartialEq, Eq)]
pub struct Response {
    pub body: Option<String>,
    pub content_type: Option<String>,
    pub status_code: StatusCodes,
}

impl Response {
    pub fn new(
        status_code: StatusCodes,
        content_type: Option<String>,
        body: Option<String>,
    ) -> Response {
        Response {
            body,
            content_type,
            status_code,
        }
    }

    pub fn build(self) -> Vec<u8> {
        let response = b"HTTP/1.1 ";
        let mut response_vec = response.to_vec();

        match self.status_code {
            StatusCodes::Ok => response_vec.extend_from_slice(b"200 OK"),
            StatusCodes::NotFound => response_vec.extend_from_slice(b"404 Not Found"),
            StatusCodes::Created => response_vec.extend_from_slice(b"201 Created"),
            StatusCodes::InternalServerError => {
                response_vec.extend_from_slice(b"500 Internal Server Error")
            }
        };

        response_vec.extend_from_slice(b"\r\n");

        if self.content_type.is_some() {
            let content_type = format!("Content-Type: {}", self.content_type.unwrap());
            response_vec.extend_from_slice(content_type.as_bytes());
            response_vec.extend_from_slice(b"\r\n");
        }

        if self.body.is_some() {
            let body_content = self.body.unwrap().clone();
            let content_length = format!("Content-Length: {}", body_content.len());
            response_vec.extend_from_slice(content_length.as_bytes());
            response_vec.extend_from_slice(b"\r\n\r\n");
            response_vec.extend_from_slice(body_content.as_bytes());
        }

        response_vec.extend_from_slice(b"\r\n");

        response_vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete_build_response() {
        let body = String::from("test response");
        let response = Response::new(
            StatusCodes::Ok,
            Some(String::from("text/plain")),
            Some(body.clone()),
        );

        let expected_response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}\r\n",
            body.len(),
            body,
        );

        assert_eq!(response.build(), expected_response.as_bytes());
    }

    #[test]
    fn test_build_response_without_body() {
        let response = Response::new(StatusCodes::Ok, None, None);

        let expected_response = b"HTTP/1.1 200 OK\r\n\r\n";

        assert_eq!(response.build(), expected_response);
    }

    #[test]
    fn test_build_response_without_content_type() {
        let response = Response::new(StatusCodes::Ok, None, Some(String::from("Hello, world!")));

        let expected_response = b"HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, world!\r\n";

        assert_eq!(response.build(), expected_response);
    }
}
