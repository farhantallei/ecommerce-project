use diesel::Queryable;

#[derive(Debug, Queryable)]
pub struct User {
  pub id: String,
  pub email: String,
  pub created_at: i64,
  pub updated_at: Option<i64>,
}
