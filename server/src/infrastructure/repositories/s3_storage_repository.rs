use std::path::Path;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use aws_sdk_s3::Client;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::get_object::{GetObjectError, GetObjectOutput};
use aws_sdk_s3::operation::put_object::{PutObjectError, PutObjectOutput};
use aws_sdk_s3::primitives::ByteStream;
use once_cell::sync::Lazy;
use crate::domain::repositories::storage_repository::StorageRepository;
use crate::infrastructure::s3;

static STORAGE_REPO: Lazy<Mutex<Option<S3StorageRepository>>> = Lazy::new(|| Mutex::new(None));

#[derive(Clone)]
pub struct S3StorageRepository {
  client: Client,
}

impl S3StorageRepository {
  pub fn new(client: Client) -> Self {
    S3StorageRepository { client }
  }

  pub fn get_storage_repo() -> &'static Mutex<Option<S3StorageRepository>> {
    let mut repos = STORAGE_REPO.lock().unwrap();
    if repos.is_none() {
      *repos = Some(S3StorageRepository::new(s3::connection::establish_connection()));
    }
    &*STORAGE_REPO
  }
}

#[async_trait]
impl StorageRepository for Arc<S3StorageRepository> {
  async fn upload(&self, bucket_name: &str, file_path: &str, key: &str, mime: Option<&mime::Mime>) -> Result<PutObjectOutput, SdkError<PutObjectError>> {
    let body = ByteStream::from_path(
      Path::new(file_path)
    ).await;

    let mut put_object = self.client.put_object()
      .bucket(bucket_name)
      .key(key)
      .body(body.unwrap());

    if mime.is_some() {
      let content_type = mime.map(|s| s.to_string());
      put_object = put_object.set_content_type(content_type)
    }

    put_object.send().await
  }

  async fn download(&self, bucket_name: &str, key: &str) -> Result<GetObjectOutput, SdkError<GetObjectError>> {
    self.client.get_object().bucket(bucket_name).key(key).send().await
  }
}
