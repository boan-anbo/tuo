use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use arrow_array::types::Float32Type;
use arrow_array::{
    Array, ArrayRef, Date64Array, Datum, FixedSizeListArray, Float32Array, Float64Array,
    Int32Array, RecordBatch, RecordBatchIterator, RecordBatchReader, StringArray,
};
use arrow_schema::{ArrowError, DataType, Field, Schema};
use tracing::{debug, info};
use uuid::Uuid;

use tuo_core::core::indexing::index_metadata::{IndexMetadata, IndexMetadataFieldName};
use tuo_core::core::messaging::content::{TextEmbedded, TextEmbeddedFieldName, TextSourceType};
use tuo_core::core::source::document::{DocumentFieldName, DocumentSourceType, DocumentType};
use tuo_core::core::source::node::{ContentType, NodeFieldName};
use tuo_core::core::source::section::SectionFieldName;
use tuo_core::core::source::sources::{SourceData, SourceInputData};
use tuo_core::model::model_metadata::{EmbeddingModelMetadata, EmbeddingModelMetadataFieldName};
use tuo_core::retrieval::search_result::SimilarResult;
use tuo_core::storage::store_metadata::{StoreMetadata, StoreMetadataFieldName};
use tuo_shared::consts::defaults::{
    D_TABLE_COLUMN_NAME_VECTOR, D_TABLE_NAME_DOCUMENTS, D_TABLE_NAME_INDEX_METADATA,
    D_TABLE_NAME_MODELS_METADATA, D_TABLE_NAME_NODES, D_TABLE_NAME_SECTIONS,
    D_TABLE_NAME_STORE_METADATA, D_TABLE_NAME_TEXT_EMBEDDED,
};
use tuo_utils::datetime::timestamp::utc_from_epoch;

pub(crate) fn convert_record_batch_to_text_embedded_search_result(
    input: SourceInputData<RecordBatch>,
    dimension: i32,
) -> Vec<SimilarResult<TextEmbedded>> {
    match input {
        SourceInputData::TextEmbedded(text_embedded) => {
            convert_record_batch_to_text_embedded(text_embedded, dimension)
        }
        _ => unimplemented!("Only TextEmbedded is supported for search results currently"),
    }
}
pub(crate) fn convert_record_batch_to_sources(
    sources: SourceInputData<RecordBatch>,
    dimension: i32,
) -> SourceData {
    match sources {
        SourceInputData::StoreMetadata(store_metadata) => {
            SourceData::StoreMetadata(convert_record_batch_to_store_metadata(store_metadata))
        }
        SourceInputData::ModelMetadata(model_metadata) => {
            SourceData::ModelMetadata(convert_record_batch_to_model_metadata(model_metadata))
        }
        SourceInputData::IndexMetadata(index_metadata) => {
            SourceData::IndexMetadata(convert_record_batch_to_index_metadata(index_metadata))
        }
        SourceInputData::TextEmbedded(text_embedded) => SourceData::TextEmbedded(
            convert_record_batch_to_text_embedded(text_embedded, dimension)
                .into_iter()
                .map(|data| data.data)
                .collect(),
        ),
        SourceInputData::Document(documents) => {
            SourceData::Document(convert_record_batch_to_document(documents))
        }
        SourceInputData::Section(sections) => {
            SourceData::Section(convert_record_batch_to_section(sections))
        }
        SourceInputData::Node(nodes) => SourceData::Node(convert_record_batch_to_node(nodes)),
        _ => unimplemented!(),
    }
}

pub(crate) fn convert_sources_to_table_data(
    sources: SourceData,
    dimension: i32,
) -> Box<dyn RecordBatchReader<Item = Result<RecordBatch, ArrowError>> + Send> {
    match sources {
        SourceData::StoreMetadata(store_metadata) => convert_store_metadata(store_metadata),
        SourceData::ModelMetadata(model_metadata) => convert_model_metadata(model_metadata),
        SourceData::IndexMetadata(index_metadata) => convert_indices_metadata(index_metadata),
        SourceData::TextEmbedded(text_embedded) => convert_texts_embedded(text_embedded, dimension),
        SourceData::Document(documents) => convert_documents(documents),
        SourceData::Section(sections) => convert_sections(sections),
        SourceData::Node(nodes) => convert_nodes(nodes, dimension),
    }
}

pub(crate) fn get_all_schema(dimension: i32) -> HashMap<String, Arc<Schema>> {
    let mut schema_map = HashMap::new();

    schema_map.insert(
        D_TABLE_NAME_STORE_METADATA.to_string(),
        store_metadata_schema(),
    );

    schema_map.insert(
        D_TABLE_NAME_INDEX_METADATA.to_string(),
        index_metadata_schema(),
    );

    schema_map.insert(D_TABLE_NAME_DOCUMENTS.to_string(), document_schema());

    schema_map.insert(
        D_TABLE_NAME_MODELS_METADATA.to_string(),
        model_metadata_schema(),
    );

    schema_map.insert(D_TABLE_NAME_SECTIONS.to_string(), section_schema());

    schema_map.insert(D_TABLE_NAME_NODES.to_string(), node_schema(dimension));

    schema_map.insert(
        D_TABLE_NAME_TEXT_EMBEDDED.to_string(),
        text_embedded_schema(dimension),
    );

    schema_map
}

