use async_openai::Client;
use async_openai::config::{OpenAIConfig};
use async_openai::error::OpenAIError;
use async_openai::types::{ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequest, CreateChatCompletionRequestArgs, CreateChatCompletionResponse};
use async_trait::async_trait;

use thiserror::Error;
use tracing::error;

use tuo_core::core::messaging::message::{Message, MessageRole};
use tuo_core::error::TuoError;
use tuo_core::model::model::CompletionModelTrait;
use tuo_core::utils::datetime::{utc_from_epoch};

use crate::messaging::message_util_traits::{ConvertMessageTo, ConverttoMessage, IntoMessageExt, MessageExt};

#[derive(Debug, Error)]
pub enum OpenAIApiError {
    #[error("Invalid Authentication")]
    InvalidAuthentication,
    #[error("Invalid Request Error")]
    InvalidRequestError,
    #[error("Incorrect API Key")]
    IncorrectAPIKey,
    #[error("Not a Member of Organization")]
    NotMemberOfOrganization,
    #[error("Rate Limit Reached")]
    RateLimitReached,
    #[error("Insufficient Quota")]
    InsufficientQuota,
    #[error("Engine Overloaded")]
    EngineOverloaded,
    #[error("Server Error")]
    ServerError,
    #[error("Billing Not Active")]
    BillingNotActive,
    #[error("Unknown Error {0}")]
    Unknown(String),
    #[error("Invalid Model")]
    InvalidModel,
}

pub fn map_openai_api_error(api_error: &OpenAIError) -> OpenAIApiError {
    let mapped_open_ai_error = match api_error {
        OpenAIError::ApiError(ref api_error_ref) => {
            match api_error_ref.r#type.clone().unwrap().as_str() {
                "invalid_authentication" => OpenAIApiError::InvalidAuthentication,
                "invalid_request_error" => {
                    match api_error_ref.code {
                        Some(ref code) => {
                            match code.as_str() {
                                Some("invalid_api_key") => {
                                    OpenAIApiError::IncorrectAPIKey
                                }
                                Some("model_not_found") => {
                                    OpenAIApiError::InvalidModel
                                }
                                _ => {
                                    OpenAIApiError::InvalidRequestError
                                }
                            }
                        }
                        None => {
                            OpenAIApiError::InvalidRequestError
                        }
                    }
                }
                "not_member_of_organization" => OpenAIApiError::NotMemberOfOrganization,
                "rate_limit_reached" => OpenAIApiError::RateLimitReached,
                "billing_not_active" => OpenAIApiError::BillingNotActive,
                "insufficient_quota" => OpenAIApiError::InsufficientQuota,
                "engine_overloaded" => OpenAIApiError::EngineOverloaded,
                "server_error" => OpenAIApiError::ServerError,
                _ => OpenAIApiError::Unknown(api_error.to_string())
            }
        }
        _ => OpenAIApiError::Unknown(api_error.to_string())
    };
    error!("OpenAI Api Error: {:?}", mapped_open_ai_error);
    mapped_open_ai_error
}

/// OpenAI Models
///
/// Last updated: 2024-02-29
/// Reference: https://openai.com/pricing && https://help.openai.com/en/articles/7127956-how-much-does-gpt-4-cost
pub enum OpenAIChatModels {
    ChatGpt4_8k,
    ChatGpt4_128k,
}

impl OpenAIChatModels {
    pub fn get_model(&self, opt: Option<
        OpenAIConfig
    >, api_key: Option<String
    >) -> ChatGpt {
        let reference_page = "https://openai.com/pricing";
        match self {
            OpenAIChatModels::ChatGpt4_128k => {
                ChatGpt::new("gpt-4-turbo-preview".to_string(), reference_page, opt, api_key, 128_000, 0.03, 0.06)
            }
            OpenAIChatModels::ChatGpt4_8k => {
                ChatGpt::new("gpt-4".to_string(), reference_page, opt, api_key, 8_000, 0.03, 0.06)
            }
        }
    }
}

pub struct ChatGpt {
    model_name: String,
    client: Client<OpenAIConfig>,
    context_window: u32,
    cost_per_1k_tokens_input: f32,
    cost_per_1k_tokens_output: f32,
    reference_page: String,
}


