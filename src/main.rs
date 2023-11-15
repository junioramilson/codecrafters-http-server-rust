use std::io::{Write, BufReader, BufRead};
use std::net::{TcpListener, TcpStream};

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

    let mut response = "HTTP/1.1 404 Not Found\r\n\r\n";
    if path == "/" {
        response = "HTTP/1.1 200 OK\r\n\r\n";
    }

    stream.write_all(response.as_bytes()).unwrap();
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
