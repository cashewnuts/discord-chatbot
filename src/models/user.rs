use serde::{Deserialize, Serialize};

/**
 * https://discord.com/developers/docs/resources/user#user-object
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordUser {
    pub id: String,
    pub username: String,
    pub discriminator: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordGuildMember {
    pub user: Option<DiscordUser>,
}
