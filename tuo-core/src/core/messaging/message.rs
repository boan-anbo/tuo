use uuid::Uuid;
use crate::core::generation::generated_content::Content;
use crate::utils::datetime::now;

#[derive(Default, Debug)]
pub enum MessageRole {
    #[default]
    USER,
    // The AI
    AI,
    SYSTEM,
}
#[derive(Default, Debug)]
pub struct Message {
    pub id: Uuid,
    // The name of the receiver agent
    pub receiver: Option<String>,
    // The name of the sender agent
    pub sender: Option<String>,
    pub role: MessageRole,
    pub content: Content,
}

impl Message {
    pub fn draft(text: String) -> Self {
        let content = Content {
            text,
            is_generated: false,
            created_at: now(),
            ..Default::default()
        };
        Self {
            id: Uuid::new_v4(),
            receiver: None,
            sender: None,
            role: MessageRole::USER,
            content,
        }
    }
}