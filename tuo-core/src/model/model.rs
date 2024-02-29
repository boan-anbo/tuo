use async_trait::async_trait;

use crate::core::messaging::message::Message;
use crate::error::TuoError;

#[async_trait]
pub trait ModelTrait {
    /// Send a message to the model and receive a response message
    async fn send(&self, message: Message) -> Result<Message, TuoError> {
        todo!()
    }
    async fn is_healthy(&self) -> Result<bool, TuoError> {
        Ok(true)
    }
    async fn get_model_name(&self) -> Result<String, TuoError> {
        Ok("Anonymous".to_string())
    }
    
}
