pub mod client;
pub mod error;
pub mod gateway;
pub mod http;
pub mod model;

pub use client::{Client, Context, EventHandler};
pub use error::{CaptchaInfo, Error, Result};
pub use http::HttpClient;
pub use model::{Channel, Message, User};

/// Prelude module for easy imports
///
/// # Example
/// ```
/// use discord_selfbot::prelude::*;
/// ```
pub mod prelude {
    pub use crate::client::{Client, Context, EventHandler};
    pub use crate::error::{CaptchaInfo, Error, Result};
    pub use crate::http::HttpClient;
    pub use crate::model::{Channel, Message, User};
    pub use async_trait::async_trait;
}
