#[cfg(test)]
mod product_create_tests {
  use actix_web::test;
  use actix_web::web::Bytes;
  use serde_json::Value;
  use crate::infrastructure::web::init_app;
  use crate::presentation::admin::test::product_test_util::delete_product;
  use crate::test::test_util::{create_multipart_form_data, setup, Part};

  #[test]
  async fn should_create_product_with_valid_data() {
    setup();

    let my_app = init_app();
    let app = test::init_service(my_app).await;

    let file_data = b"Hello, World!";
    let file_name = "test.txt";

    let client = reqwest::Client::new();

    let image_url = "https://upload.wikimedia.org/wikipedia/commons/thumb/e/eb/Ash_Tree_-_geograph.org.uk_-_590710.jpg/440px-Ash_Tree_-_geograph.org.uk_-_590710.jpg";
    let response = client.get(image_url).send().await.expect("Failed to download image");

    let image_data = response.bytes().await.expect("Failed to read image bytes");
    let image_name = "test.jpg";

    let boundary = "qWeRtYuIoP";

    let (body, headers) = create_multipart_form_data(
      vec![
        ("name", Part {
          name: "name".to_string(),
          file: Bytes::from("Test Product".as_bytes()),
          filename: None,
          content_type: None,
        }),
        ("price_in_cents", Part {
          name: "price_in_cents".to_string(),
          file: Bytes::from("1000".as_bytes()),
          filename: None,
          content_type: None,
        }),
        ("file", Part {
          name: "file".to_string(),
          file: Bytes::from(file_data.to_vec()),
          filename: Some(file_name.to_string()),
          content_type: Some(mime::TEXT_PLAIN),
        }),
        ("image", Part {
          name: "image".to_string(),
          file: image_data,
          filename: Some(image_name.to_string()),
          content_type: Some(mime::IMAGE_JPEG),
        }),
        ("description", Part {
          name: "description".to_string(),
          file: Bytes::from("This is a test product".as_bytes()),
          filename: None,
          content_type: None,
        }),
      ],
      boundary,
    );

    let req = test::TestRequest::post();

    let req = headers.into_iter().fold(req, |req, hdr| req.insert_header(hdr))
      .set_payload(body)
      .uri("/api/v1/admin/products")
      .to_request();

    let resp = test::call_service(&app, req).await;

    let status_code = resp.status();
    assert_eq!(status_code, 201);

    let body_bytes = test::read_body(resp).await;

    let json: Value = serde_json::from_slice(&body_bytes).expect("Failed to parse JSON");
    assert!(json.get("data").is_some(), "Response does not contain 'data'");

    if let Some(data) = json.get("data") {
      assert!(data.get("id").is_some(), "'data' does not contain 'id'");

      if let Some(id) = data.get("id") {
        assert!(id.is_string(), "'id' is not a string");
      } else {
        panic!("'id' property is missing or not a string");
      }
    }

    delete_product().await;
  }

  #[test]
  async fn should_create_product_without_description() {
    setup();

    let my_app = init_app();
    let app = test::init_service(my_app).await;

    let file_data = b"Hello, World!";
    let file_name = "test.txt";

    let client = reqwest::Client::new();

    let image_url = "https://upload.wikimedia.org/wikipedia/commons/thumb/e/eb/Ash_Tree_-_geograph.org.uk_-_590710.jpg/440px-Ash_Tree_-_geograph.org.uk_-_590710.jpg";
    let response = client.get(image_url).send().await.expect("Failed to download image");

    let image_data = response.bytes().await.expect("Failed to read image bytes");
    let image_name = "test.jpg";

    let boundary = "qWeRtYuIoP";

    let (body, headers) = create_multipart_form_data(
      vec![
        ("name", Part {
          name: "name".to_string(),
          file: Bytes::from("Test Product".as_bytes()),
          filename: None,
          content_type: None,
        }),
        ("price_in_cents", Part {
          name: "price_in_cents".to_string(),
          file: Bytes::from("1000".as_bytes()),
          filename: None,
          content_type: None,
        }),
        ("file", Part {
          name: "file".to_string(),
          file: Bytes::from(file_data.to_vec()),
          filename: Some(file_name.to_string()),
          content_type: Some(mime::TEXT_PLAIN),
        }),
        ("image", Part {
          name: "image".to_string(),
          file: image_data,
          filename: Some(image_name.to_string()),
          content_type: Some(mime::IMAGE_JPEG),
        }),
      ],
      boundary,
    );

    let req = test::TestRequest::post();

    let req = headers.into_iter().fold(req, |req, hdr| req.insert_header(hdr))
      .set_payload(body)
      .uri("/api/v1/admin/products")
      .to_request();

    let resp = test::call_service(&app, req).await;

    let status_code = resp.status();
    assert_eq!(status_code, 201);

    let body_bytes = test::read_body(resp).await;

    let json: Value = serde_json::from_slice(&body_bytes).expect("Failed to parse JSON");
    assert!(json.get("data").is_some(), "Response does not contain 'data'");

    if let Some(data) = json.get("data") {
      assert!(data.get("id").is_some(), "'data' does not contain 'id'");

      if let Some(id) = data.get("id") {
        assert!(id.is_string(), "'id' is not a string");
      } else {
        panic!("'id' property is missing or not a string");
      }
    }

    delete_product().await;
  }

