use uuid::Uuid;

pub enum ContentType {
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

pub enum ContentData {
    Text(String),
    Binary(Vec<u8>),
    // Used for Image, Video, Audio, and potentially File
    Table(Vec<Vec<String>>), // Structured data, corresponding to Table in ContentType
}


/// # Node
///
/// - A node is a part of a section.
/// - It's the first-class citizen of Tuo.
/// - It's the unit and source of embedding.
/// - Its unit is arbitrary--it can be a paragraph, a sentence, a photo, a video, a table, etc.
/// - Its unit _should_ always be meaningful for relevance retrieval.
pub struct Node {
    pub id: Uuid,
    pub document_id: Uuid,
    pub section_id: Uuid,
    pub content_data: ContentData,
    pub content_type: ContentType,
    /// # The index of the node in the document.
    ///
    /// - The index is 0-based.
    ///
    /// - The previous node's index is the current node's `index - 1`.
    /// - The next node's index is the current node's `index + 1`.
    pub index: i32,

    /// # The start character index of the node in the section.
    ///
    /// - The index is 0-based.
    pub start_char_index: i32,

    /// # The end character index of the node in the section.
    ///
    /// - The index is 0-based.
    pub end_char_index: i32,
}