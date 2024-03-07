use std::collections::HashMap;

use async_trait::async_trait;

use tuo_shared::types::return_type::TuoResult;

#[async_trait]
pub trait ToolTrait: Send + Sync {
    async fn call(&self, args: HashMap<String, String>) -> TuoResult<HashMap<String, String>>;
}