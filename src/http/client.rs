use crate::error::{CaptchaInfo, Error, Result};
use base64::Engine;
use rand::RngCore;
use reqwest::{Client as ReqwestClient, Method, StatusCode};
use serde::Serialize;
use serde_json::Value;
use std::fmt::Write;
use std::sync::Arc;
use std::time::{Duration, Instant};

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
    heartbeat_session: Arc<parking_lot::RwLock<HeartbeatSession>>,
}

#[derive(Debug)]
struct HeartbeatSession {
    id: String,
    created_at: Instant,
}

impl HttpClient {
    const HEARTBEAT_SESSION_TTL: Duration = Duration::from_secs(30 * 60);

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
            heartbeat_session: Arc::new(parking_lot::RwLock::new(HeartbeatSession {
                id: generate_uuid_v4_like(),
                created_at: Instant::now(),
            })),
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

    /// Returns the current analytics heartbeat session id.
    ///
    /// The id rotates automatically every 30 minutes.
    pub fn analytics_heartbeat_session_id(&self) -> String {
        self.rotate_heartbeat_session_if_needed()
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
        // Keep this heartbeat id fresh for internal analytics/debug use.
        let _heartbeat_session_id = self.rotate_heartbeat_session_if_needed();

        let mut request = self
            .client
            .request(method.clone(), url)
            .header("Authorization", &self.token)
            .header("User-Agent",   "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/145.0.0.0 Safari/537.36");

        let x_super_properties = serde_json::json!({
          "os": "Mac OS X",
          "browser": "Discord Client",
          "release_channel": "stable",
          "client_version": "0.0.377",
          "os_version": "26.3.0",
          "os_arch": "arm64",
          "app_arch": "arm64",
          "system_locale": "en-US",
          "has_client_mods": false,
          "client_launch_id": "c1f90baa-5390-43bd-a5eb-36a77d0c17c1",
          "launch_signature": "477bea01-90cb-422d-9a38-aaa66ed3e25e",
          "browser_user_agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) discord/0.0.171 Chrome/134.0.6998.179 Electron/35.1.5 Safari/537.36",
          "browser_version": "35.1.5",
          "os_sdk_version": "24",
          "client_build_number": 500334,
          "native_build_number": null,
          "client_event_source": null,
          "client_heartbeat_session_id": self.analytics_heartbeat_session_id(),
        });

        request = request.header(
            "X-Super-Properties",
            base64::engine::general_purpose::STANDARD.encode(x_super_properties.to_string()),
        );

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
                    Err(Error::CaptchaRequired(captcha_info))
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
        // Keep this heartbeat id fresh for internal analytics/debug use.
        let _heartbeat_session_id = self.rotate_heartbeat_session_if_needed();

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
                    Ok(captcha_info) => Err(Error::CaptchaRequired(captcha_info)),
                    Err(_) => {
                        // Failed to parse captcha info, treat as regular error
                        Err(Error::GatewayConnection(format!(
                            "HTTP {} - {}",
                            status, json
                        )))
                    }
                }
            } else {
                // Regular 400 error
                Err(Error::GatewayConnection(format!(
                    "HTTP {} - {}",
                    status, json
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

    fn rotate_heartbeat_session_if_needed(&self) -> String {
        {
            let session = self.heartbeat_session.read();
            if session.created_at.elapsed() < Self::HEARTBEAT_SESSION_TTL {
                return session.id.clone();
            }
        }

        let mut session = self.heartbeat_session.write();
        if session.created_at.elapsed() >= Self::HEARTBEAT_SESSION_TTL {
            session.id = generate_uuid_v4_like();
            session.created_at = Instant::now();
        }
        session.id.clone()
    }
}

fn generate_uuid_v4_like() -> String {
    let mut bytes = [0_u8; 16];
    rand::thread_rng().fill_bytes(&mut bytes);

    // RFC4122 variant + version 4 bits.
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;

    let mut out = String::with_capacity(36);
    for (i, b) in bytes.iter().enumerate() {
        let _ = write!(&mut out, "{:02x}", b);
        if i == 3 || i == 5 || i == 7 || i == 9 {
            out.push('-');
        }
    }
    out
}
