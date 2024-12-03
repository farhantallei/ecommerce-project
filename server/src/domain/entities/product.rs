use diesel::Queryable;

#[derive(Debug, Queryable)]
pub struct Product {
  pub id: String,
  pub name: String,
  pub price_in_cents: i32,
  pub file_path: String,
  pub image_path: String,
  pub description: Option<String>,
  pub is_available_for_purchase: bool,
  pub created_at: i64,
  pub updated_at: Option<i64>,
}
