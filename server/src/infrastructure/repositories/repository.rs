use crate::infrastructure::db::connection::DBPool;

#[derive(Clone)]
pub struct Repository {
  pub pool: DBPool,
}

impl Repository {
  pub fn new(pool: DBPool) -> Self {
    Repository { pool }
  }
}
