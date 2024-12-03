use std::env;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use log::info;

pub async fn run() -> std::io::Result<()> {
  let port = env::var("PORT").expect("PORT must be set");

  info!("Starting...!");

  HttpServer::new(move || {
    App::new()
      .wrap(Logger::default())
      .route("/", web::get().to(|| async { "E-Commerce API" }))
  })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
