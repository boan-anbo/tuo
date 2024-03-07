use std::sync::Arc;

use async_trait::async_trait;

use tuo_core::extraction::reader::{UniReaderProviderTrait, UniReaderTrait};

use crate::readers::uni_reader_provider::UniReaderProvider;

#[derive(Default)]
pub struct UniReader {
    reader_provider: Arc<UniReaderProvider>,
}

#[async_trait]
impl UniReaderTrait for UniReader {
    fn get_reader_providers(&self) -> Option<Arc<dyn UniReaderProviderTrait>> {
        Some(self.reader_provider.clone())
    }
}

