use diself::model::{Relationship, RelationshipType, User};
use serde_json::json;

fn sample_user() -> User {
    serde_json::from_value(json!({
        "id": "123",
        "username": "daniil",
        "discriminator": "0001",
        "avatar": "avatar_hash",
        "banner": "a_banner_hash",
        "premium_type": 2
    }))
    .expect("valid user json")
}

#[test]
fn user_helper_methods_work() {
    let user = sample_user();

    assert_eq!(user.tag(), "daniil#0001");
    assert_eq!(user.mention(), "<@123>");
    assert!(user.has_nitro());
    assert_eq!(user.premium_type_name(), "Nitro");
    assert_eq!(
        user.avatar_url().as_deref(),
        Some("https://cdn.discordapp.com/avatars/123/avatar_hash.png")
    );
    assert_eq!(
        user.banner_url().as_deref(),
        Some("https://cdn.discordapp.com/banners/123/a_banner_hash.gif")
    );
}

#[test]
fn relationship_state_helpers_work() {
    let friend: Relationship = serde_json::from_value(json!({
        "id": "42",
        "type": 1
    }))
    .expect("valid relationship json");

    let blocked = Relationship {
        id: "77".to_string(),
        kind: RelationshipType::Blocked,
        user: None,
        nickname: None,
        is_spam_request: false,
        stranger_request: false,
        user_ignored: false,
        origin_application_id: None,
        since: None,
        has_played_game: false,
    };

    assert!(friend.is_friend());
    assert!(!friend.is_blocked());
    assert!(!blocked.is_friend());
    assert!(blocked.is_blocked());
}
