mod http;
mod request;
mod response;
mod router;
mod server;

use std::{
    fs::File,
    fs::{self},
    io::{Read, Write},
};

use crate::server::Server;
use http::{HttpMethod, StatusCodes};
use request::Request;
use response::Response;

#[tokio::main]
async fn main() {
    let mut server = Server::new("127.0.0.1:4221");

    server.add_route(HttpMethod::Get, "/", &|_| {
        Response::new(StatusCodes::Ok, None, None)
    });

    server.add_route(HttpMethod::Get, "/user-agent", &|request: Request| {
        let (_, user_agent_value) = request
            .headers
            .iter()
            .find(|(header, _)| header == &"User-Agent")
            .unwrap();

        Response::new(
            StatusCodes::Ok,
            Some(String::from("text/plain")),
            Some(user_agent_value.to_string()),
        )
    });
    server.add_route(HttpMethod::Get, "/", &|_| {
        Response::new(StatusCodes::Ok, None, None)
    });
    server.add_route(HttpMethod::Get, "/echo/:value", &|request: Request| {
        let echo_value = request.path_parameters.get(&String::from("value")).unwrap();
        eprintln!("Echo value: {}", echo_value);

        Response::new(
            StatusCodes::Ok,
            Some(String::from("text/plain")),
            Some(String::from(echo_value)),
        )
    });
    server.add_route(
        HttpMethod::Get,
        "/files/:filename",
        &move |request: Request| {
            let filename = request
                .path_parameters
                .get(&String::from("filename"))
                .unwrap()
                .replace("/", "_");

            let args = std::env::args().collect::<Vec<_>>();
            let file_directory: Option<String> = std::env::args()
                .find(|arg| arg == "--directory")
                .and_then(|arg| {
                    std::env::args().nth(&args.iter().position(|x| x == &arg).unwrap() + 1)
                });

            let file = fs::read_dir(file_directory.clone().unwrap())
                .unwrap()
                .find(|entry| match entry {
                    Ok(entry) => {
                        entry
                            .file_name()
                            .to_str()
                            .unwrap()
                            .split(".")
                            .collect::<Vec<&str>>()[0]
                            == filename
                    }
                    _ => false,
                });

            match file {
                Some(file) => {
                    let found_filename = file.unwrap().file_name();
                    let file = File::open(format!(
                        "{}/{}",
                        file_directory.clone().unwrap(),
                        found_filename.to_str().unwrap()
                    ));

                    let mut buf = Vec::<u8>::new();

                    file.unwrap().read_to_end(&mut buf);

                    Response::new(
                        StatusCodes::Ok,
                        Some(String::from("application/octet-stream")),
                        Some(String::from_utf8(buf).unwrap()),
                    )
                }
                None => return Response::new(StatusCodes::NotFound, None, None),
            }
        },
    );

    server.add_route(
        HttpMethod::Post,
        "/files/:filename",
        &move |request: Request| {
            let filename = request
                .path_parameters
                .get(&String::from("filename"))
                .unwrap();

            let body = request.body.unwrap();

            let args = std::env::args().collect::<Vec<_>>();
            let file_directory: Option<String> = std::env::args()
                .find(|arg| arg == "--directory")
                .and_then(|arg| {
                    std::env::args().nth(&args.iter().position(|x| x == &arg).unwrap() + 1)
                });

            fs::File::create(format!("{}/{}", file_directory.clone().unwrap(), filename))
                .unwrap()
                .write_all(body.as_bytes())
                .unwrap();

            Response::new(
                StatusCodes::Created,
                Some(String::from("text/plain")),
                Some(String::from(filename)),
            )
        },
    );

    server.start();
}
