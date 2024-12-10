use std::env;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use diesel::dsl::sum;
use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl};
use once_cell::sync::Lazy;
use crate::domain::repositories::order_repository::OrderRepository;
use crate::infrastructure::db;
use crate::infrastructure::db::connection::DBPool;
use crate::schema::orders::dsl::orders;
use crate::schema::orders::price_in_cents;

static ORDER_REPO: Lazy<Mutex<Option<PostgresOrderRepository>>> = Lazy::new(|| Mutex::new(None));

#[derive(Clone)]
pub struct PostgresOrderRepository {
  pool: DBPool,
}

impl PostgresOrderRepository {
  pub fn new(pool: DBPool) -> Self {
    PostgresOrderRepository {
      pool,
    }
  }

  pub fn get_order_repo() -> &'static Mutex<Option<PostgresOrderRepository>> {
    let mut repos = ORDER_REPO.lock().unwrap();
    if repos.is_none() {
      let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
      let pool = db::connection::establish_connection(&database_url);
      *repos = Some(PostgresOrderRepository::new(pool));
    }
    &*ORDER_REPO
  }
}

#[async_trait]
impl OrderRepository for Arc<PostgresOrderRepository> {
  async fn count(&self) -> Result<i64, Error> {
    let result = orders.count()
      .first::<i64>(&mut self.pool.get().unwrap())?;
    Ok(result)
  }

  async fn sum_price_in_cents(&self) -> Result<i64, Error> {
    let result = orders.select(sum(price_in_cents))
      .first::<Option<i64>>(&mut self.pool.get().unwrap());

    match result {
      Ok(Some(value)) => Ok(value),
      _ => Ok(0),
    }
  }
}
