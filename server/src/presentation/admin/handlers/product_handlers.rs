use actix_multipart::form::MultipartForm;
use actix_web::{get, post, web, HttpResponse, Responder};
use crate::application::use_cases::create_product::CreateProductUseCase;
use crate::domain::entities::product::NewProductRequest;
use crate::infrastructure::repositories::postgres_product_repository::PostgresProductRepository;
use crate::infrastructure::repositories::s3_storage_repository::S3StorageRepository;

#[get("")]
pub async fn get_products_handler() -> HttpResponse {
  todo!()
}

#[post("")]
pub async fn create_product_handler(
  storage_repo: web::Data<S3StorageRepository>,
  product_repo: web::Data<PostgresProductRepository>,
  input: MultipartForm<NewProductRequest>,
) -> impl Responder {
  match CreateProductUseCase::new(product_repo.into_inner(), storage_repo.into_inner())
    .execute(input.into_inner()).await {
    Ok(response) => Ok(HttpResponse::Created().json(response)),
    Err(e) => Err(e)
  }
}
