use async_trait::async_trait;
use crate::error::TuoError;


pub struct StoreInput {}
pub struct LoadResult {}

pub struct PersistResult {}
#[async_trait]
pub trait StoreTrait {
    async fn load(&self, input: StoreInput) -> Result<LoadResult, TuoError>;
    async fn persist(&self) -> Result<PersistResult, TuoError>;
}