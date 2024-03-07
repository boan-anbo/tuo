use async_trait::async_trait;

use tuo_shared::types::return_type::TuoResult;

use crate::core::source::document::Document;
use crate::core::source::node::Node;
use crate::core::source::section::Section;
use crate::core::source::sources::SourceData;


#[derive(Default)]
pub struct ParsedDocument {
    pub document: Document,
    pub sections: Vec<Section>,
    pub nodes: Vec<Node>,
    pub section_count: i32,
    pub node_count: i32,
    pub total_tokens: i32,
    pub input_uri: String,
}

impl ParsedDocument {
    pub fn to_source_document(&self) -> SourceData {
        SourceData::Document(vec![self.document.clone()])
    }
    pub fn to_source_sections(&self) -> SourceData {
        SourceData::Section(self.sections.clone())
    }
    pub fn to_source_nodes(&self) -> SourceData {
        SourceData::Node(self.nodes.clone())
    }
}

#[async_trait]
pub trait DocumentParserTrait {
    type ParserOptions: Default;
    async fn parse(&self, input: ParsedDocument, opt: Option<Self::ParserOptions>) -> TuoResult<ParsedDocument>;
}
