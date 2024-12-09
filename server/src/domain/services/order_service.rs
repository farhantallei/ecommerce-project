use diesel::result::Error;
use crate::domain::repositories::order_repository::OrderRepository;

pub struct OrderService<T: OrderRepository> {
  order_repo: T,
}

impl<T: OrderRepository> OrderService<T> {
  pub fn new(order_repo: T) -> Self {
    OrderService {
      order_repo
    }
  }

  pub async fn get_order_count(&self) -> Result<i64, Error> {
    self.order_repo.count().await
  }

  pub async fn get_price_in_cents_total(&self) -> Result<i64, Error> {
    self.order_repo.sum_price_in_cents().await
  }
}