  #[test]
  async fn should_fail_create_product_due_to_invalid_image() {
    setup();

    let my_app = init_app();
    let app = test::init_service(my_app).await;

    let file_data = b"Hello, World!";
    let file_name = "test.txt";

    let boundary = "qWeRtYuIoP";

    let (body, headers) = create_multipart_form_data(
      vec![
        ("name", Part {
          name: "name".to_string(),
          file: Bytes::from("Test Product".as_bytes()),
          filename: None,
          content_type: None,
        }),
        ("price_in_cents", Part {
          name: "price_in_cents".to_string(),
          file: Bytes::from("1000".as_bytes()),
          filename: None,
          content_type: None,
        }),
        ("file", Part {
          name: "file".to_string(),
          file: Bytes::from(file_data.to_vec()),
          filename: Some(file_name.to_string()),
          content_type: Some(mime::TEXT_PLAIN),
        }),
        ("image", Part {
          name: "image".to_string(),
          file: Bytes::from(file_data.to_vec()),
          filename: Some(file_name.to_string()),
          content_type: Some(mime::TEXT_PLAIN),
        }),
        ("description", Part {
          name: "description".to_string(),
          file: Bytes::from("This is a test product".as_bytes()),
          filename: None,
          content_type: None,
        }),
      ],
      boundary,
    );

    let req = test::TestRequest::post();

    let req = headers.into_iter().fold(req, |req, hdr| req.insert_header(hdr))
      .set_payload(body)
      .uri("/api/v1/admin/products")
      .to_request();

    let resp = test::call_service(&app, req).await;

    let status_code = resp.status();
    assert_eq!(status_code, 400);

    let body_bytes = test::read_body(resp).await;

    let json: Value = serde_json::from_slice(&body_bytes).expect("Failed to parse JSON");
    assert!(json.get("code").is_some(), "Response does not contain 'code'");
    assert!(json.get("message").is_some(), "Response does not contain 'message'");

    if let Some(code) = json.get("code") {
      assert_eq!(code, 400);
    } else {
      panic!("'code' property is missing or not a number");
    }

    if let Some(message) = json.get("message") {
      assert_eq!(message, "Invalid image");
    } else {
      panic!("'message' property is missing or not a string");
    }

    delete_product().await;
  }

  #[test]
  async fn should_fail_create_product_due_to_invalid_content_type_image() {
    setup();

    let my_app = init_app();
    let app = test::init_service(my_app).await;

    let file_data = b"Hello, World!";
    let file_name = "test.txt";

    let client = reqwest::Client::new();

    let image_url = "https://upload.wikimedia.org/wikipedia/commons/thumb/e/eb/Ash_Tree_-_geograph.org.uk_-_590710.jpg/440px-Ash_Tree_-_geograph.org.uk_-_590710.jpg";
    let response = client.get(image_url).send().await.expect("Failed to download image");

    let image_data = response.bytes().await.expect("Failed to read image bytes");
    let image_name = "test.jpg";

    let boundary = "qWeRtYuIoP";

    let (body, headers) = create_multipart_form_data(
      vec![
        ("name", Part {
          name: "name".to_string(),
          file: Bytes::from("Test Product".as_bytes()),
          filename: None,
          content_type: None,
        }),
        ("price_in_cents", Part {
          name: "price_in_cents".to_string(),
          file: Bytes::from("1000".as_bytes()),
          filename: None,
          content_type: None,
        }),
        ("file", Part {
          name: "file".to_string(),
          file: Bytes::from(file_data.to_vec()),
          filename: Some(file_name.to_string()),
          content_type: Some(mime::TEXT_PLAIN),
        }),
        ("image", Part {
          name: "image".to_string(),
          file: image_data,
          filename: Some(image_name.to_string()),
          content_type: Some(mime::IMAGE_PNG),
        }),
        ("description", Part {
          name: "description".to_string(),
          file: Bytes::from("This is a test product".as_bytes()),
          filename: None,
          content_type: None,
        }),
      ],
      boundary,
    );

    let req = test::TestRequest::post();

    let req = headers.into_iter().fold(req, |req, hdr| req.insert_header(hdr))
      .set_payload(body)
      .uri("/api/v1/admin/products")
      .to_request();

    let resp = test::call_service(&app, req).await;

    let status_code = resp.status();
    assert_eq!(status_code, 400);

    let body_bytes = test::read_body(resp).await;

    let json: Value = serde_json::from_slice(&body_bytes).expect("Failed to parse JSON");
    assert!(json.get("code").is_some(), "Response does not contain 'code'");
    assert!(json.get("message").is_some(), "Response does not contain 'message'");

    if let Some(code) = json.get("code") {
      assert_eq!(code, 400);
    } else {
      panic!("'code' property is missing or not a number");
    }

    if let Some(message) = json.get("message") {
      assert_eq!(message, "Invalid image");
    } else {
      panic!("'message' property is missing or not a string");
    }

    delete_product().await;
  }
}
