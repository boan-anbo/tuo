use async_openai::types::{ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequest, CreateChatCompletionRequestArgs, CreateChatCompletionResponse};
use async_openai::Client;
use async_openai::config::OpenAIConfig;
use async_trait::async_trait;
use tuo_core::core::messaging::message::{Message, MessageRole};
use tuo_core::model::model::{CompletionModelTrait, ModelTrait};
use tuo_core::model::model_metadata::EmbeddingModelMetadata;
use tuo_shared::errors::core::TuoCoreError;
use tuo_shared::errors::parts::TuoPartsError;
use tuo_shared::types::return_type::TuoResult;
use crate::messaging::message_util_traits::{ConvertMessageTo, ConverttoMessage, IntoMessageExt, MessageExt};
use crate::models::openai::models::{OPEN_AI_AUTHOR, OpenAIChatModels};

pub struct ChatModel {
    model_name: String,
    model_author: String,
    client: Client<OpenAIConfig>,
    context_window: u32,
    cost_per_1k_tokens_input: f32,
    cost_per_1k_tokens_output: f32,
    reference_page: String,
}

#[async_trait]
impl ModelTrait for ChatModel {
    async fn is_healthy(&self) -> bool {
        todo!()
    }

    fn get_model_name(&self) -> String {
        self.model_name.clone()
    }

    fn get_model_metadata(&self) -> EmbeddingModelMetadata {
        EmbeddingModelMetadata::builder()
            .name(self.model_name.clone())
            .author(self.model_author.clone())
            .url(self.reference_page.clone())
            .dimensions(0)
            .pricing_per_1k_tokens(0.0)
            .max_input(0)
            .build()
    }
}

impl ChatModel {
    pub fn new(
        name: String,
        reference: &str,
        opt: Option<OpenAIConfig>,
        api_key: Option<String>,
        context_window: u32,
        cost_per_1k_tokens_input: f32,
        cost_per_1k_tokens_output: f32,
    ) -> ChatModel {
        let mut config = opt.unwrap_or(OpenAIConfig::default());
        if api_key.is_some() {
            config = config.with_api_key(api_key.unwrap());
        }
        let client = Client::with_config(config);
        ChatModel {
            model_author: OPEN_AI_AUTHOR.to_string(),
            model_name: name,
            client,
            context_window,
            cost_per_1k_tokens_input,
            cost_per_1k_tokens_output,
            reference_page: reference.to_string(),
        }
    }
}

impl ConvertMessageTo<CreateChatCompletionRequest> for Message {
    fn convert_to(message: &Message, model_name: &str) -> TuoResult<CreateChatCompletionRequest> {
        let request = CreateChatCompletionRequestArgs::default()
            .model(model_name.to_string())
            .messages([ChatCompletionRequestUserMessageArgs::default()
                .content(message.content.clone())
                .build()
                .map_err(|e| TuoPartsError::from(e))?
                .into()])
            .build()
            .unwrap();
        Ok(request)
    }
}

impl ConverttoMessage<CreateChatCompletionResponse> for Message {
    fn convert_to_message(response: &CreateChatCompletionResponse) -> TuoResult<Message> {
        let mut message = Message::default();
        let choice = response.choices[0].clone();
        message.content = choice.message.content.unwrap();
        message.role = MessageRole::AI;
        Ok(message)
    }
}

#[async_trait]
impl CompletionModelTrait for ChatModel {
    async fn complete(&self, message: Message) -> TuoResult<Message> {
        let request: CreateChatCompletionRequest = message.to_model_request(&self.model_name)?;
        let result = self
            .client
            .chat()
            .create(request)
            .await
            .map_err(|e| TuoCoreError::ModelError(e.to_string()))?;
        let message = result.to_message()?;
        Ok(message)
    }

    async fn get_model_name(&self) -> TuoResult<String> {
        Ok(self.model_name.clone())
    }

    async fn get_context_window(&self) -> TuoResult<u32> {
        Ok(self.context_window)
    }

    async fn cost_per_1k_tokens_input(&self) -> TuoResult<f32> {
        Ok(self.cost_per_1k_tokens_input)
    }

    async fn cost_per_1k_tokens_output(&self) -> TuoResult<f32> {
        Ok(self.cost_per_1k_tokens_output)
    }
}

impl OpenAIChatModels {
    pub fn get_model(&self, opt: Option<OpenAIConfig>, api_key: Option<String>) -> ChatModel {
        let reference_page = "https://openai.com/pricing";
        match self {
            OpenAIChatModels::ChatGpt4_128k => ChatModel::new(
                "gpt-4-turbo-preview".to_string(),
                reference_page,
                opt,
                api_key,
                128_000,
                0.03,
                0.06,
            ),
            OpenAIChatModels::ChatGpt4_8k => ChatModel::new(
                "gpt-4".to_string(),
                reference_page,
                opt,
                api_key,
                8_000,
                0.03,
                0.06,
            ),
        }
    }
}
