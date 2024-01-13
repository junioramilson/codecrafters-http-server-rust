mod request;
mod response;
mod router;
mod server;

use std::sync::Arc;

use crate::server::Server;
use request::Request;
use response::{Response, StatusCodes};

#[tokio::main]
async fn main() {
    let mut server = Box::new(Server::new("127.0.0.1:4221"));

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
        String::from("/echo/*"),
        Arc::new(&|request: Request| {
            let (_, echo_value) = request.path.split_once("/echo/").unwrap();
            eprintln!("Echo value: {}", echo_value);

            Response::new(
                StatusCodes::Ok,
                Some(String::from("text/plain")),
                Some(String::from(echo_value)),
            )
        }),
    );

    server.start();
}
