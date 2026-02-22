use crate::cache::{Cache, CacheConfig};
use crate::client::{ClientBuilder, Context, DispatchEvent, DispatchEventType, EventHandler};
use crate::error::{CaptchaInfo, Result};
use crate::gateway::Gateway;
use crate::http::HttpClient;
use crate::model::{Message, User};
use serde_json::Value;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::Notify;

/// Main client struct for the selfbot.   
/// Handles connection to the gateway and dispatching events to the event handler.
/// Also holds an instance of the HTTP client for making API requests.
/// # Example
/// ```ignore
/// use diself::prelude::*;
///
/// struct MyHandler;
/// impl EventHandler for MyHandler {
///     async fn on_ready(&self, ctx: &Context, user: User) {
///         println!("Logged in as {}", user.tag());
///     }
/// }
///
/// let cache_config = CacheConfig {
///     cache_users: true,
///     cache_channels: true,
///     cache_guilds: true,
///     cache_relationships: true,
/// };
///async fn main() {
///     let client = Client::new("your_token_here", MyHandler).with_cache_config(cache_config);
///     // Or
///     // let client = Client::new("your_token_here", MyHandler).without_cache();
///     client.start().await.unwrap();
/// }
///
/// ```
pub struct Client {
    token: String,
    handler: Arc<dyn EventHandler>,
    http: HttpClient,
    cache: Cache,
    shutdown_requested: Arc<AtomicBool>,
    shutdown_notify: Arc<Notify>,
}

impl Client {
    pub fn builder<H>(token: impl Into<String>, handler: H) -> ClientBuilder<H>
    where
        H: EventHandler + 'static,
    {
        ClientBuilder::new(token, handler)
    }

    /// Creates a new client
    pub fn new(token: impl Into<String>, handler: impl EventHandler + 'static) -> Self {
        let token = token.into();
        let http = HttpClient::new(token.clone());
        let cache = Cache::new();
        Self::from_parts(token, Arc::new(handler), http, cache)
    }

    pub(crate) fn from_parts(
        token: String,
        handler: Arc<dyn EventHandler>,
        http: HttpClient,
        cache: Cache,
    ) -> Self {
        Self {
            token,
            handler,
            http,
            cache,
            shutdown_requested: Arc::new(AtomicBool::new(false)),
            shutdown_notify: Arc::new(Notify::new()),
        }
    }

    /// Sets cache configuration for this client
    ///
    /// # Example
    /// ```ignore
    /// use diself::prelude::*;
    /// let config = CacheConfig {
    ///     cache_users: true,
    ///     cache_channels: true,
    ///     cache_guilds: true,
    /// };
    /// let client = Client::new(token, MyHandler).with_cache_config(config);
    /// ```
    pub fn with_cache_config(mut self, config: CacheConfig) -> Self {
        self.cache = Cache::with_config(config);
        self
    }

    /// Disables caching entirely
    pub fn without_cache(mut self) -> Self {
        self.cache = Cache::with_config(CacheConfig {
            cache_users: false,
            cache_channels: false,
            cache_guilds: false,
            cache_relationships: false,
        });
        self
    }

    /// Sets a captcha handler for this client
    ///
    /// The handler will be called when Discord requires a captcha to be solved.
    /// It should return the solved captcha key.
    ///
    /// # Example   
    /// ```ignore
    /// use diself::prelude::*;
    ///
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

    /// Returns a reference to the cache
    pub fn cache(&self) -> &Cache {
        &self.cache
    }

    /// Starts the client and listens for events
    pub async fn start(&self) -> Result<()> {
        self.shutdown_requested.store(false, Ordering::SeqCst);
        tracing::info!("Starting Discord client...");

        let mut gateway = Gateway::connect(&self.token).await?;

        tracing::info!("Client connected, listening for events...");

        let ctx = Context::create(self.http.clone(), self.cache.clone()).await?;

        loop {
            if self.shutdown_requested.load(Ordering::SeqCst) {
                tracing::info!("Shutdown requested, stopping client loop");
                gateway.shutdown().await?;
                break;
            }

            let next_event = tokio::select! {
                event = gateway.next_event() => Some(event?),
                _ = self.shutdown_notify.notified() => None,
            };

            match next_event {
                Some(event) => {
                    if let Some(event) = event {
                        if let Err(e) = self.handle_event(&ctx, event).await {
                            tracing::error!("Error handling event: {}", e);
                        }
                    } else {
                        tracing::warn!("Gateway connection closed");
                        gateway.shutdown().await?;
                        break;
                    }
                }
                None => {
                    tracing::info!("Shutdown signal received, closing gateway");
                    gateway.shutdown().await?;
                    break;
                }
            }
        }

        Ok(())
    }

    pub fn shutdown(&self) {
        self.shutdown_requested.store(true, Ordering::SeqCst);
        self.shutdown_notify.notify_waiters();
    }

    async fn handle_event(&self, ctx: &Context, event: Value) -> Result<()> {
        self.handler.on_gateway_payload(ctx, &event).await;

        let op = event.get("op").and_then(|v| v.as_u64());

        // Opcode 0 = Dispatch (events)
        if op == Some(0) {
            if let Some(event_type) = event.get("t").and_then(|v| v.as_str()) {
                let sequence = event.get("s").and_then(|v| v.as_u64());
                let data = event.get("d").cloned().unwrap_or(Value::Null);
                let dispatch = DispatchEvent::from_gateway_payload(event_type, sequence, data);

                let dispatch_kind = dispatch.kind.clone();
                ctx.collectors.dispatch(dispatch.clone());
                self.handler.on_dispatch(ctx, dispatch.clone()).await;

                match dispatch_kind {
                    DispatchEventType::Ready => {
                        if let Ok(user) =
                            serde_json::from_value::<User>(dispatch.data["user"].clone())
                        {
                            ctx.cache.initialize(dispatch.data);
                            self.handler.on_ready(ctx, user).await;
                        }
                    }
                    DispatchEventType::ReadySupplemental => {
                        self.handler
                            .on_ready_supplemental(ctx, ctx.user.clone(), dispatch.data.clone())
                            .await;
                    }
                    DispatchEventType::MessageCreate => {
                        if let Ok(message) = serde_json::from_value::<Message>(dispatch.data) {
                            ctx.cache.cache_user(message.author.clone());
                            for user in &message.mentions {
                                ctx.cache.cache_user(user.clone());
                            }
                            self.handler.on_message_create(ctx, message).await;
                        }
                    }
                    DispatchEventType::MessageUpdate => {
                        if let Ok(message) = serde_json::from_value::<Message>(dispatch.data) {
                            ctx.cache.cache_user(message.author.clone());
                            self.handler.on_message_update(ctx, message).await;
                        }
                    }
                    DispatchEventType::MessageDelete => {
                        let data = dispatch.data;
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
                    DispatchEventType::Unknown(name) => {
                        tracing::trace!("Unhandled dispatch event: {}", name);
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }
}
