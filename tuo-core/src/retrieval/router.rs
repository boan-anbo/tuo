use async_trait::async_trait;

use crate::error::TuoError;
use crate::retrieval::retriever::RetrieverTrait;

#[async_trait]
pub trait RouterTrait {
    async fn route(&self, input: RouterInput) -> Result<RouterResult, TuoError>;
}


pub struct RouterInput {}

pub struct RetriverChoice {
    pub priority: u8,
    pub retriever: Box<dyn RetrieverTrait>,
}
pub struct RouterResult {
    pub choices: Vec<RetriverChoice>,
}