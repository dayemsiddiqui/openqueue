use crate::clients::database::{get_db, insert_data, get_data};
use crate::persistence::queue::Message;
use std::error::Error as StdError;
use axum::Json;
use serde::{Serialize, Deserialize};
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

#[derive(Deserialize)]
pub struct PublishRequest {
    queue: String,
    message: String,
}

#[derive(Serialize)]
pub struct PublishResponse {
    status: String,
    message: String,
}

pub async fn publish(Json(payload): Json<PublishRequest>) -> Json<PublishResponse> {
    let queue = payload.queue;
    let message = payload.message;

    /*
     * Algorithm:   
     * 1. Check if the queue exists
     * 2. If it does, append the message to the queue
     * 3. If it doesn't, create the queue and append the message
     */
    let message = Message::new(message).expect("Failed to create message");
    Message::enqueue(queue, &message).expect("Failed to enqueue message");

    Json(PublishResponse {
        status: "success".to_string(),
        message: "Message enqueued successfully".to_string(),
    })
}

pub async fn consume(topic: &str) -> Result<(), Error> {
    let db = get_db();
    Ok(())
}

pub async fn ack(topic: &str, messageId: &str) -> Result<(), Error> {
    let db = get_db();
    Ok(())
}