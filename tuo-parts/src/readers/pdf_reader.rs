use async_trait::async_trait;
use tuo_core::core::source::document::Document;
use tuo_core::error::TuoError;
use tuo_core::extraction::reader::ReaderTrait;


#[derive(Default)]
pub struct TuoPDFReader {}


#[async_trait]
impl ReaderTrait for TuoPDFReader {
    async fn read(&self, file_path: &str) -> Result<Document, TuoError> {
        Ok(Document::default())
    }
}