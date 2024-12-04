import boto3
import os

s3_client = boto3.client(
    "s3",
    endpoint_url=f"http://localhost:4566",
    aws_access_key_id="test",
    aws_secret_access_key="test"
)

bucket_name = os.getenv("S3_BUCKET_NAME")

if not bucket_name:
  print("S3_BUCKET_NAME environment variable is not set. Exiting.")
  exit(1)

s3_client.create_bucket(Bucket=bucket_name)
