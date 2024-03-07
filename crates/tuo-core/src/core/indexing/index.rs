use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::core::indexing::index_metadata::IndexMetadata;
use tuo_shared::types::return_type::TuoResult;

use crate::core::messaging::content::{TextEmbedded, TextInput, TextSourceType};
use crate::core::source::node::Node;
use crate::core::source::sources::{
    SourceData, SourceInputData, SourceType, SourceTypeTrait, SourcesId,
};
use crate::embedding::embedder::EmbedderTrait;
use crate::extraction::reader::UniFolderReaderTrait;
use crate::model::model_metadata::EmbeddingModelMetadata;
use crate::parsing::document_parser::ParsedDocument;
use crate::retrieval::search_result::SimilarResult;
use crate::storage::store_metadata::StoreMetadata;

/// # IndexTrait
///
/// IndexTrait describes what index can do.
///
/// Index is the basic *logical* units of data organization.
///
/// Logically, an index is a collection of documents, which are collections of sections, which are collections of nodes.
///
/// By logical, we mean that an index is a logical unit of data organization, and __does not__ necessarily correspond to a particular *table* in a database, or a *collection* in a document store.
///
/// Given that an index is not reducible to its storage representation:
/// - an index is only implemented by the end-user.
/// - the index table/collection stores not index _itself_ but the metadata of the index.
/// - an index is a logical collection of things you can do with the index.
///
/// ## Database structure
///
/// See [StoreTrait](crate::core::storage::store::StoreTrait) for the database structure.
///
///
/// ## Database-agnostic
///
/// - Index by itself is database-agnostic, meaning not only that it's not concerned with the particular database implementation, but also the very existence of a database.
/// - It means index methods should not be concerned with things like query, which uses the database but is not concerned with the database persistence operations, all the DB persistence operations should be done by the StoreTrait.
#[async_trait]
pub trait IndexTrait: Sync + Send {
    /// The type of the input data entry specific to the database/collection
    ///
    /// This type will be converted to a Node, the output type of the index.
    type StoreDataType: Send + Sync;

    /// User-defined options for querying the index.
    ///
    /// Examples include:
    ///
    /// - `top_k` for the number of similar documents to return.
    ///
    /// - `filters` for filtering the search results according type (Document, Section, Node) or parents e.g. `section_id` or `document_id`.
    type QueryOptions;

    type InsertOptions: Send + Sync;

    type TableType;

    async fn add_document(
        &self,
        data: Vec<ParsedDocument>,
        opt: Self::InsertOptions,
    ) -> TuoResult<()>;
    async fn add_source_data(
        &self,
        source_data: SourceData,
        opt: Self::InsertOptions,
    ) -> TuoResult<()>;
    async fn add_text_embeddings(&self, text: &Vec<TextEmbedded>) -> TuoResult<()>;
    async fn delete(&self, source_ids: &Vec<Uuid>, source_type: &SourceType) -> TuoResult<()>;
    async fn update(&self, sources: SourceData, opt: Self::InsertOptions) -> TuoResult<()> {
        let ids = sources.get_ids();
        let source_type = sources.source_type();
        let result = self.delete(&ids, &source_type).await?;
        self.add_source_data(sources, opt).await?;
        Ok(())
    }

    /// Search for similar embedded text
    ///
    /// Operates on the embedded text collection, where text embeddings of various sources are put together.
    /// Therefore, you need to specify the [text source type] to search for.
    ///
    /// [text source type] is different from [source type].
    ///
    /// Return the similar embedded text with a calculated [distance] field to store the relevance of the text.
    ///
    /// [text source type]: crate::core::messaging::content::TextSourceType
    /// [source type]: crate::core::source::sources::SourceType
    /// [distance]: crate::core::messaging::content::TextEmbedded
    
    async fn similar_embedded_text(
        &self,
        text_input: &TextInput,
        text_source_type: &TextSourceType,
        opts: Self::QueryOptions,
    ) -> TuoResult<Vec<SimilarResult<TextEmbedded>>>;

    /// Search for similar documents/sections/nodes in the index.
    ///
    /// This calls the upstream `similar_embedded_text` method and then fetches the source data from the index based on the returned ids.
    async fn similar_sources(
        &self,
        text: &TextInput,
        source_type: &SourceType,
        opts: Self::QueryOptions,
    ) -> TuoResult<SourceData>;
    async fn get_index_embedder(&self) -> TuoResult<Arc<Box<dyn EmbedderTrait>>>;

    fn get_index_metadata(&self) -> IndexMetadata;
    fn get_store_metadata(&self) -> StoreMetadata;

    // --- Records functionalities ---
    async fn count_records(&self, source_type: &SourceType) -> TuoResult<usize>;

    /// Base method for getting *scalar* records from the index.
    ///
    /// Getting *scalar* records means relations are not fetched, e.g. getting a node without its embeddings which is stored in a separate table.
    /// To get records with relations, use `get_source_data_with_relations_by_id`.
    async fn get_source_data_by_id(
        &self,
        source_type: &SourceType,
        id: Uuid,
    ) -> TuoResult<SourceData>;

    async fn get_source_data_by_ids(
        &self,
        source_type: &SourceType,
        ids: Vec<Uuid>,
        preserve_order: bool,
    ) -> TuoResult<SourceData>;
    async fn get_source_data_with_relations_by_id(
        &self,
        source_type: &SourceType,
        id: Uuid,
    ) -> TuoResult<SourceData>;

    async fn open_source_table(&self, source_type: &SourceType) -> TuoResult<Self::TableType>;
    async fn get_unembedded_nodes(&self) -> TuoResult<Vec<Node>>;

    async fn embed_nodes(&self, doc_ids: Vec<Uuid>) -> TuoResult<()>;

    // async fn from_folder(&mut self, folder: &str) -> TuoResult<Box<dyn IndexTrait<SearchOptions=Self::SearchOptions, InputDataEntryType=Self::InputDataEntryType>>> {
    //     match reader {
    //         None => {
    //             return Err(TuoCoreError::IndexHasNoUniReader)?;
    //         }
    //         Some(reader) => {
    //             let documents = reader.read_folder(folder).await?;
    //             let folder_name = get_folder_name_from_path(folder);
    //             let loaded_index = index.load(&folder_name, documents).await?;
    //             Ok(loaded_index)
    //         }
    //     }
    // }

    // --- Accessors ---
    fn get_dimension(&self) -> i32;

    fn get_index_name(&self) -> String;
    fn get_model(&self) -> EmbeddingModelMetadata;
}
