use async_trait::async_trait;

use crate::core::messaging::message::Message;
use crate::error::TuoError;

#[async_trait]
pub trait CompletionModelTrait {
    /// Send a message to the models and receive a response message
    async fn complete(&self, message: Message) -> Result<Message, TuoError>;
    async fn is_healthy(&self) -> Result<bool, TuoError>;
    async fn get_model_name(&self) -> Result<String, TuoError>;
    async fn get_context_window(&self) -> Result<u32, TuoError> {
        todo!()
    }
    /// Cost per 1k tokens input, i.e. prompts
    async fn cost_per_1k_tokens_input(&self) -> Result<f32, TuoError> {
        todo!()
    }
    /// Cost per 1k tokens output, i.e. completions
    async fn cost_per_1k_tokens_output(&self) -> Result<f32, TuoError> {
        todo!()
    }
    
}

#[async_trait]
pub trait ChatModelTrait: CompletionModelTrait {
    async fn chat(&self, message: Message) -> Result<String, TuoError> {
        todo!()
    }
}
