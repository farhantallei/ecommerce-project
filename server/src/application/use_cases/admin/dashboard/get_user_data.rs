use log::error;
use crate::application::dto::admin::dashboard::user_response::UserResponse;
use crate::errors::api_error::ApiError;
use crate::application::response::ApiResponse;
use crate::domain::repositories::order_repository::OrderRepository;
use crate::domain::repositories::user_repository::UserRepository;
use crate::domain::services::order_service::OrderService;
use crate::domain::services::user_service::UserService;

pub struct GetUserDataUseCase<T: UserRepository, U: OrderRepository> {
  user_service: UserService<T>,
  order_service: OrderService<U>,
}

impl<T: UserRepository, U: OrderRepository> GetUserDataUseCase<T, U> {
  pub fn new(user_repo: T, order_repo: U) -> Self {
    let user_service = UserService::new(user_repo);
    let order_service = OrderService::new(order_repo);
    GetUserDataUseCase {
      user_service,
      order_service,
    }
  }

  pub async fn execute(&self) -> Result<ApiResponse<UserResponse>, ApiError> {
    let total_user = self.user_service.get_user_count().await.map_err(|e| {
      error!("Error getting user data: {:?}", e);
      ApiError::InternalServerError("Error getting user data")
    })?;
    let total_price_in_cents = self.order_service.get_price_in_cents_total().await.map_err(|e| {
      error!("Error getting user data: {:?}", e);
      ApiError::InternalServerError("Error getting user data")
    })?;

    let response = UserResponse {
      user_count: total_user,
      average_value_per_user: match total_user {
        0 => 0f32,
        _ => (total_price_in_cents / total_user) as f32 / 100f32,
      },
    };

    Ok(ApiResponse::new(response))
  }
}
