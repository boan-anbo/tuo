use async_trait::async_trait;

use tuo_shared::types::return_type::TuoResult;

pub struct RetrievalRequest {}
pub struct RetrievedResult {}
#[async_trait]
pub trait RetrieverTrait {
    async fn retrieve(&self, retrieval_request: RetrievalRequest) -> TuoResult<RetrievedResult>;
}

