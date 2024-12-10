use actix_web::web;
use crate::presentation::admin;

pub fn routes(config: &mut web::ServiceConfig) {
  config.service(
    web::scope("/admin")
      .configure(admin::routes::documentation_routes::routes)
      .configure(admin::routes::dashboard_routes::routes)
      .configure(admin::routes::product_routes::routes)
  );
}
