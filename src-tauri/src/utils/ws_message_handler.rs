use crate::{clients::discord::{DiscordRpcClient, DiscordRpcClientExt}, commands::configuration::schema::Configuration, schema::{user_location::UserLocationContent, websocket_event::WebsocketEvent}};

pub fn handle_message(message: String) -> Result<(), ()> {
    let message = serde_json::from_str::<WebsocketEvent>(&message).unwrap();

    if message.r#type != "user-location" {
        return Ok(())
    };

    let content = serde_json::from_str::<UserLocationContent>(&message.content).unwrap();

    let mut client = DiscordRpcClient::new();
    client.update_configuration(
        Configuration {
            title: "VRCHAT".to_string(),
            description: "Playing rest & sleep lol".to_string(),
            show_timestamp: true,
            show_player_status: true,
            small_image_key: "This is a small image key".to_string(),
            large_image_key: "This is a large image key".to_string(),
        }
    );
    client.update_player_info(content);
    client.update_status();

    Ok(())
}