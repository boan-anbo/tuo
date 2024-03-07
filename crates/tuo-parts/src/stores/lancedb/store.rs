use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use arrow_schema::Schema;
use async_trait::async_trait;
use futures::TryStreamExt;
use lancedb::connect;
use lancedb::connection::Connection;
use typed_builder::TypedBuilder;
use uuid::Uuid;

use tuo_core::core::indexing::index::IndexTrait;
use tuo_core::core::indexing::index_metadata::IndexMetadata;
use tuo_core::core::source::sources::{
    SourceData, SourceInputData, SourceTableName, SourceType, SourceTypeTrait,
};
use tuo_core::embedding::embedder::EmbedderTrait;
use tuo_core::model::model_metadata::EmbeddingModelMetadata;
use tuo_core::storage::store::{StoreIndexInfo, StoreTrait};
use tuo_core::storage::store_metadata::StoreMetadata;
use tuo_shared::consts::defaults::{D_TABLE_NAME_INDEX_METADATA, D_TABLE_NAME_STORE_METADATA};
use tuo_shared::errors::parts::TuoPartsError;
use tuo_shared::types::return_type::TuoResult;

use crate::stores::lancedb::index::LanceDbIndex;
use crate::stores::lancedb::schema::{
    convert_record_batch_to_sources, convert_sources_to_table_data, get_all_schema,
};

#[derive(TypedBuilder)]
pub struct LanceDb {
    pub store_metadata: StoreMetadata,
    pub embedder: Arc<Box<dyn EmbedderTrait>>,
    pub indices: HashMap<Uuid, IndexMetadata>,
}

impl LanceDb {
    pub async fn connect(&self) -> TuoResult<Connection> {
        let db = connect(self.get_store_uri().as_str()).execute().await?;
        Ok(db)
    }
}

#[async_trait]
impl StoreTrait for LanceDb {
    type IndexSchema = Schema;
    type IndexType = LanceDbIndex;
    async fn create(
        store_name: &str,
        store_folder: &str,
        embedder: Box<dyn EmbedderTrait>,
    ) -> TuoResult<Self> {
        let arc_embedder = Arc::new(embedder);
        let model_metadata = arc_embedder.get_embedding_model();
        let model_id = model_metadata.id;
        // construct store_metadata
        // construct store uri path
        let uri_path = PathBuf::from(store_folder).join(store_name);

        let store_metadata = StoreMetadata::builder()
            .name(store_name.to_string())
            .model(Some(model_metadata))
            .model_id(Some(model_id))
            .uri(uri_path.to_str().unwrap().to_string())
            .build();
        // create db file at the path if not exists
        std::fs::create_dir_all(&uri_path)?;
        // create store instance
        let instance = LanceDb::builder()
            .store_metadata(store_metadata.clone())
            .indices(HashMap::new())
            .embedder(arc_embedder.clone())
            .build();
        // get default tables schema
        let all_schema: HashMap<String, Arc<Schema>> =
            get_all_schema(instance.get_store_model_dimensions());
        // connect to the store
        let conn = instance.connect().await?;
        // create tables
        for (name, schema) in all_schema {
            conn.create_empty_table(name, schema).execute().await?;
        }
        // insert store metadata into the D_TABLE_NAME_STORE_METADATA table
        instance
            .set_store_metadata(store_metadata, instance.get_store_model_dimensions())
            .await?;

        Ok(instance)
    }

    async fn open(uri: &str, embedder: Box<dyn EmbedderTrait>) -> TuoResult<Self>
    where
        Self: Sized,
    {
        let model = embedder.get_embedding_model();
        let store_metadata = LanceDb::load_store_metadata(uri, model.dimensions).await?;
        Ok(LanceDb::builder()
            .store_metadata(store_metadata)
            .indices(HashMap::new())
            .embedder(Arc::new(embedder))
            .build())
    }

    fn get_store_metadata(&self) -> StoreMetadata {
        self.store_metadata.clone()
    }

    fn get_store_model_metadata(&self) -> EmbeddingModelMetadata {
        self.get_store_metadata()
            .model
            .ok_or(TuoPartsError::StoreError(
                "Store metadata does not have ModelMetadata set.".to_string(),
            ))
            .unwrap()
    }

