use typed_builder::TypedBuilder;
use crate::types::date_time::TuoDateTime;
use tuo_utils::datetime::timestamp::now;

#[derive(Debug, Clone, TypedBuilder)]
pub struct Embeddings {
    pub vector: Vec<f32>,
    pub model: String,
    #[builder(default = now(), setter(skip))]
    pub embedded_at: TuoDateTime,
}
