use uuid::Uuid;
use crate::core::generation::generated_content::GeneratedContent;

#[derive(Default)]
pub struct Message {
    pub id: Uuid,
    pub sender: String,
    pub receiver: String,
    pub content: GeneratedContent,
}
