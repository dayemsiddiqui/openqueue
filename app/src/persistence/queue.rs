use crate::persistence::message::Message;
use crate::persistence::error::Error;
use crate::clients::database::{insert_data, get_db};

pub struct Queue {
    name: String,
}   

impl Queue {
    pub fn get(name: String) -> Self {
        Queue { name }
    }

    pub fn enqueue(&self, message: &Message) -> Result<String, Error> {
        let message_key = Message::build_message_key(self.name.clone(), message);
        let value = serde_json::to_string(message).expect("Failed to serialize message");    
        insert_data(&message_key, &value).expect("Failed to add message to queue");

        Ok(message_key)
    }

    pub fn consume(&self) -> Result<Message, Error> {
        let prefix = format!("{}:", self.name);
        let db = get_db().lock().expect("Failed to lock database");

        // Start a transaction
        let txn = db.transaction();
        
        // Get the first message in the queue   
        let mut prefix_iter = db.prefix_iterator(&prefix.as_bytes());
        let result = match prefix_iter.next() {
            Some(Ok((key, message))) => {
                let message = Message::from_bytes(&message).expect("Failed to deserialize message");
                Ok((key, message))
            }
            Some(Err(e)) => Err(Error::new(e.to_string())),
            None => Err(Error::new("No messages in queue".to_string())),
        };   

        let (message_key, message) = result.map_err(|e| Error::new(e.to_string()))?;
        // Delete the message from the queue
        txn.delete(&message_key).expect("Failed to delete message from queue");  

        // Store this message in the "processing" queue
        let processing_key = format!("processing:{}", String::from_utf8_lossy(&message_key));
        let message_bytes = message.as_bytes().expect("Failed to serialize message");
        txn.put(processing_key.as_bytes(), message_bytes).expect("Failed to store message in processing queue");   

        txn.commit().expect("Failed to commit transaction");    
        Ok(message)
    }
}   