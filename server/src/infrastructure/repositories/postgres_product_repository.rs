use std::env;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use diesel::result::Error;
use once_cell::sync::Lazy;
use crate::application::dto::product_count_options::ProductCountAvailable;
use crate::domain::entities::product::NewProduct;
use crate::domain::repositories::product_repository::ProductRepository;
use crate::infrastructure::db;
use crate::infrastructure::db::connection::DBPool;
use crate::infrastructure::db::sequence::get_next_value;
use crate::schema;
use crate::schema::products::dsl::products;
use crate::schema::products::is_available_for_purchase;

static PRODUCT_REPO: Lazy<Mutex<Option<PostgresProductRepository>>> = Lazy::new(|| Mutex::new(None));

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

  pub fn get_product_repo() -> &'static Mutex<Option<PostgresProductRepository>> {
    let mut repos = PRODUCT_REPO.lock().unwrap();
    if repos.is_none() {
      let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
      let pool = db::connection::establish_connection(&database_url);
      *repos = Some(PostgresProductRepository::new(pool));
    }
    &*PRODUCT_REPO
  }
}

#[async_trait]
impl ProductRepository for Arc<PostgresProductRepository> {
  async fn next_val(&self) -> Result<i64, Error> {
    let pool = self.pool.clone();

    let sequence = get_next_value(pool, "products_id_seq")?;
    Ok(sequence)
  }

  async fn count(&self, available: ProductCountAvailable) -> Result<i64, Error> {
    let mut query = products.into_boxed();

    match available {
      ProductCountAvailable::All => (),
      ProductCountAvailable::Available { is_available } => {
        query = query.filter(is_available_for_purchase.eq(is_available));
      }
    }

    let result = query
      .count()
      .first::<i64>(&mut self.pool.get().unwrap())?;
    Ok(result)
  }

  async fn save(&self, product: &NewProduct) -> Result<(), Error> {
    diesel::insert_into(schema::products::table)
      .values(product)
      .execute(&mut self.pool.get().unwrap())?;

    Ok(())
  }

  async fn delete_all(&self) -> Result<(), Error> {
    diesel::delete(products)
      .execute(&mut self.pool.get().unwrap())?;

    Ok(())
  }
}
