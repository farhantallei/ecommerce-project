use async_trait::async_trait;
use diesel::result::Error;

#[async_trait]
pub trait UserRepository {
  async fn count(&self) -> Result<i64, Error>;
}
