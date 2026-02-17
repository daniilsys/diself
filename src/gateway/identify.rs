use serde::{Deserialize, Serialize};

//Authentication payload for Discord Gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identify {
    // User Discord token
    pub token: String,

    // Connection properties (OS, browser, device)
    pub properties: ConnectionProperties,

    // Initial presence (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence: Option<PresenceUpdate>,

    // Compression (must be false for selfbots)
    pub compress: Option<bool>,

    // Client capability (essential for selfbots)
    pub capabilities: u32,
}

// Connection properties sent in the Identify payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionProperties {
    // Operation System (e.g., "Windows", "Linux", "macOS")
    #[serde(rename = "$os")]
    pub os: String,
    // Browser (must be "Discord Selfbot" for selfbots)
    #[serde(rename = "$browser")]
    pub browser: String,

    // Device (empty for desktops)
    #[serde(rename = "$device")]
    pub device: String,

    // Sytem locale (e.g., "en-US")
    #[serde(rename = "$system_locale")]
    pub system_locale: String,

    // Navigator version (e.g., "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
    #[serde(rename = "$browser_version")]
    pub browser_version: String,

    // OS version (e.g., "10")
    #[serde(rename = "$os_version")]
    pub os_version: String,

    // Referrer (empty for selfbots)
    #[serde(rename = "$referrer")]
    pub referrer: String,

    // Referring domain (empty for selfbots)
    #[serde(rename = "$referring_domain")]
    pub referring_domain: String,

    // Release channel (e.g., "stable")
    #[serde(rename = "$release_channel")]
    pub release_channel: String,

    // Client build number (e.g., 9999)
    #[serde(rename = "$client_build_number")]
    pub client_build_number: u32,
}

impl ConnectionProperties {
    // Returning properties with default values (can be customized if needed)
    pub fn default_client() -> Self {
        Self {
            os: "Mac OS X".to_string(),
            browser: "Discord Client".to_string(),
            device: "".to_string(),
            system_locale: "en-US".to_string(),
            browser_version: "27.0.0".to_string(),
            os_version: "14.0.0".to_string(),
            referrer: "".to_string(),
            referring_domain: "".to_string(),
            release_channel: "stable".to_string(),
            client_build_number: 275530,
        }
    }
}

// Presence update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresenceUpdate {
    // Status (e.g., "online", "idle", "dnd", "invisible")
    pub status: String,

    // Timestamps
    pub since: Option<u64>,

    // Activities
    pub activities: Vec<Activity>,

    // AFK status
    pub afk: bool,
}

impl Default for PresenceUpdate {
    fn default() -> Self {
        Self {
            status: "online".to_string(),
            since: None,
            activities: vec![],
            afk: false,
        }
    }
}

// Activtiy (playing, streaming, listening, watching)
#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Activity {
    // Name of the activity (e.g., "Spotify")
    pub name: String,

    // Type of activity (0 = playing, 1 = streaming, 2 = listening, 3 = watching)
    #[serde(rename = "type")]
    pub kind: u8,

    // URL for streaming (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl Identify {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            properties: ConnectionProperties::default_client(),
            presence: Some(PresenceUpdate::default()),
            compress: Some(false),
            capabilities: 16381, // Standard capabilities for Discord clients
        }
    }
}
