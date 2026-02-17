use crate::client::Context;
use crate::model::{Message, User};
use async_trait::async_trait;

/// Trait for handling Discord events
///
/// Implement this trait to respond to Discord events.
///
/// # Example
/// ```no_run
/// use discord_selfbot::prelude::*;
///
/// struct MyBot;
///
/// #[async_trait]
/// impl EventHandler for MyBot {
///     async fn on_ready(&self, ctx: &Context, user: User) {
///         println!("Bot is ready!");
///     }
///     
///     async fn on_message(&self, ctx: &Context, msg: Message) {
///         if msg.content == "!ping" {
///             msg.reply(&ctx.http, "Pong!").await.ok();
///         }
///     }
/// }
/// ```
#[async_trait]
pub trait EventHandler: Send + Sync {
    /// Called when the bot is ready
    async fn on_ready(&self, ctx: &Context, user: User) {
        let _ = (ctx, user);
    }

    /// Called for every new message
    async fn on_message(&self, ctx: &Context, message: Message) {
        let _ = (ctx, message);
    }

    /// Called when a message is edited
    async fn on_message_update(&self, ctx: &Context, message: Message) {
        let _ = (ctx, message);
    }

    /// Called when a message is deleted
    async fn on_message_delete(&self, ctx: &Context, channel_id: String, message_id: String) {
        let _ = (ctx, channel_id, message_id);
    }
}
