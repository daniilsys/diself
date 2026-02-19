# diself

A modern async Rust library for building Discord selfbot workflows with a clean API, typed models, and a resilient gateway runtime.

<p align="left">
  <img alt="Rust" src="https://img.shields.io/badge/rust-2021-orange.svg" />
  <img alt="License" src="https://img.shields.io/badge/license-MIT-blue.svg" />
  <img alt="Status" src="https://img.shields.io/badge/status-alpha-yellow.svg" />
</p>

## Disclaimer

This project is intended for authorized and compliant use only.

- Make sure your usage complies with Discord's Terms of Service and platform policies.
- If you are using this for coursework, internal tooling, or controlled environments, ensure explicit authorization.
- Maintainers and contributors are not responsible for misuse.

## Why `diself`?

- Async-first design (`tokio`)
- Event-driven client API (`EventHandler`)
- Resilient gateway loop (reconnect, resume, heartbeat ACK timeout handling)
- Typed Discord models (channels, messages, guilds, roles, permissions, overwrites)
- Configurable cache architecture (users/channels/guilds/relationships)
- Builder-based ergonomics (`ClientBuilder`)
- Graceful shutdown support (`Client::shutdown()`)

## Installation

```toml
[dependencies]
diself = "0.1.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use diself::prelude::*;

struct MyHandler;

#[async_trait]
impl EventHandler for MyHandler {
    async fn on_ready(&self, _ctx: &Context, user: User) {
        println!("Logged in as {}", user.tag());
    }

    async fn on_message_create(&self, ctx: &Context, msg: Message) {
        if msg.content == ".ping" {
            let _ = msg.reply(&ctx.http, "pong").await;
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN is not set");

    let client = Client::builder(token, MyHandler)
        .with_cache_config(CacheConfig {
            cache_users: true,
            cache_channels: true,
            cache_guilds: true,
            cache_relationships: true,
        })
        .build();

    client.start().await
}
```

## Client Builder

`ClientBuilder` provides a clean, scalable entrypoint:

```rust
let client = Client::builder(token, handler)
    .without_cache()
    .with_captcha_handler(|captcha_info| async move {
        // Solve captcha and return captcha key
        Ok("captcha_key".to_string())
    })
    .build();
```

## Graceful Shutdown

Run the client in a task and stop it cooperatively:

```rust
use std::sync::Arc;

let client = Arc::new(Client::builder(token, handler).build());
let runner = Arc::clone(&client);

let task = tokio::spawn(async move {
    let _ = runner.start().await;
});

tokio::signal::ctrl_c().await?;
client.shutdown();
let _ = task.await;
```

## Gateway Reliability

The current gateway implementation includes:

- Automatic reconnect loop
- Resume support (`RESUME`)
- Heartbeat + ACK timeout handling
- `RECONNECT` and `INVALID_SESSION` handling
- Backoff with jitter for reconnect attempts

## Managers API

`Context` exposes endpoint managers for ergonomic calls:

- `ctx.users`
- `ctx.guilds`
- `ctx.channels`
- `ctx.relationships`

Example:

```rust
use diself::prelude::*;

async fn example(ctx: &Context) -> Result<()> {
    let me = ctx.users.me(&ctx.http).await?;
    println!("connected as {}", me.tag());

    // Example relationship action
    ctx.relationships.send_friend_request(&ctx.http, "username").await?;
    Ok(())
}
```

## Examples

- `examples/hello_gateway.rs`
- `examples/cache_example.rs`

Run an example:

```bash
DISCORD_TOKEN="..." cargo run --example cache_example
```

## Development

Recommended local checks:

```bash
cargo check
cargo clippy --all-targets --all-features
cargo test
```

## Testing

The project includes:

- Unit/integration tests under `tests/`
- Live endpoint smoke tests under `tests/endpoints_live.rs` (ignored by default)

Run regular tests:

```bash
cargo test
```

Run live endpoint smoke tests:

```bash
DISCORD_TOKEN="..." cargo test --test endpoints_live -- --ignored --nocapture
```

## Roadmap

Short-term priorities:

- Broader integration tests (gateway lifecycle, HTTP edge cases)
- Typed HTTP error surface improvements
- Documentation expansion and cookbook-style examples
- API stabilization toward a stronger `0.1.x`

## Contributing

Contributions are welcome.

Until a dedicated `CONTRIBUTING.md` is added, please follow:

1. Open an issue first for non-trivial changes.
2. Keep pull requests focused and small.
3. Include tests or rationale when changing behavior.
4. Run the local checks before opening a PR.

## License

MIT. See `LICENSE`.
