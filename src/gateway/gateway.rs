use crate::error::{Error, Result};
use crate::gateway::{Connection, Heartbeat, Identify};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct Gateway {
    connection: Arc<RwLock<Connection>>,
    sequence: Arc<RwLock<Option<u64>>>,
}

impl Gateway {
    //Connect to the Discord Gateway and start heartbeating
    pub async fn connect(token: impl Into<String>) -> Result<Self> {
        let url = "wss://gateway.discord.gg/?v=10&encoding=json";
        let mut connection = Connection::connect(url).await?;

        // 1. Receive HELLO
        let hello = connection.receive().await?.ok_or(Error::InvalidPayload)?;

        if hello.get("op") != Some(&json!(10)) {
            return Err(Error::InvalidPayload);
        }

        let heartbeat_interval = hello["d"]["heartbeat_interval"]
            .as_u64()
            .ok_or(Error::InvalidPayload)?;

        tracing::info!(
            "Received HELLO with heartbeat interval: {}ms",
            heartbeat_interval
        );

        // 2. Send IDENTIFY
        let identify = Identify::new(token);
        let identify_payload = json!({
            "op": 2,
            "d": identify,
        });
        tracing::debug!(
            "IDENTIFY payload: {}",
            serde_json::to_string_pretty(&identify_payload).unwrap_or_default()
        );
        connection.send(&identify_payload).await?;
        tracing::info!("Sent IDENTIFY payload");

        // 3. Preparing for heartbeating
        let heartbeat = Heartbeat::new(heartbeat_interval);
        let hearbeat_seq = heartbeat.sequence();

        let connection = Arc::new(RwLock::new(connection));

        // 4. Start heartbeating in a separate task
        let hearbeat_connection = connection.clone();
        tokio::spawn(async move {
            let result = heartbeat
                .start(move |payload| {
                    let conn = hearbeat_connection.clone();
                    tokio::task::block_in_place(|| {
                        tokio::runtime::Handle::current()
                            .block_on(async { conn.write().await.send(&payload).await })
                    })
                })
                .await;
            if let Err(e) = result {
                tracing::error!("Heartbeat task failed: {}", e);
            }
        });

        Ok(Self {
            connection,
            sequence: hearbeat_seq,
        })
    }

    pub async fn next_event(&self) -> Result<Option<Value>> {
        let mut conn = self.connection.write().await;

        if let Some(payload) = conn.receive().await? {
            // Update sequence number if present
            if let Some(seq) = payload.get("s").and_then(|s| s.as_u64()) {
                *self.sequence.write().await = Some(seq);
                tracing::trace!("Updated sequence to {}", seq);
            }
            Ok(Some(payload))
        } else {
            Ok(None)
        }
    }
}
