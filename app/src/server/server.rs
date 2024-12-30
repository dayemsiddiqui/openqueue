use axum::Router;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use std::net::SocketAddr;
use std::time::Duration;
use super::routes;

pub async fn create_server() -> Router {
    // Add middleware stack focused on performance
    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .into_inner();

    routes::configure_routes()
        .layer(middleware_stack)
}

pub async fn start_server(app: Router) {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000)); // Listen on all interfaces
    let tcp = tokio::net::TcpSocket::new_v4().unwrap();
    
    // Optimize TCP for high throughput
    tcp.set_reuseaddr(true).unwrap();
    tcp.set_reuseport(true).unwrap();
    
    // Set TCP_NODELAY to reduce latency
    tcp.set_nodelay(true).unwrap();
    
    tcp.bind(addr).unwrap();
    
    // Increase backlog significantly for high-concurrency
    let listener = tcp.listen(4096).unwrap(); // Increased from 1024
    println!("TcpListener bound to {}", addr);

    println!("Server running on http://{}", addr);
    println!("Press Ctrl+C to stop the server");

    // Configure the server with optimized settings
    let server = axum::serve(
        TcpListener::from_std(listener).unwrap(),
        app.into_make_service_with_connect_info::<SocketAddr>()
    )
    .with_graceful_shutdown(shutdown_signal());

    // Start the server
    match server.await {
        Ok(_) => println!("Server shutdown gracefully"),
        Err(e) => println!("Failed to serve: {}", e),
    };
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
}