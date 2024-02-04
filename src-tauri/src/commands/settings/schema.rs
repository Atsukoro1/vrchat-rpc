use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    #[serde(rename = "minimizeToTray")]
    pub minimize_to_tray: bool,

    #[serde(rename = "startOnWindowsStartup")]
    pub start_on_windows_startup: bool,
}