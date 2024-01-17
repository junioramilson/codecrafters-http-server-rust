use crate::request::Request;
use crate::response::{Response, StatusCodes};
use crate::router::{RouteHandler, Router};
use nom::AsBytes;
use std::collections::HashMap;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

use std::sync::{Arc, Mutex};

pub struct Server {
    pub tcp_listener: TcpListener,
    pub router: Mutex<Arc<Router>>,
}

impl Server {
    pub fn new(addr: &str) -> Server {
        Server {
            tcp_listener: TcpListener::bind(addr).unwrap(),
            router: Mutex::new(Arc::new(Router::new())),
        }
    }

    pub fn add_route(
        &mut self,
        method: String,
        endpoint: String,
        handler: RouteHandler,
    ) -> &Server {
        let mut router = self.router.lock().unwrap();
        Arc::get_mut(&mut router)
            .unwrap()
            .add_route(method, endpoint, handler.clone());

        self
    }

    fn connection_handler(&self, mut stream: TcpStream) {
        eprintln!("Accepted new connection");

        let mut http_request = Request::new(&mut stream);

        let router = self.router.lock().unwrap().clone();
        let path_params = self
            .router
            .lock()
            .unwrap()
            .parse_path_params(http_request.path.to_string());

        http_request.path_parameters = match path_params {
            Some(ref path_params) => path_params.params.clone(),
            None => HashMap::<String, String>::new(),
        };

        let route_key = match path_params {
            Some(ref path_params) => path_params.route_key.clone(),
            None => (http_request.method.clone(), http_request.path.clone()),
        };

        println!("Route key: {:?}", route_key);

        match router.get_handler_by_endpoint(route_key) {
            Some(handler) => {
                let response = handler(http_request);
                stream.write_all(response.build().as_bytes()).unwrap();
            }
            None => {
                eprintln!(
                    "No handler found for {} {}",
                    http_request.method, http_request.path
                );
                stream
                    .write_all(
                        Response::new(StatusCodes::NotFound, None, None)
                            .build()
                            .as_bytes(),
                    )
                    .unwrap();
                return;
            }
        };
    }

    pub fn start(self) {
        eprintln!(
            "Server started on {}",
            self.tcp_listener.local_addr().unwrap()
        );
        let server = Arc::new(self);
        loop {
            match server.tcp_listener.accept() {
                Ok((stream, _)) => {
                    let server_clone = Arc::clone(&server);
                    tokio::spawn(async move {
                        server_clone.connection_handler(stream);
                    });
                }
                Err(e) => {
                    println!("TcpServer error: {}", e);
                }
            }
        }
    }
}
