use async_trait::async_trait;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::delete_objects::{DeleteObjectsError, DeleteObjectsOutput};
use aws_sdk_s3::operation::get_object::{GetObjectError, GetObjectOutput};
use aws_sdk_s3::operation::put_object::{PutObjectError, PutObjectOutput};

#[async_trait]
pub trait StorageRepository {
  async fn upload(&self, bucket_name: &str, file_path: &str, key: &str, content_type: Option<&mime::Mime>) -> Result<PutObjectOutput, SdkError<PutObjectError>>;
  async fn download(&self, bucket_name: &str, key: &str) -> Result<GetObjectOutput, SdkError<GetObjectError>>;
  async fn delete_all_products(&self, bucket_name: &str) -> Result<DeleteObjectsOutput, SdkError<DeleteObjectsError>>;
}
