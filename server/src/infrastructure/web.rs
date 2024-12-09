use std::env;
use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::middleware::Logger;
use log::info;
use crate::errors::api_error::ApiError;
use crate::errors::json_error::{custom_json_error, custom_multipart_error};
use crate::infrastructure::db;
use crate::infrastructure::repositories::postgres_download_verification_repository::PostgresDownloadVerificationRepository;
use crate::infrastructure::repositories::postgres_order_repository::PostgresOrderRepository;
use crate::infrastructure::repositories::postgres_product_repository::PostgresProductRepository;
use crate::infrastructure::repositories::postgres_user_repository::PostgresUserRepository;
use crate::presentation::routes;

pub async fn run() -> std::io::Result<()> {
  let port = env::var("PORT").expect("PORT must be set");

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let pool = db::connection::establish_connection(&database_url);

  let product_repo = PostgresProductRepository::new(pool.clone());
  let user_repo = PostgresUserRepository::new(pool.clone());
  let order_repo = PostgresOrderRepository::new(pool.clone());
  let download_verification_repo = PostgresDownloadVerificationRepository::new(pool.clone());

  info!("Starting...!");

  HttpServer::new(move || {
    App::new()
      .app_data(web::JsonConfig::default().error_handler(custom_json_error))
      .app_data(actix_multipart::form::MultipartFormConfig::default().error_handler(custom_multipart_error))
      .app_data(web::Data::new(product_repo.clone()))
      .app_data(web::Data::new(user_repo.clone()))
      .app_data(web::Data::new(order_repo.clone()))
      .app_data(web::Data::new(download_verification_repo.clone()))
      .wrap(Logger::default())
      .route("/", web::get().to(|| async { "E-Commerce API" }))
      .service(
        web::scope("/api/v1").configure(
          routes::admin_routes::routes
        )
      )
      .default_service(web::route().to(|| async { Err::<HttpResponse, _>(ApiError::NotFound("Route not found")) }))
  })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
