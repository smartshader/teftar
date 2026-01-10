use super::client::SupabaseClient;
use reqwest::multipart;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileObject {
    pub name: String,
    pub id: Option<String>,
    pub updated_at: Option<String>,
    pub created_at: Option<String>,
    pub last_accessed_at: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResponse {
    pub path: String,
    pub id: String,
    pub full_path: String,
}

#[derive(Debug)]
pub struct UploadOptions {
    pub cache_control: String,
    pub content_type: String,
    pub upsert: bool,
    pub metadata: Option<serde_json::Value>,
}

impl Default for UploadOptions {
    fn default() -> Self {
        Self {
            cache_control: "3600".to_string(),
            content_type: "text/plain;charset=UTF-8".to_string(),
            upsert: false,
            metadata: None,
        }
    }
}

pub enum StorageError {
    NetworkError(String),
    NotFound(String),
    Unauthorized(String),
    InvalidRequest(String),
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            StorageError::NotFound(msg) => write!(f, "Not found: {}", msg),
            StorageError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            StorageError::InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
        }
    }
}

impl SupabaseClient {
    fn storage_url(&self, path: &str) -> String {
        // Remove /auth/v1 and replace with /storage/v1
        let base_url = self.url.replace("/auth/v1", "");
        format!("{}/storage/v1{}", base_url, path)
    }

    /// Upload a file to a bucket
    pub async fn upload(
        &self,
        bucket: &str,
        path: &str,
        file_data: Vec<u8>,
        options: Option<UploadOptions>,
    ) -> Result<UploadResponse, StorageError> {
        let opts = options.unwrap_or_default();

        let mut form = multipart::Form::new().text("cacheControl", opts.cache_control.clone());

        if let Some(metadata) = opts.metadata {
            form = form.text("metadata", serde_json::to_string(&metadata).unwrap());
        }

        // Add file part (unnamed field as per Supabase spec)
        let file_part = multipart::Part::bytes(file_data)
            .mime_str(&opts.content_type)
            .map_err(|e| StorageError::InvalidRequest(e.to_string()))?;

        form = form.part("", file_part);

        let url = self.storage_url(&format!("/object/{}/{}", bucket, path));
        let method = if opts.upsert { "PUT" } else { "POST" };

        let request = if method == "PUT" {
            self.client().put(&url)
        } else {
            self.client().post(&url)
        };

        let response = request
            .header("Authorization", format!("Bearer {}", self.anon_key()))
            .multipart(form)
            .send()
            .await
            .map_err(|e| StorageError::NetworkError(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            return Err(match status.as_u16() {
                401 | 403 => StorageError::Unauthorized("Unauthorized access".to_string()),
                404 => StorageError::NotFound("Bucket or path not found".to_string()),
                _ => StorageError::InvalidRequest(format!("Upload failed with status {}", status)),
            });
        }

        let upload_response: UploadResponse = response
            .json()
            .await
            .map_err(|e| StorageError::NetworkError(format!("Failed to parse response: {}", e)))?;

        Ok(upload_response)
    }

    /// Download a file from a bucket
    pub async fn download(&self, bucket: &str, path: &str) -> Result<Vec<u8>, StorageError> {
        let url = self.storage_url(&format!("/object/{}/{}", bucket, path));

        let response = self
            .client()
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.anon_key()))
            .send()
            .await
            .map_err(|e| StorageError::NetworkError(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            return Err(match status.as_u16() {
                401 | 403 => StorageError::Unauthorized("Unauthorized access".to_string()),
                404 => StorageError::NotFound("File not found".to_string()),
                _ => {
                    StorageError::InvalidRequest(format!("Download failed with status {}", status))
                }
            });
        }

        let bytes = response
            .bytes()
            .await
            .map_err(|e| StorageError::NetworkError(format!("Failed to read bytes: {}", e)))?;

        Ok(bytes.to_vec())
    }

    /// Get a public URL for a file (works only for public buckets)
    pub fn get_public_url(&self, bucket: &str, path: &str) -> String {
        self.storage_url(&format!("/object/public/{}/{}", bucket, path))
    }

