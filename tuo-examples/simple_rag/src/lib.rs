use tuo::tuo_core::entities::document::Document;
use tuo::tuo_core::error::TuoCoreError;
use tuo::tuo_parts::readers::pdf_reader::TuoPDFReader;
use tuo::tuo_core::traits::reader::ReaderTrait;
pub async fn read_file() -> Result<Document, TuoCoreError> {
    let reader = TuoPDFReader::default();
    let read_result = reader.read("file_path").await?;
    Ok(read_result)
}

mod main_test;