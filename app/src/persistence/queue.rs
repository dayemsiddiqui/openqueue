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

    pub fn clear(&self) -> bool {
        let prefix = format!("{}:", self.name);
        let db = get_db().lock().expect("Failed to lock database");
        let txn = db.transaction();
        
        // Return true if the queue is empty
        if db.prefix_iterator(&prefix.as_bytes()).next().is_none() {
            return true;
        }

        // Collect keys and delete in transaction
        for result in db.prefix_iterator(&prefix.as_bytes()) {
            let (key, _) = result.expect("Failed to read key");
            txn.delete(&key).expect("Failed to delete key");
        }
        
        match txn.commit() {    
            Ok(_) => true,
            Err(_) => false,
        }
    }   

    pub fn consume(&self) -> Result<Option<Message>, Error> {
        let prefix = format!("{}:", self.name);
        let db = get_db().lock().expect("Failed to lock database");
        
        // Start a transaction
        let mut txn = db.transaction();
        
        // Get the first message in the queue using the main db   
        let mut prefix_iter = db.prefix_iterator(&prefix.as_bytes());
        let result = match prefix_iter.next() {
            Some(Ok((key, message))) => {
                let message = Message::from_bytes(&message).expect("Failed to deserialize message");
                Ok(Some((key, message)))
            }
            Some(Err(e)) => Err(Error::new(e.to_string())),
            None => Ok(None),
        };   

        match result {
            Ok(Some((message_key, message))) => {
                // Delete the message from the queue
                txn.delete(&message_key).expect("Failed to delete message from queue");  

                // Store this message in the "processing" queue
                let processing_key = format!("processing:{}", message.id);
                let message_bytes = message.as_bytes().expect("Failed to serialize message");
                txn.put(processing_key.as_bytes(), &message_bytes).expect("Failed to store message in processing queue");   

                txn.commit().expect("Failed to commit transaction");    
                Ok(Some(message))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn ack(&self, message_id: &str) -> bool {
        let db = get_db().lock().expect("Failed to lock database");
        let txn = db.transaction();
        let processing_key = format!("processing:{}", message_id);
        txn.delete(&processing_key.as_bytes()).expect("Failed to delete message from processing queue");
        txn.commit().expect("Failed to commit transaction");
        true
    }

    pub fn is_processing(&self, message_id: &str) -> bool {
        let db = get_db().lock().expect("Failed to lock database");
        let processing_key = format!("processing:{}", message_id);
        let result = db.get(&processing_key.as_bytes());
        match result {
            Ok(Some(_)) => true,
            _ => false,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_consume_and_ack() {
        let queue = Queue::get("test_queue".to_string());
        let message = Message::new("test_message".to_string()).expect("Failed to create message");
        queue.enqueue(&message).expect("Failed to enqueue message");
        queue.ack(&message.id);
        let result = queue.consume();
        let response = match result {
            Ok(Some(message)) => message,
            Ok(None) => panic!("Expected to receive a message"),
            Err(e) => panic!("Expected to receive a message"),
        }; 
        assert!(response.id == message.id);

        // Assert that the message is no longer in the queue
        let result = queue.consume();
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());

        // Assert that the message is still in the processing queue
        assert!(queue.is_processing(&message.id));

        // Ack the message
        queue.ack(&message.id);

        // Assert that the message is no longer in the processing queue
        assert!(!queue.is_processing(&message.id));
    }
}