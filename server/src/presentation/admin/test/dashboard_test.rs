#[cfg(test)]
mod dashboard_sales_tests {
  use actix_web::test;
  use serde_json::Value;
  use crate::infrastructure::web::init_app;
  use crate::test::test_util::setup;

  #[test]
  async fn test_get_sales_data_handler() {
    setup();

    let my_app = init_app();
    let app = test::init_service(my_app).await;

    let req = test::TestRequest::get().uri("/api/v1/admin/dashboard/sales").to_request();
    let resp = test::call_service(&app, req).await;

    let status_code = resp.status();
    assert_eq!(status_code, 200);

    let body_bytes = test::read_body(resp).await;

    let json: Value = serde_json::from_slice(&body_bytes).expect("Failed to parse JSON");
    assert!(json.get("data").is_some(), "Response does not contain 'data'");

    if let Some(data) = json.get("data") {
      assert!(data.get("amount").is_some(), "'data' does not contain 'amount'");
      assert!(data.get("number_of_sales").is_some(), "'data' does not contain 'number_of_sales'");

      if let Some(amount) = data.get("amount") {
        assert!(amount.is_number(), "'amount' is not a number");
      } else {
        panic!("'amount' property is missing or not a number");
      }

      if let Some(number_of_sales) = data.get("number_of_sales") {
        assert!(number_of_sales.is_number(), "'number_of_sales' is not a number");
      } else {
        panic!("'number_of_sales' property is missing or not a number");
      }
    }
  }
}

#[cfg(test)]
mod dashboard_user_tests {
  use actix_web::test;
  use serde_json::Value;
  use crate::infrastructure::web::init_app;
  use crate::test::test_util::setup;

  #[test]
  async fn test_get_user_data_handler() {
    setup();

    let my_app = init_app();
    let app = test::init_service(my_app).await;

    let req = test::TestRequest::get().uri("/api/v1/admin/dashboard/user").to_request();
    let resp = test::call_service(&app, req).await;

    let status_code = resp.status();
    assert_eq!(status_code, 200);

    let body_bytes = test::read_body(resp).await;

    let json: Value = serde_json::from_slice(&body_bytes).expect("Failed to parse JSON");
    assert!(json.get("data").is_some(), "Response does not contain 'data'");

    if let Some(data) = json.get("data") {
      assert!(data.get("user_count").is_some(), "'data' does not contain 'user_count'");
      assert!(data.get("average_value_per_user").is_some(), "'data' does not contain 'average_value_per_user'");

      if let Some(user_count) = data.get("user_count") {
        assert!(user_count.is_number(), "'user_count' is not a number");
      } else {
        panic!("'user_count' property is missing or not a number");
      }

      if let Some(average_value_per_user) = data.get("average_value_per_user") {
        assert!(average_value_per_user.is_number(), "'average_value_per_user' is not a number");
      } else {
        panic!("'average_value_per_user' property is missing or not a number");
      }
    }
  }
}

#[cfg(test)]
mod dashboard_product_tests {
  use actix_web::test;
  use serde_json::Value;
  use crate::infrastructure::web::init_app;
  use crate::test::test_util::setup;

  #[test]
  async fn test_get_product_data_handler() {
    setup();

    let my_app = init_app();
    let app = test::init_service(my_app).await;

    let req = test::TestRequest::get().uri("/api/v1/admin/dashboard/product").to_request();
    let resp = test::call_service(&app, req).await;

    let status_code = resp.status();
    assert_eq!(status_code, 200);

    let body_bytes = test::read_body(resp).await;

    let json: Value = serde_json::from_slice(&body_bytes).expect("Failed to parse JSON");
    assert!(json.get("data").is_some(), "Response does not contain 'data'");

    if let Some(data) = json.get("data") {
      assert!(data.get("active_count").is_some(), "'data' does not contain 'active_count'");
      assert!(data.get("inactive_count").is_some(), "'data' does not contain 'inactive_count'");

      if let Some(active_count) = data.get("active_count") {
        assert!(active_count.is_number(), "'active_count' is not a number");
      } else {
        panic!("'active_count' property is missing or not a number");
      }

      if let Some(inactive_count) = data.get("inactive_count") {
        assert!(inactive_count.is_number(), "'inactive_count' is not a number");
      } else {
        panic!("'inactive_count' property is missing or not a number");
      }
    }
  }
}
