use serde::{Serialize, Deserialize};
use std::error::Error as StdError;
use chrono::{DateTime, Utc};
type Error = Box<dyn StdError>;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: String,
    data: String,
    visible_after: Option<DateTime<Utc>>, // None means the message is visible immediately
    receive_count: u32,
    enqueued_at: DateTime<Utc>,
}

impl Message {
    pub fn new(data: String) -> Result<Message, Error> {
        let id = uuid::Uuid::new_v4().to_string();
        let enqueued_at = Utc::now();
        Ok(Message { id, data, enqueued_at, visible_after: None, receive_count: 0 })
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Message, Error> {
        let message = serde_json::from_slice(bytes)?;
        Ok(message)
    }

    pub fn as_bytes(&self) -> Result<Vec<u8>, Error> {
        let json_string = self.to_json_string()?;
        Ok(json_string.into_bytes())
    }

    pub fn to_json_string(&self) -> Result<String, Error> {
        let json_string = serde_json::to_string(self)?;
        Ok(json_string)
    }   

    pub fn build_message_key(queue_name: String, message: &Self) -> String {
        format!("{}:{}:{}", queue_name, message.enqueued_at.timestamp_nanos(), message.id)
    }   

   
}   

/**
 * Pop a message from a queue
 * This method will return the oldest message in the queue
 * This method gets called when a consumer wants to consume a message from a queue
 */
pub fn pop(queue_name: String) -> Result<Option<Message>, Error> {
    Ok(None)
}

/**
 * Dequeue a message from a queue
 * This method will remove the message from the queue
 * This method gets called when a consumer acknowledges a message has been processed
 */
pub fn dequeue(queue_name: &str, message_id: &str) -> Result<(), Error> {
    Ok(())
}