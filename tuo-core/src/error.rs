use thiserror::Error;

use tuo_utils::error::TuoUtilError;

/// Error type for TuoCore
#[derive(Error, Debug)]
pub enum TuoCoreError {
    #[error("Invalid file path: {0}")]
    TuoUtilError(#[from] TuoUtilError),
    #[error("No readers provider for mime type: {0}")]
    ReaderNoProvider(String),
}