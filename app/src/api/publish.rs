use axum::Json;
use serde::{Serialize, Deserialize};
use crate::persistence::message::Message;
use crate::persistence::queue::Queue;

#[derive(Deserialize)]
pub struct PublishRequest {
    pub queue: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct PublishResponse {
    pub status: String,
    pub message: String,
    pub message_key: String,
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
    let queue = Queue::get(queue);
    let message_key = queue.enqueue(&message).expect("Failed to enqueue message");

    Json(PublishResponse {
        status: "success".to_string(),
        message: "Message enqueued successfully".to_string(),
        message_key,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::Json;

    #[tokio::test]
    async fn test_publish_success() {
        let request = PublishRequest {
            queue: "test_queue".to_string(),
            message: "test_message".to_string(),
        };

        let result = publish(Json(request)).await;
        
        assert_eq!(result.status, "success");
        assert_eq!(result.message, "Message enqueued successfully");
        assert!(!result.message_key.is_empty());
    }
} 