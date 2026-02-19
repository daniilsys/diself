use diself::{Cache, CacheConfig};
use diself::model::User;
use serde_json::json;

fn sample_user(id: &str) -> User {
    serde_json::from_value(json!({
        "id": id,
        "username": format!("user_{id}"),
        "discriminator": "0001"
    }))
    .expect("valid user json")
}

#[test]
fn cache_set_current_user_populates_current_user_and_user_cache() {
    let cache = Cache::new();
    let user = sample_user("123");

    cache.set_current_user(user.clone());

    assert_eq!(cache.current_user().map(|u| u.id), Some("123".to_string()));
    assert_eq!(cache.user_count(), 1);
    assert_eq!(cache.user("123").map(|u| u.username), Some("user_123".to_string()));
}

#[test]
fn cache_respects_disabled_user_cache() {
    let cache = Cache::with_config(CacheConfig {
        cache_users: false,
        cache_channels: true,
        cache_guilds: true,
        cache_relationships: true,
    });

    cache.cache_user(sample_user("999"));

    assert_eq!(cache.user_count(), 0);
    assert!(cache.user("999").is_none());
}

#[test]
fn cache_initialize_reads_ready_user() {
    let cache = Cache::new();
    let ready_payload = json!({
        "user": {
            "id": "555",
            "username": "ready_user",
            "discriminator": "1234"
        },
        "users": [],
        "guilds": [],
        "relationships": []
    });

    cache.initialize(ready_payload);

    let current = cache.current_user().expect("current user should be set");
    assert_eq!(current.id, "555");
    assert_eq!(current.username, "ready_user");
}
