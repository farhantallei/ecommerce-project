use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use diesel::{Insertable, Queryable};
use serde::Serialize;
use crate::domain::entities::timestamp;
use crate::schema::products;

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

#[derive(Debug, MultipartForm)]
pub struct NewProductRequest {
  pub name: Text<String>,
  pub price_in_cents: Text<i32>,
  pub file: TempFile,
  pub image: TempFile,
  pub description: Option<Text<String>>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = products)]
pub struct NewProduct {
  pub id: String,
  pub name: String,
  pub price_in_cents: i32,
  pub file_path: String,
  pub image_path: String,
  pub description: Option<String>,
  pub created_at: i64,
}

impl NewProduct {
  pub fn new(
    id: String,
    name: String,
    price_in_cents: i32,
    file_path: String,
    image_path: String,
    description: Option<String>,
  ) -> Self {
    NewProduct {
      id,
      name,
      price_in_cents,
      file_path,
      image_path,
      description,
      created_at: timestamp::get_created_at(),
    }
  }
}

#[derive(Debug, Serialize)]
pub struct ProductResponse {
  pub id: String,
}
