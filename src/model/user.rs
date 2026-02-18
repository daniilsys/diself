use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// User unique ID
    pub id: String,

    /// Username
    pub username: String,

    /// Discriminator (deprecated)
    pub discriminator: String,

    /// User's display name (if any)
    pub global_name: Option<String>,

    // Avatar hash
    pub avatar: Option<String>,

    #[serde(default)]
    /// Whether the user is a bot
    pub bot: bool,

    #[serde(default)]
    /// Whether the user is a system user (e.g., official Discord accounts)
    pub system: bool,

    // Whether the user has MFA enabled
    #[serde(default)]
    pub mfa_enabled: bool,

    /// User's banner hash (if any)
    pub banner: Option<String>,

    /// User's accent color (if any)
    pub accent_color: Option<u32>,

    /// User's locale (e.g., "en-US")
    pub locale: Option<String>,

    /// Whether the user has verified their email (only for the current user)
    pub verified: Option<bool>,

    /// Email (only for the current user, requires "email" scope)
    pub email: Option<String>,

    /// Phone number (only for the current user, requires "phone" scope)
    pub phone: Option<String>,

    /// Whether the use has used the desktop client before
    #[serde(default)]
    pub desktop: bool,

    /// Whether the user has used the mobile client before
    #[serde(default)]
    pub mobile: bool,

    /// Flags (bitfield representing user features)
    pub flags: Option<u64>,

    /// Premium type (0 = none, 1 = Nitro Classic, 2 = Nitro)
    pub premium_type: Option<u8>,

    /// Public flags (bitfield representing public user features)
    pub public_flags: Option<u64>,

    /// Avatar decoration data (if any)
    pub avatar_decoration: Option<AvatarDecoration>,

    /// Data for the user's collectibles (if any)
    #[serde(default)]
    pub collectibles: Option<Nameplate>,

    /// The user's primary guild
    #[serde(default)]
    pub primary_guild: Option<PrimaryGuild>,
}

impl User {
    /// Returns the user's tag (username#discriminator)
    pub fn tag(&self) -> String {
        format!("{}#{}", self.username, self.discriminator)
    }

    /// Returns the URL of the user's avatar (if any)
    pub fn avatar_url(&self) -> Option<String> {
        self.avatar.as_ref().map(|hash| {
            format!(
                "https://cdn.discordapp.com/avatars/{}/{}.png",
                self.id, hash
            )
        })
    }
    /// Returns the URL of the user's banner (if any)
    pub fn banner_url(&self) -> Option<String> {
        self.banner.as_ref().map(|hash| {
            let extension = if hash.starts_with("a_") { "gif" } else { "png" };
            format!(
                "https://cdn.discordapp.com/banners/{}/{}.{}",
                self.id, hash, extension
            )
        })
    }

    /// Returns a string representation of the user's mention (e.g., "<@123456789>")
    pub fn mention(&self) -> String {
        format!("<@{}>", self.id)
    }

    /// Checks if the user has any form of Nitro subscription
    pub fn has_nitro(&self) -> bool {
        matches!(self.premium_type, Some(1) | Some(2) | Some(3))
    }

    /// Returns a human-readable name for the user's Nitro subscription (if any)
    pub fn premium_type_name(&self) -> &str {
        match self.premium_type {
            Some(1) => "Nitro Classic",
            Some(2) => "Nitro",
            Some(3) => "Nitro Basic",
            _ => "None",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvatarDecoration {
    /// The avatar decoration hash
    pub asset: String,

    /// ID of the avatar decoration's SKU (if any)
    pub sku_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nameplate {
    /// ID of the nameplate SKU
    pub sku_id: String,

    /// Path to the nameplate asset
    pub asset: String,

    /// The label of this nameplate (Currently unused)
    pub label: String,

    /// Background color of the nameplate (crimson, berry, sky, teal, forest, bubble_gum, violet, cobalt, clover, lemon, white)
    pub palette: String,

    /// Unix timestamp of when the current nameplate expires (if any)
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimaryGuild {
    /// User's primary guild ID
    pub identity_guild_id: Option<String>,

    /// Whether the user is displaying the primary guild's server tag. This can be null if the system clears the identity, e.g. the server no longer supports tags. This will be false if the user manually removes their tag.
    pub identity_enabled: Option<bool>,

    /// The text of the user's server tag. Limited to 4 characters.
    pub tag: Option<String>,

    /// The sevrer tag badge hash
    pub badge: Option<String>,
}
