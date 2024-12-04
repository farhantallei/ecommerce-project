use std::env;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use log::info;
use crate::infrastructure::app_state::AppState;
use crate::presentation::routes;

pub async fn run() -> std::io::Result<()> {
  let port = env::var("PORT").expect("PORT must be set");

  info!("Starting...!");

  HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(AppState::new()))
      .wrap(Logger::default())
      .route("/", web::get().to(|| async { "E-Commerce API" }))
      .service(
        web::scope("/api/v1").configure(
          routes::admin_routes::routes
        )
      )
  })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
