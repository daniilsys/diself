use discord_selfbot::prelude::*;
use std::env;

struct ChannelBot;

#[async_trait]
impl EventHandler for ChannelBot {
    async fn on_ready(&self, _ctx: &Context, user: User) {
        println!("🤖 {} is ready!", user.tag());
    }

    async fn on_message(&self, ctx: &Context, msg: Message) {
        // Ignore other user's messages
        if msg.author.id != ctx.user.id {
            return;
        }
        if let Some((command, _args)) = msg.parse_command(".") {
            match command {
                "copyavatar" => {
                    let mention_user = msg.mentions.first().clone();

                    if let Some(user) = mention_user {
                        if let Some(avatar_url) = user.avatar_url() {
                            let data_uri = ctx.download_image_as_data_uri(avatar_url).await;
                            if let Err(e) = ctx.update_avatar_from_data_uri(data_uri.unwrap()).await
                            {
                                eprintln!("Failed to change avatar: {}", e);
                            } else {
                                msg.reply(
                                    &ctx.http,
                                    format!("✅ Avatar updated to match {}", user.tag()),
                                )
                                .await
                                .unwrap();
                            }
                        } else {
                            msg.reply(
                                &ctx.http,
                                format!("⚠️ {} does not have an avatar to copy", user.tag()),
                            )
                            .await
                            .unwrap();
                        }
                    } else {
                        msg.reply(
                            &ctx.http,
                            "⚠️ Please mention a user to copy their avatar".to_string(),
                        )
                        .await
                        .unwrap();
                    }
                }
                _ => {
                    // Ignore unknown commands
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");

    println!("🦀 Starting Channel Methods Bot...\n");

    // Create client with captcha handler
    let client = Client::new(token, ChannelBot).with_captcha_handler(|captcha_info| async move {
        // This is where you would implement your captcha solving logic
        // For example, using a captcha solving service or manual input

        eprintln!("⚠️  Captcha required!");
        eprintln!("Service: {}", captcha_info.captcha_service);
        eprintln!("Sitekey: {}", captcha_info.captcha_sitekey);

        if let Some(session_id) = &captcha_info.captcha_session_id {
            eprintln!("Session ID: {}", session_id);
        }
        if let Some(rqdata) = &captcha_info.captcha_rqdata {
            eprintln!("RQ Data: {}", rqdata);
        }
        if let Some(rqtoken) = &captcha_info.captcha_rqtoken {
            eprintln!("RQ Token: {}", rqtoken);
        }

        // Example: Read captcha solution from stdin (you would implement proper solving here)
        eprintln!("\n🔑 Enter the solved captcha key:");
        let mut captcha_solution = String::new();
        std::io::stdin()
            .read_line(&mut captcha_solution)
            .map_err(|e| {
                discord_selfbot::Error::CaptchaHandlerFailed(format!("Failed to read input: {}", e))
            })?;

        Ok(captcha_solution.trim().to_string())
    });

    client.start().await?;

    Ok(())
}
