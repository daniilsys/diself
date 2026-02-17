use crate::client::{Context, EventHandler};
use crate::error::{CaptchaInfo, Result};
use crate::gateway::Gateway;
use crate::http::HttpClient;
use crate::model::{Message, User};
use serde_json::Value;
use std::sync::Arc;

pub struct Client {
    token: String,
    handler: Arc<dyn EventHandler>,
    http: HttpClient,
}

impl Client {
    /// Creates a new client
    pub fn new(token: impl Into<String>, handler: impl EventHandler + 'static) -> Self {
        let token = token.into();
        let http = HttpClient::new(token.clone());

        Self {
            token,
            handler: Arc::new(handler),
            http,
        }
    }

    /// Sets a captcha handler for this client
    ///
    /// The handler will be called when Discord requires a captcha to be solved.
    /// It should return the solved captcha key.
    ///
    /// # Example
    /// ```no_run
    /// let client = Client::new(token, MyHandler)
    ///     .with_captcha_handler(|info| async move {
    ///         println!("Captcha required: {:?}", info);
    ///         // Solve captcha and return the key
    ///         Ok("captcha_key_here".to_string())
    ///     });
    /// ```
    pub fn with_captcha_handler<F, Fut>(mut self, handler: F) -> Self
    where
        F: Fn(CaptchaInfo) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<String>> + Send + 'static,
    {
        self.http = self.http.with_captcha_handler(handler);
        self
    }

    /// Returns a reference to the HTTP client
    pub fn http(&self) -> &HttpClient {
        &self.http
    }

    /// Starts the client and listens for events
    pub async fn start(&self) -> Result<()> {
        tracing::info!("Starting Discord client...");

        let gateway = Gateway::connect(&self.token).await?;

        tracing::info!("Client connected, listening for events...");

        let ctx = Context::create(self.http.clone()).await?;

        loop {
            match gateway.next_event().await? {
                Some(event) => {
                    if let Err(e) = self.handle_event(&ctx, event).await {
                        tracing::error!("Error handling event: {}", e);
                    }
                }
                None => {
                    tracing::warn!("Gateway connection closed");
                    break;
                }
            }
        }

        Ok(())
    }

    async fn handle_event(&self, ctx: &Context, event: Value) -> Result<()> {
        let op = event.get("op").and_then(|v| v.as_u64());

        // Opcode 0 = Dispatch (events)
        if op == Some(0) {
            if let Some(event_type) = event.get("t").and_then(|v| v.as_str()) {
                let data = &event["d"];

                match event_type {
                    "READY" => {
                        if let Ok(user) = serde_json::from_value::<User>(data["user"].clone()) {
                            tracing::info!("Logged in as {}", user.tag());
                            self.handler.on_ready(ctx, user).await;
                        }
                    }
                    "MESSAGE_CREATE" => {
                        if let Ok(message) = serde_json::from_value::<Message>(data.clone()) {
                            self.handler.on_message(ctx, message).await;
                        }
                    }
                    "MESSAGE_UPDATE" => {
                        if let Ok(message) = serde_json::from_value::<Message>(data.clone()) {
                            self.handler.on_message_update(ctx, message).await;
                        }
                    }
                    "MESSAGE_DELETE" => {
                        if let (Some(channel_id), Some(message_id)) =
                            (data["channel_id"].as_str(), data["id"].as_str())
                        {
                            self.handler
                                .on_message_delete(
                                    ctx,
                                    channel_id.to_string(),
                                    message_id.to_string(),
                                )
                                .await;
                        }
                    }
                    _ => {
                        // Ignore other events for now
                        tracing::trace!("Unhandled event: {}", event_type);
                    }
                }
            }
        }

        Ok(())
    }
}
