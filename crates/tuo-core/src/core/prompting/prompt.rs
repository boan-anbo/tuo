use uuid::Uuid;

use tuo_utils::datetime::timestamp::now;

use crate::types::date_time::TuoDateTime;

#[derive(Debug, Clone)]
pub enum PromptAuthor {
    User,
    Model(String),
}

#[derive(Debug, Clone)]
pub enum PromptType {
    System,
    Query,
}

#[derive(Debug, Clone)]
pub struct Prompt {
    pub id: Uuid,
    pub text: String,
    pub author: PromptAuthor,
    pub prompt: PromptType,
    pub created_at: TuoDateTime,
    pub used_at: TuoDateTime,
}

impl Prompt {
    pub fn new(text: &str, author: PromptAuthor, prompt: PromptType) -> Self {
        Self {
            id: Uuid::new_v4(),
            text: text.to_string(),
            author,
            prompt,
            created_at: now(),
            used_at: now(),
        }
    }

    pub fn new_system_prompt_by_user(text: String) -> Self {
        Self::new(text.as_str(), PromptAuthor::User, PromptType::System)
    }

    pub fn new_query_prompt_by_user(text: String) -> Self {
        Self::new(text.as_str(), PromptAuthor::User, PromptType::Query)
    }

    pub fn new_system_prompt_by_model(text: String, model_name: String) -> Self {
        Self::new(text.as_str(), PromptAuthor::Model(model_name), PromptType::System)
    }

    pub fn new_query_prompt_by_model(text: String, model_name: String) -> Self {
        Self::new(text.as_str(), PromptAuthor::Model(model_name), PromptType::Query)
    }
}