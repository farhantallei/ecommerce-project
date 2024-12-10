use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::put_object::{PutObjectError, PutObjectOutput};
use crate::domain::repositories::storage_repository::StorageRepository;

pub struct StorageService<T> {
  storage_repo: T,
}

impl<T: StorageRepository> StorageService<T> {
  pub fn new(storage_repo: T) -> Self {
    StorageService {
      storage_repo,
    }
  }

  pub async fn upload_file(&self, bucket_name: &str, file_path: &str, key: &str) -> Result<PutObjectOutput, SdkError<PutObjectError>> {
    self.storage_repo.upload(bucket_name, file_path, key).await
  }
}
