use chrono::{DateTime, Utc};

pub struct Task {
    pub user_id: i64,
    pub id: i64,
    pub text: String,
    pub note: String,
    pub category: String,
    pub schedule_time: DateTime<Utc>,
    pub schedule_interval_value: i64,
    pub schedule_interval_type: String,
    pub completed: bool,
}
