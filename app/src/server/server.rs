use axum::Router;
use tokio::net::{TcpListener, TcpSocket};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use std::net::SocketAddr;
use super::routes;

pub async fn create_server() -> Router {
    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .into_inner();

    routes::configure_routes()
        .layer(middleware_stack)
}

pub async fn start_server(app: Router) {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("TcpListener bound to {}", addr);

    println!("Server running on http://{}", addr);
    println!("Press Ctrl+C to stop the server");

    let server = axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>()
    )
    .with_graceful_shutdown(shutdown_signal());

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