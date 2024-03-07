use std::sync::Arc;

use async_trait::async_trait;

use tuo_core::core::source::document::Document;
use tuo_core::extraction::reader::{ReaderTrait, UniReaderProviderTrait};
use tuo_core::parsing::document_parser::ParsedDocument;
use tuo_shared::types::return_type::TuoResult;

#[derive(Clone, Default)]
pub struct UniReaderProvider {}

#[async_trait]
impl UniReaderProviderTrait for UniReaderProvider {
    async fn read(&self, file_path: &str, mime_type: &str) -> TuoResult<ParsedDocument> {
        todo!()
    }

    fn can_read_ext(&self, extension: &str) -> TuoResult<bool> {
        todo!()
    }

    fn get_reader_by_mime_type(&self, mime_type: &str) -> TuoResult<Option<Arc<dyn ReaderTrait>>> {
        todo!()
    }

    fn can_read_mime(&self, mime_type: &str) -> TuoResult<bool> {
        todo!()
    }
}
