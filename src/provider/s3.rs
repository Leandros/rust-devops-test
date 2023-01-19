// Copyright (c) 2023 Arvid Gerstmann. All rights reserved.
use super::s3_trait::S3Trait;

use async_trait::async_trait;
use aws_sdk_s3::error::{GetObjectError, PutObjectError};
use aws_sdk_s3::output::{GetObjectOutput, PutObjectOutput};
use aws_sdk_s3::types::ByteStream;
use aws_smithy_http::operation::Response;
use aws_smithy_http::result::SdkError;

#[derive(Debug, Clone)]
pub struct S3 {
    client: aws_sdk_s3::Client,
}

impl S3 {
    pub fn new(client: aws_sdk_s3::Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl S3Trait for S3 {
    async fn put_object(
        &mut self,
        bucket: &str,
        key: &str,
        body: ByteStream,
    ) -> Result<PutObjectOutput, SdkError<PutObjectError, Response>> {
        self.client
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(body)
            .send()
            .await
    }

    async fn get_object(
        &self,
        bucket: &str,
        key: &str,
    ) -> Result<GetObjectOutput, SdkError<GetObjectError, Response>> {
        self.client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
    }
}
