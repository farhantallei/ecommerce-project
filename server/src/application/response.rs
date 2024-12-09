use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
  pub data: T,
}

impl<T> ApiResponse<T> {
  pub fn new(data: T) -> Self {
    ApiResponse { data }
  }
}
