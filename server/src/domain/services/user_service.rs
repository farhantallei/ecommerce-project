use diesel::result::Error;
use crate::domain::repositories::user_repository::UserRepository;

pub struct UserService<T: UserRepository> {
  user_repo: T,
}

impl<T: UserRepository> UserService<T> {
  pub fn new(user_repo: T) -> Self {
    UserService {
      user_repo,
    }
  }

  pub async fn get_user_count(&self) -> Result<i64, Error> {
    self.user_repo.count().await
  }
}
