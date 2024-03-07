use std::sync::Arc;
use tuo::tuo_core::extraction::reader::{UniFolderReaderTrait, UniReaderProviderTrait, UniReaderTrait};

pub struct CustomUniReader {
}

impl UniReaderTrait for CustomUniReader {
    fn get_reader_providers(&self) -> Option<Arc<dyn UniReaderProviderTrait>> {
        todo!()
    }
}


#[derive(Default)]
pub struct CustomUniFolderReader {
}

impl UniFolderReaderTrait for CustomUniFolderReader {
    fn get_uni_reader(&self) -> Arc<dyn UniReaderTrait> {
        todo!()
    }

    fn get_folder_file_paths(&self, directory_path: &str) -> Vec<String> {
        todo!()
    }
}