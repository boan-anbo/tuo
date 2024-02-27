use uuid::Uuid;

pub enum DocumentSourceType {
    File,
    Url,
}

pub struct Document {
    /// Uuid of the document
    pub id: Uuid,
    pub title: String,
    pub content: Option<String>,
    pub source_type: DocumentSourceType,
    /// The source url of the document.
    ///
    /// If it is a file, it is the file path.
    /// If it is a url, it is the url.
    pub source: String,
}

impl Default for Document {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            title: String::new(),
            content: None,
            source_type: DocumentSourceType::File,
            source: String::new(),
        }
    }
}