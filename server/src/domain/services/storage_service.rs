use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::delete_objects::{DeleteObjectsError, DeleteObjectsOutput};
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

  pub async fn upload_file(&self, bucket_name: &str, file_path: &str, key: &str, content_type: Option<&mime::Mime>) -> Result<PutObjectOutput, SdkError<PutObjectError>> {
    self.storage_repo.upload(bucket_name, file_path, key, content_type).await
  }

  pub async fn delete_all_products(&self, bucket_name: &str) -> Result<DeleteObjectsOutput, SdkError<DeleteObjectsError>> {
    self.storage_repo.delete_all_products(bucket_name).await
  }
}
