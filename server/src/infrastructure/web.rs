use std::env;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use log::info;
use crate::infrastructure::db::connection::establish_connection;
use crate::infrastructure::repositories::postgres_download_verification_repository::PostgresDownloadVerificationRepository;
use crate::infrastructure::repositories::postgres_order_repository::PostgresOrderRepository;
use crate::infrastructure::repositories::postgres_product_repository::PostgresProductRepository;
use crate::infrastructure::repositories::postgres_user_repository::PostgresUserRepository;

pub async fn run() -> std::io::Result<()> {
  let port = env::var("PORT").expect("PORT must be set");
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let pool = establish_connection(&database_url);

  let product_repo = PostgresProductRepository::new(pool.clone());
  let user_repo = PostgresUserRepository::new(pool.clone());
  let order_repo = PostgresOrderRepository::new(pool.clone());
  let download_verification_repo = PostgresDownloadVerificationRepository::new(pool.clone());

  let app_data = web::Data::new((product_repo, user_repo, order_repo, download_verification_repo));

  info!("Starting...!");

  HttpServer::new(move || {
    App::new()
      .app_data(app_data.clone())
      .wrap(Logger::default())
      .route("/", web::get().to(|| async { "E-Commerce API" }))
  })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
