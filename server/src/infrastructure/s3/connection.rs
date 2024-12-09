use std::env;
use aws_sdk_s3::{Client};
use aws_sdk_s3::config::{Credentials, Region};

pub fn establish_connection() -> Client {
  let access_key = env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID must be set");
  let secret_key = env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY must be set");
  let region = env::var("AWS_REGION").unwrap_or_else(|_| "us-west-2".to_string());
  let profile = std::env::var("AWS_PROFILE").unwrap_or_else(|_| "default".to_string());
  let endpoint_url = std::env::var("AWS_ENDPOINT_URL").ok();

  let profile_str: &'static str = Box::leak(profile.into_boxed_str());

  let creds = Credentials::new(
    access_key,
    secret_key,
    None,
    None,
    profile_str,
  );

  let mut config = aws_sdk_s3::Config::builder()
    .region(Region::new(region))
    .credentials_provider(creds);

  if endpoint_url.is_some() {
    config = config.endpoint_url(endpoint_url.unwrap());
    config = config.force_path_style(true);
  }

  Client::from_conf(config.build())
}
