use std::sync::Arc;
use async_trait::async_trait;
use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl};
use crate::domain::repositories::user_repository::UserRepository;
use crate::infrastructure::db::connection::DBPool;
use crate::schema::users::dsl::users;

#[derive(Clone)]
pub struct PostgresUserRepository {
  pool: DBPool,
}

impl PostgresUserRepository {
  pub fn new(pool: DBPool) -> Self {
    PostgresUserRepository {
      pool,
    }
  }
}

#[async_trait]
impl UserRepository for Arc<PostgresUserRepository> {
  async fn count(&self) -> Result<i64, Error> {
    let result = users.count()
      .first::<i64>(&mut self.pool.get().unwrap())?;
    Ok(result)
  }
}
