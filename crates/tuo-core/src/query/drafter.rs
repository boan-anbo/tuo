use async_trait::async_trait;

use tuo_shared::types::return_type::TuoResult;

use crate::core::prompting::prompt::Prompt;
use crate::core::prompting::prompt_context::PromptContext;

/// A trait for a drafter of prompts
///
/// A drafter is responsible for drafting the final prompt based on a given prompt and context, to be fed to the query engine.
#[async_trait]
pub trait PromptDrafter: Sync + Send {
    async fn draft(&self, prompt: Prompt, context: PromptContext) -> TuoResult<Prompt>;
}