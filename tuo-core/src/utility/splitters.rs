use async_trait::async_trait;
use crate::core::source::node::Node;
use crate::core::source::section::Section;
use crate::error::TuoError;

#[async_trait]
pub trait SectionSplitter {
    async fn split(&self, input: &Section) -> Result<Vec<Node>, TuoError>;
}