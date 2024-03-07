use async_trait::async_trait;

use tuo_core::core::indexing::index::{ IndexTrait};
use tuo_core::query::engine::{QueryEngineFromFolderTrait, QueryEngineTrait, QueryRequest, QueryResult};
use tuo_shared::types::return_type::TuoResult;

#[derive(Default)]
pub struct QueryEngine {}

#[async_trait]
impl QueryEngineTrait for QueryEngine {

    async fn query(&self, request: QueryRequest) -> TuoResult<QueryResult> {
        todo!()
    }
}

#[async_trait]
impl QueryEngineFromFolderTrait for QueryEngine {
    async fn from_folder(folder: &str) -> TuoResult<Box<dyn QueryEngineTrait>> {
        todo!()
    }
}