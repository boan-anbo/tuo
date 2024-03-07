use uuid::Uuid;

pub struct SimilarResult<Data> {
    pub distance: f32,
    pub data: Data,
    pub data_id: Uuid,
}