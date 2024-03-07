use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequest,
    CreateChatCompletionRequestArgs, CreateChatCompletionResponse, CreateEmbeddingRequestArgs,
    Model,
};
use async_openai::Client;
use async_trait::async_trait;
use tracing::{debug, info};
use tuo_core::core::messaging::content::{TextEmbeddingOptions, TextEmbedded, TextInput};

use tuo_core::core::messaging::message::{Message, MessageRole};
use tuo_core::core::source::node::{Node, NodeRelationTrait};
use tuo_core::embedding::embedder::EmbedderTrait;
use tuo_core::embedding::embeddings::Embeddings;
use tuo_core::model::model::CompletionModelTrait;
use tuo_core::model::model_metadata::{EmbeddingModelMetadata, EmbeddingModelMetadataTrait};
use tuo_shared::errors::core::TuoCoreError;
use tuo_shared::errors::openai::open_ai::{map_openai_api_error, OpenAIApiError};
use tuo_shared::errors::parts::TuoPartsError;
use tuo_shared::types::return_type::TuoResult;
use tuo_utils::datetime::timestamp::utc_from_epoch;

use crate::messaging::message_util_traits::{
    ConvertMessageTo, ConverttoMessage, IntoMessageExt, MessageExt,
};

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

pub struct OpenAIEmbedder {
    metadata: EmbeddingModelMetadata,
    client: Client<OpenAIConfig>,
}

impl OpenAIEmbedder {
    pub fn new(model: &OpenAIEmbeddingModels, config: Option<OpenAIConfig>) -> OpenAIEmbedder {
        let client = Client::with_config(config.unwrap_or_default());
        let metadata = model.get_embedding_model();
        OpenAIEmbedder { metadata, client }
    }
}

#[async_trait]
impl EmbedderTrait for OpenAIEmbedder {
    fn get_embedding_model(&self) -> EmbeddingModelMetadata {
        self.metadata.clone()
    }

    fn get_embedding_model_name(&self) -> String {
        self.metadata.name.clone()
    }

    async fn embed_string(&self, text: &str) -> TuoResult<Embeddings> {
        debug!("Embedding text: {}", text);
        let request = CreateEmbeddingRequestArgs::default()
            .model(self.get_embedding_model_name().clone())
            .input([text.clone()])
            .build()
            .unwrap();
        let result = self
            .client
            .embeddings()
            .create(request)
            .await
            .map_err(|e| TuoCoreError::ModelError(e.to_string()))?;
        let embeddings = result.data.first().unwrap().embedding.clone();
        debug!("Embeddings: {:?}", embeddings);
        Ok(Embeddings::builder()
            .model(self.get_embedding_model_name())
            .vector(embeddings)
            .build())
    }

    async fn embed_input(
        &self,
        input: &TextInput,
        opt: &TextEmbeddingOptions,
    ) -> TuoResult<TextEmbedded> {
        let embeddings = self.embed_string(input.text.as_str()).await?;
        let result = input.to_embedded(embeddings, opt);
        Ok(result)
    }

    
    /// Embeds a list of nodes
    /// 
    /// Embed result is merged into nodes via [merge_embedded_text](Node::merge_embedded_text)
    async fn embed_nodes(
        &self,
        nodes: Vec<Node>,
        opt: &TextEmbeddingOptions,
    ) -> TuoResult<Vec<Node>> {
        let mut result: Vec<Node> = Vec::new();
        for mut node in nodes.into_iter() {
            let input = TextInput::from_node_text(node.content.as_str(), node.id);
            let text_embedded = self.embed_input(&input, &opt).await?;
            node.merge_embedded_text(&text_embedded);
            result.push(node)
        }
        Ok(result)
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

pub struct ChatModel {
    model_name: String,
    model_author: String,
    client: Client<OpenAIConfig>,
    context_window: u32,
    cost_per_1k_tokens_input: f32,
    cost_per_1k_tokens_output: f32,
    reference_page: String,
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

    async fn is_healthy(&self) -> TuoResult<bool> {
        let result = self
            .client
            .models()
            .retrieve(self.model_name.as_str())
            .await;
        match result {
            Ok(_) => Ok(true),
            Err(e) => {
                let openai_api_error = map_openai_api_error(&e);
                match openai_api_error {
                    OpenAIApiError::InvalidModel => Ok(false),
                    _ => Err(TuoCoreError::ModelError(openai_api_error.to_string()))?,
                }
            }
        }
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

    #[ignore]
    #[test(tokio::test)]
    async fn basic_completion() {
        dotenv().ok();
        let model = OpenAIChatModels::ChatGpt4_128k.get_model(None, None);
        let message_text = r#"Hello, please respond with only the word "READY""#.to_string();
        let message = Message::draft(message_text, None);
        let response_message = model.complete(message).await.unwrap();
        assert_eq!(response_message.content, "READY");
    }
}
