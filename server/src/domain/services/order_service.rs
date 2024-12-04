use diesel::result::Error;
use crate::application::dto::admin::dashboard::sales_response::SalesResponse;
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

  pub async fn get_sales_data(&self) -> Result<SalesResponse, Error> {
    let amount = self.order_repo.sum_price_in_cents().await?;
    let number_of_sales = self.order_repo.count().await?;

    Ok(SalesResponse {
      amount,
      number_of_sales,
    })
  }
}
