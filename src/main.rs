use std::fmt::format;
use std::io::{Write, BufReader, BufRead};
use std::net::{TcpListener, TcpStream};

fn build_response(status: &str, content_type: Option<&str>, content: Option<&str>) -> String {
    let mut response_lines = Vec::<String>::new();
    response_lines.push(format!("HTTP/1.1 {:}", status));

    if content_type.is_some() {
        response_lines.push(format!("Content-Type: {}", content_type.unwrap()));
    }

    if content.is_some() {
        response_lines.push(format!("Content-Length: {}\r\n", content.unwrap().bytes().len()));
        response_lines.push(content.unwrap().to_string());
    }

    response_lines.push(String::from("\r\n"));
    let response = response_lines.join("\r\n");

    response
}

fn handle_connection(mut stream: TcpStream) {
    println!("accepted new connection");

    let buff_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buff_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let (method, rest) = http_request.first().unwrap().split_once(" ").expect("Unable to get Method from Http request");
    let (path, _) = rest.split_once(" ").expect("Unable to get Path from Http request");

    match path {
        "/" => {
            stream.write_all(build_response("200 OK", None, None).as_bytes()).unwrap();
        },
        path if path.contains("/echo/") => {
            let (_, echo_value) = path.split_once("/echo/").unwrap();

            stream.write_all(build_response("200 OK", Some("text/plain"), Some(echo_value)).as_bytes()).unwrap();
        }
        _ => {
            stream.write_all(build_response("404 Not Found", None, None).as_bytes()).unwrap();
        }
    }
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program twill appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
