use uuid::Uuid;

pub enum PromptAuthor {
    User,
    Model(String),
}

pub enum PromptType {
    System,
    Query,
}

pub struct Prompt {
    pub id: Uuid,
    pub content: String,
    pub author: PromptAuthor,
    pub prompt: PromptType,
}