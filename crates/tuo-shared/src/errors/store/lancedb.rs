use crate::errors::parts::TuoPartsError;
use crate::errors::tuo::TuoError;

impl From<lancedb::Error> for TuoPartsError {
    fn from(err: lancedb::Error) -> Self {
        TuoPartsError::StoreError(err.to_string())
    }
}

impl From<lancedb::Error> for TuoError {
    fn from(err: lancedb::Error) -> Self {
        TuoError::from(TuoPartsError::StoreError(err.to_string()))
    }
}