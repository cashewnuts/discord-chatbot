use serde::{Deserialize, Serialize};

use super::user::{DiscordGuildMember, DiscordUser};

#[derive(Debug, Serialize, Deserialize)]
pub struct InteractionRequest {
    pub id: String,
    pub token: String,
    #[serde(rename = "type")]
    pub type_: u32,
    pub user: Option<DiscordUser>,
    pub member: Option<DiscordGuildMember>,
}
