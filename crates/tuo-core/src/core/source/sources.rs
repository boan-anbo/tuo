use strum::{AsRefStr, EnumString};
use uuid::Uuid;

use tuo_shared::consts::defaults::{
    D_TABLE_NAME_DOCUMENTS, D_TABLE_NAME_INDEX_METADATA, D_TABLE_NAME_MODELS_METADATA,
    D_TABLE_NAME_NODES, D_TABLE_NAME_SECTIONS, D_TABLE_NAME_STORE_METADATA,
    D_TABLE_NAME_TEXT_EMBEDDED,
};

use crate::core::indexing::index_metadata::IndexMetadata;
use crate::core::messaging::content::TextEmbedded;
use crate::core::source::document::Document;
use crate::core::source::node::Node;
use crate::core::source::section::Section;
use crate::model::model_metadata::EmbeddingModelMetadata;
use crate::storage::store_metadata::StoreMetadata;

pub trait SourceTableName {
    fn table_name(&self) -> String;
}

pub trait SourceTypeTrait {
    fn source_type(&self) -> SourceType;
}

#[derive(Debug, Clone, EnumString, AsRefStr)]
pub enum SourceType {
    StoreMetadata,
    ModelMetadata,
    IndexMetadata,
    TextEmbedded,
    Document,
    Section,
    Node,
}

impl SourceTableName for SourceType {
    fn table_name(&self) -> String {
        match self {
            SourceType::StoreMetadata => D_TABLE_NAME_STORE_METADATA,
            SourceType::ModelMetadata => D_TABLE_NAME_MODELS_METADATA,
            SourceType::IndexMetadata => D_TABLE_NAME_INDEX_METADATA,
            SourceType::TextEmbedded => D_TABLE_NAME_TEXT_EMBEDDED,
            SourceType::Document => D_TABLE_NAME_DOCUMENTS,
            SourceType::Section => D_TABLE_NAME_SECTIONS,
            SourceType::Node => D_TABLE_NAME_NODES,
        }
        .to_string()
    }
}

/// # Sources
#[derive(Debug)]
pub enum SourceData {
    StoreMetadata(Vec<StoreMetadata>),
    ModelMetadata(Vec<EmbeddingModelMetadata>),
    IndexMetadata(Vec<IndexMetadata>),
    TextEmbedded(Vec<TextEmbedded>),
    Document(Vec<Document>),
    Section(Vec<Section>),
    Node(Vec<Node>),
}

impl SourceData {
    pub fn get_ids(&self) -> Vec<Uuid> {
        match self {
            SourceData::StoreMetadata(data) => data.iter().map(|x| x.id).collect(),
            SourceData::ModelMetadata(data) => data.iter().map(|x| x.id).collect(),
            SourceData::IndexMetadata(data) => data.iter().map(|x| x.id).collect(),
            SourceData::TextEmbedded(data) => data.iter().map(|x| x.id).collect(),
            SourceData::Document(data) => data.iter().map(|x| x.id).collect(),
            SourceData::Section(data) => data.iter().map(|x| x.id).collect(),
            SourceData::Node(data) => data.iter().map(|x| x.id).collect(),
        }
    }
}

// First, declare the macro.
macro_rules! impl_getter {
    ($fn_name:ident, $variant:ident, $ret_type:ty) => {
        pub fn $fn_name(&self) -> Option<$ret_type> {
            if let SourceData::$variant(data) = self {
                Some(data.clone())
            } else {
                None
            }
        }
    };
}

impl SourceData {
    impl_getter!(get_store_metadata, StoreMetadata, Vec<StoreMetadata>);
    impl_getter!(
        get_model_metadata,
        ModelMetadata,
        Vec<EmbeddingModelMetadata>
    );
    impl_getter!(get_index_metadata, IndexMetadata, Vec<IndexMetadata>);
    impl_getter!(get_text_embedded, TextEmbedded, Vec<TextEmbedded>);
    impl_getter!(get_document, Document, Vec<Document>);
    impl_getter!(get_section, Section, Vec<Section>);
    impl_getter!(get_node, Node, Vec<Node>);
}
impl SourceTypeTrait for SourceData {
    fn source_type(&self) -> SourceType {
        match self {
            SourceData::StoreMetadata(_) => SourceType::StoreMetadata,
            SourceData::ModelMetadata(_) => SourceType::ModelMetadata,
            SourceData::IndexMetadata(_) => SourceType::IndexMetadata,
            SourceData::TextEmbedded(_) => SourceType::TextEmbedded,
            SourceData::Document(_) => SourceType::Document,
            SourceData::Section(_) => SourceType::Section,
            SourceData::Node(_) => SourceType::Node,
        }
    }
}

