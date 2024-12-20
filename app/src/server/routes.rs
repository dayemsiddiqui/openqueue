use axum::{
    routing::get,
    Router,
};
use crate::api::index::{ping, index};

// Configure all application routes
pub fn configure_routes() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/ping", get(ping))
}