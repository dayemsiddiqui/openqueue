use crate::persistence::queue::Queue;
use serde::{Serialize, Deserialize};
use axum::Json;
use crate::persistence::message::Message;
#[derive(Debug, Serialize, Deserialize)]
pub struct AckRequest {
    queue: String,
    message_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AckResponse {
    status: String,
    message_id: String,
    error: Option<String>,
}

pub async fn ack(Json(payload): Json<AckRequest>) -> Json<AckResponse> {
    let queue = Queue::get(payload.queue);
    match queue.ack(&payload.message_id) {
        Ok(true) => Json(AckResponse { status: "success".to_string(), message_id: payload.message_id, error: None }),
        Ok(false) => Json(AckResponse { status: "error".to_string(), message_id: "".to_string(), error: None }),
        Err(e) => Json(AckResponse { status: "error".to_string(), message_id: "".to_string(), error: Some(e.to_string()) }),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ack_when_message_is_in_queue() {
        let queue = Queue::get("test_queue".to_string());
        let message = Message::new("test_message".to_string()).expect("Failed to create message");
        queue.enqueue(&message).expect("Failed to publish message");


        let payload = AckRequest { queue: "test_queue".to_string(), message_id: "test_message_id".to_string() };
        let response = ack(Json(payload)).await;
        assert!(response.status == "success");
    }

    #[tokio::test]
    async fn test_ack_when_message_is_not_in_queue() {
        let queue = Queue::get("test_queue".to_string());
        queue.clear();
        let payload = AckRequest { queue: "test_queue".to_string(), message_id: "test_message_id".to_string() };
        let response = ack(Json(payload)).await;
        assert!(response.status == "error");
    }
}