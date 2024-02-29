use async_trait::async_trait;
use tuo_core::core::indexing::index::Index;
use tuo_core::error::TuoError;
use tuo_core::query::engine::{QueryEngineFromFolderTrait, QueryEngineTrait, QueryRequest, QueryResult};

#[derive(Default)]
pub struct QueryEngine {
    
}

#[async_trait]
impl QueryEngineTrait for QueryEngine {
    async fn load_index(&self, index: Index) -> Result<Box<dyn QueryEngineTrait>, TuoError> {
        todo!()
    }

    async fn query(&self, request: QueryRequest) -> Result<QueryResult, TuoError> {
        todo!()
    }
}

#[async_trait]
impl QueryEngineFromFolderTrait for QueryEngine {
    async fn from_folder(folder: &str) -> Result<Box<dyn QueryEngineTrait>, TuoError> {
        todo!()
    }
}