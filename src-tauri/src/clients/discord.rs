use std::sync::Mutex;

use crate::commands::configuration::schema::Configuration;
use crate::schema::user_location::UserLocationContent;
use crate::utils::replacer::replace_magic_string;
use discord_presence::{Client, Event};
use lazy_static::lazy_static;

pub static DISCORD_CLIENT_ID: u64 = 1202945591739420702;

lazy_static! {
    pub static ref DISCORD_CLIENT: Mutex<DiscordIpcClient> = Mutex::new(DiscordIpcClient::new());
}

pub struct DiscordIpcClient {
    pub client: Client,
    pub configuration: Option<Configuration>,
    pub data: Option<UserLocationContent>,
}

impl DiscordIpcClient {
    pub fn new() -> Self {
        let mut drpc_client = Client::new(DISCORD_CLIENT_ID);

        drpc_client.start();
        match drpc_client.block_until_event(Event::Ready) {
            Ok(_) => println!("Discord RPC client ready"),
            Err(e) => println!("Error starting discord RPC client: {}", e),
        }

        Self {
            client: drpc_client,
            configuration: Some(Configuration {
                title: "VRCHAT".to_string(),
                description: "{{user.display_name}} is playing rest & sleep lol".to_string(),
                show_timestamp: true,
                show_player_status: true,
                small_image_key: "This is a small image key".to_string(),
                large_image_key: "This is a large image key".to_string(),
            }),
            data: None,
        }
    }

    pub fn set_activity(&mut self) {
        let config = self.configuration.clone().unwrap();
        let data = self.data.clone().unwrap();

        match self.client.set_activity(|a| {
            a.state(replace_magic_string(config.title, &data))
                .details(replace_magic_string(config.description, &data))
                .assets(|assets| {
                    assets
                        .large_image(data.world.unwrap().image_url.unwrap_or("vrchat".to_string()))
                        .large_text("VRChat")
                        .small_image("vrchat")
                        .small_text("VRChat")
                })
        }) {
            Ok(_) => println!("Discord presence updated"),
            Err(e) => println!("Error updating discord presence: {}", e),
        }
    }

    pub fn set_configuration(&mut self, configuration: Configuration) {
        self.configuration = Some(configuration);
    }

    pub fn set_data(&mut self, data: UserLocationContent) {
        self.data = Some(data);
    }
}