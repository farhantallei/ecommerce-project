use crate::infrastructure::db::connection::DBPool;

#[derive(Clone)]
pub struct PostgresDownloadVerificationRepository {
  pool: DBPool,
}

impl PostgresDownloadVerificationRepository {
  pub fn new(pool: DBPool) -> Self {
    PostgresDownloadVerificationRepository {
      pool,
    }
  }
}
