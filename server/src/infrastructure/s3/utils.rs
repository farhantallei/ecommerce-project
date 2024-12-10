pub fn generate_url(key: &str) -> String {
  let bucket_name = std::env::var("S3_BUCKET_NAME").expect("S3_BUCKET_NAME must be set");
  let endpoint_url = std::env::var("AWS_ENDPOINT_URL").ok();

  match endpoint_url {
    Some(url) => format!("{}/{}/{}", url, bucket_name, key),
    None => format!("https://{}.s3.amazonaws.com/{}", bucket_name, key),
  }
}
