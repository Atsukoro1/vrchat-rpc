use futures_util::FutureExt;

use crate::{
    clients::discord::DISCORD_CLIENT, 
    schema::{user_location::UserLocationContent, websocket_event::WebsocketEvent}
};

pub async fn handle_message(message: String) -> Result<(), ()> {
    let message = serde_json::from_str::<WebsocketEvent>(&message).unwrap();

    if message.r#type != "user-location" {
        return Ok(())
    };

    let content = serde_json::from_str::<UserLocationContent>(&message.content).unwrap();
    
    tokio::spawn(async move {
        println!("ok");
        if let Ok(mut discord_client) = DISCORD_CLIENT.try_lock() {
            discord_client.set_data(content);
            discord_client.set_activity();
            println!("set");
        } else {
            println!("Failed to acquire lock on DISCORD_CLIENT");
        }
    });

    Ok(())
}