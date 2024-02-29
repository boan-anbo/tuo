use std::sync::Arc;

use tuo_core::extraction::reader::{UniFolderReaderTrait, UniReaderTrait};
use crate::readers::uni_reader_provider::UniReaderProvider;

#[derive(Default)]
pub struct UniFolderReader {
    provider: Arc<UniReaderProvider>,
}

impl UniFolderReaderTrait for UniFolderReader {
    fn get_uni_reader(&self) -> Arc<dyn UniReaderTrait> {
        todo!()
    }

    fn get_folder_file_paths(&self, directory_path: &str) -> Vec<String> {
        todo!()
    }
}