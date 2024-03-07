use async_trait::async_trait;

use tuo_shared::types::return_type::TuoResult;

use crate::core::messaging::message::Message;

#[async_trait]
pub trait CompletionModelTrait {
    /// Send a message to the models and receive a response message
    async fn complete(&self, message: Message) -> TuoResult<Message>;
    async fn is_healthy(&self) -> TuoResult<bool>;
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
