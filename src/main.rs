mod request;
mod response;
mod router;
mod server;

use std::{
    fs::File,
    fs::{self, create_dir},
    io::Read,
    sync::Arc,
};

use crate::server::Server;
use request::Request;
use response::{Response, StatusCodes};

#[tokio::main]
async fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let file_directory: Arc<Option<String>> = Arc::new(
        std::env::args()
            .find(|arg| arg == "--directory")
            .and_then(|arg| std::env::args().nth(args.iter().position(|x| x == &arg).unwrap() + 1)),
    );

    if let Err(err) = create_dir("test_dir") {
        eprintln!("Failed to create directory: {}", err);
    }

    let mut server = Server::new("127.0.0.1:4221");

    server.add_route(
        String::from("GET"),
        String::from("/"),
        Arc::new(&|_| Response::new(StatusCodes::Ok, None, None)),
    );
    server.add_route(
        String::from("GET"),
        String::from("/user-agent"),
        Arc::new(&|request: Request| {
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
        }),
    );
    server.add_route(
        String::from("GET"),
        String::from("/"),
        Arc::new(&|_| Response::new(StatusCodes::Ok, None, None)),
    );
    server.add_route(
        String::from("GET"),
        String::from("/echo/:value"),
        Arc::new(&|request: Request| {
            let echo_value = request.path_parameters.get(&String::from("value")).unwrap();
            eprintln!("Echo value: {}", echo_value);

            Response::new(
                StatusCodes::Ok,
                Some(String::from("text/plain")),
                Some(String::from(echo_value)),
            )
        }),
    );
    server.add_route(
        String::from("GET"),
        String::from("/files/:filename"),
        Arc::new(move |request: Request| {
            let filename = request
                .path_parameters
                .get(&String::from("filename"))
                .unwrap()
                .replace("/", "_");

            let file_directory_clone = file_directory.as_ref().clone().unwrap_or(String::from("/"));

            let file =
                fs::read_dir(file_directory_clone.clone())
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
                        file_directory_clone.clone(),
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
        }),
    );

    server.start();
}
