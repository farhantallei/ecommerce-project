use actix_web::web;
use crate::presentation::handlers::admin_dashboard_handlers::get_sales_data_handler;

pub fn routes(config: &mut web::ServiceConfig) {
  config.service(
    web::scope("/api/v1/admin/dashboard")
      .service(get_sales_data_handler)
  );
}
