use thiserror::Error;

use tuo_utils::error::TuoUtilError;

/// Error type for TuoCore
#[derive(Error, Debug)]
pub enum TuoError {
    #[error("Generic error: {0}")]
    GenericError(String),
    #[error("Invalid file path: {0}")]
    TuoUtilError(#[from] TuoUtilError),
    #[error("No readers provider for mime type: {0}")]
    ReaderNoProvider(String),
    #[error("No reader provider found.\nSolution: Either provide a reader provider to your implementation of the UniReaderTrait, or override the `read` method of the default implementation of the UniReaderTrait to avoid using a reader provider.")]
    UniReaderHasNoReaderProvider,
    #[error("No Universal Reader for Index found.\nSolution: Either provide a Universal Reader to your implementation of the IndexFromFolderTrait, or override the `from_folder` method of the default implementation of the IndexFromFolderTrait to avoid using a Universal Reader.")]
    IndexHasNoUniReader,
    #[error("Model Error: {0}")]
    ModelError(String),
    #[error("Util Error: {0}")]
    UtilError(String)
}

