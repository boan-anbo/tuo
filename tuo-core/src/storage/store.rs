use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::core::indexing::index::Index;
use crate::core::source::document::Document;
use crate::core::source::node::Node;
use crate::core::source::section::Section;
use crate::error::TuoError;

pub struct StoreInput {}

pub struct StoreIndexInfo {}

pub struct StoreInfo {
    /// Extra info provided about the stores
    pub extra_info: HashMap<String, String>,
}

pub struct PersistResult {}

#[async_trait]
pub trait StoreTrait {
    /// Initialize the stores
    async fn init(&self, store_uri: &str) -> Result<Box<dyn StoreTrait>, TuoError>;
    /// Open the stores
    async fn open_store(&self, store_uri: &str) -> Result<Arc<dyn StoreTrait>, TuoError>;


    /// Open indexing by name
    async fn open_index_by_name(&self, index_name: &str) -> Result<Option<Index>, TuoError>;
    
    /// Open indexing by id
    async fn open_index(&self, index_id: Uuid) -> Result<Option<Index>, TuoError>;

    /// Add an indexing to the stores
    async fn add_index(&self, index: Index) -> Result<StoreIndexInfo, TuoError>;
    async fn list_indices(&self) -> Result<Vec<StoreIndexInfo>, TuoError>;
    async fn remove_index(&self, index_id: Uuid) -> Result<(), TuoError>;

    /// Add documents to the indexing
    async fn add_documents(&self, index_id: Uuid, documents: Vec<String>) -> Result<(), TuoError>;
    async fn remove_documents(&self, index_id: Uuid, document_ids: Vec<Uuid>) -> Result<(), TuoError>;
    async fn update_document_alone(&self, index_id: Uuid, document_id: Uuid, document: Document) -> Result<(), TuoError>;
    async fn update_document_children(&self, index_id: Uuid, document_id: Uuid, sections: Vec<Section>) -> Result<(), TuoError>;
    /// Add sections to the document
    async fn add_sections(&self, index_id: Uuid, document_id: Uuid, sections: Vec<String>) -> Result<(), TuoError>;
    async fn remove_sections(&self, index_id: Uuid, section_ids: Vec<Uuid>) -> Result<(), TuoError>;
    async fn update_section_alone(&self, index_id: Uuid, section_id: Uuid, section: Section) -> Result<(), TuoError>;
    async fn update_section_children(&self, index_id: Uuid, section_id: Uuid, nodes: Vec<Node>) -> Result<(), TuoError>;

    /// Add nodes to the section
    async fn add_nodes(&self, index_id: Uuid, section_id: Uuid, nodes: Vec<Node>) -> Result<(), TuoError>;
    async fn remove_nodes(&self, index_id: Uuid, node_ids: Vec<Uuid>) -> Result<(), TuoError>;
    async fn update_nodes(&self, index_id: Uuid, node: Vec<Node>) -> Result<(), TuoError>;

    /// Check health
    async fn check_health(&self) -> Result<StoreInfo, TuoError>;
}
