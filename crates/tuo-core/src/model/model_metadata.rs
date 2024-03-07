use std::collections::HashMap;
use field_types::FieldName;
use strum::{AsRefStr, EnumString};
use typed_builder::TypedBuilder;
use uuid::Uuid;
use crate::types::date_time::TuoDateTime;
use tuo_utils::datetime::timestamp::now;
use crate::embedding::embedder::EmbedderTrait;

pub trait EmbeddingModelMetadataTrait<T> {
    type Embedder: EmbedderTrait;
    type Config;
    fn get_embedder(&self, opt: Option<Self::Config>) -> Self::Embedder;
    fn get_embedding_model(&self) -> EmbeddingModelMetadata;
    fn from_model_name(name: &str) -> Option<T>;
    fn get_model_name(&self) -> String;
}
#[derive(Debug, Clone, FieldName, TypedBuilder)]
pub struct EmbeddingModelMetadata {
    #[builder(default = Uuid::new_v4(), setter(skip))]
    pub id: Uuid,
    pub name: String,
    pub author: String,
    #[builder(default = None, setter(skip))]
    pub description: Option<String>,
    pub url: String,
    #[builder(default = now(), setter(skip))]
    pub accessed_at: TuoDateTime,
    pub dimensions: i32,
    pub max_input: i32,
    /// The pricing per 1k tokens. Unit: USD
    pub pricing_per_1k_tokens: f32,
    #[builder(default = now(), setter(skip))]
    pub pricing_update_at: TuoDateTime,
}

#[derive(Debug, Clone, FieldName, TypedBuilder)]
pub struct ChatModelMetadata {
    #[builder(default = Uuid::new_v4(), setter(skip))]
    pub id: Uuid,
    pub name: String,
    pub author: String,
    #[builder(default = None, setter(skip))]
    pub description: Option<String>,
    pub url: String,
    #[builder(default = now(), setter(skip))]
    pub accessed_at: TuoDateTime,
    pub context_window: i32,
    pub dimensions: i32,
    pub pricing_per_1k_tokens_input: f32,
    pub pricing_per_1k_tokens_output: f32,
    #[builder(default = now(), setter(skip))]
    pub pricing_update_at: TuoDateTime,
}
