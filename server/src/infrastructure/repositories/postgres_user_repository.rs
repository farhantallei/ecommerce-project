use crate::infrastructure::db::connection::DBPool;
use crate::infrastructure::repositories::repository::Repository;

#[derive(Clone)]
pub struct PostgresUserRepository {
  repo: Repository,
}

impl PostgresUserRepository {
  pub fn new(pool: DBPool) -> Self {
    PostgresUserRepository {
      repo: Repository::new(pool),
    }
  }
}
