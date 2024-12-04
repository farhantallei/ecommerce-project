use actix_web::{get, web, HttpResponse};
use log::error;
use crate::application::use_cases::admin::dashboard::get_product_data::GetProductDataUseCase;
use crate::application::use_cases::admin::dashboard::get_sales_data::GetSalesDataUseCase;
use crate::application::use_cases::admin::dashboard::get_user_data::GetUserDataUseCase;
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

#[get("/user")]
pub async fn get_user_data_handler(
  app_state: web::Data<AppState>
) -> HttpResponse {
  let user_repo = app_state.user_repo.clone();
  let order_repo = app_state.order_repo.clone();

  match GetUserDataUseCase::new(user_repo, order_repo)
    .execute().await {
    Ok(user_data) => HttpResponse::Ok().json(user_data),
    Err(_) => {
      error!("Error getting user data!");
      HttpResponse::InternalServerError().body("Please try again later")
    }
  }
}

#[get("/product")]
pub async fn get_product_data_handler(
  app_state: web::Data<AppState>
) -> HttpResponse {
  let product_repo = app_state.product_repo.clone();

  match GetProductDataUseCase::new(product_repo)
    .execute().await {
    Ok(product_data) => HttpResponse::Ok().json(product_data),
    Err(_) => {
      error!("Error getting product data!");
      HttpResponse::InternalServerError().body("Please try again later")
    }
  }
}
