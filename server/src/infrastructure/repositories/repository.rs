use crate::infrastructure::db::connection::DBPool;

#[derive(Clone)]
pub struct Repository {
  pool: DBPool,
}

impl Repository {
  pub fn new(pool: DBPool) -> Self {
    Repository { pool }
  }
}
