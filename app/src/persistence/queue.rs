
pub struct Topic {
    id: String,
    name: String,
    description: String,
}

pub struct Message {
    id: String,
    topic_id: String,
    data: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    // It can be null if the message is not locked
    locked_at: Option<DateTime<Utc>>,
}

/**
 * Enqueue a message into a topic
 * This method will create a new message and add it to the topic
 * This method gets called when a publisher publishes a message to a topic
 */
pub fn enqueue(topic: &Topic, message: &Message) -> Result<(), Error> {
    let db = get_db();
}   

/**
 * Pop a message from a topic
 * This method will return the oldest message in the topic
 * This method gets called when a consumer wants to consume a message from a topic
 */
pub fn pop(topic: &Topic) -> Result<Option<Message>, Error> {
    let db = get_db();
}

/**
 * Dequeue a message from a topic
 * This method will remove the message from the topic
 * This method gets called when a consumer acknowledges a message has been processed
 */
pub fn dequeue(topic: &Topic, messageId: &str) -> Result<(), Error> {
    let db = get_db();
}