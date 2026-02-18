use super::{Channel, Emoji, Nameplate, Permissions, Role, Sticker, User};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GuildFeatures {
    AnimatedBanner,
    AnimatedIcon,
    ApplicationCommandPermissionsV2,
    AutoModeration,
    Banner,
    Community,
    CreatorMonetizableProvisions,
    DeveloperSupportServer,
    Discoverable,
    Featurable,
    InviteSplash,
    InviteDisable,
    MemberVerificationGateEnabled,
    MoreSoundboard,
    MoreStickers,
    News,
    Partnered,
    PreviewEnabled,
    RaidAlertsDisabled,
    RoleIcons,
    RoleSubscriptionsAvailableForPurchase,
    RoleSubscriptionsEnabled,
    Soundboard,
    TicketedEventsEnabled,
    VanityUrl,
    Verified,
    VipRegions,
    WelcomeScreenEnabled,
    GuestsEnabled,
    GuildTags,
    EnhancedRoleColors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guild {
    /// Guild unique ID
    /// Example: "123456789012345678"
    pub id: String,

    /// Guild name
    pub name: Option<String>,

    /// Icon hash (if the guild has an icon)
    pub icon_hash: Option<String>,

    /// Splash hash
    pub splash: Option<String>,

    /// Guild's member count
    member_count: Option<u64>,

    /// Discord splash hash; only present for guilds with the "DISCOVERABLE" feature
    pub discovery_splash: Option<String>,

    /// Whether the user is the owner of the guild
    #[serde(default)]
    pub owner: bool,

    /// Id of the owner user
    pub owner_id: Option<String>,

    /// Total permissions for the user in the guild (bitfield)
    pub permissions: Option<Permissions>,

    /// Voice region ID for the guild (deprecated)
    pub region: Option<String>,

    /// ID of the AFK channel
    pub afk_channel_id: Option<String>,

    /// AFK timeout in seconds
    pub afk_timeout: Option<u64>,

    /// Whether the server widget is enabled
    #[serde(default)]
    pub widget_enabled: Option<bool>,

    /// The channel id that the widget will generate an invite to, or null if set to no invite
    pub widget_channel_id: Option<String>,

    /// Verification level required for the guild
    /// 0: None, 1: Low, 2: Medium, 3: High, 4: Very High
    pub verification_level: Option<u8>,

    /// Default message notifications level
    /// 0: ALL_MESSAGES, 1: ONLY_MENTIONS
    pub default_message_notifications: Option<u8>,

    /// Explicit content filter level
    /// 0: DISABLED, 1: MEMBERS_WITHOUT_ROLES, 2: ALL_MEMBERS
    pub explicit_content_filter: Option<u8>,

    /// Members in the guild
    #[serde(default)]
    pub members: Vec<Member>,

    /// Channels in the guild
    #[serde(default)]
    pub channels: Vec<Channel>,

    /// Roles in the guild
    #[serde(default)]
    pub roles: Vec<Role>,

    /// Emojis in the guild
    #[serde(default)]
    pub emojis: Vec<Emoji>,

    /// Features enabled for the guild
    #[serde(default)]
    pub features: Vec<GuildFeatures>,

    /// MFA level required for the guild
    /// 0: NONE, 1: ELEVATED
    pub mfa_level: Option<u8>,

    /// Application ID of the guild creator if it is bot-created
    pub application_id: Option<String>,

    /// The id of the channel where guild notices such as welcome messages and boost events are posted
    pub system_channel_id: Option<String>,

    /// The system channel flags
    pub system_channel_flags: Option<u64>,

    /// The rules channel id
    pub rules_channel_id: Option<String>,

    /// The maximum number of presences for the guild (null is always returned, apart from the largest of guilds)
    pub max_presences: Option<u64>,

    /// The maximum number of members for the guild
    pub max_members: Option<u64>,

    /// The vanity URL code for the guild
    pub vanity_url_code: Option<String>,

    /// The description for the guild (if the guild has one)
    pub description: Option<String>,

    /// The banner hash for the guild (if the guild has one)
    pub banner: Option<String>,

    /// The premium tier of the guild
    /// 0: NONE, 1: TIER_1, 2: TIER_2, 3: TIER_3
    pub premium_tier: Option<u8>,

    /// The number of boosts the guild currently has
    pub premium_subscription_count: Option<u64>,

    /// The preferred locale of the guild (e.g., "en-US")
    pub preferred_locale: Option<String>,

    /// The public updates channel id
    /// This channel is used for announcements and boost events, and is not returned for private guilds
    pub public_updates_channel_id: Option<String>,

    /// The maximum amount of users in a video channel
    pub max_video_channel_users: Option<u64>,

    /// The maximum of users in a stage video channel
    pub max_stage_video_channel_users: Option<u64>,

    /// The approximate number of members in this guild, returned from the GET /guilds/{guild.id} endpoint when with_counts is true
    pub approximate_member_count: Option<u64>,

    /// The approximate number of non-offline members in this guild, returned from the GET /guilds/{guild.id} endpoint when with_counts is true
    pub approximate_presence_count: Option<u64>,

    /// The welcome screen of the guild, shown to new members, returned in an Invite's guild object
    pub welcome_screen: Option<WelcomeScreen>,

    /// The NSFW level of the guild
    /// 0: DEFAULT, 1: EXPLICIT, 2: SAFE, 3: AGE_RESTRICTED
    pub nsfw_level: Option<u8>,

    /// The custom guild stickers
    pub stickers: Option<Vec<Sticker>>,

    /// Whether the guild has the boost progress bar enabled
    #[serde(default)]
    pub boost_progress_bar_enabled: Option<bool>,

    /// The id of the channel where admins and moderators of Community guilds receive safety alerts from Discord
    pub safety_alerts_channel_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    /// The user this guild member represents
    pub user: User,

    /// The nickname of the member in the guild
    pub nick: Option<String>,

    /// The member's guild avatar hash (if any)
    pub avatar: Option<String>,

    /// The member's equipped collectibles
    #[serde(default)]
    pub collectibles: Vec<Nameplate>,

    /// The member's guild banner hash
    pub banner: Option<String>,

    /// The member's guild bio
    pub bio: Option<String>,

    /// The member's roles in the guild
    #[serde(default)]
    pub roles: Vec<String>,

    /// Whether the member is deafened in voice channels
    #[serde(default)]
    pub deaf: bool,

    /// Whether the member is muted in voice channels
    #[serde(default)]
    pub mute: bool,

    /// The timestamp when the member joined the guild, in ISO8601 format
    pub joined_at: String,

    /// The timestamp when the member started boosting the guild, in ISO8601 format (if any)
    pub premium_since: Option<String>,

    /// Whether the member is pending (i.e., has not yet passed the guild's Membership Screening requirements)
    #[serde(default)]
    pub pending: bool,

    /// WHen the member's timeout expires, in ISO8601 format (if any)
    pub communication_disabled_until: Option<String>,

    /// When the member's unusual DM activity flag will expire (if any)
    pub unusual_dm_activity_until: Option<String>,

    /// The member's flags
    pub flags: u64,

    /// The member's permissions
    pub permissions: Option<Permissions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WelcomeScreen {
    /// The server description shown in the welcome screen
    pub description: Option<String>,

    /// The channels shown in the welcome screen, up to 5
    pub welcome_channels: Vec<WelcomeScreenChannel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WelcomeScreenChannel {
    /// The channel ID to which the user will be taken when they click the welcome screen invite
    pub channel_id: String,

    /// The description shown for the channel in the welcome screen
    pub description: String,

    /// The emoji displayed for the channel in the welcome screen
    pub emoji_id: Option<String>,

    /// The unicode emoji displayed for the channel in the welcome screen (if no custom emoji is set)
    pub emoji_name: Option<String>,
}
