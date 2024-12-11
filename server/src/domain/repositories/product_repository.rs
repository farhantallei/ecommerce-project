use async_trait::async_trait;
use diesel::result::Error;
use crate::application::dto::product_count_options::ProductCountAvailable;
use crate::domain::entities::product::NewProduct;

#[async_trait]
pub trait ProductRepository {
  async fn next_val(&self) -> Result<i64, Error>;
  async fn count(&self, option: ProductCountAvailable) -> Result<i64, Error>;
  async fn save(&self, product: &NewProduct) -> Result<(), Error>;
  async fn delete_all(&self) -> Result<(), Error>;
}
