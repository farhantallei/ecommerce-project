use async_trait::async_trait;
use diesel::result::Error;

#[async_trait]
pub trait OrderRepository {
  async fn count(&self) -> Result<i64, Error>;
  async fn sum_price_in_cents(&self) -> Result<i64, Error>;
}
