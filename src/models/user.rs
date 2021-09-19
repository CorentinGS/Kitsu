use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct KitsuUser {
    pub user_id: String,
    pub user_name: String,
    pub vip: bool,
}
