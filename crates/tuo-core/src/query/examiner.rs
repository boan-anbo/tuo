use async_trait::async_trait;

use tuo_shared::types::return_type::TuoResult;

use crate::core::prompting::prompt::Prompt;
use crate::query::engine::QueryResult;

/// A trait for an examiner of queries
///
/// Examiner is the executor of the query, responsible for sending the query to the models and returning the result.
#[async_trait]
pub trait QueryExecutor {
    async fn examine(&self, prompt: Prompt) -> TuoResult<QueryResult>;
}