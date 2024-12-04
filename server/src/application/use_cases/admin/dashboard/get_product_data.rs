use diesel::result::Error;
use crate::application::dto::admin::dashboard::product_response::ProductResponse;
use crate::domain::repositories::product_repository::ProductRepository;
use crate::domain::services::product_service::ProductService;

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

  pub async fn execute(&self) -> Result<ProductResponse, Error> {
    self.product_service.get_product_data().await
  }
}
