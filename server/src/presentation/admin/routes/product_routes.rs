use actix_web::web;
use crate::presentation::admin::handlers::product_handlers::{create_product_handler, get_products_handler};

pub fn routes(config: &mut web::ServiceConfig) {
  config.service(
    web::scope("/products")
      .service(get_products_handler)
      .service(create_product_handler)
  );
}
