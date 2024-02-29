use std::sync::Arc;
use async_trait::async_trait;

use uuid::Uuid;

use tuo_core::core::indexing::index::Index;
use tuo_core::core::source::document::Document;
use tuo_core::core::source::node::Node;
use tuo_core::core::source::section::Section;
use tuo_core::error::TuoError;
use tuo_core::storage::store::{StoreIndexInfo, StoreInfo, StoreTrait};

#[derive(Default)]
pub struct Store {}

#[async_trait]
impl StoreTrait for Store {
    async fn init(&self, store_uri: &str) -> Result<Box<dyn StoreTrait>, TuoError> {
        todo!()
    }

    async fn open_store(&self, store_uri: &str) -> Result<Arc<dyn StoreTrait>, TuoError> {
        todo!()
    }

    async fn open_index(&self, index_id: Uuid) -> Result<Option<Index>, TuoError> {
        todo!()
    }

    async fn open_index_by_name(&self, index_name: &str) -> Result<Option<Index>, TuoError> {
        todo!()
    }

    async fn add_index(&self, index: Index) -> Result<StoreIndexInfo, TuoError> {
        todo!()
    }

    async fn list_indices(&self) -> Result<Vec<StoreIndexInfo>, TuoError> {
        todo!()
    }

    async fn remove_index(&self, index_id: Uuid) -> Result<(), TuoError> {
        todo!()
    }

    async fn add_documents(&self, index_id: Uuid, documents: Vec<String>) -> Result<(), TuoError> {
        todo!()
    }

    async fn remove_documents(&self, index_id: Uuid, document_ids: Vec<Uuid>) -> Result<(), TuoError> {
        todo!()
    }

    async fn update_document_alone(&self, index_id: Uuid, document_id: Uuid, document: Document) -> Result<(), TuoError> {
        todo!()
    }

    async fn update_document_children(&self, index_id: Uuid, document_id: Uuid, sections: Vec<Section>) -> Result<(), TuoError> {
        todo!()
    }

    async fn add_sections(&self, index_id: Uuid, document_id: Uuid, sections: Vec<String>) -> Result<(), TuoError> {
        todo!()
    }

    async fn remove_sections(&self, index_id: Uuid, section_ids: Vec<Uuid>) -> Result<(), TuoError> {
        todo!()
    }

    async fn update_section_alone(&self, index_id: Uuid, section_id: Uuid, section: Section) -> Result<(), TuoError> {
        todo!()
    }

    async fn update_section_children(&self, index_id: Uuid, section_id: Uuid, nodes: Vec<Node>) -> Result<(), TuoError> {
        todo!()
    }

    async fn add_nodes(&self, index_id: Uuid, section_id: Uuid, nodes: Vec<Node>) -> Result<(), TuoError> {
        todo!()
    }

    async fn remove_nodes(&self, index_id: Uuid, node_ids: Vec<Uuid>) -> Result<(), TuoError> {
        todo!()
    }

    async fn update_nodes(&self, index_id: Uuid, node: Vec<Node>) -> Result<(), TuoError> {
        todo!()
    }

    async fn check_health(&self) -> Result<StoreInfo, TuoError> {
        todo!()
    }
}