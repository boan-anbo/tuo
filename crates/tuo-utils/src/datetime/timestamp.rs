use chrono::{DateTime, Utc};

use tuo_shared::errors::utils::TuoUtilError;

pub fn now() -> DateTime<Utc> {
    Utc::now()
}

pub fn utc_from_epoch(epoch: i64) -> DateTime<Utc> {
    DateTime::from_timestamp(epoch, 0).expect("Failed to create DateTime from epoch")
}
pub fn now_string() -> String {
    now().to_rfc3339()
}