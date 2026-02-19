use diself::{ChannelsManager, GuildsManager, HttpClient, RelationshipsManager, UsersManager};

fn required_env(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("{name} must be set to run live endpoint tests"))
}

fn optional_env(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|v| !v.trim().is_empty())
}

fn live_http() -> HttpClient {
    HttpClient::new(required_env("DISCORD_TOKEN"))
}

#[tokio::test]
#[ignore = "Live Discord endpoint smoke test; requires DISCORD_TOKEN"]
async fn users_me_endpoint_smoke() -> diself::Result<()> {
    let http = live_http();
    let users = UsersManager;

    let me = users.me(&http).await?;
    assert!(!me.id.is_empty());
    assert!(!me.username.is_empty());
    Ok(())
}

#[tokio::test]
#[ignore = "Live Discord endpoint smoke test; requires DISCORD_TOKEN"]
async fn guilds_list_endpoint_smoke() -> diself::Result<()> {
    let http = live_http();
    let guilds = GuildsManager;

    let list = guilds.list(&http).await?;
    assert!(list.iter().all(|g| !g.id.is_empty()));
    Ok(())
}

#[tokio::test]
#[ignore = "Live Discord endpoint smoke test; requires DISCORD_TOKEN"]
async fn channels_dm_channels_endpoint_smoke() -> diself::Result<()> {
    let http = live_http();
    let channels = ChannelsManager;

    let dms = channels.dm_channels(&http).await?;
    assert!(dms.iter().all(|c| !c.id.is_empty()));
    Ok(())
}

#[tokio::test]
#[ignore = "Live Discord endpoint smoke test; requires DISCORD_TOKEN"]
async fn relationships_list_endpoint_smoke() -> diself::Result<()> {
    let http = live_http();
    let relationships = RelationshipsManager;

    let list = relationships.list(&http).await?;
    assert!(list.iter().all(|r| !r.id.is_empty()));
    Ok(())
}

#[tokio::test]
#[ignore = "Live Discord endpoint smoke test; requires DISCORD_TOKEN and DISELF_TEST_GUILD_ID"]
async fn guilds_get_endpoint_smoke() -> diself::Result<()> {
    let Some(guild_id) = optional_env("DISELF_TEST_GUILD_ID") else {
        eprintln!("Skipping: DISELF_TEST_GUILD_ID is not set");
        return Ok(());
    };

    let http = live_http();
    let guilds = GuildsManager;

    let guild = guilds.get(&http, guild_id).await?;
    assert!(!guild.id.is_empty());
    Ok(())
}
