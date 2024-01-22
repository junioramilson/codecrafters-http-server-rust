use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;

use itertools::Itertools;
use nom::InputTake;

pub struct Request {
    pub headers: Vec<(String, String)>,
    pub path_parameters: HashMap<String, String>,
    pub method: String,
    pub path: String,
    pub body: Option<String>,
}

impl Request {
    pub fn new(stream: &mut TcpStream) -> Request {
        let mut buffer = [0u8; 4096];
        stream.read(&mut buffer);

        let request_lines = String::from_utf8_lossy(&buffer);
        let lines = request_lines.split("\r\n").collect::<Vec<&str>>();

        if lines.len() == 0 {
            return Request {
                headers: Vec::<(String, String)>::new(),
                path_parameters: HashMap::<String, String>::new(),
                method: String::new(),
                path: String::new(),
                body: None,
            };
        }

        let first_line = lines
            .first()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>();

        let method = first_line.get(0).unwrap().to_string();
        let path = first_line.get(1).unwrap().to_string();
        let http_version = first_line.get(2).unwrap().to_string();

        let mut headers = Vec::<(String, String)>::new();

        for line in lines.iter().skip(1) {
            match line.split_once(": ") {
                Some((header, value)) => headers.push((header.to_string(), value.to_string())),
                None => continue,
            }
        }

        let content_length = headers
            .iter()
            .find(|(header, _)| header == &"Content-Length")
            .map(|(_, value)| value.parse::<usize>().unwrap_or(0))
            .unwrap_or(0);

        let body = lines
            .iter()
            .skip_while(|item| !item.is_empty())
            .skip(1)
            .collect::<Vec<&&str>>()
            .get(0)
            .unwrap()
            .take(content_length);

        Request {
            headers,
            path_parameters: HashMap::<String, String>::new(),
            method,
            path,
            body: Some(body.to_owned()),
        }
    }
}
