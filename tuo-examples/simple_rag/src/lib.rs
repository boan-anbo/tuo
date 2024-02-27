use tuo::tuo_core::core::source::document::Document;
use tuo::tuo_core::error::TuoError;
use tuo::tuo_core::extraction::reader::ReaderTrait;
use tuo::tuo_parts::readers::pdf_reader::TuoPDFReader;

pub async fn read_file() -> Result<Document, TuoError> {
    let reader = TuoPDFReader::default();
    let read_result = reader.read("file_path").await?;
    Ok(read_result)
}

mod main_test;