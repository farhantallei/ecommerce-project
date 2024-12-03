use diesel::Queryable;

#[derive(Debug, Queryable)]
pub struct Order {
  pub id: String,
  pub user_id: String,
  pub product_id: String,
  pub price_in_cents: i32,
  pub created_at: i64,
  pub updated_at: Option<i64>,
}
