use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordUser {
    pub id: String,
    pub username: String,
    pub discriminator: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordGuildMember {
    pub user: Option<DiscordUser>,
}
