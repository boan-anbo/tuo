use std::collections::HashMap;
use field_types::FieldName;

use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::model::model_metadata::EmbeddingModelMetadata;
use crate::types::date_time::TuoDateTime;
use tuo_utils::datetime::timestamp::now;

#[derive(Debug, Clone, FieldName, TypedBuilder)]
pub struct StoreMetadata {
    #[builder(default = Uuid::new_v4(), setter(skip))]
    pub id: Uuid,
    pub name: String,
    pub uri: String,
    #[builder(default = now(), setter(skip))]
    pub created_at: TuoDateTime,
    /// Extra info provided about the stores
    pub model: Option<EmbeddingModelMetadata>,
    pub model_id: Option<Uuid>,

}


