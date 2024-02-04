use discord_rich_presence::DiscordIpcClient;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

pub static DISCORD_CLIENT_ID: &str = "1202945591739420702";

#[allow(dead_code)]
pub static DISCORD_IPC_CLIENT: Lazy<Mutex<Option<Arc<DiscordIpcClient>>>> = Lazy::new(|| {
    let result = DiscordIpcClient::new(DISCORD_CLIENT_ID);

    let client_option = result.ok().map(Arc::new);

    Mutex::new(client_option)
});
