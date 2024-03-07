use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;
use tuo_shared::consts::defaults::{D_TABLE_NAME_DOCUMENTS, D_TABLE_NAME_INDEX_METADATA, D_TABLE_NAME_MODELS_METADATA, D_TABLE_NAME_NODES, D_TABLE_NAME_SECTIONS, D_TABLE_NAME_STORE_METADATA, D_TABLE_NAME_TEXT_EMBEDDED};
use tuo_shared::errors::parts::TuoPartsError;

use tuo_shared::types::return_type::TuoResult;

use crate::core::indexing::index::IndexTrait;
use crate::core::indexing::index_metadata::IndexMetadata;
use crate::core::source::document::Document;
use crate::core::source::node::Node;
use crate::core::source::section::Section;
use crate::core::source::sources::{SourceData, SourcesId, SourceType};
use crate::embedding::embedder::EmbedderTrait;
use crate::model::model_metadata::EmbeddingModelMetadata;
use crate::storage::store_metadata::StoreMetadata;

pub struct StoreInput {}

pub struct StoreIndexInfo {}

pub struct PersistResult {}

/// ## Store
///
/// A store is the largest unit of data organization.
/// - A store has multiple indices.
/// - An index has multiple documents.
/// - A document has multiple sections.
/// - A section has multiple nodes.
///
/// A store usually, though not necessarily, corresponds to a database.
///
/// ### How to initialize a store
///
/// 1. Create an embedding [model](crate::model::model_metadata::EmbeddingModelMetadata) to be used for the store.
/// 2. Create a [store metadata](crate::storage::store_metadata::StoreMetadata) with the given model.
///
/// ### Database
///
/// Such a database consists, though not necessarily, of the following five tables.
///
/// #### 1. metadata table
///
/// Contains metadata about the store.
///
/// #### 2. index table
///
/// Contains rows of [index_metadata].
///
/// #### 3. document table
///
/// Contains rows of document.
///
/// #### 4. section table
///
/// Contains rows of section.
///
/// #### 5. node table
///
/// Contains rows of node.
///
/// Each node contains the actual [content] and its [embeddings].
///
/// #### 6. prompt_embedding table
///
/// Contains prompt_plain_text, its embeddings, and hash (for quick lookup).
///
/// Can be used for quick lookup and caching of prompts.
///
/// #### 7. model table
///
/// Contains a single row of **embedding** [ModelMetadata].
///
/// ### Index
///
/// An index, therefore, __DOES NOT__, corresponds to a particular *table* or  *collection* in the store, but is an organization of data spread across the four (except for the metadata table) tables.
///
/// Instead, an index is a *conceptual* unit of data organization.
///
/// See [IndexTrait] for more details.
///
/// ### FAQ
///
/// #### Why do you call the rows of stores and indices "store_metadata" and "index_metadata", but not document_metadata, section_metadata, and node_metadata?
///
/// It's because both stores and indices are conceptual and acting entities implemented by the end-user.
///
/// So what's stored in the store and index tables are __metadata about the store and index, not the store and index _themselves___.
///
/// Document, section, and node are already representational entities, which is persisted as is, and therefore, are not called "metadata".
///
/// [index_metadata]: crate::core::indexing::index_metadata::IndexMetadata
/// [IndexTrait]: crate::core::indexing::index::IndexTrait
/// [content]: crate::core::source::node::Node
/// [embeddings]: crate::embedding::embeddings::Embeddings
/// [ModelMetadata]: crate::model::model_metadata::EmbeddingModelMetadata
#[async_trait]
pub trait StoreTrait {
    type IndexSchema;
    type IndexType: IndexTrait;

    /// Initialize the store
    ///
    /// # Steps
    /// 1. Create the store at the specified uri if it doesn't exist.
    /// 2. Create the essential tables/collections if they don't exist.
    async fn create(store_name: &str, store_folder: &str, embedder: Box<dyn EmbedderTrait>) -> TuoResult<Self> where Self: Sized;
    
    async fn open(uri: &str, embedder: Box<dyn EmbedderTrait>) -> TuoResult<Self> where Self: Sized;

    // --- Accessors ---

    /// Get the metadata of the store
    fn get_store_metadata(&self) -> StoreMetadata;
    fn get_store_model_metadata(&self) -> EmbeddingModelMetadata;

    /// Get the dimensions of the store
    ///
    /// Each store (usually a database) uses a single dimension/model for all its indices. This is for the sake of uniformity.
    ///
    /// To switch models, use *reset* method, which will reset the store.
    fn get_store_model_dimensions(&self) -> i32;

    /// Get the uri of the store
    fn get_store_uri(&self) -> String;



    // --- DB operations ---
    async fn load_store_metadata(uri: &str, dimension: i32) -> TuoResult<StoreMetadata>;
    /// --- Set store metadata ---
    async fn set_store_metadata(&self, store_metadata: StoreMetadata, dimension: i32) -> TuoResult<()>;

    // --- Index functionalities ---
    /// Open indexing by name
    async fn index_open(&self, index_name: &str) -> TuoResult<Self::IndexType>;

    async fn index_exists(&self, index_name: &str) -> TuoResult<bool>;
    async fn list_indices(&self) -> TuoResult<Vec<IndexMetadata>>;
    /// Crate an index
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the index
    /// * `dimension` - The dimension of the model used for embedding.
    async fn index_create(&self, name: &str) -> TuoResult<Self::IndexType>;
    async fn index_count_records(&self, index_name: &str, source_type: &SourceType) -> TuoResult<usize>;
    

    async fn index_remove(&self, index_id: Uuid) -> TuoResult<()>;

     /// Check health
    async fn check_health(&self) -> TuoResult<StoreMetadata>;
}
