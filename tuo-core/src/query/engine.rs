use async_trait::async_trait;
use crate::core::indexing::index::Index;
use crate::error::TuoError;

pub struct QueryContext {}
pub struct QueryResult {}
#[derive(Default)]
pub struct QueryRequest {

}

pub struct ChatRequest {}
pub struct ChatResult {}

#[async_trait]
pub trait QueryEngineTrait {
    async fn load_index(&self, index: Index) -> Result<Box<dyn QueryEngineTrait>, TuoError>;
    async fn query(&self, request: QueryRequest) -> Result<QueryResult, TuoError>;
}


#[async_trait]
pub trait ChatEngineTrait {
    async fn chat(&self, request: ChatRequest) -> Result<ChatResult, TuoError>;
}

/// A helper trait to create a query engine from a folder
/// 
/// This invokes the Folder Trait
#[async_trait]
pub trait QueryEngineFromFolderTrait {
    async fn from_folder(folder: &str) -> Result<Box<dyn QueryEngineTrait>, TuoError>;
}