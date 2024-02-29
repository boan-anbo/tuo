use async_trait::async_trait;
use std::sync::Arc;
use tuo_core::core::source::document::Document;
use tuo_core::error::TuoError;
use tuo_core::extraction::reader::{ReaderTrait, UniReaderProviderTrait};

#[derive(Clone, Default)]
pub struct UniReaderProvider {}

#[async_trait]
impl UniReaderProviderTrait for UniReaderProvider {
    async fn read(&self, file_path: &str, mime_type: &str) -> Result<Document, TuoError> {
        todo!()
    }

    fn can_read_ext(&self, extension: &str) -> Result<bool, TuoError> {
        todo!()
    }

    fn get_reader_by_mime_type(&self, mime_type: &str) -> Result<Option<Arc<dyn ReaderTrait>>, TuoError> {
        todo!()
    }

    fn can_read_mime(&self, mime_type: &str) -> Result<bool, TuoError> {
        todo!()
    }
}
