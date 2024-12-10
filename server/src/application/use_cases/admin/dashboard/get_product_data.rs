use log::error;
use crate::application::dto::admin::dashboard::product_response::ProductResponse;
use crate::application::dto::product_count_options::ProductCountAvailable;
use crate::domain::repositories::product_repository::ProductRepository;
use crate::domain::services::product_service::ProductService;
use crate::errors::api_error::ApiError;
use crate::application::response::ApiResponse;

pub struct GetProductDataUseCase<T: ProductRepository> {
  product_service: ProductService<T>,
}

impl<T: ProductRepository> GetProductDataUseCase<T> {
  pub fn new(product_repo: T) -> Self {
    let product_service = ProductService::new(product_repo);
    GetProductDataUseCase {
      product_service
    }
  }

  pub async fn execute(&self) -> Result<ApiResponse<ProductResponse>, ApiError> {
    let active_count = self.product_service.get_product_count(ProductCountAvailable::Available { is_available: true }).await.map_err(|e| {
      error!("Error getting product data: {:?}", e);
      ApiError::InternalServerError("Error getting product data")
    })?;

    let inactive_count = self.product_service.get_product_count(ProductCountAvailable::Available { is_available: false }).await.map_err(|e| {
      error!("Error getting product data: {:?}", e);
      ApiError::InternalServerError("Error getting product data")
    })?;

    let response = ProductResponse {
      active_count,
      inactive_count,
    };

    Ok(ApiResponse::new(response))
  }
}
