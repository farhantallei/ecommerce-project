use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UserResponse {
  pub user_count: i64,
  pub average_value_per_user: f32,
}
