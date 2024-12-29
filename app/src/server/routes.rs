use axum::{
    routing::{get, post},
    Router,
};
use crate::api::index::{ping, index};
use crate::api::publish::publish;
use crate::api::stats::stats;
use crate::api::control_panel::control_panel;
use crate::api::ack::ack;
use crate::api::consume::consume;
// Configure all application routes
pub fn configure_routes() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/ping", get(ping))
        .route("/publish", post(publish))
        .route("/stats", get(stats))
        .route("/cp", get(control_panel))
        .route("/ack", post(ack))
        .route("/consume", get(consume))
}