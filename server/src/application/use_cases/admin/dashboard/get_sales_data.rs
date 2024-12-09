use log::error;
use crate::application::dto::admin::dashboard::sales_response::SalesResponse;
use crate::errors::api_error::ApiError;
use crate::application::response::ApiResponse;
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

  pub async fn execute(&self) -> Result<ApiResponse<SalesResponse>, ApiError> {
    let total_price_in_cents = self.order_service.get_price_in_cents_total().await.map_err(|e| {
      error!("Error getting sales data: {:?}", e);
      ApiError::InternalServerError("Error getting sales data")
    })?;

    let total_sales = self.order_service.get_order_count().await.map_err(|e| {
      error!("Error getting sales data: {:?}", e);
      ApiError::InternalServerError("Error getting sales data")
    })?;

    let response = SalesResponse {
      amount: total_price_in_cents,
      number_of_sales: total_sales,
    };

    Ok(ApiResponse::new(response))
  }
}
