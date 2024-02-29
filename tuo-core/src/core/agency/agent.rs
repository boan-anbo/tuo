use async_trait::async_trait;
use crate::core::agency::profile::ProfileTrait;

use crate::core::messaging::memory::{Memory, MemoryTrait};
use crate::core::messaging::message::Message;
use crate::error::TuoError;
use crate::model::model::CompletionModelTrait;

pub struct Agenda {}

pub struct Accomplishment {}


#[async_trait]
pub trait AgentTrait: Sync + Send + CompletionModelTrait + ProfileTrait {
    /// Get the agent_role_prompt
    fn agent_role_prompt(&self) -> Result<String, TuoError> {
        Ok("I am an agent. This is a default message.".to_string())
    }


    async fn memory(&self) -> Result<Memory, TuoError> {
        Ok(Memory::default())
    }

    async fn memorize(&self, message: Message) -> Result<(), TuoError> {
        let mut memory = self.memory().await?;
        memory.memorize(message)?;
        Ok(())
    }

    async fn name(&self) -> Result<String, TuoError> {
        Ok("Anonymous".to_string())
    }


    /// Method for outside agents to send a message to this agent and receives a response message
    async fn send(&mut self, message: Message) -> Result<Message, TuoError> {
        todo!()
    }


    /// Method for this agent to compose and envelope a message to send to another agent
    fn draft(&self, prompt: String) -> Result<Message, TuoError> {
        todo!()
    }
}
