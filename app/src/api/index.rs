use crate::clients::database::{get_db, insert_data, get_data};
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

pub async fn publish(topic: &str, message: &str) -> Result<(), Error> {
    let db = get_db();
    Ok(())
}

pub async fn consume(topic: &str) -> Result<(), Error> {
    let db = get_db();
    Ok(())
}

pub async fn ack(topic: &str, messageId: &str) -> Result<(), Error> {
    let db = get_db();
    Ok(())
}