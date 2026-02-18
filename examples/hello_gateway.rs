use diself::gateway::Connection;
use diself::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Actiate logging with tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    println!("🦀 Testing Discord Gateway connection...\n");

    // Discord Gateway URL with version and encoding parameters
    let gateway_url = "wss://gateway.discord.gg/?v=10&encoding=json";

    let mut connection = Connection::connect(gateway_url).await?;

    println!("✅ Connected! Waiting for HELLO...\n");

    if let Some(payload) = connection.receive().await? {
        println!("📨 Received payload:");
        println!("{}\n", serde_json::to_string_pretty(&payload)?);

        // Checking for the HELLO opcode (10)
        if let Some(op) = payload.get("op") {
            if op == 10 {
                println!("✅ Received HELLO opcode!");

                // Extracting heartbeat interval from the payload
                if let Some(interval) = payload["d"]["heartbeat_interval"].as_u64() {
                    println!("💓 Heartbeat interval: {}ms", interval);
                }
            }
        }
    }

    println!("\n🎉 Test successful!");

    Ok(())
}
