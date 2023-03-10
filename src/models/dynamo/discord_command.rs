use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DiscordCommand {
    id: String,
    command_type: CommandType,
    created_at: u64,
    updated_at: u64,
}

impl DiscordCommand {
    pub fn chat_command<S>(id: S, channel_id: S, now: u64) -> Self
    where
        S: Into<String>,
    {
        Self {
            id: id.into(),
            command_type: CommandType::Chat(ChatCommand::new(channel_id)),
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", tag = "CommandType", content = "Command")]
pub enum CommandType {
    Chat(ChatCommand),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCommand {
    channel_id: String,
}

impl ChatCommand {
    pub fn new<S: Into<String>>(channel_id: S) -> Self {
        Self {
            channel_id: channel_id.into(),
        }
    }
}
