use crate::request::Request;
use crate::response::Response;
use std::collections::HashMap;
use std::sync::Arc;

pub type RouteHandler = Arc<dyn Fn(Request) -> Response + Send + Sync>;
pub type Endpoint = String;
pub type Method = String;
pub type RouteKey = (Method, Endpoint);
pub type RouteMap = HashMap<RouteKey, RouteHandler>;

#[derive(Clone)]
pub struct Router {
    routes: Arc<RouteMap>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            routes: Arc::new(RouteMap::new()),
        }
    }

    pub fn get_route_handler(
        &self,
        method: Method,
        request_endpoint: Endpoint,
    ) -> Option<RouteHandler> {
        let matched_path = self.routes.iter().find(|((_, path), _)| {
            let base_path = match path.split("/*").collect::<Vec<_>>().first() {
                Some(base_path) => Some(base_path.to_string()),
                _ => None,
            };

            let base_endpoint = match request_endpoint
                .split("/")
                .collect::<Vec<_>>()
                .iter()
                .nth(1)
            {
                Some(base_endpoint) => Some(base_endpoint.to_string()),
                _ => None,
            };

            // FIXME
            if base_path.is_some() && base_endpoint.is_some() {
                base_path.unwrap() == format!("/{}", base_endpoint.unwrap())
            } else {
                false
            }
        });

        if matched_path.is_some() {
            Some(matched_path.unwrap().1.clone())
        } else {
            self.routes.get(&(method, request_endpoint)).cloned()
        }
    }

    pub fn add_route(
        &mut self,
        method: Method,
        endpoint: Endpoint,
        handler: RouteHandler,
    ) -> &Router {
        Arc::make_mut(&mut self.routes).insert((method, endpoint), handler);

        self
    }
}
