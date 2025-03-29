mod routes;
mod handlers;

use crate::handlers::request_handler::dynamic_handler;
use crate::routes::routing_service::RoutingService;

use axum::routing::get;

use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::new("axum=off,tower_http=off"))
    .with_writer(std::io::sink)
    .init();


    let ip_address = "127.0.0.1";
    let port_service = "3000";
 
    axum::serve(
        tokio::net::TcpListener::bind(
            RoutingService::new(
                ip_address.to_string(),
                port_service.to_string()
            ).get_address()
        ).await.unwrap(),
        RoutingService::new(
            ip_address.to_string(),
            port_service.to_string()
        ).route_with_method(
            "/",
            "".to_string(),
            get(dynamic_handler)
        )
    ).await.unwrap();

}
