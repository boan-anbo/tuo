use async_trait::async_trait;

use tuo_shared::types::return_type::TuoResult;
use crate::core::indexing::index::IndexTrait;


pub struct QueryContext {}
pub struct QueryResult {}
#[derive(Default)]
pub struct QueryRequest {

}

pub struct ChatRequest {}
pub struct ChatResult {}

#[async_trait]
pub trait QueryEngineTrait {
    async fn query(&self, request: QueryRequest) -> TuoResult<QueryResult>;
}


#[async_trait]
pub trait ChatEngineTrait {
    async fn chat(&self, request: ChatRequest) -> TuoResult<ChatResult>;
}

/// A helper trait to create a query engine from a folder
/// 
/// This invokes the Folder Trait
#[async_trait]
pub trait QueryEngineFromFolderTrait {
    async fn from_folder(folder: &str) -> TuoResult<Box<dyn QueryEngineTrait>>;
}