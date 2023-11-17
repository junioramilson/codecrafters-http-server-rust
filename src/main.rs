use nom::{AsBytes, ExtendInto};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

struct Response {
    pub body: Option<String>,
    pub content_type: Option<String>,
    pub status_code: StatusCodes,
}

enum StatusCodes {
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

fn handle_connection(mut stream: TcpStream) {
    println!("accepted new connection");

    let buff_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buff_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let (method, rest) = http_request
        .first()
        .unwrap()
        .split_once(" ")
        .expect("Unable to get Method from Http request");

    let (path, _) = rest
        .split_once(" ")
        .expect("Unable to get Path from Http request");

    let headers = http_request
        .iter()
        .skip(1)
        .map(|header| header.split_once(": ").unwrap())
        .collect::<Vec<_>>();

    match (path, method) {
        ("/", "GET") => {
            stream
                .write_all(
                    Response::new(StatusCodes::Ok, None, None)
                        .build()
                        .as_bytes(),
                )
                .unwrap();
        }
        ("/user-agent", "GET") => {
            let (_, user_agent_value) = headers
                .iter()
                .find(|(header, _)| header == &"User-Agent")
                .unwrap();

            stream
                .write_all(
                    Response::new(
                        StatusCodes::Ok,
                        Some(String::from("text/plain")),
                        Some(user_agent_value.to_string()),
                    )
                    .build()
                    .as_bytes(),
                )
                .unwrap();
        }
        (path, method) if path.contains("/echo/") && method == "GET" => {
            let (_, echo_value) = path.split_once("/echo/").unwrap();

            stream
                .write_all(
                    Response::new(
                        StatusCodes::Ok,
                        Some(String::from("text/plain")),
                        Some(String::from(echo_value)),
                    )
                    .build()
                    .as_bytes(),
                )
                .unwrap();
        }
        _ => {
            stream
                .write_all(
                    Response::new(StatusCodes::NotFound, None, None)
                        .build()
                        .as_bytes(),
                )
                .unwrap();
        }
    }
}

#[tokio::main]
async fn main() {
    println!("Logs from your program twill appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    loop {
        match listener.accept() {
            Ok((stream, _)) => {
                tokio::spawn(async move {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
