use async_trait::async_trait;

use tuo_shared::types::return_type::TuoResult;

use crate::core::messaging::content::{TextEmbeddingOptions, TextEmbedded, TextInput};
use crate::core::source::node::Node;
use crate::embedding::embeddings::Embeddings;
use crate::model::model_metadata::EmbeddingModelMetadata;



#[derive(Debug)]
pub struct EmbedResultStats {}

#[async_trait]
pub trait EmbedderTrait: Sync + Send {
    fn get_embedding_model(&self) -> EmbeddingModelMetadata;
    fn get_embedding_model_name(&self) -> String {
        self.get_embedding_model().name.clone()
    }
    async fn embed_string(&self, text: &str) -> TuoResult<Embeddings>;
    async fn embed_input(&self, input: &TextInput, opt: &TextEmbeddingOptions) -> TuoResult<TextEmbedded>;
    async fn embed_nodes(&self, nodes: Vec<Node>, opt: &TextEmbeddingOptions) -> TuoResult<Vec<Node>>;
}