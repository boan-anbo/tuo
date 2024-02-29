use chrono::{DateTime, TimeZone, Utc};
use crate::error::TuoError;
use crate::types::date_time::TuoDateTime;

pub fn now() -> TuoDateTime {
    Utc::now()
}

pub fn utc_from_epoch(epoch: i64) -> Result<TuoDateTime, TuoError> {
    let result = DateTime::from_timestamp(epoch, 0);
    match result {
        Some(date_time) => Ok(date_time),
        None => Err(TuoError::UtilError("Failed to convert epoch to date time".to_string()))
    }
}