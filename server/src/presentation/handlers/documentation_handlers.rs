use actix_web::{get, HttpResponse, Responder};
use scalar_doc::scalar_actix::ActixDocumentation;

#[get("/")]
pub async fn documentation() -> impl Responder {
  ActixDocumentation::new("E-Commerce API Documentation Title", "/openapi")
    .theme(scalar_doc::Theme::Kepler)
    .service()
}

#[get("/openapi")]
pub async fn openapi() -> impl Responder {
  let open = include_str!("../../../openapi.json");
  HttpResponse::Ok().body(open)
}
