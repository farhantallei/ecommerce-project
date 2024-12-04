use actix_web::web;
use crate::presentation::admin::handlers::admin_dashboard_handlers::{get_product_data_handler, get_sales_data_handler, get_user_data_handler};

pub fn routes(config: &mut web::ServiceConfig) {
  config.service(
    web::scope("/dashboard")
      .service(get_sales_data_handler)
      .service(get_user_data_handler)
      .service(get_product_data_handler)
  );
}
