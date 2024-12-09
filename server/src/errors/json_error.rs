use std::fmt;
use actix_multipart::MultipartError;
use actix_web::http::StatusCode;
use actix_web::{Error, HttpResponse, ResponseError};
use actix_web::error::JsonPayloadError;
use crate::errors::error_response::ErrorResponse;

impl fmt::Display for ErrorResponse {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "error: {}", self.message)
  }
}

impl ResponseError for ErrorResponse {
  fn status_code(&self) -> StatusCode {
    StatusCode::BAD_REQUEST
  }

  fn error_response(&self) -> HttpResponse {
    HttpResponse::BadRequest().json(self)
  }
}

enum CustomError {
  Json(JsonPayloadError),
  Multipart(MultipartError),
}

impl fmt::Display for CustomError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let str = match *self {
      CustomError::Json(ref err) => err.to_string(),
      CustomError::Multipart(ref err) => err.to_string(),
    };
    write!(f, "{}", str)
  }
}

fn custom_error(err: CustomError, _req: &actix_web::HttpRequest) -> Error
{
  Error::from(ErrorResponse {
    code: 400,
    message: err.to_string(),
    // error_type: ErrorType::BadRequest,
  })
}

pub fn custom_json_error(err: JsonPayloadError, req: &actix_web::HttpRequest) -> Error {
  custom_error(CustomError::Json(err), req)
}

pub fn custom_multipart_error(err: MultipartError, req: &actix_web::HttpRequest) -> Error {
  custom_error(CustomError::Multipart(err), req)
}
