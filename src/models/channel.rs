use serde::{Deserialize, Serialize};

/**
 * https://discord.com/developers/docs/resources/channel#channels-resource
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    pub id: String,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub type_: u32,
    pub topic: Option<String>,
    pub guild_id: Option<String>,
    pub parent_id: Option<String>,
    pub owner_id: Option<String>,
    pub last_message_id: Option<String>,
    pub message_count: Option<u32>,
    pub member_count: Option<u32>,
    pub rate_limit_per_user: Option<u32>,
    pub total_message_sent: Option<u32>,
}
