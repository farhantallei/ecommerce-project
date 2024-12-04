use std::sync::Arc;
use async_trait::async_trait;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use diesel::result::Error;
use crate::application::dto::product_count_options::ProductCountAvailable;
use crate::domain::repositories::product_repository::ProductRepository;
use crate::infrastructure::db::connection::DBPool;
use crate::schema::products::dsl::products;
use crate::schema::products::is_available_for_purchase;

#[derive(Clone)]
pub struct PostgresProductRepository {
  pool: DBPool,
}

impl PostgresProductRepository {
  pub fn new(pool: DBPool) -> Self {
    PostgresProductRepository {
      pool,
    }
  }
}

#[async_trait]
impl ProductRepository for Arc<PostgresProductRepository> {
  async fn count(&self, available: ProductCountAvailable) -> Result<i64, Error> {
    let mut query = products.into_boxed();

    match available {
      ProductCountAvailable::All => (),
      ProductCountAvailable::Available { is_available } => {
        query = query.filter(is_available_for_purchase.eq(is_available));
      }
    }

    let result = products
      .count()
      .first::<i64>(&mut self.pool.get().unwrap())?;
    Ok(result)
  }
}
