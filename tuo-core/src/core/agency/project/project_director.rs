use async_trait::async_trait;

use crate::core::agency::agent::AgentTrait;
use crate::core::agency::profile::ProfileTrait;
use crate::error::TuoError;
use crate::model::model::ModelTrait;

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


impl ModelTrait for ProjectDirector {}

impl ProjectDirectorTrait for ProjectDirector {
    fn append_to_final_result(&mut self, content: &str) -> Result<(), TuoError> {
        self.final_result.push_str(content);
        Ok(())
    }

    fn read_final_result(&self) -> Result<String, TuoError> {
        Ok(self.final_result.clone())
    }
}