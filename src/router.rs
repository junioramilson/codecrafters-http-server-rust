use crate::http::HttpMethod;
use crate::request::Request;
use crate::response::Response;
use std::collections::HashMap;
use std::sync::Arc;

pub type RouteHandler = dyn Fn(Request) -> Response + Send + Sync;
pub type Endpoint = String;
pub type RouteKey = (String, Endpoint);
pub type RouteMap = HashMap<RouteKey, Arc<RouteHandler>>;

#[derive(Clone)]
pub struct Router {
    routes: Arc<RouteMap>,
}

pub struct ParsedPathParams {
    pub route_key: RouteKey,
    pub params: HashMap<String, String>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            routes: Arc::new(RouteMap::new()),
        }
    }

    pub fn get_handler_by_endpoint(&self, route_key: RouteKey) -> Option<&Arc<RouteHandler>> {
        self.routes.get(&route_key)
    }

    pub fn parse_path_params(
        &self,
        req_method: String,
        req_endpoint: String,
    ) -> Option<ParsedPathParams> {
        let mut path_params = HashMap::<String, String>::new();
        let mut route_key: Option<(String, String)> = None;

        let splited_req_endpoint = req_endpoint
            .split("/")
            .filter(|v| !v.is_empty())
            .collect::<Vec<&str>>();

        self.routes
            .keys()
            .filter(|(method, _)| method == &req_method)
            .for_each(|(method, endpoint)| {
                let endpoint = endpoint.clone();
                let splited_defined_enpoint = endpoint
                    .split("/")
                    .filter(|v| !v.is_empty())
                    .collect::<Vec<&str>>();

                if route_key.is_some() {
                    return;
                }

                for (index, endpoint_value) in splited_defined_enpoint.clone().iter().enumerate() {
                    let request_path_by_index = match splited_req_endpoint.clone().iter().nth(index)
                    {
                        Some(&request_path_by_index) => request_path_by_index,
                        None => return,
                    };

                    if endpoint_value.contains(":") {
                        path_params.insert(
                            endpoint_value.replace(":", ""),
                            splited_req_endpoint[index..].join("/"),
                        );
                    } else if endpoint_value == &request_path_by_index {
                        route_key = Some((method.clone(), endpoint.clone()));
                    } else {
                        path_params.clear();
                        route_key = None;
                        break;
                    }
                }
            });

        match route_key {
            Some(route_key) => Some(ParsedPathParams {
                route_key,
                params: path_params,
            }),
            _ => None,
        }
    }

    pub fn add_route(
        &mut self,
        method: HttpMethod,
        endpoint: Endpoint,
        handler: &'static RouteHandler,
    ) -> &Router {
        Arc::make_mut(&mut self.routes).insert((method.to_string(), endpoint), Arc::new(handler));

        self
    }
}
