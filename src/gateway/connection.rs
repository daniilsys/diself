use crate::error::{Error, Result};
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

pub struct Connection {
    pub ws: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl Connection {
    //Connecting to the Discord Gateway
    pub async fn connect(url: &str) -> Result<Self> {
        tracing::info!("Connecting to Discord Gateway at {}", url);

        let (ws, _response) = connect_async(url)
            .await
            .map_err(|e| Error::GatewayConnection(e.to_string()))?;

        tracing::info!("Successfully connected!");
        Ok(Self { ws })
    }

    pub async fn receive(&mut self) -> Result<Option<Value>> {
        while let Some(msg) = self.ws.next().await {
            let msg = msg?;

            match msg {
                Message::Text(text) => {
                    tracing::debug!("Received: {}", text);
                    let payload: Value = serde_json::from_str(&text)?;
                    return Ok(Some(payload));
                }
                Message::Close(frame) => {
                    tracing::warn!("WebSocket closed: {:?}", frame);
                    return Ok(None);
                }
                _ => {
                    //ignore other message types (binary, ping, pong)
                    continue;
                }
            }
        }
        Ok(None)
    }

    pub async fn send(&mut self, payload: &Value) -> Result<()> {
        let text = serde_json::to_string(payload)?;
        tracing::debug!("Sending: {}", text);

        self.ws.send(Message::Text(text)).await?;
        Ok(())
    }

    pub async fn close(&mut self) -> Result<()> {
        self.ws.close(None).await?;
        Ok(())
    }
}
