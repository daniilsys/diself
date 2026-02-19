use diself::http;

#[test]
fn api_url_uses_v10_base_url() {
    let url = http::api_url("/channels/123/messages");
    assert_eq!(url, "https://discord.com/api/v10/channels/123/messages");
}
