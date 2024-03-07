use field_types::{FieldName, FieldType};
use strum::{AsRefStr, EnumString};
use typed_builder::TypedBuilder;
use uuid::Uuid;

use tuo_utils::datetime::timestamp::now;
use tuo_utils::hash::hash_str::hash_str;

use crate::embedding::embeddings::Embeddings;
use crate::types::date_time::TuoDateTime;

/// Specify how this text is used by the source, i.e. as a user query, a summary document, a summary section, a summary node, or a node content etc.
#[derive(Default, Debug, Clone, AsRefStr, EnumString)]
pub enum TextSourceType {
    #[default]
    UserQuery,
    SummaryDocument,
    SummarySection,
    SummaryNode,
    NodeContent,
}

#[derive(Default, Debug, Clone)]
pub struct TextInput {
    pub text: String,
    pub source_type: TextSourceType,
    pub source_id: Option<Uuid>,
}

// impl Into trait for TextInput, as user query
impl From<&str> for TextInput {
    fn from(text: &str) -> Self {
        Self {
            text: text.to_string(),
            source_type: TextSourceType::UserQuery,
            source_id: None,
        }
    }
}

impl TextInput {
    pub fn from_user_str(text: &str) -> Self {
        Self {
            text: text.to_string(),
            source_type: TextSourceType::UserQuery,
            source_id: None,
        }
    }

    pub fn from_node_text(text: &str, node_id: Uuid) -> Self {
        Self {
            text: text.to_string(),
            source_type: TextSourceType::NodeContent,
            source_id: Some(node_id),
        }
    }
    pub fn to_embedded(&self, embeddings: Embeddings, opt: &TextEmbeddingOptions) -> TextEmbedded {
        TextEmbedded::new(
            self.text.as_str(),
            embeddings,
            self.source_type.clone(),
            self.source_id,
            Some(opt.clone()),
        )
    }
}

#[derive(TypedBuilder, Clone)]
pub struct TextEmbeddingOptions {
    /// Whether save the original text in the [embedded text](TextEmbedded)
    #[builder(default = false)]
    pub save_text: bool,
}

#[derive(Default, Debug, Clone, FieldName)]
pub struct TextEmbedded {
    pub id: Uuid,
    // Whether the text is saved is controlled by the TextEmbeddingOptions
    pub text: Option<String>,
    pub hash: String,
    pub embedding_model: String,
    /// generated_at timestamp
    pub created_at: TuoDateTime,
    pub embeddings: Vec<f32>,
    pub embedded_at: TuoDateTime,
    pub used_at: TuoDateTime,
    pub source_type: TextSourceType,
    pub source_id: Option<Uuid>,
}

impl TextEmbedded {
    pub fn new(
        text: &str,
        embeddings: Embeddings,
        source_type: TextSourceType,
        source_id: Option<Uuid>,
        opt: Option<TextEmbeddingOptions>,
    ) -> Self {
        let opt = opt.unwrap_or(TextEmbeddingOptions::builder().build());
        Self {
            id: Uuid::new_v4(),
            hash: hash_str(text),
            text: opt.save_text.then(|| text.to_string()),
            embedding_model: embeddings.model,
            embeddings: embeddings.vector,
            embedded_at: embeddings.embedded_at,
            created_at: now(),
            used_at: now(),
            source_type,
            source_id,
        }
    }

    pub fn new_query_text(text: &str, embeddings: Embeddings) -> Self {
        Self::new(
            text,
            embeddings,
            TextSourceType::UserQuery,
            None,
            Some(TextEmbeddingOptions::builder().save_text(true).build()),
        )
    }
}
