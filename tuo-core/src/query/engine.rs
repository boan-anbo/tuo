use async_trait::async_trait;
use crate::error::TuoError;

pub struct QueryContext {}
pub struct QueryResult {}
pub struct QueryRequest {

}

pub struct ChatRequest {}
pub struct ChatResult {}

#[async_trait]
pub trait QueryEngineTrait {
    async fn query(&self, request: QueryRequest) -> Result<QueryResult, TuoError>;
}

#[async_trait]
pub trait ChatEngineTrait {
    async fn chat(&self, request: ChatRequest) -> Result<ChatResult, TuoError>;
}