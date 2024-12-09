use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;
use crate::errors::error_response::ErrorResponse;

#[derive(Debug, Error)]
pub enum ApiError {
  #[error("{0}")]
  InternalServerError(&'static str),
  #[error("{0}")]
  BadRequest(&'static str),
  #[error("{0}")]
  NotFound(&'static str),
}

impl ApiError {
  // fn error_type(&self) -> ErrorType {
  //   match *self {
  //     ApiError::InternalServerError(_) => ErrorType::InternalServerError,
  //     ApiError::BadRequest(_) => ErrorType::BadRequest,
  //     ApiError::NotFound(_) => ErrorType::NotFound,
  //   }
  // }

  fn error_code(&self) -> u16 {
    match *self {
      ApiError::InternalServerError(_) => 500,
      ApiError::BadRequest(_) => 400,
      ApiError::NotFound(_) => 404,
    }
  }
}

impl ResponseError for ApiError {
  fn status_code(&self) -> actix_web::http::StatusCode {
    actix_web::http::StatusCode::from_u16(self.error_code()).unwrap()
  }

  fn error_response(&self) -> HttpResponse {
    let error_response = ErrorResponse {
      code: self.error_code(),
      message: self.to_string(),
      // error_type: self.error_type(),
    };

    HttpResponse::build(actix_web::http::StatusCode::from_u16(self.error_code()).unwrap())
      .json(error_response)
  }
}
