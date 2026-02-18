use crate::cache::{CacheConfig, ChannelCache, GuildCache, RelationshipCache, UserCache};
use crate::model::{Channel, Guild, Relationship, User};
use parking_lot::RwLock;
use std::sync::Arc;

/// Thread-safe cache for Discord entities
#[derive(Clone)]
pub struct Cache {
    config: CacheConfig,
    user_cache: UserCache,
    channel_cache: ChannelCache,
    guild_cache: GuildCache,
    relationship_cache: RelationshipCache,
    /// Current user
    current_user: Arc<RwLock<Option<User>>>,
}

impl Cache {
    /// Creates a new cache with default configuration
    pub fn new() -> Self {
        Self::with_config(CacheConfig::default())
    }

    /// Creates a new cache with custom configuration
    pub fn with_config(config: CacheConfig) -> Self {
        Self {
            user_cache: UserCache::new(config.cache_users),
            channel_cache: ChannelCache::new(config.cache_channels),
            guild_cache: GuildCache::new(config.cache_guilds),
            relationship_cache: RelationshipCache::new(config.cache_relationships),
            config,
            current_user: Arc::new(RwLock::new(None)),
        }
    }
    // ==================== Initialization ====================

    /// Initializes the caches with data from the READY event
    pub fn initialize(&self, data: serde_json::Value) {
        if let Ok(user) = serde_json::from_value::<User>(data["user"].clone()) {
            if user.bot {
                eprintln!("discord-selfbot-rs is intended for use with user accounts. If you are seeing this message, it means you have logged in with a bot token, which is not supported. Please log in with a user token instead.");
            }

            self.set_current_user(user);
        }
        self.initialize_users(data["users"].clone());
        self.initialize_guilds(data["guilds"].clone());
        self.initialize_relationships(data["relationships"].clone());
    }

    // ==================== Current User ====================

    /// Gets the current user from cache
    pub fn current_user(&self) -> Option<User> {
        self.current_user.read().clone()
    }

    /// Sets the current user in cache
    pub fn set_current_user(&self, user: User) {
        *self.current_user.write() = Some(user.clone());
        self.user_cache.insert(user);
    }

    // ==================== Users ====================

    /// Initializes user cache with data from the READY event
    pub fn initialize_users(&self, data: serde_json::Value) {
        self.user_cache.initialize_from_ready(data);
    }

    /// Gets a user from cache by ID
    pub fn user(&self, user_id: &str) -> Option<User> {
        self.user_cache.get(user_id)
    }

    /// Inserts or updates a user in cache
    pub fn cache_user(&self, user: User) {
        self.user_cache.insert(user);
    }

    /// Removes a user from cache
    pub fn remove_user(&self, user_id: &str) -> Option<User> {
        self.user_cache.remove(user_id)
    }

    /// Returns the number of cached users
    pub fn user_count(&self) -> usize {
        self.user_cache.count()
    }

    /// Gets all cached users
    pub fn users(&self) -> Vec<User> {
        self.user_cache.all()
    }

    // ==================== Relationships ====================

    /// Initalizes relationships cache with data from the READY event
    pub fn initialize_relationships(&self, data: serde_json::Value) {
        self.relationship_cache.initialize_from_ready(data);
    }

    /// Gets a relationship from cache by user ID
    pub fn relationship(&self, user_id: &str) -> Option<Relationship> {
        self.relationship_cache.get(user_id)
    }

    /// Inserts or updates a relationship in cache
    pub fn cache_relationship(&self, relationship: Relationship) {
        self.relationship_cache.insert(relationship);
    }

    /// Removes a relationship from cache
    pub fn remove_relationship(&self, user_id: &str) -> Option<Relationship> {
        self.relationship_cache.remove(user_id)
    }

    /// Returns the number of cached relationships
    pub fn relationship_count(&self) -> usize {
        self.relationship_cache.count()
    }

    // ==================== Channels ====================

    /// Gets a channel from cache by ID
    pub fn channel(&self, channel_id: &str) -> Option<Channel> {
        self.channel_cache.get(channel_id)
    }

    /// Inserts or updates a channel in cache
    pub fn cache_channel(&self, channel: Channel) {
        self.channel_cache.insert(channel);
    }

    /// Removes a channel from cache
    pub fn remove_channel(&self, channel_id: &str) -> Option<Channel> {
        self.channel_cache.remove(channel_id)
    }

    /// Returns the number of cached channels
    pub fn channel_count(&self) -> usize {
        self.channel_cache.count()
    }

    /// Gets all cached channels
    pub fn channels(&self) -> Vec<Channel> {
        self.channel_cache.all()
    }

    // ==================== Guilds ====================

    /// Initializes guild cache with data from the READY event
    pub fn initialize_guilds(&self, data: serde_json::Value) {
        self.channel_cache.initialize_from_ready(data.clone());
        self.guild_cache.initialize_from_ready(data);
    }

    /// Gets a guild from cache by ID
    pub fn guild(&self, guild_id: &str) -> Option<Guild> {
        self.guild_cache.get(guild_id)
    }

    /// Inserts or updates a guild in cache
    pub fn cache_guild(&self, guild: Guild) {
        self.guild_cache.insert(guild);
    }

    /// Removes a guild from cache
    pub fn remove_guild(&self, guild_id: &str) -> Option<Guild> {
        self.guild_cache.remove(guild_id)
    }

    /// Returns the number of cached guilds
    pub fn guild_count(&self) -> usize {
        self.guild_cache.count()
    }

    /// Gets all cached guilds
    pub fn guilds(&self) -> Vec<Guild> {
        self.guild_cache.all()
    }

    // ==================== Cache Management ====================

    pub fn config(&self) -> &CacheConfig {
        &self.config
    }

    /// Clears all caches
    pub fn clear(&self) {
        self.user_cache.clear();
        self.channel_cache.clear();
        self.guild_cache.clear();
        self.relationship_cache.clear();
        *self.current_user.write() = None;
    }

    /// Clears only the user cache
    pub fn clear_users(&self) {
        self.user_cache.clear();
    }

    /// Clears only the channel cache
    pub fn clear_channels(&self) {
        self.channel_cache.clear();
    }

    /// Clears only the guild cache
    pub fn clear_guilds(&self) {
        self.guild_cache.clear();
    }

    /// Clears only the relationship cache
    pub fn clear_relationships(&self) {
        self.relationship_cache.clear();
    }

    /// Gets cache statistics
    pub fn stats(&self) -> CacheStats {
        CacheStats {
            users: self.user_count(),
            channels: self.channel_count(),
            guilds: self.guild_count(),
        }
    }
}

impl Default for Cache {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about the cache
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub users: usize,
    pub channels: usize,
    pub guilds: usize,
}
