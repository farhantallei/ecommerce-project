use crate::infrastructure::db::connection::DBPool;

#[derive(Clone)]
pub struct PostgresProductRepository {
  pool: DBPool,
}

impl PostgresProductRepository {
  pub fn new(pool: DBPool) -> Self {
    PostgresProductRepository {
      pool,
    }
  }
}
