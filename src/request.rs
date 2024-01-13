use std::io::{BufRead, BufReader};
use std::net::TcpStream;

pub struct Request {
    pub headers: Vec<(String, String)>,
    pub path_parameters: Vec<String>,
    pub method: String,
    pub path: String,
}

impl Request {
    pub fn new(stream: &mut TcpStream) -> Request {
        let buff_reader = BufReader::new(stream);

        eprintln!("Reading request: {:?}", buff_reader);

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
            .map(|(header, value)| (header.to_string(), value.to_string()))
            .collect::<Vec<_>>();

        Request {
            headers,
            path: path.to_owned(),
            method: method.to_owned(),
            path_parameters: Vec::new(),
        }
    }
}
