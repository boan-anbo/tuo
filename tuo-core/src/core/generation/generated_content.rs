use crate::embedding::embeddings::Embeddings;
use crate::types::date_time::TuoDateTime;

#[derive(Default)]
pub struct GeneratedContent {
    pub content: String,
    // The llm used to generate the summary
    pub model: String,
    /// generated_at timestamp
    pub generated_at: TuoDateTime,
    pub embeddings: Option<Embeddings>,
}
