use std::sync::Arc;

use async_trait::async_trait;
use tuo_core::core::source::document::Document;
use tuo_core::error::TuoError;
use tuo_core::extraction::reader::{ReaderProviderTrait, ReaderTrait, UniversalReaderTrait};


pub struct TuoUniversalReader {
    reader_provider: Arc<TuoReaderProvider>,
}

#[async_trait]
impl UniversalReaderTrait for TuoUniversalReader {
    fn get_reader_providers(&self) -> Arc<dyn ReaderProviderTrait> {
        self.reader_provider.clone()
    }
}

#[derive(Clone)]
pub struct TuoReaderProvider {}

#[async_trait]
impl ReaderProviderTrait for TuoReaderProvider {
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
