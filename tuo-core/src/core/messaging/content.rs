use crate::embedding::embeddings::Embeddings;
use crate::types::date_time::TuoDateTime;

#[derive(Default, Debug)]
pub struct Content {
    pub text: String,
    // The llm used to generate the summary
    pub is_generated: bool,
    pub model: Option<String>,
    /// generated_at timestamp
    pub created_at: TuoDateTime,
    pub embeddings: Option<Embeddings>,
}
