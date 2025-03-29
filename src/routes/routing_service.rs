

use axum::{
    routing::{delete, get, head, options, patch, post, put, MethodRouter},
    Router
};


pub struct RoutingService {
    ip_address: String,
    port_service: String,
}



impl RoutingService {
    //Constructor for routage application
    pub fn new(ip_address: String, port_service: String) -> Self {
        RoutingService {
            ip_address,
            port_service,
        }
    }

    pub fn route_with_method(
        &self,
        route: &str,
        param: String,
        method: fn(MethodRouter) -> MethodRouter,
    ) -> Router {
        Router::new().route(route, method(axum::routing::any(|| async move { param })))
    }
    //Return ip + port
    pub fn get_address(&self) -> String {
        return format!("{}:{}", self.ip_address, self.port_service);
    }

    //GET
    pub fn routes_modules_get(&self, route: &str, param: String) -> Router {
        self.route_with_method(route, param, get)
    }
    //POST
    pub fn routes_modules_post(&self, route: &str, param: String) -> Router {
        self.route_with_method(route, param, post)
    }
    //PUT
    pub fn routes_modules_put(&self, route: &str, param: String) -> Router {
        self.route_with_method(route, param, put)
    }
    //DELETE
    pub fn routes_modules_delete(&self, route: &str, param: String) -> Router {
        self.route_with_method(route, param, delete)
    }
    //PATCH
    pub fn routes_modules_patch(&self, route: &str, param: String) -> Router {
        self.route_with_method(route, param, patch)
    }
    //OPTIONS
    pub fn routes_modules_options(&self, route: &str, param: String) -> Router {
        self.route_with_method(route, param, options)
    }
    //HEAD
    pub fn routes_modules_head(&self, route: &str, param: String) -> Router {
        self.route_with_method(route, param, head)
    }
}
