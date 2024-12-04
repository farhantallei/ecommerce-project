use diesel::result::Error;
use crate::application::dto::admin::dashboard::product_response::ProductResponse;
use crate::application::dto::product_count_options::ProductCountAvailable;
use crate::domain::repositories::product_repository::ProductRepository;

pub struct ProductService<T> {
  product_repo: T,
}

impl<T: ProductRepository> ProductService<T> {
  pub fn new(product_repo: T) -> Self {
    ProductService {
      product_repo,
    }
  }

  pub async fn get_product_data(&self) -> Result<ProductResponse, Error> {
    let active_count = self.product_repo
      .count(ProductCountAvailable::Available { is_available: true }).await?;
    let inactive_count = self.product_repo
      .count(ProductCountAvailable::Available { is_available: false }).await?;

    Ok(ProductResponse {
      active_count,
      inactive_count,
    })
  }
}
