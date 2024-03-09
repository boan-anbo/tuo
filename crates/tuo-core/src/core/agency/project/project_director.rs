use async_trait::async_trait;
use tuo_shared::types::return_type::TuoResult;

use crate::core::agency::agent::AgentTrait;
use crate::core::agency::profile::ProfileTrait;
use crate::core::messaging::message::Message;
use crate::model::model::CompletionModelTrait;

pub trait ProjectDirectorTrait: AgentTrait {
    fn append_to_final_result(&mut self, content: &str) -> TuoResult<()>;
    fn read_final_result(&self) -> TuoResult<String>;
}

pub struct ProjectDirector {
    final_result: String,

}


#[async_trait]
impl ProfileTrait for ProjectDirector {
    async fn get_profile_prompt(&self) -> TuoResult<String> {
        todo!()
    }
}

#[async_trait]
impl AgentTrait for ProjectDirector {}


#[async_trait]
impl CompletionModelTrait for ProjectDirector {
    async fn complete(&self, message: Message) -> TuoResult<Message> {
        todo!()
    }


    async fn get_model_name(&self) -> TuoResult<String> {
        todo!()
    }
}

impl ProjectDirectorTrait for ProjectDirector {
    fn append_to_final_result(&mut self, content: &str) -> TuoResult<()> {
        self.final_result.push_str(content);
        Ok(())
    }

    fn read_final_result(&self) -> TuoResult<String> {
        Ok(self.final_result.clone())
    }
}