impl SourceTableName for SourceData {
    fn table_name(&self) -> String {
        self.source_type().table_name()
    }
}

/// # Source Input data
///
/// This is the input data for the sources specific to a store implementation, e.g. LanceDb uses RecordBatch as the input data.
///
/// This should be used for user-defined input conversion towards the standard SourceData.
pub enum SourceInputData<SOURCE> {
    StoreMetadata(Vec<SOURCE>),
    ModelMetadata(Vec<SOURCE>),
    IndexMetadata(Vec<SOURCE>),
    TextEmbedded(Vec<SOURCE>),
    Document(Vec<SOURCE>),
    Section(Vec<SOURCE>),
    Node(Vec<SOURCE>),
}


impl<SOURCE> SourceInputData<SOURCE> {
    pub fn from_data(data: Vec<SOURCE>, source_type: &SourceType) -> Self {
        match source_type {
            SourceType::StoreMetadata => SourceInputData::StoreMetadata(data),
            SourceType::ModelMetadata => SourceInputData::ModelMetadata(data),
            SourceType::IndexMetadata => SourceInputData::IndexMetadata(data),
            SourceType::TextEmbedded => SourceInputData::TextEmbedded(data),
            SourceType::Document => SourceInputData::Document(data),
            SourceType::Section => SourceInputData::Section(data),
            SourceType::Node => SourceInputData::Node(data),
        }
    }
}

impl<SOURCE> SourceTypeTrait for SourceInputData<SOURCE> {
    fn source_type(&self) -> SourceType {
        match self {
            SourceInputData::StoreMetadata(_) => SourceType::StoreMetadata,
            SourceInputData::ModelMetadata(_) => SourceType::ModelMetadata,
            SourceInputData::IndexMetadata(_) => SourceType::IndexMetadata,
            SourceInputData::TextEmbedded(_) => SourceType::TextEmbedded,
            SourceInputData::Document(_) => SourceType::Document,
            SourceInputData::Section(_) => SourceType::Section,
            SourceInputData::Node(_) => SourceType::Node,
        }
    }
}

impl SourceTableName for SourceInputData<SourceData> {
    fn table_name(&self) -> String {
        self.source_type().table_name()
    }
}

pub enum SourcesId {
    StoreMetadataId(Uuid),
    ModelMetadataId(Uuid),
    IndexMetadataId(Uuid),
    TextEmbeddedId(Uuid),
    DocumentId(Uuid),
    SectionId(Uuid),
    NodeId(Uuid),
}

impl SourcesId {
    pub fn get_id(&self) -> Uuid {
        match self {
            SourcesId::StoreMetadataId(id) => *id,
            SourcesId::ModelMetadataId(id) => *id,
            SourcesId::IndexMetadataId(id) => *id,
            SourcesId::TextEmbeddedId(id) => *id,
            SourcesId::DocumentId(id) => *id,
            SourcesId::SectionId(id) => *id,
            SourcesId::NodeId(id) => *id,
        }
    }

    pub fn get_id_str(&self) -> String {
        self.get_id().to_string()
    }
}

impl SourceTypeTrait for SourcesId {
    fn source_type(&self) -> SourceType {
        match self {
            SourcesId::StoreMetadataId(_) => SourceType::StoreMetadata,
            SourcesId::ModelMetadataId(_) => SourceType::ModelMetadata,
            SourcesId::IndexMetadataId(_) => SourceType::IndexMetadata,
            SourcesId::TextEmbeddedId(_) => SourceType::TextEmbedded,
            SourcesId::DocumentId(_) => SourceType::Document,
            SourcesId::SectionId(_) => SourceType::Section,
            SourcesId::NodeId(_) => SourceType::Node,
        }
    }
}

impl SourceTableName for SourcesId {
    fn table_name(&self) -> String {
        self.source_type().table_name()
    }
}
