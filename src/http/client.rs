use crate::error::{CaptchaInfo, Error, Result};
use reqwest::{Client as ReqwestClient, Method, StatusCode};
use serde::Serialize;
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;

/// Type for captcha handler callback
/// Takes captcha info and returns the solved captcha key
pub type CaptchaHandler = Arc<
    dyn Fn(
            CaptchaInfo,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String>> + Send>>
        + Send
        + Sync,
>;

/// Minimal HTTP client for Discord API
#[derive(Clone)]
pub struct HttpClient {
    token: String,
    client: ReqwestClient,
    captcha_handler: Option<CaptchaHandler>,
}

impl HttpClient {
    /// Creates a new HTTP client
    pub fn new(token: impl Into<String>) -> Self {
        let client = ReqwestClient::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            token: token.into(),
            client,
            captcha_handler: None,
        }
    }

    /// Sets a captcha handler for this HTTP client
    pub fn with_captcha_handler<F, Fut>(mut self, handler: F) -> Self
    where
        F: Fn(CaptchaInfo) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<String>> + Send + 'static,
    {
        self.captcha_handler = Some(Arc::new(move |info| Box::pin(handler(info))));
        self
    }

    /// Sends a GET request
    pub async fn get(&self, url: impl AsRef<str>) -> Result<Value> {
        self.request(Method::GET, url.as_ref(), None::<&()>).await
    }

    /// Sends a POST request
    pub async fn post<T: Serialize>(&self, url: impl AsRef<str>, body: T) -> Result<Value> {
        self.request(Method::POST, url.as_ref(), Some(&body)).await
    }

    /// Sends a PATCH request
    pub async fn patch<T: Serialize>(&self, url: impl AsRef<str>, body: T) -> Result<Value> {
        self.request(Method::PATCH, url.as_ref(), Some(&body)).await
    }

    /// Sends a PUT request
    pub async fn put<T: Serialize>(&self, url: impl AsRef<str>, body: T) -> Result<Value> {
        self.request(Method::PUT, url.as_ref(), Some(&body)).await
    }

    /// Sends a DELETE request
    pub async fn delete(&self, url: impl AsRef<str>) -> Result<Value> {
        self.request(Method::DELETE, url.as_ref(), None::<&()>)
            .await
    }

    /// Generic HTTP request
    async fn request<T: Serialize>(
        &self,
        method: Method,
        url: &str,
        body: Option<&T>,
    ) -> Result<Value> {
        self.request_with_captcha(method, url, body, None).await
    }

    /// Generic HTTP request with optional captcha key
    async fn request_with_captcha<T: Serialize>(
        &self,
        method: Method,
        url: &str,
        body: Option<&T>,
        captcha_key: Option<String>,
    ) -> Result<Value> {
        let mut request = self
            .client
            .request(method.clone(), url)
            .header("Authorization", &self.token)
            .header("User-Agent", "Discord Client (diself, 0.1.0)");

        // Prepare body with captcha key if provided
        if let Some(body) = body {
            let mut json_body = serde_json::to_value(body)?;
            if let Some(ref key) = captcha_key {
                if let Some(obj) = json_body.as_object_mut() {
                    obj.insert("captcha_key".to_string(), Value::String(key.clone()));
                }
            }
            request = request.json(&json_body);
        } else if let Some(key) = captcha_key {
            // No body but we have captcha key
            request = request.json(&serde_json::json!({ "captcha_key": key }));
        }

        let response = request.send().await?;

        // Handle response, check for captcha
        match self.handle_response(response).await {
            Err(Error::CaptchaRequired(captcha_info)) => {
                // Try to solve captcha if handler is available
                if let Some(ref handler) = self.captcha_handler {
                    tracing::info!("Captcha required, calling handler...");
                    // Clone the fields we need before moving captcha_info
                    let session_id = captcha_info.captcha_session_id.clone();
                    let rqtoken = captcha_info.captcha_rqtoken.clone();
                    let solved_key = handler(captcha_info).await?;
                    tracing::info!("Captcha solved, retrying request...");
                    // Retry the request with the captcha key using Box::pin for recursion
                    let body_json = if let Some(b) = body {
                        Some(serde_json::to_value(b)?)
                    } else {
                        None
                    };
                    return Box::pin(self.request_with_captcha_value(
                        method,
                        url,
                        body_json,
                        Some(solved_key),
                        session_id,
                        rqtoken,
                    ))
                    .await;
                } else {
                    // No handler available
                    return Err(Error::CaptchaRequired(captcha_info));
                }
            }
            result => result,
        }
    }

    /// Helper for recursion with owned values
    async fn request_with_captcha_value(
        &self,
        method: Method,
        url: &str,
        body: Option<Value>,
        captcha_key: Option<String>,
        captcha_session_id: Option<String>,
        captcha_rqtoken: Option<String>,
    ) -> Result<Value> {
        let mut request = self
            .client
            .request(method, url)
            .header("Authorization", &self.token)
            .header("User-Agent", "Discord Client (diself, 0.1.0)")
            .header("X-Captcha-Key", captcha_key.clone().unwrap_or_default());

        if let Some(session_id) = captcha_session_id {
            request = request.header("X-Captcha-Session-Id", session_id);
        }
        if let Some(rqtoken) = captcha_rqtoken {
            request = request.header("X-Captcha-RqToken", rqtoken);
        }

        // Prepare body with captcha key if provided
        if let Some(mut json_body) = body {
            if let Some(ref key) = captcha_key {
                if let Some(obj) = json_body.as_object_mut() {
                    obj.insert("captcha_key".to_string(), Value::String(key.clone()));
                }
            }
            request = request.json(&json_body);
        } else if let Some(key) = captcha_key {
            // No body but we have captcha key
            request = request.json(&serde_json::json!({ "captcha_key": key }));
        }

        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Handles HTTP response
    async fn handle_response(&self, response: reqwest::Response) -> Result<Value> {
        let status = response.status();

        if status.is_success() {
            // If no content (204 No Content), return null
            if status == StatusCode::NO_CONTENT {
                return Ok(Value::Null);
            }

            let json = response.json::<Value>().await?;
            Ok(json)
        } else if status == StatusCode::TOO_MANY_REQUESTS {
            // Rate limit
            let json = response.json::<Value>().await?;
            let retry_after = json["retry_after"].as_f64().unwrap_or(1.0);
            Err(Error::RateLimit { retry_after })
        } else if status == StatusCode::BAD_REQUEST {
            // Check if it's a captcha error
            let json = response.json::<Value>().await?;

            if json.get("captcha_sitekey").is_some() {
                // It's a captcha error, try to deserialize
                match serde_json::from_value::<CaptchaInfo>(json.clone()) {
                    Ok(captcha_info) => {
                        return Err(Error::CaptchaRequired(captcha_info));
                    }
                    Err(_) => {
                        // Failed to parse captcha info, treat as regular error
                        return Err(Error::GatewayConnection(format!(
                            "HTTP {} - {}",
                            status,
                            json.to_string()
                        )));
                    }
                }
            } else {
                // Regular 400 error
                Err(Error::GatewayConnection(format!(
                    "HTTP {} - {}",
                    status,
                    json.to_string()
                )))
            }
        } else {
            let text = response.text().await.unwrap_or_default();
            Err(Error::GatewayConnection(format!(
                "HTTP {} - {}",
                status, text
            )))
        }
    }
}
