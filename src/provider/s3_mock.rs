// Copyright (c) 2023 Arvid Gerstmann. All rights reserved.
use std::collections::HashMap;
use std::io::ErrorKind;

use super::s3_trait::S3Trait;

use async_trait::async_trait;
use aws_sdk_s3::error::{GetObjectError, PutObjectError};
use aws_sdk_s3::output::{get_object_output, put_object_output, GetObjectOutput, PutObjectOutput};
use aws_sdk_s3::types::ByteStream;
use aws_smithy_http::body::SdkBody;
use aws_smithy_http::byte_stream::error::Error;
use aws_smithy_http::operation::Response;
use aws_smithy_http::result::SdkError;
use bytes::Bytes;

type FileList = HashMap<String, Bytes>;

#[derive(Debug, Clone)]
pub struct S3Mock {
    buckets: HashMap<String, FileList>,
}

impl S3Mock {
    pub fn new() -> Self {
        Self {
            buckets: HashMap::new(),
        }
    }
}

impl Default for S3Mock {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl S3Trait for S3Mock {
    async fn put_object(
        &mut self,
        bucket: &str,
        key: &str,
        body: ByteStream,
    ) -> Result<PutObjectOutput, SdkError<PutObjectError, Response>> {
        let mock_bucket = self
            .buckets
            .entry(bucket.into())
            .or_insert_with(HashMap::new);

        let data: Result<Bytes, Error> = body.collect().await.map(|data| data.into_bytes());
        let bytes = data.map_err(|e| SdkError::construction_failure(Box::new(e)))?;

        let _ = mock_bucket.entry(key.into()).or_insert_with(|| bytes);

        Ok(put_object_output::Builder::default().build())
    }

    async fn get_object(
        &self,
        bucket: &str,
        key: &str,
    ) -> Result<GetObjectOutput, SdkError<GetObjectError, Response>> {
        if let Some(mock_bucket) = self.buckets.get(bucket) {
            if let Some(mock_file) = mock_bucket.get(key) {
                let byte_stream = ByteStream::new(SdkBody::from(mock_file.as_ref()));
                let ret = get_object_output::Builder::default()
                    .body(byte_stream)
                    .build();

                return Ok(ret);
            }
        }

        Err(SdkError::construction_failure(std::io::Error::new(
            ErrorKind::Other,
            "bucket not found",
        )))
    }
}
