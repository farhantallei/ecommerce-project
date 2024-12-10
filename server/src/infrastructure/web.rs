use std::env;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::middleware::Logger;
use log::info;
use crate::errors::api_error::ApiError;
use crate::errors::json_error::{custom_json_error, custom_multipart_error};
use crate::infrastructure::repositories::s3_storage_repository::S3StorageRepository;
use crate::infrastructure::repositories::postgres_download_verification_repository::PostgresDownloadVerificationRepository;
use crate::infrastructure::repositories::postgres_order_repository::PostgresOrderRepository;
use crate::infrastructure::repositories::postgres_product_repository::PostgresProductRepository;
use crate::infrastructure::repositories::postgres_user_repository::PostgresUserRepository;
use crate::presentation::routes;

pub trait MyAppTrait: ServiceFactory<
  ServiceRequest,
  Config=(),
  Response=ServiceResponse,
  Error=actix_web::Error,
  InitError=()
> {}

impl<T: ServiceFactory<
  ServiceRequest,
  Config=(),
  Response=ServiceResponse,
  Error=actix_web::Error,
  InitError=()
>> MyAppTrait for T {}

pub fn init_app() -> App<impl MyAppTrait> {
  let storage_repo = S3StorageRepository::get_storage_repo()
    .lock().unwrap().clone().unwrap();
  let product_repo = PostgresProductRepository::get_product_repo()
    .lock().unwrap().clone().unwrap();
  let user_repo = PostgresUserRepository::get_user_repo()
    .lock().unwrap().clone().unwrap();
  let order_repo = PostgresOrderRepository::get_order_repo()
    .lock().unwrap().clone().unwrap();
  let download_verification_repo = PostgresDownloadVerificationRepository::get_download_verification_repo()
    .lock().unwrap().clone().unwrap();

  App::new()
    .app_data(web::JsonConfig::default().error_handler(custom_json_error))
    .app_data(actix_multipart::form::MultipartFormConfig::default().error_handler(custom_multipart_error))
    .app_data(web::Data::new(storage_repo.clone()))
    .app_data(web::Data::new(product_repo.clone()))
    .app_data(web::Data::new(user_repo.clone()))
    .app_data(web::Data::new(order_repo.clone()))
    .app_data(web::Data::new(download_verification_repo.clone()))
    .route("/", web::get().to(|| async { "E-Commerce API" }))
    .service(
      web::scope("/api/v1").configure(
        routes::admin_routes::routes
      )
    )
    .default_service(web::route().to(|| async { Err::<HttpResponse, _>(ApiError::NotFound("Route not found")) }))
}

pub async fn run() -> std::io::Result<()> {
  let port = env::var("PORT").expect("PORT must be set");

  info!("Starting...!");

  HttpServer::new(move || init_app().wrap(Logger::default()))
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
