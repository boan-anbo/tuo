use async_trait::async_trait;

use crate::core::agency::agent::AgentTrait;
use crate::core::agency::profile::ProfileTrait;
use crate::core::messaging::message::Message;
use crate::error::TuoError;
use crate::model::model::CompletionModelTrait;

pub trait ProjectDirectorTrait: AgentTrait {
    fn append_to_final_result(&mut self, content: &str) -> Result<(), TuoError>;
    fn read_final_result(&self) -> Result<String, TuoError>;
}

pub struct ProjectDirector {
    final_result: String,

}


#[async_trait]
impl ProfileTrait for ProjectDirector {
    async fn get_profile_prompt(&self) -> Result<String, TuoError> {
        todo!()
    }
}

#[async_trait]
impl AgentTrait for ProjectDirector {}


#[async_trait]
impl CompletionModelTrait for ProjectDirector {
    async fn complete(&self, message: Message) -> Result<Message, TuoError> {
        todo!()
    }

    async fn is_healthy(&self) -> Result<bool, TuoError> {
        todo!()
    }

    async fn get_model_name(&self) -> Result<String, TuoError> {
        todo!()
    }
}

impl ProjectDirectorTrait for ProjectDirector {
    fn append_to_final_result(&mut self, content: &str) -> Result<(), TuoError> {
        self.final_result.push_str(content);
        Ok(())
    }

    fn read_final_result(&self) -> Result<String, TuoError> {
        Ok(self.final_result.clone())
    }
}