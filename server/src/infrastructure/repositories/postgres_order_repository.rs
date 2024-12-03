use crate::infrastructure::db::connection::DBPool;
use crate::infrastructure::repositories::repository::Repository;

#[derive(Clone)]
pub struct PostgresOrderRepository {
  repo: Repository,
}

impl PostgresOrderRepository {
  pub fn new(pool: DBPool) -> Self {
    PostgresOrderRepository {
      repo: Repository::new(pool),
    }
  }
}
