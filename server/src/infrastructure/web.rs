use std::env;
use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use log::info;
use crate::infrastructure::app_state::AppState;
use crate::infrastructure::db::connection::establish_connection;
use crate::infrastructure::repositories::postgres_download_verification_repository::PostgresDownloadVerificationRepository;
use crate::infrastructure::repositories::postgres_order_repository::PostgresOrderRepository;
use crate::infrastructure::repositories::postgres_product_repository::PostgresProductRepository;
use crate::infrastructure::repositories::postgres_user_repository::PostgresUserRepository;
use crate::presentation::routes;

pub async fn run() -> std::io::Result<()> {
  let port = env::var("PORT").expect("PORT must be set");

  info!("Starting...!");

  HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(AppState::new()))
      .wrap(Logger::default())
      .configure(routes::documentation_routes::routes)
      .configure(routes::admin_dashboard_routes::routes)
  })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
