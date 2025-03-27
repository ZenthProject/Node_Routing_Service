
use axum::{routing::{get, post}, Router};
pub struct RoutingService{
    ip_address: String,
    port_service: String 
}

impl RoutingService {
    //Constructor for routage application
    pub fn new(ip_address: String, port_service: String) -> Self {
        RoutingService {
            ip_address,
            port_service,
        }
    }

    pub fn RoutingDefault(flag: String){
        
    }

    //Return ip + port 
    pub fn get_address(&self) -> String {
        return format!("{}:{}", self.ip_address, self.port_service)
    }

    pub fn routes_modules_get(&self, route: String, param: String) -> Router {
        Router::new().route(&route, get(move || async move { param.to_string() }))    
    }

}