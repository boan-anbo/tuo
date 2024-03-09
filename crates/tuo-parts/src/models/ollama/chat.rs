use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::generation::completion::GenerationResponse;
use tuo_core::core::messaging::message::{Message, MessageRole};
use tuo_shared::types::return_type::TuoResult;

use crate::messaging::message_util_traits::{
    ConvertMessageTo, ConverttoMessage,
};

impl ConvertMessageTo<GenerationRequest> for Message {
    fn convert_to(message: &Message, model_name: &str) -> TuoResult<GenerationRequest> {
        let request = GenerationRequest::new(model_name.to_string(), message.content.clone());
        Ok(request)
    }
}

impl ConverttoMessage<GenerationResponse> for Message {
    fn convert_to_message(response: &GenerationResponse) -> TuoResult<Message> {
        let mut message = Message::default();
        message.content = response.response.clone();
        message.role = MessageRole::AI;
        Ok(message)
    }
}
