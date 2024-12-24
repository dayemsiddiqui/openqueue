use crate::persistence::queue::Queue;
use crate::persistence::message::Message;
use crate::clients::database::get_db;
use serde::{Serialize, Deserialize};
use axum::Json;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumeResponse {
    message: Message,
}   
pub async fn consume(queue: String) -> Json<ConsumeResponse> {
    let queue = Queue::get(queue);
    let message = queue.consume().expect("Failed to consume message");
    let result = match message {        
        Some(message) => message,
        None => return Json(ConsumeResponse { message: Message::new("".to_string()).expect("Failed to create message") }),
    };
    Json(ConsumeResponse { message: result })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_consume_when_queue_is_empty() {
        // Initialize test data
        {
            let response = consume("test_queue".to_string()).await;
            println!("Response: {:?}", response);
            assert!(response.message.data.is_empty());
        }
    }

    #[tokio::test]
    async fn test_consume_when_queue_is_not_empty() {
        // Initialize test data
        {
            let queue = Queue::get("test_queue".to_string());
            let message = Message::new( "test_message".to_string()).expect("Failed to create message");
            queue.enqueue(&message).expect("Failed to enqueue message");

            // Ensure database operations are completed
            let response = consume("test_queue".to_string()).await;
            println!("Response: {:?}", response);
            assert!(!response.message.data.is_empty());
        }
    }
}