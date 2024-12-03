use crate::infrastructure::db::connection::DBPool;
use crate::infrastructure::repositories::repository::Repository;

#[derive(Clone)]
pub struct PostgresProductRepository {
  repo: Repository,
}

impl PostgresProductRepository {
  pub fn new(pool: DBPool) -> Self {
    PostgresProductRepository {
      repo: Repository::new(pool),
    }
  }
}
