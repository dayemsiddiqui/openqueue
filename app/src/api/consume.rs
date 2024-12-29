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

    #[tokio::test]
    async fn test_when_multiple_consumers_are_not_consuming_the_same_message() {
        // Initialize test data
        {
            let queue = Queue::get("test_queue".to_string());

            // List of messages to enqueue
            let messages = vec![
                Message::new("test_message_1".to_string()).expect("Failed to create message"),
                Message::new("test_message_2".to_string()).expect("Failed to create message"),
                Message::new("test_message_3".to_string()).expect("Failed to create message"),
            ];

            // Enqueue messages
            for message in messages {
                queue.enqueue(&message).expect("Failed to enqueue message");
            }

            // Ensure database operations are completed
            let response = consume("test_queue".to_string()).await;
            println!("Response: {:?}", response);
            assert!(!response.message.data.is_empty());
            // Using tokio::spawn
            let consumer1 = tokio::spawn(async  {
                consume("test_queue".to_string()).await
            });

            let consumer2 = tokio::spawn(async {
                consume("test_queue".to_string()).await
            });

            let consumer3 = tokio::spawn(async  {
                consume("test_queue".to_string()).await
            });

            // Make multiple concurrent requests to consume the message
            let (consumer1, consumer2, consumer3) = tokio::join!(consumer1, consumer2, consumer3);

            // Unwrap results first
            let response1 = consumer1.unwrap();
            let response2 = consumer2.unwrap();
            let response3 = consumer3.unwrap();

            // Assert each response is different
            assert!(response1.message.data != response2.message.data);
            assert!(response1.message.data != response3.message.data);
            assert!(response2.message.data != response3.message.data);

            // Ensure the message is not consumed by multiple consumers
            assert!(!response.message.data.is_empty());
        }
    }
}