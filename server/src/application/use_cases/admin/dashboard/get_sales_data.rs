use crate::application::dto::admin::dashboard::sales_response::SalesResponse;
use crate::domain::repositories::order_repository::OrderRepository;
use crate::domain::services::order_service::OrderService;

pub struct GetSalesDataUseCase<T: OrderRepository> {
  order_service: OrderService<T>,
}

impl<T: OrderRepository> GetSalesDataUseCase<T> {
  pub fn new(order_repo: T) -> Self {
    let order_service = OrderService::new(order_repo);
    GetSalesDataUseCase {
      order_service
    }
  }

  pub async fn execute(&self) -> Result<SalesResponse, diesel::result::Error> {
    self.order_service.get_sales_data().await
  }
}
