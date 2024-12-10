use std::env;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl};
use once_cell::sync::Lazy;
use crate::domain::repositories::user_repository::UserRepository;
use crate::infrastructure::db;
use crate::infrastructure::db::connection::DBPool;
use crate::schema::users::dsl::users;

static USER_REPO: Lazy<Mutex<Option<PostgresUserRepository>>> = Lazy::new(|| Mutex::new(None));

#[derive(Clone)]
pub struct PostgresUserRepository {
  pool: DBPool,
}

impl PostgresUserRepository {
  pub fn new(pool: DBPool) -> Self {
    PostgresUserRepository {
      pool,
    }
  }

  pub fn get_user_repo() -> &'static Mutex<Option<PostgresUserRepository>> {
    let mut repos = USER_REPO.lock().unwrap();
    if repos.is_none() {
      let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
      let pool = db::connection::establish_connection(&database_url);
      *repos = Some(PostgresUserRepository::new(pool));
    }
    &*USER_REPO
  }
}

#[async_trait]
impl UserRepository for Arc<PostgresUserRepository> {
  async fn count(&self) -> Result<i64, Error> {
    let result = users.count()
      .first::<i64>(&mut self.pool.get().unwrap())?;
    Ok(result)
  }
}
