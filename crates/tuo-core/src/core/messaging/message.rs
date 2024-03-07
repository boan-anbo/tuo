use crate::types::date_time::TuoDateTime;
use tuo_utils::datetime::timestamp::now;
use uuid::Uuid;

#[derive(Default, Debug)]
pub enum MessageRole {
    #[default]
    USER,
    // The AI
    AI,
    SYSTEM,
}

#[derive(Debug)]
pub enum MessageAuthor {
    USER(String),
    Model(String),
}

impl Default for MessageAuthor {
    fn default() -> Self {
        MessageAuthor::USER("Anonymous".to_string())
    }
}

#[derive(Debug, Default)]
pub struct Message {
    pub id: Uuid,
    // The name of the receiver agent
    pub receiver: Option<String>,
    // The name of the sender agent
    pub sender: Option<String>,
    pub role: MessageRole,
    pub author: MessageAuthor,
    pub content: String,
    pub created_at: TuoDateTime,
}

impl Message {
    pub fn draft(text: String, author: Option<MessageAuthor>) -> Self {
        Self {
            id: Uuid::new_v4(),
            receiver: None,
            sender: None,
            role: MessageRole::USER,
            author: author.unwrap_or_default(),
            content: text,
            created_at: now(),
        }
    }
}
