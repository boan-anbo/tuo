use crate::errors::tuo::TuoError;

pub type TuoResult<T> = std::result::Result<T, TuoError>;