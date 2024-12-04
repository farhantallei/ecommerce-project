use std::sync::Arc;
use async_trait::async_trait;
use diesel::dsl::sum;
use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl};
use crate::domain::repositories::order_repository::OrderRepository;
use crate::infrastructure::db::connection::DBPool;
use crate::schema::orders::dsl::orders;
use crate::schema::orders::price_in_cents;

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
