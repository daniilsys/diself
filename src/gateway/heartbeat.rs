use crate::error::Result;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};

pub struct Heartbeat {
    interval_ms: u64,
    sequence: Arc<RwLock<Option<u64>>>,
}

impl Heartbeat {
    pub fn new(interval_ms: u64) -> Self {
        Self {
            interval_ms,
            sequence: Arc::new(RwLock::new(None)),
        }
    }

    pub fn sequence(&self) -> Arc<RwLock<Option<u64>>> {
        self.sequence.clone()
    }

    pub async fn start<F>(self, mut send_fn: F) -> Result<()>
    where
        F: FnMut(serde_json::Value) -> Result<()> + Send,
    {
        let mut ticker = interval(Duration::from_millis(self.interval_ms));

        let jitter = (self.interval_ms as f64 * 0.1 * rand::random::<f64>()).round() as u64;
        tokio::time::sleep(Duration::from_millis(jitter)).await;

        tracing::info!("Heartbeat started with interval {}ms", self.interval_ms);

        loop {
            ticker.tick().await;

            let seq = *self.sequence.read().await;
            let payload = json!({
                "op": 1,
                "d": seq,
            });

            tracing::debug!("Sending heartbeat (seq: {:?})", seq);
            send_fn(payload)?;
        }
    }
}
