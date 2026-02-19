pub mod cache;
pub mod client;
pub mod error;
pub mod gateway;
pub mod http;
pub mod model;

pub use cache::{Cache, CacheConfig};
pub use client::{
    ChannelsManager, Client, ClientBuilder, Context, DispatchEvent, DispatchEventType,
    EventHandler, GuildsManager, RelationshipsManager, SearchThreadsParams, UsersManager,
};
pub use error::{CaptchaInfo, Error, Result};
pub use http::HttpClient;
pub use model::{Channel, Message, User};

/// Prelude module for easy imports
///
/// # Example
/// ```
/// use diself::prelude::*;
/// ```
pub mod prelude {
    pub use crate::cache::{Cache, CacheConfig};
    pub use crate::client::{
        ChannelsManager, Client, ClientBuilder, Context, DispatchEvent, DispatchEventType,
        EventHandler, GuildsManager, RelationshipsManager, SearchThreadsParams, UsersManager,
    };
    pub use crate::error::{CaptchaInfo, Error, Result};
    pub use crate::http::HttpClient;
    pub use crate::model::{Channel, Message, User};
    pub use async_trait::async_trait;
}
