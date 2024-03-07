use async_trait::async_trait;

use tuo_core::core::source::document::Document;
use tuo_core::extraction::reader::ReaderTrait;
use tuo_core::parsing::document_parser::ParsedDocument;
use tuo_shared::types::return_type::TuoResult;

#[derive(Default)]
pub struct PDFReader {}


#[async_trait]
impl ReaderTrait for PDFReader {
    async fn read(&self, file_path: &str) -> TuoResult<ParsedDocument> {
        todo!()
    }
}