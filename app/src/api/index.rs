use crate::clients::database::get_db;
use std::error::Error as StdError;
use axum::Json;
use serde::Serialize;
type Error = Box<dyn StdError>;


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

pub async fn consume(queue: String) -> Result<(), Error> {
    let db = get_db();
    Ok(())
}

pub async fn ack(queue: String, messageId: String) -> Result<(), Error> {
    let db = get_db();
    Ok(())
}