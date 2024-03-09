use async_openai::Client;
use async_openai::config::OpenAIConfig;
use async_trait::async_trait;
use tracing::debug;
use async_openai::types::CreateEmbeddingRequestArgs;
use tuo_core::core::messaging::content::{TextEmbedded, TextEmbeddingOptions, TextInput};
use tuo_core::core::source::node::{Node, NodeRelationTrait};
use tuo_core::embedding::embedder::EmbedderTrait;
use tuo_core::embedding::embeddings::Embeddings;
use tuo_core::model::model::ModelTrait;
use tuo_core::model::model_metadata::{EmbeddingModelMetadata, EmbeddingModelMetadataTrait};
use tuo_shared::errors::core::TuoCoreError;
use tuo_shared::types::return_type::TuoResult;
use crate::models::openai::models::OpenAIEmbeddingModels;

pub struct OpenAIEmbedder {
    metadata: EmbeddingModelMetadata,
    client: Client<OpenAIConfig>,
}

#[async_trait]
impl ModelTrait for OpenAIEmbedder {
    async fn is_healthy(&self) -> bool {
        let result = self
            .client
            .models()
            .retrieve(self.get_model_name().as_str())
            .await;
        match result {
            Ok(_) => true,
            Err(e) => false,
        }
    }

    fn get_model_name(&self) -> String {
        self.metadata.name.clone()
    }

    fn get_model_metadata(&self) -> EmbeddingModelMetadata {
        self.metadata.clone()
    }
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
    async fn embed_string(&self, text: &str) -> TuoResult<Embeddings> {
        debug!("Embedding text: {}", text);
        let request = CreateEmbeddingRequestArgs::default()
            .model(self.get_model_name().clone())
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
            .model(self.get_model_name())
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
