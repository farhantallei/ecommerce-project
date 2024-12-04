use crate::application::dto::admin::dashboard::user_response::UserResponse;
use crate::domain::repositories::order_repository::OrderRepository;
use crate::domain::repositories::user_repository::UserRepository;

pub struct UserService<T: UserRepository, U: OrderRepository> {
  user_repo: T,
  order_repo: U,
}

impl<T: UserRepository, U: OrderRepository> UserService<T, U> {
  pub fn new(user_repo: T, order_repo: U) -> Self {
    UserService {
      user_repo,
      order_repo,
    }
  }

  pub async fn get_user_data(&self) -> Result<UserResponse, diesel::result::Error> {
    let amount = self.order_repo.sum_price_in_cents().await?;
    let user_count = self.user_repo.count().await?;

    Ok(UserResponse {
      user_count,
      average_value_per_user: match user_count {
        0 => 0f32,
        _ => (amount / user_count) as f32 / 100f32,
      },
    })
  }
}
