use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DiscordCommand {
    pub id: String,
    #[serde(flatten)]
    pub command_type: CommandType,
    pub created_at: u64,
    pub updated_at: u64,
}

impl DiscordCommand {
    pub fn chat_command<S>(id: S, channel_id: S, interaction_token: S, now: u64) -> Self
    where
        S: Into<String>,
    {
        Self {
            id: id.into(),
            command_type: CommandType::Chat(ChatCommand::new(channel_id, interaction_token)),
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", tag = "CommandType", content = "Command")]
pub enum CommandType {
    Chat(ChatCommand),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCommand {
    pub channel_id: String,
    pub interaction_token: String,
}

impl ChatCommand {
    pub fn new<S: Into<String>>(channel_id: S, interaction_token: S) -> Self {
        Self {
            channel_id: channel_id.into(),
            interaction_token: interaction_token.into(),
        }
    }
}
