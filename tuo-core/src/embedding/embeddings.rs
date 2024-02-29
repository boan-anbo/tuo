use crate::types::date_time::TuoDateTime;

#[derive(Debug)]
pub struct Embeddings {
    pub vector: Vec<f32>,
    pub model: String,
    pub embedded_at: TuoDateTime,
}
