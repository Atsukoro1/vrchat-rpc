use discord_rich_presence::activity::{self, Timestamps};
use discord_rich_presence::{DiscordIpc, DiscordIpcClient};

use crate::commands::configuration::schema::Configuration;
use crate::schema::user_location::UserLocationContent;

pub static DISCORD_CLIENT_ID: &str = "1202945591739420702";

pub struct DiscordRpcClient {
    pub client: DiscordIpcClient,
    pub configuration: Option<Configuration>,
    pub player_info: Option<UserLocationContent>,
}

pub trait DiscordRpcClientExt {
    fn new() -> DiscordRpcClient;
    fn update_configuration(&mut self, activity: Configuration);
    fn update_player_info(&mut self, user: UserLocationContent);
    fn update_status(&mut self);
}

impl DiscordRpcClientExt for DiscordRpcClient {
    fn new() -> DiscordRpcClient {
        let mut client = DiscordIpcClient::new(DISCORD_CLIENT_ID)
            .expect("Failed to create Discord IPC client");

        client.connect().unwrap();

        DiscordRpcClient { client, configuration: None, player_info: None }
    }

    fn update_configuration(&mut self, activity: Configuration) {
        self.configuration = Some(activity);
    }

    fn update_player_info(&mut self, user: UserLocationContent) {
        self.player_info = Some(user);
    }

    fn update_status(&mut self) {
        let activity = self.configuration.as_ref().unwrap();
        let _user = self.player_info.as_ref().unwrap();

        let new_activity = activity::Activity::new()
            .state(&activity.description)
            .details(&activity.title)
            .timestamps(Timestamps::new());

        let _  = self.client.set_activity(new_activity);
    }
}