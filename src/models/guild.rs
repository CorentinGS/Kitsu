use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct KitsuGuild {
    pub guild_id: String,
    pub guild_name: String,
    pub vip: bool,
}
