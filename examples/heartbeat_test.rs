use discord_selfbot::gateway::Gateway;
use discord_selfbot::Result;
use std::env;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    println!("🦀 Testing Discord Gateway with IDENTIFY...\n");

    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN environment variable not set");

    println!("🔐 Connecting with token...\n");

    let gateway = Gateway::connect(token).await?;

    println!("✅ Connected and authenticated!\n");
    println!("⏳ Waiting for READY event...\n");

    let mut ready_received = false;

    // 2 minutes timeout to verify heartbeats after READY
    let start = std::time::Instant::now();

    while start.elapsed() < Duration::from_secs(120) {
        if let Some(event) = gateway.next_event().await? {
            if let Some(op) = event.get("op").and_then(|v| v.as_u64()) {
                match op {
                    0 => {
                        // Dispatch event
                        if let Some(event_type) = event.get("t").and_then(|v| v.as_str()) {
                            println!("📨 Event: {}", event_type);

                            if event_type == "READY" && !ready_received {
                                ready_received = true;
                                println!("\n🎉 READY event received!");
                                if let Some(user) = event["d"].get("user") {
                                    println!(
                                        "👤 Logged in as: {}#{}",
                                        user["username"].as_str().unwrap_or("Unknown"),
                                        user["discriminator"].as_str().unwrap_or("0000")
                                    );
                                }
                                println!(
                                    "\n⏱️  Now listening for 2 minutes to verify heartbeats...\n"
                                );
                            }
                        }
                    }
                    11 => {
                        println!("💓 Heartbeat ACK received at {:?}", start.elapsed());
                    }
                    _ => {
                        println!("❓ Unknown opcode: {}", op);
                    }
                }
            }
        } else {
            println!("⚠️ Connection closed!");
            break;
        }
    }

    println!("\n✅ Test completed!");
    println!("⏱️  Total duration: {:?}", start.elapsed());

    Ok(())
}