    fn get_store_model_dimensions(&self) -> i32 {
        self.get_store_model_metadata().dimensions
    }

    fn get_store_uri(&self) -> String {
        self.get_store_metadata().uri.clone()
    }

    async fn load_store_metadata(uri: &str, dimension: i32) -> TuoResult<StoreMetadata> {
        let connection = connect(uri).execute().await?;
        let store_metadata_table = connection
            .open_table(D_TABLE_NAME_STORE_METADATA)
            .execute()
            .await?;
        let result = store_metadata_table
            .query()
            .limit(1)
            .execute_stream()
            .await?
            .try_collect::<Vec<_>>()
            .await
            .map_err(|_| {
                TuoPartsError::StoreError("Error collecting store metadata results".to_string())
            })?;
        let convertion =
            convert_record_batch_to_sources(SourceInputData::StoreMetadata(result), dimension)
                .get_store_metadata();
        Ok(convertion
            .map(|x| x[0].clone())
            .ok_or(TuoPartsError::StoreError(
                "Cannot find store metadata in the store".to_string(),
            ))?)
    }

    async fn set_store_metadata(
        &self,
        store_metadata: StoreMetadata,
        dimension: i32,
    ) -> TuoResult<()> {
        let conn = self.connect().await?;
        let store_metadata_table = conn
            .open_table(D_TABLE_NAME_STORE_METADATA)
            .execute()
            .await?;
        let store_metadata_data = SourceData::StoreMetadata(vec![store_metadata.clone()]);
        let converted = convert_sources_to_table_data(store_metadata_data, dimension);
        store_metadata_table.add(converted).execute().await?;
        Ok(())
    }

    async fn index_open(&self, index_name: &str) -> TuoResult<Self::IndexType> {
        // check if index exists
        let exists = self.index_exists(index_name).await?;
        if !exists {
            return Err(TuoPartsError::StoreError(format!(
                "Index {} does not exist",
                index_name
            )))?;
        }
        let model = self.get_store_model_metadata();

        let connection = self.connect().await?;
        let indices_table = connection
            .open_table(D_TABLE_NAME_INDEX_METADATA)
            .execute()
            .await?;
        let result = indices_table
            .query()
            .filter(format!("name = '{}'", index_name))
            .limit(1)
            .execute_stream()
            .await?
            .try_collect::<Vec<_>>()
            .await
            .map_err(|_| {
                TuoPartsError::StoreError("Error collecting indices results".to_string())
            })?;
        let source_input_data = SourceInputData::IndexMetadata(result);
        let source_data =
            convert_record_batch_to_sources(source_input_data, self.get_store_model_dimensions());
        let results = source_data.get_index_metadata().unwrap();
        let index = results.first().ok_or(TuoPartsError::StoreError(
            "Cannot find index metadata".to_string(),
        ))?;
        Ok(LanceDbIndex::builder()
            .index_name(index_name.to_string())
            .model(model)
            .index_metadata(index.clone())
            .store_metadata(self.get_store_metadata())
            .embedder(Some(self.embedder.clone()))
            .build())
    }

    async fn index_exists(&self, index_name: &str) -> TuoResult<bool> {
        let connection = self.connect().await?;
        let indices_table = connection
            .open_table(D_TABLE_NAME_INDEX_METADATA)
            .execute()
            .await?;
        let result = indices_table
            .query()
            .filter(format!("name = '{}'", index_name))
            .limit(1)
            .execute_stream()
            .await?
            .try_collect::<Vec<_>>()
            .await
            .map_err(|_| {
                TuoPartsError::StoreError("Error collecting indices results".to_string())
            })?;
        Ok(result.len() > 0)
    }

    async fn list_indices(&self) -> TuoResult<Vec<IndexMetadata>> {
        let connection = self.connect().await?;
        let indices_table = connection
            .open_table(D_TABLE_NAME_INDEX_METADATA)
            .execute()
            .await?;
        // select all records
        let result = indices_table
            .query()
            .execute_stream()
            .await?
            .try_collect::<Vec<_>>()
            .await
            .unwrap();
        let source_data = SourceInputData::IndexMetadata(result);
        let indices =
            convert_record_batch_to_sources(source_data, self.get_store_model_dimensions())
                .get_index_metadata()
                .unwrap();
        Ok(indices)
    }

