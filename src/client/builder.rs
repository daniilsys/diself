use crate::cache::{Cache, CacheConfig};
use crate::client::{Client, EventHandler};
use crate::error::{CaptchaInfo, Result};
use crate::http::HttpClient;
use std::sync::Arc;

pub struct ClientBuilder<H>
where
    H: EventHandler + 'static,
{
    token: String,
    handler: H,
    http: HttpClient,
    cache_config: CacheConfig,
}

impl<H> ClientBuilder<H>
where
    H: EventHandler + 'static,
{
    pub fn new(token: impl Into<String>, handler: H) -> Self {
        let token = token.into();
        let http = HttpClient::new(token.clone());

        Self {
            token,
            handler,
            http,
            cache_config: CacheConfig::default(),
        }
    }

    pub fn with_cache_config(mut self, config: CacheConfig) -> Self {
        self.cache_config = config;
        self
    }

    pub fn without_cache(mut self) -> Self {
        self.cache_config = CacheConfig {
            cache_users: false,
            cache_channels: false,
            cache_guilds: false,
            cache_relationships: false,
        };
        self
    }

    pub fn with_captcha_handler<F, Fut>(mut self, handler: F) -> Self
    where
        F: Fn(CaptchaInfo) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<String>> + Send + 'static,
    {
        self.http = self.http.with_captcha_handler(handler);
        self
    }

    pub fn build(self) -> Client {
        let cache = Cache::with_config(self.cache_config);
        Client::from_parts(self.token, Arc::new(self.handler), self.http, cache)
    }
}
