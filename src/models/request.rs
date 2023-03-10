use serde::{Deserialize, Serialize};

use super::user::{DiscordGuildMember, DiscordUser};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandInteractionOption {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: u32,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InteractionData {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: u32,
    pub options: Option<Vec<CommandInteractionOption>>
}

/**
 * https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct InteractionRequest {
    pub id: String,
    pub token: String,
    #[serde(rename = "type")]
    pub type_: u32,
    pub channel_id: Option<String>,
    pub data: Option<InteractionData>,
    pub user: Option<DiscordUser>,
    pub member: Option<DiscordGuildMember>,
}
