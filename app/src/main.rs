use axum::{
    routing::get,
    Router,
};

use app::clients::database::{get_db, insert_data, get_data};
use rand::Rng;
use app::api::index::ping;
#[tokio::main]
async fn main() {
    // Initialize the router with a route
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/ping", get(ping));

    // Run the server
    let listener = match tokio::net::TcpListener::bind("127.0.0.1:3000").await {
        Ok(listener) => {
            println!("TcpListener bound to 127.0.0.1:3000");
            listener
        },
        Err(err) => {
            println!("Failed to bind to port 3000: {}", err);
            // Terminate the program
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

// Handler that returns "Hello, World!"
async fn hello_world() -> String {
    let db = get_db();  
    let random_name = get_random_name();
    let _ = insert_data(&db, b"name", random_name.as_bytes());
    let value = get_data(&db, b"name").expect("Failed to get data");
    let value = String::from_utf8(value.unwrap()).expect("Failed to convert value to string");
    format!("Hello, World! This is a test and the value is: {:?}", value)   
}

fn get_random_name() -> String {
    // Create a list of random names
    let names = vec!["John", "Jane", "Alice", "Bob", "Charlie", "Diana", "Eve", "Frank", "Grace", "Hank"];
    // Return a random name from the list
    names[rand::thread_rng().gen_range(0..names.len())].to_string()
}   