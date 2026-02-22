use diself::{
    CollectorHub, CollectorOptions, DispatchEvent, DispatchEventType, ReactionEventType,
};
use serde_json::json;

#[tokio::test]
async fn message_collector_collects_filtered_messages() {
    let hub = CollectorHub::new();
    let mut collector = hub.message_collector(
        CollectorOptions {
            time: None,
            max: Some(1),
        },
        |msg| msg.content == "ping",
    );

    hub.dispatch(DispatchEvent {
        kind: DispatchEventType::MessageCreate,
        sequence: Some(1),
        data: json!({
            "id": "m1",
            "channel_id": "c1",
            "author": { "id": "u1", "username": "name", "discriminator": "0001" },
            "content": "not-matching",
            "timestamp": "2026-02-22T00:00:00.000Z",
            "type": 0
        }),
    });

    hub.dispatch(DispatchEvent {
        kind: DispatchEventType::MessageCreate,
        sequence: Some(2),
        data: json!({
            "id": "m2",
            "channel_id": "c1",
            "author": { "id": "u1", "username": "name", "discriminator": "0001" },
            "content": "ping",
            "timestamp": "2026-02-22T00:00:00.000Z",
            "type": 0
        }),
    });

    let item = collector.next().await.expect("expected collected message");
    assert_eq!(item.id, "m2");
    assert_eq!(item.content, "ping");
}

#[tokio::test]
async fn reaction_collector_collects_reaction_add() {
    let hub = CollectorHub::new();
    let mut collector = hub.reaction_collector(
        CollectorOptions {
            time: None,
            max: Some(1),
        },
        |evt| evt.kind == ReactionEventType::Add && evt.message_id == "m42",
    );

    hub.dispatch(DispatchEvent {
        kind: DispatchEventType::MessageReactionAdd,
        sequence: Some(10),
        data: json!({
            "channel_id": "c9",
            "message_id": "m42",
            "user_id": "u4",
            "emoji": { "id": null, "name": "👍" }
        }),
    });

    let item = collector.next().await.expect("expected collected reaction");
    assert_eq!(item.channel_id, "c9");
    assert_eq!(item.message_id, "m42");
    assert_eq!(item.user_id, "u4");
}
