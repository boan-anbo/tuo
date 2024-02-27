#[derive(Default)]
pub struct GeneratedContent {
    pub content: String,
    // The llm used to generate the summary
    pub model: String,
    /// generated_at timestamp
    pub generated_at: String,
}