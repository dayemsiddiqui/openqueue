use axum::Json;
use serde::Serialize;


pub async fn ping() -> &'static str {
    "pong"
}

#[derive(Serialize)]
pub struct IndexResponse {
    status: String,
    message: String,
}
pub async fn index() -> Json<IndexResponse> {
    Json(IndexResponse {    
        status: "OpenQueue is running".to_string(),
        message: "All systems are operational".to_string(),
    })
}
