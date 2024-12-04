use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ProductResponse {
  pub active_count: i64,
  pub inactive_count: i64,
}
