use crate::error::TuoError;

pub struct EmbedInput {}

pub struct EmbedResult {}

#[derive(Debug)]
pub struct EmbedResultStats {}

pub trait EmbedderTrait {
    fn embed(&self, input: EmbedInput) -> Result<EmbedResult, TuoError>;
}