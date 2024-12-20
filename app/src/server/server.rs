use axum::Router;
use super::routes;

pub async fn create_server() -> Router {
    routes::configure_routes()
}

pub async fn start_server(app: Router) {
    let listener = match tokio::net::TcpListener::bind("127.0.0.1:3000").await {
        Ok(listener) => {
            println!("TcpListener bound to 127.0.0.1:3000");
            listener
        },
        Err(err) => {
            println!("Failed to bind to port 3000: {}", err);
            std::process::exit(1);  
        },
    };

    println!("Server running on http://127.0.0.1:3000");
    println!("Press Ctrl+C to stop the server");

    match axum::serve(listener, app).await {
        Ok(_) => println!("Server shutdown gracefully"),
        Err(e) => println!("Failed to serve: {}", e),
    };
}