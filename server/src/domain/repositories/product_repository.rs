use async_trait::async_trait;
use diesel::result::Error;
use crate::application::dto::product_count_options::ProductCountAvailable;

#[async_trait]
pub trait ProductRepository {
  async fn count(&self, option: ProductCountAvailable) -> Result<i64, Error>;
}
