use uuid::Uuid;
use crate::core::generation::generated_content::GeneratedContent;

pub struct Section {
    id: Uuid,
    document_id: Uuid,
    name: String,
    /// # The section number in the source.
    ///
    /// - The indexing is 0-based.
    ///
    /// - If there is no pagination, this value is always 0.
    ///
    /// - If there is only one section, this value is always 0.
    ///
    /// ## Why not call it `section_number`?
    ///
    /// Because not all materials use sections. For example, a structured Markdown has no sections but has sections, and sections have orders and levels
    pub section_order: i32,

    /// # Section level in the source.
    ///
    /// - The indexing is 0-based.
    ///
    /// - The highest is 0.
    ///
    /// - The default value is 0.
    ///
    /// ## Example
    ///
    /// - `#` in Markdown is level 0.
    /// - `sections` in a book is level 0.
    /// - Single section in a book which has numbering schema (Romans for frontmatter for example) for non-content sections is level 1.
    pub section_level: i32,

    /// # Content of the section
    ///
    /// The content of the section.
    ///
    /// ## Why is it an Option?
    /// In actuality a section must contain something, even an empty string, because section content is the basis and source for node content.
    ///
    /// But in implementation, it's possible that we do not give section content after we gave the section content to the nodes in order to save memory, for example.
    pub content: Option<String>,

    /// # The start character indexing of the node in the source.
    ///
    /// - The indexing is 0-based.
    pub start_char_index: Option<i32>,

    /// # The end character indexing of the node in the source.
    ///
    /// - The indexing is 0-based.
    pub end_char_index: Option<i32>,
    
    pub summary: Option<GeneratedContent>,
}