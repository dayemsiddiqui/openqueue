use crate::persistence::message::Message;
use crate::persistence::error::Error;
use crate::clients::database::insert_data;

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
}   