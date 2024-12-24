use crate::persistence::queue::Queue;
use crate::persistence::message::Message;
use serde::{Serialize, Deserialize};
use axum::Json;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumeResponse {
    message: Message,
}   
pub async fn consume(queue: String) -> Json<ConsumeResponse> {
    let queue = Queue::get(queue);
    let message = queue.consume().expect("Failed to consume message");
    Json(ConsumeResponse { message })
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    fn setup() {
        // Clean up any existing test database
        if Path::new("database").exists() {
            fs::remove_dir_all("database").expect("Failed to clean up test database");
        }
    }

    #[tokio::test]
    async fn test_consume() {
        setup();
        
        let queue = Queue::get("test_queue".to_string());
        let message = Message::new("test_message".to_string()).expect("Failed to create message");
        let message_id = message.id.clone();
        queue.enqueue(&message).expect("Failed to enqueue message"); 

        let response = consume("test_queue".to_string()).await;
        assert_eq!(response.message.id, message_id);
    }
}