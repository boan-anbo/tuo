use field_types::FieldName;
use strum::{AsRefStr, EnumString};
use typed_builder::TypedBuilder;
use uuid::Uuid;

use tuo_shared::types::return_type::TuoResult;

use crate::core::messaging::content::TextEmbedded;
use crate::types::date_time::TuoDateTime;
use crate::utility::token::{count_tokens, TokenUtility};

#[derive(Default, Debug, Clone, EnumString, AsRefStr)]
pub enum ContentType {
    #[default]
    Text,
    Image,
    // Corresponds to Binary in ContentData
    Video,
    // Corresponds to Binary in ContentData
    Audio,
    // Corresponds to Binary in ContentData
    File,
    // Could correspond to Binary or Text in ContentData
    Table, // Custom data structure or Text if serialized
}

/// # Node
///
/// - A node is a part of a section.
/// - It's the first-class citizen of Tuo.
/// - It's the unit and source of embedding.
/// - Its unit is arbitrary--it can be a paragraph, a sentence, a photo, a video, a table, etc.
/// - Its unit _should_ always be meaningful for relevance retrieval.
#[derive(Debug, Clone, FieldName, TypedBuilder)]
pub struct Node {
    #[builder(default = Uuid::new_v4())]
    pub id: Uuid,
    pub index_id: Uuid,
    pub document_id: Uuid,
    pub section_id: Uuid,
    pub content: String,
    #[builder(default = ContentType::Text)]
    pub content_type: ContentType,
    #[builder(default = None)]
    pub content_embeddings_id: Option<Uuid>,
    // Content embeddings needs to be assembled
    #[builder(default = None)]
    pub content_embeddings: Option<TextEmbedded>,
    #[builder(default = None)]
    pub content_embedded_at: Option<TuoDateTime>,
    pub tokens: i32,
    /// # The indexing of the node in the source.
    ///
    /// - The indexing is 0-based.
    ///
    /// - The previous node's indexing is the current node's `indexing - 1`.
    /// - The next node's indexing is the current node's `indexing + 1`.
    pub index: i32,

    /// # The start character indexing of the node in the section.
    ///
    /// - The indexing is 0-based.
    #[builder(default = 0)]
    pub start_char_index: i32,

    /// # The end character indexing of the node in the section.
    ///
    /// - The indexing is 0-based.
    #[builder(default = 0)]
    pub end_char_index: i32,
}

/// NodeConvertTrait
///
/// This trait is used to convert a source to a node and vice versa.
pub trait NodeConvertTrait<SOURCE>: Send + Sync {
    fn input_to_node(&self, input: &SOURCE) -> TuoResult<Node>;
    fn node_to_input(&self, node: &Node) -> TuoResult<SOURCE>;
}

pub trait NodeRelationTrait {
    fn merge_embedded_text(&mut self, text: &TextEmbedded);
}

impl NodeRelationTrait for Node {
    fn merge_embedded_text(&mut self, text: &TextEmbedded) {
        self.content_embeddings_id = Some(text.id);
        self.content_embedded_at = Some(text.embedded_at);
        self.content_embeddings = Some(text.clone());
    }
}

impl TokenUtility for Node {
    fn count_tokens(&self) -> usize {
        count_tokens(&self.content)
    }
}

impl TokenUtility for Vec<Node> {
    fn count_tokens(&self) -> usize {
        self.iter().map(|node| node.count_tokens()).sum()
    }
}
