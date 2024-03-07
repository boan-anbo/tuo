use async_trait::async_trait;

use tuo_shared::types::return_type::TuoResult;

use crate::core::source::node::Node;
use crate::core::source::section::Section;

#[async_trait]
pub trait SectionSplitter {
    async fn split(&self, input: &Section) -> TuoResult<Vec<Node>>;
}