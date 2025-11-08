use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::{Result, anyhow};

pub mod albums;

/// PhotoPrism API client
pub struct PhotoPrismClient {
    http_client: Client,
    base_url: String,
    username: String,
    password: String,
    session_token: Arc<RwLock<Option<String>>>,
}

impl PhotoPrismClient {
    /// Create a new PhotoPrism API client
    pub fn new(base_url: String, username: String, password: String) -> Result<Self> {
        let http_client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        Ok(Self {
            http_client,
            base_url: base_url.trim_end_matches('/').to_string(),
            username,
            password,
            session_token: Arc::new(RwLock::new(None)),
        })
    }

    /// Ensure we have a valid session token
    async fn ensure_authenticated(&self) -> Result<String> {
        // Check if we have a cached token
        {
            let token_guard = self.session_token.read().await;
            if let Some(token) = token_guard.as_ref() {
                return Ok(token.clone());
            }
        }

        // Login to get a new token
        #[derive(Serialize)]
        struct LoginRequest {
            username: String,
            password: String,
        }

        #[derive(Deserialize)]
        struct LoginResponse {
            id: String,
        }

        let login_url = format!("{}/api/v1/session", self.base_url);
        let response = self
            .http_client
            .post(&login_url)
            .json(&LoginRequest {
                username: self.username.clone(),
                password: self.password.clone(),
            })
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("Authentication failed: {}", response.status()));
        }

        let login_response: LoginResponse = response.json().await?;
        let token = login_response.id;

        // Cache the token
        {
            let mut token_guard = self.session_token.write().await;
            *token_guard = Some(token.clone());
        }

        Ok(token)
    }

    /// Make an authenticated GET request
    pub async fn get<T: for<'de> Deserialize<'de>>(&self, path: &str) -> Result<T> {
        let token = self.ensure_authenticated().await?;
        let url = format!("{}{}", self.base_url, path);

        let response = self
            .http_client
            .get(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Request failed ({}): {}", status, error_text));
        }

        Ok(response.json().await?)
    }

    /// Make an authenticated POST request
    pub async fn post<T: Serialize, R: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<R> {
        let token = self.ensure_authenticated().await?;
        let url = format!("{}{}", self.base_url, path);

        let response = self
            .http_client
            .post(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", token))
            .json(body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Request failed ({}): {}", status, error_text));
        }

        Ok(response.json().await?)
    }

    /// Make an authenticated PUT request
    pub async fn put<T: Serialize, R: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<R> {
        let token = self.ensure_authenticated().await?;
        let url = format!("{}{}", self.base_url, path);

        let response = self
            .http_client
            .put(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", token))
            .json(body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Request failed ({}): {}", status, error_text));
        }

        Ok(response.json().await?)
    }

    /// Make an authenticated DELETE request
    pub async fn delete(&self, path: &str) -> Result<()> {
        let token = self.ensure_authenticated().await?;
        let url = format!("{}{}", self.base_url, path);

        let response = self
            .http_client
            .delete(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Request failed ({}): {}", status, error_text));
        }

        Ok(())
    }

    /// Make an authenticated DELETE request with body
    pub async fn delete_with_body<T: Serialize>(&self, path: &str, body: &T) -> Result<()> {
        let token = self.ensure_authenticated().await?;
        let url = format!("{}{}", self.base_url, path);

        let response = self
            .http_client
            .delete(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", token))
            .json(body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Request failed ({}): {}", status, error_text));
        }

        Ok(())
    }

    /// Download binary data (e.g., ZIP files)
    pub async fn download(&self, path: &str) -> Result<Vec<u8>> {
        let token = self.ensure_authenticated().await?;
        let url = format!("{}{}", self.base_url, path);

        let response = self
            .http_client
            .get(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            return Err(anyhow!("Download failed: {}", status));
        }

        Ok(response.bytes().await?.to_vec())
    }
}
