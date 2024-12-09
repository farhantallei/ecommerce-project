use serde::Serialize;

// #[derive(Debug, Serialize)]
// pub enum ErrorType {
//   ValidationError,
//   InternalServerError,
//   BadRequest,
//   NotFound
// }

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
  pub code: u16,
  pub message: String,
  // pub error_type: ErrorType
}
