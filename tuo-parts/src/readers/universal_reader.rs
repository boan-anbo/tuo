use std::sync::Arc;

use async_trait::async_trait;

use tuo_core::entities::document::Document;
use tuo_core::error::TuoCoreError;
use tuo_core::traits::reader::{ReaderProviderTrait, ReaderTrait, UniversalReaderTrait};

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
    async fn read(&self, file_path: &str, mime_type: &str) -> Result<Document, TuoCoreError> {
        todo!()
    }

    fn can_read_ext(&self, extension: &str) -> Result<bool, TuoCoreError> {
        todo!()
    }

    fn get_reader_by_mime_type(&self, mime_type: &str) -> Result<Option<Arc<dyn ReaderTrait>>, TuoCoreError> {
        todo!()
    }

    fn can_read_mime(&self, mime_type: &str) -> Result<bool, TuoCoreError> {
        todo!()
    }
}
