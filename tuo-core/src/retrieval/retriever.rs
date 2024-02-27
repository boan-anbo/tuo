use async_trait::async_trait;
use crate::error::TuoError;

pub struct RetrievalRequest {}
pub struct RetrievedResult {}
#[async_trait]
pub trait RetrieverTrait {
    async fn retrieve(&self, retrieval_request: RetrievalRequest) -> Result<RetrievedResult, TuoError>;
}

