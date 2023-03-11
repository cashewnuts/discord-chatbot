use serde::{Deserialize, Serialize};

use super::dynamo::discord_command::ChatCommand;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionMessage {
    pub role: String,
    pub content: String,
}

impl ChatCompletionMessage {
    pub fn system<S: Into<String>>(content: S) -> Self {
        Self {
            role: "system".to_string(),
            content: content.into(),
        }
    }
    pub fn assistant<S: Into<String>>(content: S) -> Self {
        Self {
            role: "assistant".to_string(),
            content: content.into(),
        }
    }
    pub fn user<S: Into<String>>(content: S) -> Self {
        Self {
            role: "user".to_string(),
            content: content.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionChoice {
    pub index: u32,
    pub finish_reason: String,
    pub message: ChatCompletionMessage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub usage: ChatCompletionUsage,
    pub choices: Vec<ChatCompletionChoice>,
}

impl ChatCompletionResponse {
    pub fn get_total_token_usage(&self) -> u32 {
        self.usage.total_tokens
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatCompletionMessage>,
}

impl From<ChatCommand> for ChatCompletionRequest {
    fn from(value: ChatCommand) -> Self {
        let system_message = if let Some(t) = value.topic {
            ChatCompletionMessage::system(t)
        } else {
            ChatCompletionMessage::system("You're concise")
        };
        let user_message = ChatCompletionMessage::user("How to clear bash");
        Self {
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![system_message, user_message],
        }
    }
}
