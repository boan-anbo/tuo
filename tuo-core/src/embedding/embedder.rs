use crate::error::TuoError;

pub struct EmbedInput {}

pub struct EmbedResult {}

pub trait EmbedderTrait {
    fn embed(&self, input: EmbedInput) -> Result<EmbedResult, TuoError>;
}