/// Schema for [StoreMetadata](tuo_core::storage::store_metadata::StoreMetadata)
fn store_metadata_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new(StoreMetadataFieldName::Id.name(), DataType::Utf8, false),
        Field::new(StoreMetadataFieldName::Name.name(), DataType::Utf8, false),
        Field::new(
            StoreMetadataFieldName::CreatedAt.name(),
            DataType::Date64,
            false,
        ),
        Field::new(StoreMetadataFieldName::ModelId.name(), DataType::Utf8, true),
        Field::new(StoreMetadataFieldName::Uri.name(), DataType::Utf8, false),
    ]))
}

fn convert_store_metadata(
    sources: Vec<StoreMetadata>,
) -> Box<dyn RecordBatchReader<Item = Result<RecordBatch, ArrowError>> + Send> {
    let batches = RecordBatchIterator::new(
        vec![RecordBatch::try_new(
            store_metadata_schema(),
            vec![
                Arc::new(StringArray::from_iter_values(
                    sources.iter().map(|data| data.id.to_string()),
                )),
                Arc::new(StringArray::from_iter_values(
                    sources.iter().map(|data| data.name.clone()),
                )),
                Arc::new(Date64Array::from(
                    sources
                        .iter()
                        .map(|(er)| er.created_at.timestamp())
                        .collect::<Vec<i64>>(),
                )),
                Arc::new(StringArray::from(
                    sources
                        .iter()
                        .map(|data| data.model_id.map(|id| id.to_string()))
                        .collect::<Vec<Option<String>>>(),
                )),
                Arc::new(StringArray::from_iter_values(
                    sources.iter().map(|data| data.uri.clone()),
                )),
            ],
        )
        .unwrap()]
        .into_iter()
        .map(Ok),
        store_metadata_schema(),
    );
    Box::new(batches)
}

