use std::collections::HashMap;

use async_trait::async_trait;

use crate::error::TuoError;

#[async_trait]
pub trait ToolTrait: Send + Sync {
    async fn call(&self, args: HashMap<String, String>) -> Result<HashMap<String, String>, TuoError>;
}