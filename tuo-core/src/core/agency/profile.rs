use std::collections::HashMap;
use async_trait::async_trait;
use crate::core::prompting::prompt::Prompt;
use crate::error::TuoError;
use crate::query::drafter::PromptDrafter;

#[async_trait]
pub trait ProfileTrait {
    async fn get_profile_prompt(&self) -> Result<String, TuoError>;
}

pub struct Profile {
    var_map: HashMap<String, String>,
    drafter: Box<dyn PromptDrafter>,
    output_prompt: Prompt,
}

#[async_trait]
impl ProfileTrait for Profile {
    async fn get_profile_prompt(&self) -> Result<String, TuoError> {
        todo!()
    }
}
