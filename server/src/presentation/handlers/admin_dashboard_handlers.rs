use actix_web::{get, web, HttpResponse};
use log::error;
use crate::application::use_cases::get_sales_data::GetSalesDataUseCase;
use crate::infrastructure::repositories::postgres_order_repository::PostgresOrderRepository;
use crate::infrastructure::app_state::AppState;

#[get("/sales")]
pub async fn get_sales_data_handler(
  app_state: web::Data<AppState>
) -> HttpResponse {
  let order_repo = app_state.order_repo.clone();

  match GetSalesDataUseCase::new(order_repo)
    .execute().await {
    Ok(sales_data) => HttpResponse::Ok().json(sales_data),
    Err(_) => {
      error!("Error getting sales data!");
      HttpResponse::InternalServerError().body("Please try again later")
    }
  }
}
