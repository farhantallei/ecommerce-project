use actix_web::{web};
use crate::presentation::handlers::documentation_handlers::{documentation, openapi};

pub fn routes(config: &mut web::ServiceConfig) {
  config.service(documentation).service(openapi);
}
