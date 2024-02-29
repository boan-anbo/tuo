use chrono::{DateTime, Utc};

pub type TuoDateTime = DateTime<Utc>;

pub fn now() -> TuoDateTime {
    Utc::now()
}