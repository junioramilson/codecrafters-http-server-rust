mod request;
mod response;
mod router;
mod server;

use std::{fs::create_dir, fs::File, io::Read, sync::Arc};

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
                .unwrap();

            if filename.split("/").count() > 0 && filename.split(".").count() != 2 {
                return Response::new(StatusCodes::NotFound, None, None);
            }

            println!("Filename: {}", filename);

            let file_directory_clone = file_directory.as_ref().clone();

            let file = File::open(format!(
                "{}/{}",
                file_directory_clone.unwrap_or(String::from("/")),
                filename
            ));

            if file.is_err() {
                return Response::new(StatusCodes::NotFound, None, None);
            }

            let mut file_content = String::new();
            file.unwrap().read_to_string(&mut file_content);

            Response::new(
                StatusCodes::Ok,
                Some(String::from("application/octet-stream")),
                Some(file_content),
            )
        }),
    );

    server.start();
}
