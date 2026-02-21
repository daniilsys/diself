use diself::prelude::*;
use std::env;

struct MyBot;

#[async_trait]
impl EventHandler for MyBot {
    async fn on_ready(&self, _ctx: &Context, user: User) {
        println!("🤖 {} is ready!", user.tag())
    }

    async fn on_message_create(&self, ctx: &Context, msg: Message) {
        if msg.author.id != ctx.user.id {
            return;
        }
        let (command, args) = if let Some(content) = msg.content.strip_prefix('!') {
            let mut parts = content.split_whitespace();
            let command = parts.next().unwrap_or("");
            let args: Vec<&str> = parts.collect();
            (command, args)
        } else {
            return;
        };

        match command {
            "ping" => {
                let _ = msg.reply(&ctx.http, "Pong!").await;
            }
            "echo" => {
                let response = args.join(" ");
                let _ = msg.reply(&ctx.http, response).await;
            }
            _ => {
                let _ = msg
                    .reply(
                        &ctx.http,
                        "Unknown command. Try `!ping` or `!echo <message>`.",
                    )
                    .await;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");

    println!("🦀 Starting Bot...\n");

    let client = Client::new(token, MyBot);
    client.start().await?;
    Ok(())
}
