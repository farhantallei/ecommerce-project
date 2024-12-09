use actix_web::{get, web, HttpResponse, Responder};
use crate::application::use_cases::admin::dashboard::get_product_data::GetProductDataUseCase;
use crate::application::use_cases::admin::dashboard::get_sales_data::GetSalesDataUseCase;
use crate::application::use_cases::admin::dashboard::get_user_data::GetUserDataUseCase;
use crate::infrastructure::repositories::postgres_order_repository::PostgresOrderRepository;
use crate::infrastructure::repositories::postgres_product_repository::PostgresProductRepository;
use crate::infrastructure::repositories::postgres_user_repository::PostgresUserRepository;

#[get("/sales")]
pub async fn get_sales_data_handler(
  order_repo: web::Data<PostgresOrderRepository>
) -> impl Responder {
  match GetSalesDataUseCase::new(order_repo.into_inner()).execute().await {
    Ok(sales_data) => Ok(HttpResponse::Ok().json(sales_data)),
    Err(e) => Err(e)
  }
}

#[get("/user")]
pub async fn get_user_data_handler(
  user_repo: web::Data<PostgresUserRepository>,
  order_repo: web::Data<PostgresOrderRepository>,
) -> impl Responder {
  match GetUserDataUseCase::new(user_repo.into_inner(), order_repo.into_inner())
    .execute().await {
    Ok(user_data) => Ok(HttpResponse::Ok().json(user_data)),
    Err(e) => Err(e)
  }
}

#[get("/product")]
pub async fn get_product_data_handler(
  product_repo: web::Data<PostgresProductRepository>,
) -> impl Responder {
  match GetProductDataUseCase::new(product_repo.into_inner())
    .execute().await {
    Ok(product_data) => Ok(HttpResponse::Ok().json(product_data)),
    Err(e) => Err(e)
  }
}
