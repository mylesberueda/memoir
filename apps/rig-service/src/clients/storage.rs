use aws_sdk_s3::Client;
use std::ops::Deref;

type InnerClient = Client;

#[derive(Clone)]
pub(crate) struct StorageClient {
    client: InnerClient,
    bucket: String,
}

impl Deref for StorageClient {
    type Target = InnerClient;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum StorageError {
    #[error("storage error: {0}")]
    S3(String),
}

impl StorageClient {
    pub(crate) fn new(client: InnerClient, bucket: String) -> Self {
        Self { client, bucket }
    }

    #[allow(dead_code)] // Will be used later
    pub(crate) fn bucket(&self) -> &str {
        &self.bucket
    }

    /// Upload an object to S3
    pub(crate) async fn upload_object(&self, key: &str, body: Vec<u8>, content_type: &str) -> Result<(), StorageError> {
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .content_type(content_type)
            .body(body.into())
            .send()
            .await
            .map_err(|e| StorageError::S3(format!("PUT {key}: {e}")))?;

        Ok(())
    }

    #[allow(unused)]
    /// Download an object's bytes.
    pub(crate) async fn download_object(&self, key: &str) -> Result<Vec<u8>, StorageError> {
        let resp = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| StorageError::S3(format!("GET {key}: {e}")))?;

        resp.body
            .collect()
            .await
            .map(|data| data.into_bytes().to_vec())
            .map_err(|e| StorageError::S3(format!("read body {key}: {e}")))
    }

    /// Delete an object.
    pub(crate) async fn delete_object(&self, key: &str) -> Result<(), StorageError> {
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| StorageError::S3(format!("DELETE {key}: {e}")))?;
        Ok(())
    }

    #[allow(dead_code)]
    /// Check if an object exists (HEAD).
    pub(crate) async fn head_object(&self, key: &str) -> Result<bool, StorageError> {
        match self.client.head_object().bucket(&self.bucket).key(key).send().await {
            Ok(_) => Ok(true),
            Err(e) => {
                let svc_err = e.into_service_error();
                if svc_err.is_not_found() {
                    Ok(false)
                } else {
                    Err(StorageError::S3(format!("HEAD {key}: {svc_err}")))
                }
            }
        }
    }

    #[allow(dead_code)]
    /// Generate a presigned PUT URL for uploading.
    pub(crate) async fn presign_put(
        &self,
        key: &str,
        content_type: &str,
        expiry_secs: u64,
    ) -> Result<String, StorageError> {
        let config = aws_sdk_s3::presigning::PresigningConfig::builder()
            .expires_in(std::time::Duration::from_secs(expiry_secs))
            .build()
            .map_err(|e| StorageError::S3(format!("presign config: {e}")))?;

        let url = self
            .client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .content_type(content_type)
            .presigned(config)
            .await
            .map_err(|e| StorageError::S3(format!("presign PUT {key}: {e}")))?;

        Ok(url.uri().to_string())
    }

    /// Generate a presigned GET URL for downloading.
    pub(crate) async fn presign_get(&self, key: &str, expiry_secs: u64) -> Result<String, StorageError> {
        let config = aws_sdk_s3::presigning::PresigningConfig::builder()
            .expires_in(std::time::Duration::from_secs(expiry_secs))
            .build()
            .map_err(|e| StorageError::S3(format!("presign config: {e}")))?;

        let url = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .presigned(config)
            .await
            .map_err(|e| StorageError::S3(format!("presign GET {key}: {e}")))?;

        Ok(url.uri().to_string())
    }
}
