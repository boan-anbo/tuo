use async_trait::async_trait;
use tuo::tuo_core::core::indexing::index::Index;
use tuo::tuo_core::error::TuoError;
use tuo::tuo_core::query::engine::{QueryEngineFromFolderTrait, QueryEngineTrait, QueryRequest, QueryResult};

#[derive(Default)]
pub struct CustomQueryEngine {}

#[async_trait]
impl QueryEngineTrait for CustomQueryEngine {
    async fn load_index(&self, index: Index) -> Result<Box<dyn QueryEngineTrait>, TuoError> {
        todo!()
    }

    async fn query(&self, request: QueryRequest) -> Result<QueryResult, TuoError> {
        todo!()
    }
}


#[async_trait]
impl QueryEngineFromFolderTrait for CustomQueryEngine {
    async fn from_folder(folder: &str) -> Result<Box<dyn QueryEngineTrait>, TuoError> {
        todo!()
    }
}