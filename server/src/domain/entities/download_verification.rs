use diesel::Queryable;

#[derive(Debug, Queryable)]
pub struct DownloadVerification {
  pub id: String,
  pub product_id: String,
  pub expires_at: i64,
  pub created_at: i64,
}
