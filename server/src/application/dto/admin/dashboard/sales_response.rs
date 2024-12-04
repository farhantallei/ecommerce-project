use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SalesResponse {
  pub amount: i64,
  pub number_of_sales: i64,
}