fn convert_record_batch_to_store_metadata(record_batch: Vec<RecordBatch>) -> Vec<StoreMetadata> {
    record_batch
        .iter()
        .flat_map(|batch| {
            (0..batch.num_rows())
                .map(|row| {
                    let id = batch
                        .column_by_name(StoreMetadataFieldName::Id.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                        .unwrap()
                        .value(row);
                    let name = batch
                        .column_by_name(StoreMetadataFieldName::Name.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                        .unwrap()
                        .value(row);
                    let created_at = batch
                        .column_by_name(StoreMetadataFieldName::CreatedAt.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<Date64Array>()
                        .unwrap()
                        .value(row);
                    let model_id = match batch
                        .column_by_name(StoreMetadataFieldName::ModelId.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                    {
                        Some(array) => {
                            if array.is_null(row) {
                                None
                            } else {
                                Some(array.value(row))
                            }
                        }
                        None => None,
                    };
                    let uri = batch
                        .column_by_name(StoreMetadataFieldName::Uri.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                        .unwrap()
                        .value(row);

                    StoreMetadata {
                        id: Uuid::try_parse(id).unwrap(),
                        name: name.to_string(),
                        created_at: utc_from_epoch(created_at),
                        uri: uri.to_string(),
                        model_id: model_id.map(|id| Uuid::try_parse(id).unwrap()),
                        model: None,
                    }
                })
                .collect::<Vec<StoreMetadata>>()
        })
        .collect()
}

/// Schema for [IndexMetadata](tuo_core::core::indexing::index_metadata::IndexMetadata)
fn index_metadata_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new(IndexMetadataFieldName::Id.name(), DataType::Utf8, false),
        Field::new(IndexMetadataFieldName::Name.name(), DataType::Utf8, false),
        Field::new(
            IndexMetadataFieldName::Description.name(),
            DataType::Utf8,
            true,
        ),
        Field::new(
            IndexMetadataFieldName::DocumentCount.name(),
            DataType::Int32,
            false,
        ),
        Field::new(
            IndexMetadataFieldName::CreatedAt.name(),
            DataType::Date64,
            false,
        ),
        Field::new(
            IndexMetadataFieldName::UpdatedAt.name(),
            DataType::Date64,
            false,
        ),
    ]))
}

fn convert_indices_metadata(
    index_metadata: Vec<IndexMetadata>,
) -> Box<dyn RecordBatchReader<Item = Result<RecordBatch, ArrowError>> + Send> {
    let sources = index_metadata;
    let batches = RecordBatchIterator::new(
        vec![RecordBatch::try_new(
            index_metadata_schema(),
            vec![
                Arc::new(StringArray::from_iter_values(
                    sources.iter().map(|data| data.id.to_string()),
                )),
                Arc::new(StringArray::from_iter_values(
                    sources.iter().map(|data| data.name.clone()),
                )),
                Arc::new(StringArray::from(
                    sources
                        .iter()
                        .map(|data| data.description.clone())
                        .collect::<Vec<Option<String>>>(),
                )),
                Arc::new(Int32Array::from_iter_values(
                    sources.iter().map(|data| data.document_count as i32),
                )),
                Arc::new(Date64Array::from_iter_values(
                    sources.iter().map(|(er)| er.created_at.timestamp()),
                )),
                Arc::new(Date64Array::from_iter_values(
                    sources.iter().map(|(er)| er.updated_at.timestamp()),
                )),
            ],
        )
        .unwrap()]
        .into_iter()
        .map(Ok),
        index_metadata_schema(),
    );
    Box::new(batches)
}

fn convert_record_batch_to_index_metadata(record_batch: Vec<RecordBatch>) -> Vec<IndexMetadata> {
    record_batch
        .iter()
        .flat_map(|batch| {
            (0..batch.num_rows())
                .map(|row| {
                    let id = batch
                        .column_by_name(IndexMetadataFieldName::Id.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                        .unwrap()
                        .value(row);
                    let name = batch
                        .column_by_name(IndexMetadataFieldName::Name.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                        .unwrap()
                        .value(row)
                        .to_string();
                    let description = match batch
                        .column_by_name(IndexMetadataFieldName::Description.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                    {
                        Some(array) => {
                            if array.is_null(row) {
                                None
                            } else {
                                Some(array.value(row))
                            }
                        }
                        None => None,
                    }
                    .map(|s| s.to_string());
                    let document_count = batch
                        .column_by_name(IndexMetadataFieldName::DocumentCount.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<Int32Array>()
                        .unwrap()
                        .value(row);
                    let created_at = batch
                        .column_by_name(IndexMetadataFieldName::CreatedAt.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<Date64Array>()
                        .unwrap()
                        .value(row);
                    let updated_at = batch
                        .column_by_name(IndexMetadataFieldName::UpdatedAt.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<Date64Array>()
                        .unwrap()
                        .value(row);

                    IndexMetadata {
                        id: Uuid::try_parse(id).unwrap(),
                        name: name.to_string(),
                        description,
                        document_count,
                        created_at: utc_from_epoch(created_at),
                        updated_at: utc_from_epoch(updated_at),
                    }
                })
                .collect::<Vec<IndexMetadata>>()
        })
        .collect()
}

/// Schema for [ModelMetadata](tuo_core::model::model_metadata::EmbeddingModelMetadata)
fn model_metadata_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new(
            EmbeddingModelMetadataFieldName::Id.name(),
            DataType::Utf8,
            false,
        ),
        Field::new(
            EmbeddingModelMetadataFieldName::Name.name(),
            DataType::Utf8,
            false,
        ),
        Field::new(
            EmbeddingModelMetadataFieldName::Author.name(),
            DataType::Utf8,
            false,
        ),
        Field::new(
            EmbeddingModelMetadataFieldName::Description.name(),
            DataType::Utf8,
            true,
        ),
        Field::new(
            EmbeddingModelMetadataFieldName::Url.name(),
            DataType::Utf8,
            false,
        ),
        Field::new(
            EmbeddingModelMetadataFieldName::AccessedAt.name(),
            DataType::Date64,
            false,
        ),
        Field::new(
            EmbeddingModelMetadataFieldName::Dimensions.name(),
            DataType::Int32,
            false,
        ),
        Field::new(
            EmbeddingModelMetadataFieldName::MaxInput.name(),
            DataType::Int32,
            false,
        ),
        Field::new(
            EmbeddingModelMetadataFieldName::PricingPer1kTokens.name(),
            DataType::Float32,
            false,
        ),
        Field::new(
            EmbeddingModelMetadataFieldName::PricingUpdateAt.name(),
            DataType::Date64,
            false,
        ),
    ]))
}

fn convert_model_metadata(
    sources: Vec<EmbeddingModelMetadata>,
) -> Box<dyn RecordBatchReader<Item = Result<RecordBatch, ArrowError>> + Send> {
    let batches = RecordBatchIterator::new(
        vec![RecordBatch::try_new(
            model_metadata_schema(),
            vec![
                Arc::new(StringArray::from_iter_values(
                    sources.iter().map(|data| data.id.to_string()),
                )),
                Arc::new(StringArray::from_iter_values(
                    sources.iter().map(|data| data.name.clone()),
                )),
                Arc::new(StringArray::from_iter_values(
                    sources.iter().map(|data| data.author.clone()),
                )),
                Arc::new(StringArray::from(
                    sources
                        .iter()
                        .map(|data| data.description.clone())
                        .collect::<Vec<Option<String>>>(),
                )),
                Arc::new(StringArray::from_iter_values(
                    sources.iter().map(|data| data.url.clone()),
                )),
                Arc::new(Date64Array::from_iter_values(
                    sources.iter().map(|(er)| er.accessed_at.timestamp()),
                )),
                Arc::new(Int32Array::from_iter_values(
                    sources.iter().map(|data| data.dimensions),
                )),
                Arc::new(Int32Array::from_iter_values(
                    sources.iter().map(|data| data.max_input),
                )),
                Arc::new(Float32Array::from_iter_values(
                    sources.iter().map(|data| data.pricing_per_1k_tokens),
                )),
                Arc::new(Date64Array::from_iter_values(
                    sources
                        .iter()
                        .map(|data| data.pricing_update_at.timestamp()),
                )),
            ],
        )
        .unwrap()]
        .into_iter()
        .map(Ok),
        model_metadata_schema(),
    );
    Box::new(batches)
}

fn convert_record_batch_to_model_metadata(
    record_batch: Vec<RecordBatch>,
) -> Vec<EmbeddingModelMetadata> {
    record_batch
        .iter()
        .flat_map(|batch| {
            (0..batch.num_rows())
                .map(|row| {
                    let id = batch
                        .column_by_name(EmbeddingModelMetadataFieldName::Id.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                        .unwrap()
                        .value(row);
                    let name = batch
                        .column_by_name(EmbeddingModelMetadataFieldName::Name.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                        .unwrap()
                        .value(row);
                    let author = batch
                        .column_by_name(EmbeddingModelMetadataFieldName::Author.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                        .unwrap()
                        .value(row);
                    let description = match batch
                        .column_by_name(EmbeddingModelMetadataFieldName::Description.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                    {
                        Some(array) => {
                            if array.is_null(row) {
                                None
                            } else {
                                Some(array.value(row))
                            }
                        }
                        None => None,
                    }
                    .map(|s| s.to_string());
                    let url = batch
                        .column_by_name(EmbeddingModelMetadataFieldName::Url.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                        .unwrap()
                        .value(row);
                    let accessed_at = batch
                        .column_by_name(EmbeddingModelMetadataFieldName::AccessedAt.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<Date64Array>()
                        .unwrap()
                        .value(row);
                    let dimensions = batch
                        .column_by_name(EmbeddingModelMetadataFieldName::Dimensions.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<Int32Array>()
                        .unwrap()
                        .value(row);
                    let max_input = batch
                        .column_by_name(EmbeddingModelMetadataFieldName::MaxInput.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<Int32Array>()
                        .unwrap()
                        .value(row);
                    let pricing_per_1k_tokens = batch
                        .column_by_name(EmbeddingModelMetadataFieldName::PricingPer1kTokens.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<Float32Array>()
                        .unwrap()
                        .value(row);
                    let pricing_update_at = batch
                        .column_by_name(EmbeddingModelMetadataFieldName::PricingUpdateAt.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<Date64Array>()
                        .unwrap()
                        .value(row);

                    EmbeddingModelMetadata {
                        id: Uuid::try_parse(id).unwrap(),
                        name: name.to_string(),
                        author: author.to_string(),
                        description: description.map(|s| s.to_string()),
                        url: url.to_string(),
                        accessed_at: utc_from_epoch(accessed_at),
                        dimensions,
                        max_input,
                        pricing_per_1k_tokens,
                        pricing_update_at: utc_from_epoch(pricing_update_at),
                    }
                })
                .collect::<Vec<EmbeddingModelMetadata>>()
        })
        .collect()
}

/// Schema for [TextEmbedded](tuo_core::core::messaging::content::TextEmbedded)
// Refactor using TextEmbeddedFieldName
fn text_embedded_schema(dimension: i32) -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new(TextEmbeddedFieldName::Id.name(), DataType::Utf8, false),
        Field::new(TextEmbeddedFieldName::Text.name(), DataType::Utf8, true),
        Field::new(TextEmbeddedFieldName::Hash.name(), DataType::Utf8, false),
        Field::new(
            TextEmbeddedFieldName::EmbeddingModel.name(),
            DataType::Utf8,
            false,
        ),
        Field::new(
            TextEmbeddedFieldName::CreatedAt.name(),
            DataType::Date64,
            false,
        ),
        Field::new(
            D_TABLE_COLUMN_NAME_VECTOR,
            DataType::FixedSizeList(
                Arc::new(Field::new("item", DataType::Float32, true)),
                dimension,
            ),
            true,
        ),
        Field::new(
            TextEmbeddedFieldName::EmbeddedAt.name(),
            DataType::Date64,
            false,
        ),
        Field::new(
            TextEmbeddedFieldName::UsedAt.name(),
            DataType::Date64,
            false,
        ),
        Field::new(
            TextEmbeddedFieldName::SourceType.name(),
            DataType::Utf8,
            false,
        ),
        Field::new(TextEmbeddedFieldName::SourceId.name(), DataType::Utf8, true),
    ]))
}

fn convert_texts_embedded(
    embedding_result: Vec<TextEmbedded>,
    dimension: i32,
) -> Box<dyn RecordBatchReader<Item = Result<RecordBatch, ArrowError>> + Send> {
    let sources = embedding_result;
    let batches = RecordBatchIterator::new(
        vec![RecordBatch::try_new(
            text_embedded_schema(dimension),
            vec![
                Arc::new(StringArray::from_iter_values(
                    sources.iter().map(|data| data.id.to_string()),
                )),
                Arc::new(StringArray::from(
                    sources
                        .iter()
                        .map(|data| data.text.clone())
                        .collect::<Vec<Option<String>>>(),
                )),
                Arc::new(StringArray::from_iter_values(
                    sources.iter().map(|data| data.hash.clone()),
                )),
                Arc::new(StringArray::from_iter_values(
                    sources.iter().map(|data| data.embedding_model.clone()),
                )),
                Arc::new(Date64Array::from_iter_values(
                    sources.iter().map(|(er)| er.created_at.timestamp()),
                )),
                Arc::new(
                    FixedSizeListArray::from_iter_primitive::<Float32Type, _, _>(
                        sources.iter().map(|data| {
                            Some(
                                data.embeddings
                                    .iter()
                                    .map(|v| Some(*v))
                                    .collect::<Vec<Option<f32>>>(),
                            )
                        }),
                        dimension,
                    ),
                ),
                Arc::new(Date64Array::from_iter_values(
                    sources.iter().map(|(er)| er.embedded_at.timestamp()),
                )),
                Arc::new(Date64Array::from_iter_values(
                    sources.iter().map(|(er)| er.used_at.timestamp()),
                )),
                Arc::new(StringArray::from_iter_values(
                    sources.iter().map(|data| data.source_type.as_ref()),
                )),
                Arc::new(StringArray::from(
                    sources
                        .iter()
                        .map(|data| data.source_id.map(|id| id.to_string()))
                        .collect::<Vec<Option<String>>>(),
                )),
            ],
        )
        .unwrap()]
        .into_iter()
        .map(Ok),
        text_embedded_schema(dimension),
    );
    Box::new(batches)
}

fn convert_record_batch_to_text_embedded(
    record_batch: Vec<RecordBatch>,
    dimension: i32,
) -> Vec<SimilarResult<TextEmbedded>> {
    record_batch
        .iter()
        .flat_map(|batch| {
            (0..batch.num_rows())
                .map(|row| {
                    let id = batch
                        .column_by_name(TextEmbeddedFieldName::Id.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                        .unwrap()
                        .value(row);

                    // optional text
                    let text = match batch
                        .column_by_name(TextEmbeddedFieldName::Text.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                    {
                        Some(array) => {
                            if array.is_null(row) {
                                None
                            } else {
                                Some(array.value(row))
                            }
                        }
                        None => None,
                    };
                    let hash = batch
                        .column_by_name(TextEmbeddedFieldName::Hash.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                        .unwrap()
                        .value(row);
                    let embed_model = batch
                        .column_by_name(TextEmbeddedFieldName::EmbeddingModel.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                        .unwrap()
                        .value(row);
                    let created_at = batch
                        .column_by_name(TextEmbeddedFieldName::CreatedAt.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<Date64Array>()
                        .unwrap()
                        .value(row);
                    let embeddings = match batch
                        .column_by_name(D_TABLE_COLUMN_NAME_VECTOR)
                        .unwrap()
                        .as_any()
                        .downcast_ref::<FixedSizeListArray>()
                    {
                        Some(array) => {
                            // use the first array of the vector array
                            let array: ArrayRef = array.value(row);
                            // specific to dimension size
                            let mut embeddings: Vec<f32> = Vec::with_capacity(dimension as usize);
                            for i in 0..dimension {
                                let value = array
                                    .as_any()
                                    .downcast_ref::<Float32Array>()
                                    .unwrap()
                                    .value(i as usize);

                                embeddings.push(value);
                            }
                            embeddings
                        }
                        None => {
                            vec![0.0; dimension as usize]
                        }
                    };
                    let embedded_at = batch
                        .column_by_name(TextEmbeddedFieldName::EmbeddedAt.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<Date64Array>()
                        .unwrap()
                        .value(row);
                    let used_at = batch
                        .column_by_name(TextEmbeddedFieldName::UsedAt.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<Date64Array>()
                        .unwrap()
                        .value(row);
                    let source_type = batch
                        .column_by_name(TextEmbeddedFieldName::SourceType.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                        .unwrap()
                        .value(row);
                    let source_id = match batch
                        .column_by_name(TextEmbeddedFieldName::SourceId.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                    {
                        Some(array) => {
                            if array.is_null(row) {
                                None
                            } else {
                                Some(array.value(row))
                            }
                        }
                        None => None,
                    };
                    let distance_column = batch.column_by_name("_distance");

                    let distance = match distance_column {
                        Some(column) => match column.as_any().downcast_ref::<Float32Array>() {
                            Some(array) => Some(array.value(row)),
                            None => None,
                        },
                        None => None,
                    }
                    .map(|d| d.abs());
                    let text_embedded = TextEmbedded {
                        id: Uuid::try_parse(id).unwrap(),
                        text: text.map(|s| s.to_string()),
                        hash: hash.to_string(),
                        embedding_model: embed_model.to_string(),
                        created_at: utc_from_epoch(created_at),
                        embeddings,
                        embedded_at: utc_from_epoch(embedded_at),
                        used_at: utc_from_epoch(used_at),
                        source_type: TextSourceType::from_str(source_type).unwrap(),
                        source_id: source_id.map(|id| Uuid::try_parse(id).unwrap()),
                    };
                    let search_result = SimilarResult {
                        data_id: text_embedded.id,
                        data: text_embedded,
                        distance: distance.unwrap_or(0.0),
                    };
                    search_result
                })
                .collect::<Vec<SimilarResult<TextEmbedded>>>()
        })
        .collect()
}

/// Schema for [Document](tuo_core::core::source::document::Document)
fn document_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new(DocumentFieldName::Id.name(), DataType::Utf8, false),
        Field::new(DocumentFieldName::IndexId.name(), DataType::Utf8, false),
        Field::new(DocumentFieldName::Name.name(), DataType::Utf8, false),
        Field::new(
            DocumentFieldName::DocumentType.name(),
            DataType::Utf8,
            false,
        ),
        Field::new(DocumentFieldName::RawContent.name(), DataType::Utf8, true),
        Field::new(DocumentFieldName::SourceType.name(), DataType::Utf8, false),
        Field::new(DocumentFieldName::SourceUri.name(), DataType::Utf8, false),
        Field::new(
            DocumentFieldName::SummaryTextId.name(),
            DataType::Utf8,
            true,
        ),
    ]))
}

fn convert_documents(
    document: Vec<tuo_core::core::source::document::Document>,
) -> Box<dyn RecordBatchReader<Item = Result<RecordBatch, ArrowError>> + Send> {
    let sources = document;
    let batches = RecordBatchIterator::new(
        vec![RecordBatch::try_new(
            document_schema(),
            vec![
                Arc::new(StringArray::from_iter_values(
                    sources.iter().map(|data| data.id.to_string()),
                )),
                Arc::new(StringArray::from_iter_values(
                    sources.iter().map(|data| data.index_id.to_string()),
                )),
                Arc::new(StringArray::from_iter_values(
                    sources.iter().map(|data| data.name.clone()),
                )),
                Arc::new(StringArray::from_iter_values(
                    sources.iter().map(|data| data.document_type.as_ref()),
                )),
                Arc::new(StringArray::from(
                    sources
                        .iter()
                        .map(|data| data.raw_content.clone())
                        .collect::<Vec<Option<String>>>(),
                )),
                Arc::new(StringArray::from_iter_values(
                    sources.iter().map(|data| data.source_type.as_ref()),
                )),
                Arc::new(StringArray::from_iter_values(
                    sources.iter().map(|data| data.source_uri.clone()),
                )),
                Arc::new(StringArray::from(
                    sources
                        .iter()
                        .map(|data| data.summary.as_ref().map(|summary| summary.id.to_string()))
                        .collect::<Vec<Option<String>>>(),
                )),
            ],
        )
        .unwrap()]
        .into_iter()
        .map(Ok),
        document_schema(),
    );
    Box::new(batches)
}

fn convert_record_batch_to_document(
    record_batch: Vec<RecordBatch>,
) -> Vec<tuo_core::core::source::document::Document> {
    record_batch
        .iter()
        .flat_map(|batch| {
            (0..batch.num_rows())
                .map(|row| {
                    let id = batch
                        .column_by_name(DocumentFieldName::Id.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                        .unwrap()
                        .value(row);
                    let index_id = batch
                        .column_by_name(DocumentFieldName::IndexId.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                        .unwrap()
                        .value(row);
                    let name = batch
                        .column_by_name(DocumentFieldName::Name.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                        .unwrap()
                        .value(row);
                    let document_type = batch
                        .column_by_name(DocumentFieldName::DocumentType.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                        .unwrap()
                        .value(row);
                    let content = match batch
                        .column_by_name(DocumentFieldName::RawContent.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                    {
                        Some(array) => {
                            if array.is_null(row) {
                                None
                            } else {
                                Some(array.value(row))
                            }
                        }
                        None => None,
                    };
                    let source_type = batch
                        .column_by_name(DocumentFieldName::SourceType.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                        .unwrap()
                        .value(row);
                    let source_uri = batch
                        .column_by_name(DocumentFieldName::SourceUri.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                        .unwrap()
                        .value(row);
                    let summary_text_id = match batch
                        .column_by_name(DocumentFieldName::SummaryTextId.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                    {
                        Some(array) => {
                            if array.is_null(row) {
                                None
                            } else {
                                Some(array.value(row))
                            }
                        }
                        None => None,
                    };

                    tuo_core::core::source::document::Document {
                        id: Uuid::try_parse(id).unwrap(),
                        index_id: Uuid::try_parse(index_id).unwrap(),
                        name: name.to_string(),
                        document_type: DocumentType::from_str(document_type).unwrap(),
                        raw_content: content.map(|content| content.to_string()),
                        source_type: DocumentSourceType::from_str(source_type).unwrap(),
                        source_uri: source_uri.to_string(),
                        summary: None,
                        summary_text_id: summary_text_id.map(|id| Uuid::try_parse(id).unwrap()),
                    }
                })
                .collect::<Vec<tuo_core::core::source::document::Document>>()
        })
        .collect()
}

/// Schema for [Section](tuo_core::core::source::section::Section)
fn section_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new(
            String::from(SectionFieldName::Id.name()),
            DataType::Utf8,
            false,
        ),
        Field::new(
            String::from(SectionFieldName::IndexId.name()),
            DataType::Utf8,
            false,
        ),
        Field::new(
            String::from(SectionFieldName::DocumentId.name()),
            DataType::Utf8,
            false,
        ),
        Field::new(
            String::from(SectionFieldName::Name.name()),
            DataType::Utf8,
            false,
        ),
        Field::new(
            String::from(SectionFieldName::SectionOrder.name()),
            DataType::Int32,
            false,
        ),
        Field::new(
            String::from(SectionFieldName::SectionLevel.name()),
            DataType::Int32,
            false,
        ),
        Field::new(
            String::from(SectionFieldName::Content.name()),
            DataType::Utf8,
            true,
        ),
        Field::new(
            String::from(SectionFieldName::StartCharIndex.name()),
            DataType::Int32,
            true,
        ),
        Field::new(
            String::from(SectionFieldName::EndCharIndex.name()),
            DataType::Int32,
            true,
        ),
        Field::new(
            String::from(SectionFieldName::SummaryTextId.name()),
            DataType::Utf8,
            true,
        ),
    ]))
}

fn convert_sections(
    section: Vec<tuo_core::core::source::section::Section>,
) -> Box<dyn RecordBatchReader<Item = Result<RecordBatch, ArrowError>> + Send> {
    let sources = section;
    let batches = RecordBatchIterator::new(
        vec![RecordBatch::try_new(
            section_schema(),
            vec![
                Arc::new(StringArray::from_iter_values(
                    sources.iter().map(|data| data.id.to_string()),
                )),
                Arc::new(StringArray::from_iter_values(
                    sources.iter().map(|data| data.index_id.to_string()),
                )),
                Arc::new(StringArray::from_iter_values(
                    sources.iter().map(|data| data.document_id.to_string()),
                )),
                Arc::new(StringArray::from_iter_values(
                    sources.iter().map(|data| data.name.clone()),
                )),
                Arc::new(Int32Array::from_iter_values(
                    sources.iter().map(|data| data.section_order as i32),
                )),
                Arc::new(Int32Array::from_iter_values(
                    sources.iter().map(|data| data.section_level as i32),
                )),
                Arc::new(StringArray::from(
                    sources
                        .iter()
                        .map(|data| data.content.clone())
                        .collect::<Vec<Option<String>>>(),
                )),
                Arc::new(Int32Array::from(
                    sources
                        .iter()
                        .map(|data| data.start_char_index.map(|index| index as i32))
                        .collect::<Vec<Option<i32>>>(),
                )),
                Arc::new(Int32Array::from(
                    sources
                        .iter()
                        .map(|data| data.end_char_index.map(|index| index as i32))
                        .collect::<Vec<Option<i32>>>(),
                )),
                Arc::new(StringArray::from(
                    sources
                        .iter()
                        .map(|data| data.summary.as_ref().map(|summary| summary.id.to_string()))
                        .collect::<Vec<Option<String>>>(),
                )),
            ],
        )
        .unwrap()]
        .into_iter()
        .map(Ok),
        section_schema(),
    );
    Box::new(batches)
}

fn convert_record_batch_to_section(
    record_batch: Vec<RecordBatch>,
) -> Vec<tuo_core::core::source::section::Section> {
    record_batch
        .iter()
        .flat_map(|batch| {
            (0..batch.num_rows())
                .map(|row| {
                    let id = batch
                        .column_by_name(SectionFieldName::Id.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                        .unwrap()
                        .value(row);
                    let index_id = batch
                        .column_by_name(SectionFieldName::IndexId.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                        .unwrap()
                        .value(row);
                    let document_id = batch
                        .column_by_name(SectionFieldName::DocumentId.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                        .unwrap()
                        .value(row);
                    let name = batch
                        .column_by_name(SectionFieldName::Name.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                        .unwrap()
                        .value(row);
                    let section_order = batch
                        .column_by_name(SectionFieldName::SectionOrder.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<Int32Array>()
                        .unwrap()
                        .value(row);
                    let section_level = batch
                        .column_by_name(SectionFieldName::SectionLevel.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<Int32Array>()
                        .unwrap()
                        .value(row);
                    let content = match batch
                        .column_by_name(SectionFieldName::Content.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                    {
                        Some(array) => {
                            if array.is_null(row) {
                                None
                            } else {
                                Some(array.value(row))
                            }
                        }
                        None => None,
                    };
                    let start_char_index = match batch
                        .column_by_name(SectionFieldName::StartCharIndex.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<Int32Array>()
                    {
                        Some(array) => {
                            if array.is_null(row) {
                                None
                            } else {
                                Some(array.value(row))
                            }
                        }
                        None => None,
                    };
                    let end_char_index = match batch
                        .column_by_name(SectionFieldName::EndCharIndex.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<Int32Array>()
                    {
                        Some(array) => {
                            if array.is_null(row) {
                                None
                            } else {
                                Some(array.value(row))
                            }
                        }
                        None => None,
                    };
                    let summary_text_id = match batch
                        .column_by_name(SectionFieldName::SummaryTextId.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                    {
                        Some(array) => {
                            if array.is_null(row) {
                                None
                            } else {
                                Some(array.value(row))
                            }
                        }
                        None => None,
                    };

                    tuo_core::core::source::section::Section {
                        id: Uuid::try_parse(id).unwrap(),
                        index_id: Uuid::try_parse(index_id).unwrap(),
                        document_id: Uuid::try_parse(document_id).unwrap(),
                        name: name.to_string(),
                        section_order,
                        section_level,
                        content: content.map(|content| content.to_string()),
                        start_char_index,
                        end_char_index,
                        summary: None,
                        summary_text_id: summary_text_id.map(|id| Uuid::try_parse(id).unwrap()),
                    }
                })
                .collect::<Vec<tuo_core::core::source::section::Section>>()
        })
        .collect()
}

/// Schema for [Node](tuo_core::core::source::node::Node)
fn node_schema(dimension: i32) -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new(NodeFieldName::Id.name(), DataType::Utf8, false),
        Field::new(NodeFieldName::IndexId.name(), DataType::Utf8, false),
        Field::new(NodeFieldName::DocumentId.name(), DataType::Utf8, false),
        Field::new(NodeFieldName::SectionId.name(), DataType::Utf8, false),
        Field::new(NodeFieldName::Content.name(), DataType::Utf8, false),
        Field::new(
            NodeFieldName::ContentEmbeddingsId.name(),
            DataType::Utf8,
            true,
        ),
        Field::new(
            NodeFieldName::ContentEmbeddedAt.name(),
            DataType::Date64,
            true,
        ),
        Field::new(NodeFieldName::ContentType.name(), DataType::Utf8, false),
        Field::new(NodeFieldName::Tokens.name(), DataType::Int32, false),
        Field::new(NodeFieldName::Index.name(), DataType::Int32, false),
        Field::new(NodeFieldName::StartCharIndex.name(), DataType::Int32, false),
        Field::new(NodeFieldName::EndCharIndex.name(), DataType::Int32, false),
    ]))
}

fn convert_nodes(
    node: Vec<tuo_core::core::source::node::Node>,
    dimension: i32,
) -> Box<dyn RecordBatchReader<Item = Result<RecordBatch, ArrowError>> + Send> {
    let sources = node;
    let batches = RecordBatchIterator::new(
        vec![RecordBatch::try_new(
            node_schema(dimension),
            vec![
                Arc::new(StringArray::from_iter_values(
                    sources.iter().map(|data| data.id.to_string()),
                )),
                Arc::new(StringArray::from_iter_values(
                    sources.iter().map(|data| data.index_id.to_string()),
                )),
                Arc::new(StringArray::from_iter_values(
                    sources.iter().map(|data| data.document_id.to_string()),
                )),
                Arc::new(StringArray::from_iter_values(
                    sources.iter().map(|data| data.section_id.to_string()),
                )),
                Arc::new(StringArray::from_iter_values(
                    sources.iter().map(|data| data.content.clone()),
                )),
                Arc::new(StringArray::from(
                    sources
                        .iter()
                        .map(|data| data.content_embeddings_id.map(|id| id.to_string()))
                        .collect::<Vec<Option<String>>>(),
                )),
                Arc::new(Date64Array::from(
                    sources
                        .iter()
                        .map(|data| data.content_embedded_at.map(|date| date.timestamp()))
                        .collect::<Vec<Option<i64>>>(),
                )),
                Arc::new(StringArray::from_iter_values(
                    sources.iter().map(|data| data.content_type.as_ref()),
                )),
                Arc::new(Int32Array::from_iter_values(
                    sources.iter().map(|data| data.tokens),
                )),
                Arc::new(Int32Array::from_iter_values(
                    sources.iter().map(|data| data.index),
                )),
                Arc::new(Int32Array::from_iter_values(
                    sources.iter().map(|data| data.start_char_index as i32),
                )),
                Arc::new(Int32Array::from_iter_values(
                    sources.iter().map(|data| data.end_char_index as i32),
                )),
            ],
        )
        .unwrap()]
        .into_iter()
        .map(Ok),
        node_schema(dimension),
    );
    Box::new(batches)
}

fn convert_record_batch_to_node(
    record_batch: Vec<RecordBatch>,
) -> Vec<tuo_core::core::source::node::Node> {
    record_batch
        .iter()
        .flat_map(|batch| {
            (0..batch.num_rows())
                .map(|row| {
                    let id = batch
                        .column_by_name(NodeFieldName::Id.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                        .unwrap()
                        .value(row);
                    let index_id = batch
                        .column_by_name(NodeFieldName::IndexId.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                        .unwrap()
                        .value(row);
                    let document_id = batch
                        .column_by_name(NodeFieldName::DocumentId.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                        .unwrap()
                        .value(row);
                    let section_id = batch
                        .column_by_name(NodeFieldName::SectionId.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                        .unwrap()
                        .value(row);
                    let content = batch
                        .column_by_name(NodeFieldName::Content.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                        .unwrap()
                        .value(row);
                    let content_embeddings_id = match batch
                        .column_by_name(NodeFieldName::ContentEmbeddingsId.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                    {
                        Some(array) => {
                            if array.is_null(row) {
                                None
                            } else {
                                Some(array.value(row))
                            }
                        }
                        None => None,
                    };
                    let content_embedded_at = batch
                        .column_by_name(NodeFieldName::ContentEmbeddedAt.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<Date64Array>()
                        .unwrap()
                        .value(row);
                    let content_type = batch
                        .column_by_name(NodeFieldName::ContentType.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<StringArray>()
                        .unwrap()
                        .value(row);
                    let tokens = batch
                        .column_by_name(NodeFieldName::Tokens.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<Int32Array>()
                        .unwrap()
                        .value(row);
                    let index = batch
                        .column_by_name(NodeFieldName::Index.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<Int32Array>()
                        .unwrap()
                        .value(row);

                    let start_char_index = batch
                        .column_by_name(NodeFieldName::StartCharIndex.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<Int32Array>()
                        .unwrap()
                        .value(row);
                    let end_char_index = batch
                        .column_by_name(NodeFieldName::EndCharIndex.name())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<Int32Array>()
                        .unwrap()
                        .value(row);

                    tuo_core::core::source::node::Node {
                        id: Uuid::try_parse(id).unwrap(),
                        index_id: Uuid::try_parse(index_id).unwrap(),
                        document_id: Uuid::try_parse(document_id).unwrap(),
                        section_id: Uuid::try_parse(section_id).unwrap(),
                        content: content.to_string(),
                        content_embeddings_id: content_embeddings_id
                            .map(|id| Uuid::try_parse(id).unwrap()),
                        content_embeddings: None,
                        content_embedded_at: Some(utc_from_epoch(content_embedded_at)),
                        tokens,
                        content_type: ContentType::from_str(content_type).unwrap(),
                        index,
                        start_char_index,
                        end_char_index,
                    }
                })
                .collect::<Vec<tuo_core::core::source::node::Node>>()
        })
        .collect()
}
