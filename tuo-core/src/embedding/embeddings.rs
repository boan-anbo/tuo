use crate::types::date_time::TuoDateTime;

pub struct Embeddings {
    pub vector: Vec<f32>,
    pub model: String,
    pub embedded_at: TuoDateTime,
}
