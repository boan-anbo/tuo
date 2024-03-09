use async_openai::config::OpenAIConfig;
use tuo_core::model::model_metadata::{EmbeddingModelMetadata, EmbeddingModelMetadataTrait};
use crate::models::openai::embedder::OpenAIEmbedder;

pub const OPEN_AI_AUTHOR: &str = "OpenAI";
pub const OPEN_AI_EMBEDDING_PAGE: &str =
    "https://platform.openai.com/docs/guides/embeddings/what-are-embeddings";
/// OpenAI Models
///
/// Last updated: 2024-02-29
/// Reference: https://openai.com/pricing && https://help.openai.com/en/articles/7127956-how-much-does-gpt-4-cost
pub enum OpenAIChatModels {
    ChatGpt4_8k,
    ChatGpt4_128k,
}

pub const OPENAI_EMBEDDING_MODEL_NAME_ADA_002: &str = "text-embedding-ada-002";
pub const OPENAI_EMBEDDING_MODEL_NAME_3_SMALL: &str = "text-embedding-3-small";
pub const OPENAI_EMBEDDING_MODEL_NAME_3_LARGE: &str = "text-embedding-3-large";

pub enum OpenAIEmbeddingModels {
    TextEmbedding_3_Small,
    TextEmbedding_3_Large,
    TextEmbedding_Ada_002,
}

impl EmbeddingModelMetadataTrait<OpenAIEmbeddingModels> for OpenAIEmbeddingModels {
    type Embedder = OpenAIEmbedder;
    type Config = OpenAIConfig;

    fn get_embedder(&self, opt: Option<Self::Config>) -> Self::Embedder {
        OpenAIEmbedder::new(self, opt)
    }

    fn get_embedding_model(&self) -> EmbeddingModelMetadata {
        match self {
            OpenAIEmbeddingModels::TextEmbedding_Ada_002 => EmbeddingModelMetadata::builder()
                .name(OPENAI_EMBEDDING_MODEL_NAME_ADA_002.to_string())
                .author(OPEN_AI_AUTHOR.to_string())
                .url(OPEN_AI_EMBEDDING_PAGE.to_string())
                .dimensions(1536)
                .pricing_per_1k_tokens(0.000_10) // 0.02 / 1m tokens
                .max_input(8191)
                .build(),
            OpenAIEmbeddingModels::TextEmbedding_3_Small => EmbeddingModelMetadata::builder()
                .name(OPENAI_EMBEDDING_MODEL_NAME_3_SMALL.to_string())
                .author(OPEN_AI_AUTHOR.to_string())
                .url(OPEN_AI_EMBEDDING_PAGE.to_string())
                .dimensions(1536)
                .pricing_per_1k_tokens(0.000_02) // 0.02 / 1m tokens
                .max_input(8191)
                .build(),
            OpenAIEmbeddingModels::TextEmbedding_3_Large => EmbeddingModelMetadata::builder()
                .name(OPENAI_EMBEDDING_MODEL_NAME_3_LARGE.to_string())
                .author(OPEN_AI_AUTHOR.to_string())
                .url(OPEN_AI_EMBEDDING_PAGE.to_string())
                .dimensions(0)
                .pricing_per_1k_tokens(0.000_13) // 0.02 / 1m tokens
                .max_input(3072)
                .build(),
            _ => todo!(),
        }
    }
    fn from_model_name(name: &str) -> Option<OpenAIEmbeddingModels> {
        match name {
            OPENAI_EMBEDDING_MODEL_NAME_ADA_002 => {
                Some(OpenAIEmbeddingModels::TextEmbedding_Ada_002)
            }
            OPENAI_EMBEDDING_MODEL_NAME_3_SMALL => {
                Some(OpenAIEmbeddingModels::TextEmbedding_3_Small)
            }
            OPENAI_EMBEDDING_MODEL_NAME_3_LARGE => {
                Some(OpenAIEmbeddingModels::TextEmbedding_3_Large)
            }
            _ => None,
        }
    }
    fn get_model_name(&self) -> String {
        match self {
            OpenAIEmbeddingModels::TextEmbedding_Ada_002 => {
                OPENAI_EMBEDDING_MODEL_NAME_ADA_002.to_string()
            }
            OpenAIEmbeddingModels::TextEmbedding_3_Small => {
                OPENAI_EMBEDDING_MODEL_NAME_3_SMALL.to_string()
            }
            OpenAIEmbeddingModels::TextEmbedding_3_Large => {
                OPENAI_EMBEDDING_MODEL_NAME_3_LARGE.to_string()
            }
        }
    }
}
