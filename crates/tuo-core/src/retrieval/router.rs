use async_trait::async_trait;

use tuo_shared::types::return_type::TuoResult;

use crate::retrieval::retriever::RetrieverTrait;

#[async_trait]
pub trait RouterTrait {
    async fn route(&self, input: RouterInput) -> TuoResult<RouterResult>;
}


pub struct RouterInput {}

pub struct RetriverChoice {
    pub priority: u8,
    pub retriever: Box<dyn RetrieverTrait>,
}
pub struct RouterResult {
    pub choices: Vec<RetriverChoice>,
}