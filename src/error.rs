use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Gateway connection error: {0}")]
    GatewayConnection(String),

    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid payload received")]
    InvalidPayload,
}

pub type Result<T> = std::result::Result<T, Error>;
