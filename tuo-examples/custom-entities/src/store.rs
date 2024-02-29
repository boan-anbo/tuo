use std::sync::Arc;

use async_trait::async_trait;
use tuo::tuo_core::core::indexing::index::Index;

use tuo::tuo_core::core::source::document::Document;
use tuo::tuo_core::core::source::node::Node;
use tuo::tuo_core::core::source::section::Section;
use tuo::tuo_core::error::TuoError;
use tuo::tuo_core::storage::store::{StoreIndexInfo, StoreInfo, StoreTrait};

#[derive(Default)]
pub struct CustomStore {}

#[async_trait]
impl StoreTrait for CustomStore {
    async fn init(&self, store_uri: &str) -> Result<Box<dyn StoreTrait>, TuoError> {
        todo!()
    }

    async fn open_store(&self, store_uri: &str) -> Result<Arc<dyn StoreTrait>, TuoError> {
        todo!()
    }

    async fn open_index_by_name(&self, index_name: &str) -> Result<Option<Index>, TuoError> {
        todo!()
    }

    async fn open_index(&self, index_id: uuid::Uuid) -> Result<Option<Index>, TuoError> {
        todo!()
    }

    async fn add_index(&self, index: Index) -> Result<StoreIndexInfo, TuoError> {
        todo!()
    }

    async fn list_indices(&self) -> Result<Vec<StoreIndexInfo>, TuoError> {
        todo!()
    }

    async fn remove_index(&self, index_id: uuid::Uuid) -> Result<(), TuoError> {
        todo!()
    }

    async fn add_documents(&self, index_id: uuid::Uuid, documents: Vec<String>) -> Result<(), TuoError> {
        todo!()
    }

    async fn remove_documents(&self, index_id: uuid::Uuid, document_ids: Vec<uuid::Uuid>) -> Result<(), TuoError> {
        todo!()
    }

    async fn update_document_alone(&self, index_id: uuid::Uuid, document_id: uuid::Uuid, document: Document) -> Result<(), TuoError> {
        todo!()
    }

    async fn update_document_children(&self, index_id: uuid::Uuid, document_id: uuid::Uuid, sections: Vec<Section>) -> Result<(), TuoError> {
        todo!()
    }

    async fn add_sections(&self, index_id: uuid::Uuid, document_id: uuid::Uuid, sections: Vec<String>) -> Result<(), TuoError> {
        todo!()
    }

    async fn remove_sections(&self, index_id: uuid::Uuid, section_ids: Vec<uuid::Uuid>) -> Result<(), TuoError> {
        todo!()
    }

    async fn update_section_alone(&self, index_id: uuid::Uuid, section_id: uuid::Uuid, section: Section) -> Result<(), TuoError> {
        todo!()
    }

    async fn update_section_children(&self, index_id: uuid::Uuid, section_id: uuid::Uuid, nodes: Vec<Node>) -> Result<(), TuoError> {
        todo!()
    }

    async fn add_nodes(&self, index_id: uuid::Uuid, section_id: uuid::Uuid, nodes: Vec<Node>) -> Result<(), TuoError> {
        todo!()
    }

    async fn remove_nodes(&self, index_id: uuid::Uuid, node_ids: Vec<uuid::Uuid>) -> Result<(), TuoError> {
        todo!()
    }

    async fn update_nodes(&self, index_id: uuid::Uuid, node: Vec<Node>) -> Result<(), TuoError> {
        todo!()
    }

    async fn check_health(&self) -> Result<StoreInfo, TuoError> {
        todo!()
    }
}