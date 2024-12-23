use axum::{
    routing::{get, post},
    Router,
};
use crate::api::index::{ping, index};
use crate::api::publish::publish;
use crate::api::stats::stats;
// Configure all application routes
pub fn configure_routes() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/ping", get(ping))
        .route("/publish", post(publish))
        .route("/stats", get(stats))
}