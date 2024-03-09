use async_trait::async_trait;

use tuo_shared::types::return_type::TuoResult;

use crate::core::messaging::content::{TextEmbedded, TextEmbeddingOptions, TextInput};
use crate::core::source::node::{Node, NodeRelationTrait};
use crate::embedding::embeddings::Embeddings;
use crate::model::model::ModelTrait;
use crate::model::model_metadata::EmbeddingModelMetadata;

#[derive(Debug)]
pub struct EmbedResultStats {}

#[async_trait]
pub trait EmbedderTrait: Sync + Send + ModelTrait {
    async fn embed_string(&self, text: &str) -> TuoResult<Embeddings>;
    async fn embed_input(
        &self,
        input: &TextInput,
        opt: &TextEmbeddingOptions,
    ) -> TuoResult<TextEmbedded> {
        let embeddings = self.embed_string(input.text.as_str()).await?;
        let result = input.to_embedded(embeddings, opt);
        Ok(result)
    }
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
