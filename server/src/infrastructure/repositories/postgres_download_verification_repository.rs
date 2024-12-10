use std::env;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::infrastructure::db;
use crate::infrastructure::db::connection::DBPool;

static DOWNLOAD_VERIFICATION_REPO: Lazy<Mutex<Option<PostgresDownloadVerificationRepository>>> = Lazy::new(|| Mutex::new(None));

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

  pub fn get_download_verification_repo() -> &'static Mutex<Option<PostgresDownloadVerificationRepository>> {
    let mut repos = DOWNLOAD_VERIFICATION_REPO.lock().unwrap();
    if repos.is_none() {
      let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
      let pool = db::connection::establish_connection(&database_url);
      *repos = Some(PostgresDownloadVerificationRepository::new(pool));
    }
    &*DOWNLOAD_VERIFICATION_REPO
  }
}
