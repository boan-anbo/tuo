use std::sync::Arc;

use arrow_array::{RecordBatch, RecordBatchReader};
use async_trait::async_trait;
use futures::TryStreamExt;
use lancedb::connection::Connection;
use lancedb::index::MetricType;
use lancedb::{connect, Table};
use tracing::{debug, info};
use typed_builder::TypedBuilder;
use uuid::Uuid;

use tuo_core::core::indexing::index::IndexTrait;
use tuo_core::core::indexing::index_metadata::IndexMetadata;
use tuo_core::core::messaging::content::{
    TextEmbedded, TextEmbeddingOptions, TextInput, TextSourceType,
};
use tuo_core::core::source::node::{Node, NodeRelationTrait};
use tuo_core::core::source::sources::{
    SourceData, SourceInputData, SourceTableName, SourceType, SourceTypeTrait, SourcesId,
};
use tuo_core::embedding::embedder::EmbedderTrait;
use tuo_core::model::model_metadata::EmbeddingModelMetadata;
use tuo_core::parsing::document_parser::ParsedDocument;
use tuo_core::retrieval::search_result::SimilarResult;
use tuo_core::storage::store_metadata::StoreMetadata;
use tuo_shared::consts::defaults::D_TABLE_NAME_TEXT_EMBEDDED;
use tuo_shared::errors::parts::TuoPartsError;
use tuo_shared::types::return_type::TuoResult;

use crate::stores::lancedb::schema::{
    convert_record_batch_to_sources, convert_record_batch_to_text_embedded_search_result,
    convert_sources_to_table_data,
};

#[derive(TypedBuilder)]
pub struct LanceDbIndex {
    pub index_metadata: IndexMetadata,
    pub store_metadata: StoreMetadata,
    pub index_name: String,
    /// The embedder to use for the index
    pub embedder: Option<Arc<Box<dyn EmbedderTrait>>>,
    pub model: EmbeddingModelMetadata,
}

#[async_trait]
impl IndexTrait for LanceDbIndex {
    type StoreDataType = RecordBatch;
    type QueryOptions = Option<LanceDbIndexSearchOptions>;

    type InsertOptions = Option<()>;
    type TableType = Table;

    async fn add_document(
        &self,
        data: Vec<ParsedDocument>,
        opt: Self::InsertOptions,
    ) -> TuoResult<()> {
        for doc in data {
            // add nodes first
            let nodes = doc.to_source_nodes();
            self.add_to_table(nodes).await?;
            // add sections
            let sections = doc.to_source_sections();
            self.add_to_table(sections).await?;
            // add docs
            let parsed_doc = doc.to_source_document();
            self.add_to_table(parsed_doc).await?;
        }
        Ok(())
    }

    async fn add_source_data(
        &self,
        source_data: SourceData,
        opt: Self::InsertOptions,
    ) -> TuoResult<()> {
        let table_ref = self.open_source_table(&source_data.source_type()).await?;
        let dimension = self.get_dimension();
        let insert_data = convert_sources_to_table_data(source_data, dimension);
        table_ref.add(insert_data).execute().await?;
        Ok(())
    }

    async fn add_text_embeddings(&self, text_embedded: &Vec<TextEmbedded>) -> TuoResult<()> {
        let source_data = SourceData::TextEmbedded(text_embedded.clone());
        self.add_source_data(source_data, None).await?;
        Ok(())
    }

    async fn delete(&self, source_ids: &Vec<Uuid>, source_type: &SourceType) -> TuoResult<()> {
        let conn = self.connect().await?;
        let table = conn.open_table(source_type.table_name()).execute().await?;

        let predicate = format!(
            "id IN ({})",
            source_ids
                .iter()
                .map(|id| format!("'{}'", id))
                .collect::<Vec<String>>()
                .join(",")
        );
        table.delete(predicate.as_str()).await?;
        Ok(())
    }

    async fn update(&self, sources: SourceData, opt: Self::InsertOptions) -> TuoResult<()> {
        let ids = sources.get_ids();
        let source_type = sources.source_type();
        let result = self.delete(&ids, &source_type).await?;
        self.add_source_data(sources, opt).await?;
        Ok(())
    }

