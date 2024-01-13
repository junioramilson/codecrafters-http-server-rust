#[derive(Debug, PartialEq, Eq)]
pub struct Response {
    pub body: Option<String>,
    pub content_type: Option<String>,
    pub status_code: StatusCodes,
}

#[derive(PartialEq, Debug, Eq)]
pub enum StatusCodes {
    Ok = 200,
    NotFound = 404,
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
