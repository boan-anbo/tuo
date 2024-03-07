use field_types::FieldName;
use typed_builder::TypedBuilder;
use uuid::Uuid;
use crate::types::date_time::TuoDateTime;
use tuo_utils::datetime::timestamp::now;
/// Index struct
#[derive(Default, Debug, Clone, TypedBuilder, FieldName)]
pub struct IndexMetadata {
    #[builder(default = Uuid::new_v4())]
    pub id: Uuid,
    /// The name of the actual table/collection/index name
    ///
    /// This must be a legal table/collection/index name for the database.
    ///
    /// Preferably, this should be a lower-cased, snake_case name, e.g. `my_index`.
    pub name: String,
    #[builder(default = None)]
    pub description: Option<String>,
    #[builder(default = 0)]
    pub document_count: i32,
    #[builder(default = now())]
    pub created_at: TuoDateTime,
    #[builder(default = now())]
    pub updated_at: TuoDateTime,
}

