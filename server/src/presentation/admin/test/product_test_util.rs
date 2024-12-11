use std::env;
use std::sync::Arc;
use crate::domain::services::product_service::ProductService;
use crate::domain::services::storage_service::StorageService;
use crate::infrastructure::repositories::postgres_product_repository::PostgresProductRepository;
use crate::infrastructure::repositories::s3_storage_repository::S3StorageRepository;

pub async fn delete_product() {
  let product_repo = PostgresProductRepository::get_product_repo()
    .lock().unwrap().clone().unwrap();
  let storage_repo = S3StorageRepository::get_storage_repo()
    .lock().unwrap().clone().unwrap();

  let product_service = ProductService::new(Arc::new(product_repo));
  let storage_service = StorageService::new(Arc::new(storage_repo));

  let bucket_name = env::var("S3_BUCKET_NAME").unwrap();

  product_service.delete_all().await.unwrap();
  storage_service.delete_all_products(&bucket_name).await.unwrap();
}
