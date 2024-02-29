use uuid::Uuid;
use crate::core::messaging::content::Content;

pub enum TableType {
    /// ## Why separate this from Tabular
    ///
    /// Because you can run generated queries over it.
    ///
    /// ## Features:
    /// Can run sql engine over it.
    ///
    /// ## Structure:
    /// - Document = SQL Table
    /// - Section = Row
    /// - Node = Columns of a row
    SQL,
    /// ## Examples
    /// - Excel
    /// - CSV
    /// - DataFrame
    ///
    /// ## Structure:
    /// - Document = table
    /// - Section = row
    /// - Node of Columns of a row
    Tabular,
}

#[derive(Default)]
pub enum TreeType {
    #[default]
    BOOK,
    ARTICLE,
}

#[derive(Default)]
pub enum DocumentSourceType {
    #[default]
    File,
    Url,
}

/// Document type
///
/// Describe the nature of the abstract source
///
/// ## Why?
///
/// Because to appropriately feed the source to LLM for certain tasks, such as summarization, we need to know the appropriate way to present the data represented in the source.
///
/// For example, to summarize a book, a tree-like source, we need to first find most relevant pages containing most relevant paragraphs such as conclusions or abstracts to provide to LLM.
/// But when we want to summarize a table, we need to provide all the sections, i.e. column names, to the LLM in order to generate its summary.
pub enum DocumentType {
    /// Tree-like documents
    ///
    /// ## Examples
    /// - A book:
    ///     - book = source 
    ///     - section = page
    ///     - node = paragraph/sentences
    /// - A Markdown file:
    ///     - markdown file = source
    ///     - section = markdown headers section
    ///     - node = paragraph/sentences
    /// - A plain text file:
    ///     - text file = source
    ///     - section = arbitrary page divisions
    ///     - node = paragraph/sentences
    Tree(TreeType),

    /// Tabular data documents
    ///
    /// ## Example
    /// - A data table = a source -> a section = a column -> node = a row under a column
    Table(TableType),
}

impl Default for DocumentType {
    fn default() -> Self {
        DocumentType::Tree(TreeType::BOOK)
    }
}

#[derive(Default)]
pub struct Document {
    /// Uuid of the source
    pub id: Uuid,
    pub name: String,
    pub document_type: DocumentType,
    pub content: Option<String>,
    pub source_type: DocumentSourceType,
    /// The source url of the source.
    ///
    /// If it is a file, it is the file path.
    /// If it is a url, it is the url.
    pub source: String,

    /// The summary of the source
    pub summary: Option<Content>,
}
