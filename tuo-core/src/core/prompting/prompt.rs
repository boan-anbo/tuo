use uuid::Uuid;

use crate::types::date_time::TuoDateTime;
use crate::utils::datetime::now;

#[derive(Debug)]
pub enum PromptAuthor {
    User,
    Model(String),
}

#[derive(Debug)]
pub enum PromptType {
    System,
    Query,
}

#[derive(Debug)]
pub struct Prompt {
    pub id: Uuid,
    pub content: String,
    pub author: PromptAuthor,
    pub prompt: PromptType,
    pub created_at: TuoDateTime,
}
 
impl Prompt {
    pub fn new(content: String, author: PromptAuthor, prompt: PromptType) -> Self {
        Self {
            id: Uuid::new_v4(),
            content,
            author,
            prompt,
            created_at: now(),
        }
    }

    pub fn new_system_prompt_by_user(content: String) -> Self {
        Self::new(content, PromptAuthor::User, PromptType::System)
    }

    pub fn new_query_prompt_by_user(content: String) -> Self {
        Self::new(content, PromptAuthor::User, PromptType::Query)
    }

    pub fn new_system_prompt_by_model(content: String, model_name: String) -> Self {
        Self::new(content, PromptAuthor::Model(model_name), PromptType::System)
    }

    pub fn new_query_prompt_by_model(content: String, model_name: String) -> Self {
        Self::new(content, PromptAuthor::Model(model_name), PromptType::Query)
    }
}