impl ChatGpt {
    pub fn new(name: String, reference: &str, opt: Option<OpenAIConfig>, api_key: Option<String>, context_window: u32, cost_per_1k_tokens_input: f32, cost_per_1k_tokens_output: f32) -> ChatGpt {
        let mut config = opt.unwrap_or(OpenAIConfig::default());
        if api_key.is_some() {
            config = config.with_api_key(api_key.unwrap());
        }
        let client = Client::with_config(
            config
        );
        ChatGpt { model_name: name, client, context_window, cost_per_1k_tokens_input, cost_per_1k_tokens_output, reference_page: reference.to_string() }
    }
}

impl ConvertMessageTo<CreateChatCompletionRequest> for Message {
    fn convert_to(message: &Message, model_name: &str) -> Result<CreateChatCompletionRequest,TuoError> {
        let request = CreateChatCompletionRequestArgs::default()
            .model(model_name.to_string())
            .messages([
                ChatCompletionRequestUserMessageArgs::default()
                    .content(message.content.text.clone())
                    .build()
                    .map_err(|e| TuoError::ModelError(e.to_string()))?
                    .into()
            ])
            .build().unwrap();
        Ok(request)
    }
}

impl ConverttoMessage<CreateChatCompletionResponse> for Message {
    fn convert_to_message(response: &CreateChatCompletionResponse) -> Result<Message, TuoError> {
        let mut message = Message::default();
        let choice = response.choices[0].clone();
        let model_name = response.model.clone();
        message.content.model = Some(model_name);
        message.content.is_generated = true;
        message.content.text = choice.message.content.unwrap_or("".to_string());
        message.content.created_at = utc_from_epoch(response.created as i64)?;
        message.role = MessageRole::AI;
        Ok(message)
    }
}

#[async_trait]
impl CompletionModelTrait for ChatGpt {
    async fn complete(&self, message: Message) -> Result<Message, TuoError> {
        let request: CreateChatCompletionRequest = message.to_model_request(&self.model_name)?;
        let result = self.client.chat().create(request).await.map_err(|e| TuoError::ModelError(e.to_string()))?;
        let message = result.to_message()?;
        Ok(message)
    }

    async fn is_healthy(&self) -> Result<bool, TuoError> {
        let result = self.client.models().retrieve(self.model_name.as_str()).await;
        match result {
            Ok(_) => Ok(true),
            Err(e) => {
                let openai_api_error = map_openai_api_error(&e);
                match openai_api_error {
                    OpenAIApiError::InvalidModel => Ok(false),
                    _ => Err(TuoError::ModelError(openai_api_error.to_string()))
                }
            }
        }
    }

    async fn get_model_name(&self) -> Result<String, TuoError> {
        Ok(self.model_name.clone())
    }

    async fn get_context_window(&self) -> Result<u32, TuoError> {
        Ok(self.context_window)
    }

    async fn cost_per_1k_tokens_input(&self) -> Result<f32, TuoError> {
        Ok(self.cost_per_1k_tokens_input)
    }

    async fn cost_per_1k_tokens_output(&self) -> Result<f32, TuoError> {
        Ok(self.cost_per_1k_tokens_output)
    }
}

#[cfg(test)]
mod test {
    use async_openai::Client;
    use dotenv::dotenv;
    use test_log::test;

    use tuo_core::core::messaging::message::Message;
    use tuo_core::model::model::CompletionModelTrait;

    use crate::models::open_ai::OpenAIChatModels;

    #[test(tokio::test)]
    async fn can_check_health() {
        dotenv().ok();
        let wrong_model = OpenAIChatModels::ChatGpt4_128k.get_model(None, None);
        let result = wrong_model.is_healthy().await.unwrap();
        assert!(result);
    }

    #[test(tokio::test)]
    async fn list_models() {
        dotenv().ok();
        let client = Client::new();
        let result = client.models().list().await.unwrap();
        for model in result.data {
            println!("{:?}", model);
        }
    }

    #[test(tokio::test)]
    async fn basic_completion() {
        dotenv().ok();
        let model = OpenAIChatModels::ChatGpt4_128k.get_model(None, None);
        let message_text = r#"Hello, please respond with only the word "READY""#.to_string();
        let message = Message::draft(message_text);
        let response_message = model.complete(message).await.unwrap();
        assert_eq!(response_message.content.text, "READY");
    }
}