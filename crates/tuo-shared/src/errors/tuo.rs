use thiserror::Error;

use crate::errors::core::TuoCoreError;
use crate::errors::parts::TuoPartsError;
use crate::errors::utils::TuoUtilError;

#[derive(Debug, Error)]
pub enum TuoError {
    #[error("Generic error: {0}")]
    GenericError(String),
    #[error("Core Error: {0}")]
    CoreError(#[from] TuoCoreError),
    #[error("Parts Error: {0}")]
    PartsError(#[from] TuoPartsError),
    #[error("Util Error: {0}")]
    UtilError(#[from] TuoUtilError),
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
}