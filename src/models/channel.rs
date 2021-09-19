use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct KitsuChannel {
    pub channel_id: String,
    pub channel_name: String,
    pub guild_id: i8,
    pub ignore: bool,
}
