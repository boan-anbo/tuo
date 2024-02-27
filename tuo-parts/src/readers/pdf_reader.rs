use async_trait::async_trait;

use tuo_core::entities::document::Document;
use tuo_core::error::TuoCoreError;
use tuo_core::traits::reader::ReaderTrait;

#[derive(Default)]
pub struct TuoPDFReader {}


#[async_trait]
impl ReaderTrait for TuoPDFReader {
    async fn read(&self, file_path: &str) -> Result<Document, TuoCoreError> {
        Ok(Document::default())
    }
}