    /// Create a signed URL for temporary access (expires in seconds)
    pub async fn create_signed_url(
        &self,
        bucket: &str,
        path: &str,
        expires_in: u32,
    ) -> Result<String, StorageError> {
        let url = self.storage_url(&format!("/object/sign/{}/{}", bucket, path));

        let response = self
            .client()
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.anon_key()))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "expiresIn": expires_in
            }))
            .send()
            .await
            .map_err(|e| StorageError::NetworkError(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            return Err(StorageError::InvalidRequest(format!(
                "Failed to create signed URL with status {}",
                status
            )));
        }

        #[derive(Deserialize)]
        struct SignedUrlResponse {
            #[serde(rename = "signedURL")]
            signed_url: String,
        }

        let signed_response: SignedUrlResponse = response
            .json()
            .await
            .map_err(|e| StorageError::NetworkError(format!("Failed to parse response: {}", e)))?;

        Ok(signed_response.signed_url)
    }

    /// Delete one or more files
    pub async fn delete(
        &self,
        bucket: &str,
        paths: Vec<String>,
    ) -> Result<Vec<FileObject>, StorageError> {
        let url = self.storage_url(&format!("/object/{}", bucket));

        let response = self
            .client()
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.anon_key()))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "prefixes": paths
            }))
            .send()
            .await
            .map_err(|e| StorageError::NetworkError(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            return Err(StorageError::InvalidRequest(format!(
                "Delete failed with status {}",
                status
            )));
        }

        let deleted_files: Vec<FileObject> = response
            .json()
            .await
            .map_err(|e| StorageError::NetworkError(format!("Failed to parse response: {}", e)))?;

        Ok(deleted_files)
    }

    /// List files in a bucket path
    pub async fn list(
        &self,
        bucket: &str,
        path: Option<&str>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<FileObject>, StorageError> {
        let url = self.storage_url(&format!("/object/list/{}", bucket));

        let mut body = serde_json::json!({
            "limit": limit.unwrap_or(100),
            "offset": offset.unwrap_or(0)
        });

        if let Some(prefix) = path {
            body["prefix"] = serde_json::json!(prefix);
        }

        let response = self
            .client()
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.anon_key()))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| StorageError::NetworkError(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            return Err(StorageError::InvalidRequest(format!(
                "List failed with status {}",
                status
            )));
        }

        let files: Vec<FileObject> = response
            .json()
            .await
            .map_err(|e| StorageError::NetworkError(format!("Failed to parse response: {}", e)))?;

        Ok(files)
    }

    /// Move a file
    pub async fn move_file(
        &self,
        bucket: &str,
        from_path: &str,
        to_path: &str,
    ) -> Result<String, StorageError> {
        let url = self.storage_url("/object/move");

        let response = self
            .client()
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.anon_key()))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "bucketId": bucket,
                "sourceKey": from_path,
                "destinationKey": to_path
            }))
            .send()
            .await
            .map_err(|e| StorageError::NetworkError(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            return Err(StorageError::InvalidRequest(format!(
                "Move failed with status {}",
                status
            )));
        }

        #[derive(Deserialize)]
        struct MoveResponse {
            message: String,
        }

        let move_response: MoveResponse = response
            .json()
            .await
            .map_err(|e| StorageError::NetworkError(format!("Failed to parse response: {}", e)))?;

        Ok(move_response.message)
    }

    /// Copy a file
    pub async fn copy_file(
        &self,
        bucket: &str,
        from_path: &str,
        to_path: &str,
    ) -> Result<String, StorageError> {
        let url = self.storage_url("/object/copy");

        let response = self
            .client()
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.anon_key()))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "bucketId": bucket,
                "sourceKey": from_path,
                "destinationKey": to_path
            }))
            .send()
            .await
            .map_err(|e| StorageError::NetworkError(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            return Err(StorageError::InvalidRequest(format!(
                "Copy failed with status {}",
                status
            )));
        }

        #[derive(Deserialize)]
        struct CopyResponse {
            path: String,
        }

        let copy_response: CopyResponse = response
            .json()
            .await
            .map_err(|e| StorageError::NetworkError(format!("Failed to parse response: {}", e)))?;

        Ok(copy_response.path)
    }
}
