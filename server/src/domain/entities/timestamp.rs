use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_created_at() -> i64 {
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards")
    .as_millis() as i64
}
