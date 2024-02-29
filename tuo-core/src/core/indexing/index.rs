use std::sync::Arc;
use async_trait::async_trait;
use tracing::info;
use uuid::Uuid;
use crate::core::source::document::Document;
use crate::embedding::embedder::EmbedResultStats;
use crate::error::TuoError;
use crate::extraction::reader::UniFolderReaderTrait;
use crate::utility::fs::get_folder_name_from_path;

#[async_trait]
pub trait IndexTrait {
    async fn load(mut self, name: &str, documents: Vec<Document>) -> Result<Index, TuoError>;
    /// Embed nodes in the indexing.
    async fn embed_nodes(&mut self) -> Result<EmbedResultStats, TuoError>;

    // This method now works directly on the mutable reference.
    // It doesn't need to return the self object anymore, reducing ownership issues.
    async fn embed(&mut self) -> Result<(), TuoError> {
        let result = self.embed_nodes().await?;
        info!("Embedding result: {:?}", result);
        Ok(())
    }
}

/// # Helper trait for creating an index from a folder
///
/// This trait is used to create an index from a folder.
///
/// The folder is read by a UniFolderReader and the documents are then loaded into the index.
#[async_trait]
pub trait IndexFromFolderTrait: IndexTrait {
    async fn from_folder(folder: &str) -> Result<Index, TuoError> {
        let index = Index::default();
        let reader = index.get_uni_reader();
        match reader {
            None => {
                return Err(TuoError::IndexHasNoUniReader);
            }
            Some(reader) => {
                let documents = reader.read_folder(folder).await?;
                let folder_name = get_folder_name_from_path(folder);
                let loaded_index = index.load(&folder_name, documents).await?;
                Ok(loaded_index)
            }
        }
    }

    fn get_uni_reader(&self) -> Option<Arc<dyn UniFolderReaderTrait>>;
}

/// Index struct
#[derive(Default)]
pub struct Index {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub documents: Vec<Document>,
    pub uni_reader: Option<Arc<dyn UniFolderReaderTrait>>,
}

#[async_trait]
impl IndexTrait for Index {
    async fn load(mut self, name: &str, documents: Vec<Document>) -> Result<Index, TuoError> {
        self.name = name.to_string();
        self.documents = documents;
        Ok(
            self
        )
    }

    async fn embed_nodes(&mut self) -> Result<EmbedResultStats, TuoError> {
        todo!()
    }
}


impl IndexFromFolderTrait for Index {
    fn get_uni_reader(&self) -> Option<Arc<dyn UniFolderReaderTrait>> {
        self.uni_reader.clone()
    }
}
