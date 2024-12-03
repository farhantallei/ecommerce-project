use crate::infrastructure::db::connection::DBPool;
use crate::infrastructure::repositories::repository::Repository;

#[derive(Clone)]
pub struct PostgresDownloadVerificationRepository {
  repo: Repository,
}

impl PostgresDownloadVerificationRepository {
  pub fn new(pool: DBPool) -> Self {
    PostgresDownloadVerificationRepository {
      repo: Repository::new(pool),
    }
  }
}
