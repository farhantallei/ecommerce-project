use std::env;
use std::sync::Arc;
use crate::infrastructure::db::connection::establish_connection;
use crate::infrastructure::repositories::postgres_download_verification_repository::PostgresDownloadVerificationRepository;
use crate::infrastructure::repositories::postgres_order_repository::PostgresOrderRepository;
use crate::infrastructure::repositories::postgres_product_repository::PostgresProductRepository;
use crate::infrastructure::repositories::postgres_user_repository::PostgresUserRepository;

pub struct AppState {
  pub product_repo: Arc<PostgresProductRepository>,
  pub user_repo: Arc<PostgresUserRepository>,
  pub order_repo: Arc<PostgresOrderRepository>,
  pub download_verification_repo: Arc<PostgresDownloadVerificationRepository>,
}

impl AppState {
  pub fn new() -> Self {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = establish_connection(&database_url);

    let product_repo = Arc::new(PostgresProductRepository::new(pool.clone()));
    let user_repo = Arc::new(PostgresUserRepository::new(pool.clone()));
    let order_repo = Arc::new(PostgresOrderRepository::new(pool.clone()));
    let download_verification_repo = Arc::new(PostgresDownloadVerificationRepository::new(pool.clone()));

    Self {
      product_repo,
      user_repo,
      order_repo,
      download_verification_repo,
    }
  }
}
