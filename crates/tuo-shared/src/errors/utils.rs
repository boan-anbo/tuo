use thiserror::Error;

#[derive(Debug, Error)]
pub enum TuoUtilError {
    #[error("Invalid file path: {0}")]
    InvalidFilePath(String),
    #[error("Unsupported file type: {0}")]
    UnsupportedFileType(String),
    #[error("Cannot determine file type: {0}")]
    CannotDetermineFileType(String),
    #[error("Timestamp error: {0}")]
    TimestampError(String),

}