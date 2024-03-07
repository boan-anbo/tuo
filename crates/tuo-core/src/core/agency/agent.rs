use async_trait::async_trait;

use tuo_shared::types::return_type::TuoResult;

use crate::core::agency::profile::ProfileTrait;
use crate::core::messaging::memory::{Memory, MemoryTrait};
use crate::core::messaging::message::Message;
use crate::model::model::CompletionModelTrait;

pub struct Agenda {}

pub struct Accomplishment {}


#[async_trait]
pub trait AgentTrait: Sync + Send + CompletionModelTrait + ProfileTrait {
    /// Get the agent_role_prompt
    fn agent_role_prompt(&self) -> TuoResult<String> {
        Ok("I am an agent. This is a default message.".to_string())
    }


    async fn memory(&self) -> TuoResult<Memory> {
        Ok(Memory::default())
    }

    async fn memorize(&self, message: Message) -> TuoResult<()> {
        let mut memory = self.memory().await?;
        memory.memorize(message)?;
        Ok(())
    }

    async fn name(&self) -> TuoResult<String> {
        Ok("Anonymous".to_string())
    }


    /// Method for outside agents to send a message to this agent and receives a response message
    async fn send(&mut self, message: Message) -> TuoResult<Message> {
        todo!()
    }


    /// Method for this agent to compose and envelope a message to send to another agent
    fn draft(&self, prompt: String) -> TuoResult<Message> {
        todo!()
    }
}
