use actix_web::{get, HttpResponse, Responder};
use scalar_doc::scalar_actix::ActixDocumentation;

#[get("")]
pub async fn documentation() -> impl Responder {
  ActixDocumentation::new("E-Commerce API Documentation Title", "/api/v1/admin/openapi")
    .theme(scalar_doc::Theme::Kepler)
    .service()
}

#[get("/openapi")]
pub async fn openapi() -> impl Responder {
  let open = include_str!("../../../../openapi/admin.json");
  HttpResponse::Ok().body(open)
}
