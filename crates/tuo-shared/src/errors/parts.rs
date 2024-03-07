use thiserror::Error;

#[derive(Error, Debug)]
pub enum TuoPartsError {
    #[error("API Error: {0}")]
    ApiError(String),
    #[error("Store Error {0}")]
    StoreError(String),
    #[error("Index Error: {0}")]
    IndexError(String),
}
