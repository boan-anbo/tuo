use async_trait::async_trait;

use tuo_shared::types::return_type::TuoResult;

use crate::core::messaging::message::Message;
use crate::model::model_metadata::EmbeddingModelMetadata;

/// Shared traits for all kinds of models, e.g. embedding, completion, chat, etc.
#[async_trait]
pub trait ModelTrait {
    async fn is_healthy(&self) -> bool;
    
    fn get_model_name(&self) -> String;

    fn get_model_metadata(&self) -> EmbeddingModelMetadata;
}

#[async_trait]
pub trait CompletionModelTrait {
    /// Send a message to the models and receive a response message
    async fn complete(&self, message: Message) -> TuoResult<Message>;
    async fn get_model_name(&self) -> TuoResult<String>;
    async fn get_context_window(&self) -> TuoResult<u32> {
        todo!()
    }
    /// Cost per 1k tokens input, i.e. prompts
    async fn cost_per_1k_tokens_input(&self) -> TuoResult<f32> {
        todo!()
    }
    /// Cost per 1k tokens output, i.e. completions
    async fn cost_per_1k_tokens_output(&self) -> TuoResult<f32> {
        todo!()
    }
}

#[async_trait]
pub trait ChatModelTrait: CompletionModelTrait {
    async fn chat(&self, message: Message) -> TuoResult<String> {
        todo!()
    }
}

