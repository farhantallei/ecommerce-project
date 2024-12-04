use crate::application::dto::admin::dashboard::user_response::UserResponse;
use crate::domain::repositories::order_repository::OrderRepository;
use crate::domain::repositories::user_repository::UserRepository;
use crate::domain::services::user_service::UserService;

pub struct GetUserDataUseCase<T: UserRepository, U: OrderRepository> {
  user_service: UserService<T, U>,
}

impl<T: UserRepository, U: OrderRepository> GetUserDataUseCase<T, U> {
  pub fn new(user_repo: T, order_repo: U) -> Self {
    let user_service = UserService::new(user_repo, order_repo);
    GetUserDataUseCase {
      user_service
    }
  }

  pub async fn execute(&self) -> Result<UserResponse, diesel::result::Error> {
    self.user_service.get_user_data().await
  }
}