use log::error;
use crate::application::response::ApiResponse;
use crate::domain::entities::product::{NewProduct, NewProductRequest, ProductResponse};
use crate::errors::api_error::ApiError;
use crate::domain::repositories::product_repository::ProductRepository;
use crate::domain::repositories::storage_repository::StorageRepository;
use crate::domain::services::product_service::ProductService;
use crate::domain::services::storage_service::StorageService;

pub struct CreateProductUseCase<T: ProductRepository, U: StorageRepository> {
  product_service: ProductService<T>,
  storage_service: StorageService<U>,
}

impl<T: ProductRepository, U: StorageRepository> CreateProductUseCase<T, U> {
  pub fn new(product_repo: T, storage_repo: U) -> Self {
    let product_service = ProductService::new(product_repo);
    let storage_service = StorageService::new(storage_repo);
    CreateProductUseCase {
      product_service,
      storage_service,
    }
  }

  pub async fn execute(&self, new_product_request: NewProductRequest) -> Result<ApiResponse<ProductResponse>, ApiError> {
    let bucket_name = std::env::var("S3_BUCKET_NAME").expect("S3_BUCKET_NAME must be set");

    let id = self.product_service.create_id().await.map_err(|e| {
      error!("Error creating product id: {:?}", e);
      ApiError::InternalServerError("Error creating product id")
    })?;

    let file_key = format!("products/{}/file{}", id.1, match new_product_request.file.content_type {
      Some(content_type) => match content_type.to_string().split("/").last() {
        Some(ext) => format!(".{}", ext),
        None => "".to_string(),
      },
      None => "".to_string(),
    });

    let image_key = format!("products/{}/image{}", id.1, match new_product_request.image.content_type {
      Some(content_type) => match content_type.to_string().split("/").last() {
        Some(ext) => format!(".{}", ext),
        None => "".to_string(),
      },
      None => "".to_string(),
    });

    new_product_request.file.file.path().to_str().map(|path| async {
      self.storage_service.upload_file(&bucket_name, path, &file_key).await.map_err(|e| {
        error!("Error uploading file: {:?}", e);
        ApiError::InternalServerError("Error uploading file")
      })
    }).unwrap().await?;

    new_product_request.image.file.path().to_str().map(|path| async {
      self.storage_service.upload_file(&bucket_name, path, &image_key).await.map_err(|e| {
        error!("Error uploading image: {:?}", e);
        ApiError::InternalServerError("Error uploading image")
      })
    }).unwrap().await?;

    let new_product = NewProduct::new(
      id.0,
      new_product_request.name.to_string(),
      new_product_request.price_in_cents.to_be(),
      file_key,
      image_key,
      match new_product_request.description {
        Some(description) => Some(description.to_string()),
        None => None,
      },
    );

    self.product_service.create(&new_product).await.map_err(|e| {
      error!("Error creating product: {:?}", e);
      ApiError::InternalServerError("Error creating product")
    })?;

    Ok(ApiResponse::new(ProductResponse { id: id.1 }))
  }
}