    async fn index_create(&self, name: &str) -> TuoResult<Self::IndexType> {
        let index = IndexMetadata::builder().name(name.to_string()).build();

        let dimensions = self.get_store_model_dimensions();
        let index_source_data = SourceData::IndexMetadata(vec![index]);
        let converted = convert_sources_to_table_data(index_source_data, dimensions);

        let connection = self.connect().await?;
        let index_storage_table = connection
            .open_table(D_TABLE_NAME_INDEX_METADATA)
            .execute()
            .await?;
        let _result = index_storage_table.add(converted).execute().await?;
        self.index_open(name).await
    }

    async fn index_count_records(
        &self,
        index_name: &str,
        source_type: &SourceType,
    ) -> TuoResult<usize> {
        let table = self.index_open(index_name).await?;
        let count = table.count_records(source_type).await?;
        Ok(count)
    }

    async fn index_remove(&self, index_id: Uuid) -> TuoResult<()> {
        todo!()
    }

    async fn check_health(&self) -> TuoResult<StoreMetadata> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use arrow_array::RecordBatch;
    use lancedb::Error;
    use test_log::test;
    use tracing::{debug, info};

    use tuo_core::core::messaging::content::{TextInput, TextSourceType};
    use tuo_core::core::source::document::{Document, DocumentSourceType};
    use tuo_core::core::source::node::{ContentType, Node};
    use tuo_core::core::source::section::Section;
    use tuo_core::model::model_metadata::EmbeddingModelMetadataTrait;
    use tuo_core::parsing::document_parser::ParsedDocument;
    use tuo_core::utility::token::{count_tokens, TokenUtility};
    use tuo_utils::testing::get_random_test_temp_folder;

    use crate::models::open_ai::OpenAIEmbeddingModels;

    use super::*;

