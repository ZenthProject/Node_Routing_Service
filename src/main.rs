mod routes;

use crate::routes::routing_service::routing_service;
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
            routing_service::new(
                ip_address.to_string(),
                port_service.to_string()
            ).get_address()
        ).await.unwrap(), 
        routing_service::new(
            ip_address.to_string(),
            port_service.to_string()
        ).routes_modules_get(
            "/".to_string(),
            "Welcome to the server relay Zenth !".to_string()
        )
    ).await.unwrap();

}
