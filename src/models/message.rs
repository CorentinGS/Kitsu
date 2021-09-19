use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct KitsuMessage {
    pub message_id: String,
    pub user_id: i8,
    pub guild_id: i8,
    pub channel_id: i8,
    pub date_id: i64,
}