    async fn similar_embedded_text(
        &self,
        text: &TextInput,
        text_source_type: &TextSourceType,
        opts: Self::QueryOptions,
    ) -> TuoResult<Vec<SimilarResult<TextEmbedded>>> {
        let embedder = self.get_index_embedder().await?;
        let embedding_opt = TextEmbeddingOptions::builder().save_text(true).build();
        let embedded_text = embedder.embed_input(text, &embedding_opt).await?;
        // persist text embedded for caching
        self.add_text_embeddings(&vec![embedded_text.clone()])
            .await?;
        let opts = opts.unwrap_or(LanceDbIndexSearchOptions::builder().build());
        let top_k = opts.top_k;
        let source_type = SourceType::TextEmbedded;
        let table = self.open_source_table(&source_type).await?;
        let record_batch = table
            .search(&embedded_text.embeddings)
            .prefilter(true)
            .filter(format!("source_type = '{}'", text_source_type.as_ref()))
            .metric_type(MetricType::Cosine)
            .limit(top_k)
            .execute_stream()
            .await?
            .try_collect::<Vec<_>>()
            .await
            .expect("Error collecting search results");
        let result = SourceInputData::from_data(record_batch, &source_type);
        let data =
            convert_record_batch_to_text_embedded_search_result(result, self.get_dimension());
        Ok(data)
    }

    async fn similar_sources(
        &self,
        text: &TextInput,
        source_type: &SourceType,
        opts: Self::QueryOptions,
    ) -> TuoResult<SourceData> {
        // continue to search

        // unwrap or error
        let source_data = match source_type {
            SourceType::Node => {
                // when user ask for similarity search on node, match the node content type among text embedded records.
                let text_embedded = self
                    .similar_embedded_text(text, &TextSourceType::NodeContent, opts)
                    .await?;

                for text_embedded in text_embedded.iter() {
                    debug!(
                        "For query text '{:?}': similar embedded text and distance: {:?} {:?}",
                        text.text, text_embedded.distance, text_embedded.data.text
                    );
                }

                // node ids
                let node_ids: Vec<Uuid> = text_embedded
                    .iter()
                    .map(|text_embedded| {
                        text_embedded
                            .data
                            .source_id
                            .expect("Node id should be present")
                    })
                    .collect();

                info!("Node ids: {:?}", node_ids);
                self.get_source_data_by_ids(source_type, node_ids, true)
                    .await?
            }
            _ => {
                unimplemented!()
            }
        };

        Ok(source_data)
    }

    async fn get_index_embedder(&self) -> TuoResult<Arc<Box<dyn EmbedderTrait>>> {
        let test = self
            .embedder
            .as_ref()
            .ok_or(TuoPartsError::IndexError(format!(
                "No embedder found for the index {}",
                self.index_name
            )))?;
        Ok(test.clone())
    }

    fn get_index_metadata(&self) -> IndexMetadata {
        self.index_metadata.clone()
    }
    fn get_store_metadata(&self) -> StoreMetadata {
        self.store_metadata.clone()
    }

    async fn count_records(&self, source_type: &SourceType) -> TuoResult<usize> {
        let table = self.open_source_table(source_type).await?;
        Ok(table.count_rows(None).await?)
    }

    async fn get_source_data_by_id(
        &self,
        source_type: &SourceType,
        id: Uuid,
    ) -> TuoResult<SourceData> {
        let table = self.open_source_table(source_type).await?;
        let record = table
            .query()
            .filter(format!("id = '{}'", id.to_string()))
            .limit(1)
            .execute_stream()
            .await?
            .try_collect::<Vec<_>>()
            .await
            .expect("Error collecting search results");
        let load_data = SourceInputData::from_data(record, &source_type);

        let converted_data = convert_record_batch_to_sources(load_data, self.get_dimension());

        Ok(converted_data)
    }

    /// Get source data by ids
    ///
    /// __Note that the order of the ids in the result is not guaranteed to be the same as the input ids.__
    ///
    /// So if you need to preserve order, you should use `get_source_data_by_id` method.
    async fn get_source_data_by_ids(
        &self,
        source_type: &SourceType,
        ids: Vec<Uuid>,
        preserve_order: bool,
    ) -> TuoResult<SourceData> {
        let table = self.open_source_table(source_type).await?;

        let record = match preserve_order {
            true => {
                let mut all_results: Vec<RecordBatch> = Vec::new();
                for id in ids {
                    info!("Collecting id: {:?}", id);
                    let record = table
                        .query()
                        .filter(format!("id = '{}'", id.to_string()))
                        .limit(1)
                        .execute_stream()
                        .await?
                        .try_collect::<Vec<_>>()
                        .await
                        .expect("Error collecting search results");
                    all_results.extend(record);
                }
                all_results
            }
            false => table
                .query()
                .filter(format!(
                    "id IN ({})",
                    ids.iter()
                        .map(|id| format!("'{}'", id))
                        .collect::<Vec<String>>()
                        .join(",")
                ))
                .execute_stream()
                .await?
                .try_collect::<Vec<_>>()
                .await
                .expect("Error collecting search results"),
        };
        let load_data = SourceInputData::from_data(record, &source_type);
        let converted_data = convert_record_batch_to_sources(load_data, self.get_dimension());
        info!("Converted data: {:?}", converted_data);

        Ok(converted_data)
    }

