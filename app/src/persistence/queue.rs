use crate::clients::database::{get_db, insert_data};
use serde::{Serialize, Deserialize};
use std::error::Error as StdError;
use chrono::{DateTime, Utc};
type Error = Box<dyn StdError>;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    id: String,
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

    pub fn build_message_key(queue_name: String, message: &Self) -> String {
        format!("{}:{}:{}", queue_name, message.enqueued_at.timestamp_nanos(), message.id)
    }   

    pub fn enqueue(queue_name: String, message: &Self) -> Result<(), Error> {
        let message_key = Self::build_message_key(queue_name, message);
        let value = serde_json::to_string(message)?;    
        insert_data(&message_key, &value)?;
        Ok(())
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