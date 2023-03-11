use serde::{Deserialize, Serialize};

use super::user::DiscordUser;

/**
 * https://discord.com/developers/docs/resources/channel#message-object
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    // https://discord.com/developers/docs/resources/channel#message-object-message-types
    #[serde(rename = "type")]
    pub type_: u32,
    pub timestamp: String,
    pub content: Option<String>,
    pub author: DiscordUser,
}
