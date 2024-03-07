use async_openai::error::OpenAIError;
use thiserror::Error;
use tracing::error;
use crate::errors::parts::TuoPartsError;

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

impl From<OpenAIError> for TuoPartsError {
    fn from(error: OpenAIError) -> Self {
        let mapped = map_openai_api_error(&error);
        TuoPartsError::ApiError(mapped.to_string())
    }
}
