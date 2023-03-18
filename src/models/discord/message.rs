use serde::{Deserialize, Serialize};

use crate::models::user::DiscordUser;

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
    pub referenced_message: Option<Box<Message>>,
}

impl Message {
    pub fn get_message_content(&self) -> Option<String> {
        if let Some(referenced_message) = self.referenced_message.clone() {
            referenced_message.content.to_owned()
        } else {
            self.content.clone()
        }
    }
}
