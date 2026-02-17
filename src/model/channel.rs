use serde::{Deserialize, Serialize};

// Represents a Discord channel (text, voice, DM, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum ChannelType {
    GuildText = 0,
    DM = 1,
    GuildVoice = 2,
    GroupDM = 3,
    GuildCategory = 4,
    GuildAnnouncement = 5,
    AnnouncementThread = 10,
    PublicThread = 11,
    PrivateThread = 12,
    GuildStageVoice = 13,
    GuildDirectory = 14,
    GuildForum = 15,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    // Unique ID of the channel
    pub id: String,

    // Type of the channel (text, voice, DM, etc.)
    #[serde(rename = "type")]
    pub kind: ChannelType,

    // Id of the guild (if applicable)
    pub guild_id: Option<String>,

    // Position of the channel in the guild (if applicable)
    pub position: Option<i32>,

    // !!!!!! TODO permissions_overwrites !!!!!!!

    // Name of the channel (if applicable)
    pub name: Option<String>,

    // Topic of the channel (if applicable)
    pub topic: Option<String>,

    // Whether the channel is NSFW (if applicable)
    #[serde(default)]
    pub nsfw: bool,

    // Id of the last message sent in the channel (if applicable)
    pub last_message_id: Option<String>,

    // Bitrate (for voice channels)
    pub bitrate: Option<u64>,

    // User limit (for voice channels)
    pub user_limit: Option<u64>,

    // Rate limit per user (for text channels)
    pub rate_limit_per_user: Option<u64>,

    // recipients (for DM channels)
    pub recipients: Option<Vec<String>>,

    // Icon hash (for group DM channels)
    pub icon: Option<String>,

    // Owner ID (for group DM channels)
    pub owner_id: Option<String>,

    // Application ID (for group DM channels)
    pub application_id: Option<String>,

    // Whether the channel is managed
    #[serde(default)]
    pub managed: bool,

    // Channel's parent category ID (if applicable)
    pub parent_id: Option<String>,

    // The channel's last pinned message ID (if applicable)
    pub last_pin_timestamp: Option<String>,

    // The channel's rtc region (for voice channels)
    pub rtc_region: Option<String>,

    // The channel's video quality mode (for voice channels)
    pub video_quality_mode: Option<u8>,

    // The channel's message count (for threads)
    pub message_count: Option<u64>,

    // The channel's member count (for threads)
    pub member_count: Option<u64>,

    // Thread metdata
    pub thread_metadata: Option<ThreadMetadata>,

    // Thread member object (for threads the current user has joined)
    pub member: Option<ThreadMember>,

    // Default auto archive duration for threads in this channel (if applicable)
    pub default_auto_archive_duration: Option<u64>,

    // Permissions (for threads)
    pub permissions: Option<String>,

    // Flags
    pub flags: Option<u64>,

    // Total number of messages in the thread, even when messages are deleted (if applicable)
    pub total_messages: Option<u64>,

    // Available tags in a guild forum channel
    pub available_tags: Option<Vec<ForumTag>>,

    // Applied tags IDs in a thread in a guild forum channel
    pub applied_tags: Option<Vec<String>>,

    // Default sort order type for a guild forum channel
    pub default_sort_order: Option<u8>,

    // Default forum layout view for a guild forum channel
    pub default_forum_layout: Option<u8>,
}

impl Channel {
    pub fn is_dm(&self) -> bool {
        matches!(self.kind, ChannelType::DM | ChannelType::GroupDM)
    }

    pub fn mention(&self) -> String {
        if self.is_dm() {
            format!("<@{}>", self.id)
        } else {
            format!("<#{}>", self.id)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelMention {
    pub id: String,

    pub guild_id: String,

    pub name: String,

    #[serde(rename = "type")]
    pub kind: ChannelType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumTag {
    // The id of the tag
    pub id: String,

    // The name of the tag
    pub name: String,

    // Moderated (whether users can add this tag to their threads)
    pub moderated: bool,

    // Custom emoji ID associated with the tag (if any)
    pub emoji_id: Option<String>,

    // Emoji name associated with the tag (if any, used if emoji_id is null)
    pub emoji_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadMember {
    // The ID of the thread
    pub thread_id: String,

    // The ID of the user
    pub user_id: String,

    // The timestamp when the user joined the thread
    pub join_timestamp: String,

    // The flags for the user in the thread
    pub flags: u64,
    // TODO: GUILD + MEMBER
    // Guild member object (if the thread is in a guild and the user is a member of that guild)
    // pub member: Option<GuildMember>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadMetadata {
    // Whether the thread is archived
    pub archived: bool,

    // Timestamp when the thread was archived
    pub archive_timestamp: String,

    // Whether the thread is locked
    pub locked: bool,

    // Whether non-moderators can unarchive the thread
    pub invitable: Option<bool>,

    // Create Timestamp of the thread (for threads created before 2022-01-09)
    pub create_timestamp: Option<String>,
}
