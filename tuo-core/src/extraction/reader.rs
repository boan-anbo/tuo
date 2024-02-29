use std::sync::Arc;

use async_trait::async_trait;
use tracing::debug;

use tuo_utils::fs::file_type::{get_mime_type_str, get_mime_type_str_from_extension};

use crate::core::source::document::Document;
use crate::error::TuoError;

#[async_trait]
pub trait ReaderTrait: Sync + Send {
    async fn read(&self, file_path: &str) -> Result<Document, TuoError>;
}

/// Trait for providing readers for different mime types
///
/// The readers provider is responsible for providing the correct readers for a given mime type
///
/// The readers provider is a helper trait, meaning that the user-defined universal reader needs not to have an implementation of the ReaderProvider in order to work.
///
/// However, if the user does provide an implementation of the `UniReaderProviderTrait`, they can simply use the default implmentations of the UniversalReaderTrait and the UniversalFolderReaderTrait.
#[async_trait]
pub trait UniReaderProviderTrait: Sync + Send {
    /// Read a file with the given mime type using the readers associated with the mime type
    async fn read(&self, file_path: &str, mime_type: &str) -> Result<Document, TuoError> {
        let reader = self.get_reader_by_mime_type(mime_type)?;
        match reader {
            None => Err(TuoError::ReaderNoProvider(mime_type.to_string())),
            Some(reader) => {
                let document = reader
                    .read(file_path)
                    .await?;
                Ok(document)
            }
        }
    }
    fn can_read_ext(&self, extension: &str) -> Result<bool, TuoError> {
        let mime_type_str = get_mime_type_str_from_extension(extension)?;
        self.can_read_mime(&mime_type_str)
    }

    fn get_reader_by_mime_type(&self, mime_type: &str) -> Result<Option<Arc<dyn ReaderTrait>>, TuoError>;
    fn can_read_mime(&self, mime_type: &str) -> Result<bool, TuoError> {
        let reader = self.get_reader_by_mime_type(mime_type)?;
        Ok(reader.is_some())
    }
}

#[async_trait]
pub trait UniReaderTrait: Sync + Send {
    fn get_reader_providers(&self) -> Option<Arc<dyn UniReaderProviderTrait>>;
    async fn read(&self, file_path: &str) -> Result<Document, TuoError> {
        let mime_type = get_mime_type_str(file_path)?;
        let reader_providers = self.get_reader_providers();
        match reader_providers {
            None => {
                Err(TuoError::UniReaderHasNoReaderProvider)
            }
            Some(reader_provider) => {
                let document = reader_provider.read(file_path, &mime_type).await?;
                Ok(document)
            }
        }
    }
    fn can_read_ext(&self, extension: &str) -> Result<bool, TuoError> {
        let mime_type_str = get_mime_type_str_from_extension(extension)?;
        let reader_providers = self.get_reader_providers();
        match reader_providers {
            None => {
                Err(TuoError::UniReaderHasNoReaderProvider)
            }
            Some(reader_provider) => {
                let result = reader_provider.can_read_mime(&mime_type_str)?;
                Ok(result)
            }
        }
    }
}

/// Trait for reading from a directory
///
/// Read all files in a directory tha can be read by the UniversalReader
#[async_trait]
pub trait UniFolderReaderTrait: Sync + Send {
    fn get_uni_reader(&self) -> Arc<dyn UniReaderTrait>;
    fn get_folder_file_paths(&self, directory_path: &str) -> Vec<String>;
    async fn read_folder(&self, directory_path: &str) -> Result<Vec<Document>, TuoError> {
        let uni_reader = self.get_uni_reader();
        let file_paths = self.get_folder_file_paths(directory_path);
        // filter out files that cannot be read by the universal readers
        let file_paths: Vec<String> = file_paths
            .into_iter()
            .filter(|file_path| {
                let mime_type = match get_mime_type_str(file_path) {
                    Ok(mime_type) => mime_type,
                    Err(_) => return false,
                };
                let result = uni_reader.can_read_ext(&mime_type);
                match result {
                    Ok(can_read) => can_read,
                    Err(err) => {
                        debug!("Ignoring mime type: {} because of error: {}", mime_type, err);
                        false
                    }
                }
            })
            .collect();
        let mut documents = Vec::new();
        for file_path in file_paths {
            let document = uni_reader.read(&file_path).await?;
            documents.push(document);
        }
        Ok(documents)
    }
}