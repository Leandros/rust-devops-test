// Copyright (c) 2023 Arvid Gerstmann. All rights reserved.
use async_trait::async_trait;
use aws_sdk_s3::error::{GetObjectError, PutObjectError};
use aws_sdk_s3::output::{GetObjectOutput, PutObjectOutput};
use aws_sdk_s3::types::ByteStream;
use aws_smithy_client::SdkError;
use aws_smithy_http::operation::Response;

#[async_trait]
pub trait S3Trait {
    /// Upload a file to AWS S3.
    ///
    /// * `bucket`: Bucket ARN.
    /// * `key`: Remote "path" for the file to put.
    /// * `body`: Body of the file.
    async fn put_object(
        &mut self,
        bucket: &str,
        key: &str,
        body: ByteStream,
    ) -> Result<PutObjectOutput, SdkError<PutObjectError, Response>>;

    /// Retrieve a file from AWS S3.
    ///
    /// * `bucket`: Bucket ARN.
    /// * `key`: "Path" to the file to retrieve in S3.
    async fn get_object(
        &self,
        bucket: &str,
        key: &str,
    ) -> Result<GetObjectOutput, SdkError<GetObjectError, Response>>;
}