    async fn get_source_data_with_relations_by_id(
        &self,
        source_type: &SourceType,
        id: Uuid,
    ) -> TuoResult<SourceData> {
        let data = self.get_source_data_by_id(source_type, id).await?;
        match data {
            SourceData::Node(mut nodes) => {
                for node in nodes.iter_mut() {
                    let text_embedded_id = node.content_embeddings_id;
                    if text_embedded_id.is_none() {
                        continue;
                    }
                    let text_embedded_id = text_embedded_id.unwrap();
                    let text_embedded = self
                        .get_source_data_by_id(&SourceType::TextEmbedded, text_embedded_id)
                        .await?
                        .get_text_embedded();
                    if text_embedded.is_none() {
                        continue;
                    }
                    let text_embedded = text_embedded.unwrap();
                    let first_text_embedded = text_embedded.first().unwrap();
                    node.merge_embedded_text(first_text_embedded);
                }
                Ok(SourceData::Node(nodes))
            }
            _ => Ok(data),
        }
    }

    async fn open_source_table(&self, source_type: &SourceType) -> TuoResult<Self::TableType> {
        let table_name = source_type.table_name();
        let conn = self.connect().await?;
        let table = conn.open_table(table_name).execute().await?;
        Ok(table)
    }

    async fn get_unembedded_nodes(&self) -> TuoResult<Vec<Node>> {
        let table = self.open_source_table(&SourceType::Node).await?;
        let nodes = table
            .query()
            .filter("content_embeddings_id IS NULL")
            .execute_stream()
            .await?
            .try_collect::<Vec<_>>()
            .await
            .unwrap();
        let converted = SourceInputData::from_data(nodes, &SourceType::Node);
        let nodes = convert_record_batch_to_sources(converted, self.get_dimension())
            .get_node()
            .unwrap();
        Ok(nodes)
    }
    async fn embed_nodes(&self, doc_ids: Vec<Uuid>) -> TuoResult<()> {
        let unembbedded_nodes = self.get_unembedded_nodes().await?;
        let embedder = self.get_index_embedder().await?;
        // Do not duplicate node text to text embeddings to save space
        let text_embedding_opt = TextEmbeddingOptions::builder().save_text(true).build();
        let embedded_nodes = embedder
            .embed_nodes(unembbedded_nodes, &text_embedding_opt)
            .await?;
        // persist the node embeddings first
        let node_embeddings: Vec<TextEmbedded> = embedded_nodes
            .iter()
            .map(|node| {
                node.content_embeddings
                    .clone()
                    .expect("Node embeddings should be present after embedding")
            })
            .collect();
        let source_text_embedded = SourceData::TextEmbedded(node_embeddings);
        self.add_source_data(source_text_embedded, None).await?;
        // update the nodes with the embeddings
        let source_data = SourceData::Node(embedded_nodes);
        self.update(source_data, None).await?;
        Ok(())
    }

    fn get_dimension(&self) -> i32 {
        self.model.dimensions
    }

    fn get_index_name(&self) -> String {
        self.index_name.clone()
    }

    fn get_model(&self) -> EmbeddingModelMetadata {
        self.model.clone()
    }
}

#[derive(TypedBuilder)]
pub struct LanceDbIndexSearchOptions {
    #[builder(default = 10)]
    pub top_k: usize,
}

impl LanceDbIndex {
    async fn add_to_table(&self, source_data: SourceData) -> TuoResult<()> {
        let table_ref = self.open_source_table(&source_data.source_type()).await?;
        let dimension = self.get_dimension();
        let insert_data = convert_sources_to_table_data(source_data, dimension);
        table_ref.add(insert_data).execute().await?;
        Ok(())
    }

    async fn connect(&self) -> TuoResult<Connection> {
        let conn = connect(self.get_store_metadata().uri.as_str())
            .execute()
            .await?;
        Ok(conn)
    }
}