    #[test(tokio::test)]
    async fn test_lancedb_store() {
        dotenv::dotenv().ok();
        let temp_folder = get_random_test_temp_folder();
        let model = OpenAIEmbeddingModels::TextEmbedding_3_Small.get_embedder(None);
        let store = LanceDb::create("test_store", temp_folder.as_str(), Box::new(model))
            .await
            .unwrap();
        let store_uri = store.get_store_uri();
        let count = store.list_indices().await.unwrap();
        assert_eq!(count.len(), 0);
        let index_name = "test_index";
        let result = store.index_create(index_name).await.unwrap();
        let created_index_id = result.get_index_metadata().id;
        let indices_count = store.list_indices().await.unwrap();
        assert_eq!(indices_count.len(), 1);
        let opened_index = store.index_open(index_name).await.unwrap();
        let opened_index_id = opened_index.get_index_metadata().id;
        assert_eq!(created_index_id, opened_index_id);

        let document = Document::builder()
            .name("test_doc".to_string())
            .index_id(created_index_id)
            .source_uri(store_uri)
            .source_type(DocumentSourceType::File)
            .build();

        let document_id = document.id;

        let section = Section::builder()
            .index_id(created_index_id)
            .document_id(document_id)
            .name("test_section".to_string())
            .section_order(0)
            .section_level(0)
            .content(Some("test content".to_string()))
            .start_char_index(Some(0))
            .end_char_index(Some(10))
            .build();

        let section_id = section.id;

        let node_content = "test content".to_string();

        let node = Node::builder()
            .index_id(created_index_id)
            .document_id(document_id)
            .section_id(section_id)
            .tokens(count_tokens(&node_content) as i32)
            .content(node_content)
            .content_type(ContentType::Text)
            .content_embeddings(None)
            .content_embeddings_id(None)
            .content_embedded_at(None)
            .index(0)
            .start_char_index(0)
            .end_char_index(10)
            .build();

        let node_content_2 = "中文例子".to_string();
        let node_2 = Node::builder()
            .index_id(created_index_id)
            .document_id(document_id)
            .section_id(section_id)
            .tokens(count_tokens(&node_content_2) as i32)
            .content(node_content_2)
            .content_type(ContentType::Text)
            .content_embeddings(None)
            .content_embeddings_id(None)
            .content_embedded_at(None)
            .index(1)
            .start_char_index(0)
            .end_char_index(10)
            .build();

        let node_1_id = node.id;
        let node_2_id = node_2.id;

        let nodes = vec![node.clone(), node_2.clone()];
        let parsed_document = ParsedDocument {
            document,
            sections: vec![section],
            total_tokens: nodes.count_tokens() as i32,
            nodes,
            section_count: 0,
            node_count: 0,
            input_uri: "".to_string(),
        };

        opened_index
            .add_document(vec![parsed_document], None)
            .await
            .unwrap();

        let node_count = opened_index.count_records(&SourceType::Node).await.unwrap();
        assert_eq!(node_count, 2);
        let section_count = opened_index
            .count_records(&SourceType::Section)
            .await
            .unwrap();
        assert_eq!(section_count, 1);
        let document_count = opened_index
            .count_records(&SourceType::Document)
            .await
            .unwrap();
        assert_eq!(document_count, 1);

        let source_data = opened_index
            .get_source_data_by_id(&SourceType::Node, node_1_id)
            .await
            .unwrap();

        let retrieved_nodes = source_data.get_node().unwrap();
        let first_node = retrieved_nodes.first().unwrap();
        assert_eq!(first_node.id, node_1_id);
        assert!(first_node.content_embeddings.is_none());

        let unembedded_nodes = opened_index.get_unembedded_nodes().await.unwrap();
        assert_eq!(unembedded_nodes.len(), 2);

        opened_index
            .embed_nodes(unembedded_nodes.iter().map(|x| x.id).collect())
            .await
            .unwrap();
        let unembedded_nodes = opened_index.get_unembedded_nodes().await.unwrap();
        assert_eq!(unembedded_nodes.len(), 0);

        let node_after = opened_index
            .get_source_data_by_id(&SourceType::Node, node_1_id)
            .await
            .unwrap();
        let node_after = node_after.get_node().unwrap();
        let node_after = node_after.first().unwrap();

        let node_after_embeddings_id = node_after.content_embeddings_id.clone().unwrap();

        let text_embeddeds = opened_index
            .get_source_data_by_id(&SourceType::TextEmbedded, node_after_embeddings_id)
            .await
            .unwrap();

        let text_embedded = text_embeddeds.get_text_embedded().unwrap();
        let text_embedded = text_embedded.first().unwrap();

        assert_eq!(
            node_after.content_embeddings_id.clone().unwrap(),
            text_embedded.id
        );

        // after update there should still be only two node
        let node_count = opened_index.count_records(&SourceType::Node).await.unwrap();
        assert_eq!(node_count, 2);

        let retrieved_nodes_after_embedding = opened_index
            .get_source_data_with_relations_by_id(&SourceType::Node, node_1_id)
            .await
            .unwrap()
            .get_node()
            .unwrap();

        let first_node_after_embedding = retrieved_nodes_after_embedding.first().unwrap();
        assert_eq!(first_node_after_embedding.id, node_1_id);
        assert!(first_node_after_embedding.content_embeddings_id.is_some());
        assert!(first_node_after_embedding.content_embeddings.is_some());

        let text_input = TextInput::from_user_str("test");
        let results = opened_index
            .similar_sources(&text_input, &SourceType::Node, None)
            .await
            .unwrap();
        info!("results for English similarity: {:?}", results);
        let first_similar_result_to_english = results.get_node().unwrap();
        let first_similar_result_to_english = first_similar_result_to_english.first().unwrap();
        assert_eq!(first_similar_result_to_english.id, node_1_id);

        let text_input = TextInput::from_user_str("中文");
        let results = opened_index
            .similar_sources(&text_input, &SourceType::Node, None)
            .await
            .unwrap();
        info!("results for Chinese similarity: {:?}", results);
        let first_similar_result_to_chinese = results.get_node().unwrap();
        let first_similar_result_to_chinese = first_similar_result_to_chinese.first().unwrap();
        assert_eq!(first_similar_result_to_chinese.id, node_2_id);

        let result_text_embedded = opened_index
            .similar_embedded_text(&text_input, &TextSourceType::NodeContent, None)
            .await
            .unwrap();

        for similar_result in result_text_embedded {
            info!(
                "Distance: {:?}: Text {:?}",
                similar_result.distance, similar_result.data.text
            );
        }
    }
}
