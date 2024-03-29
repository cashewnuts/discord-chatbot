use serde::{Deserialize, Serialize};

use crate::models::dynamo::discord_command::{ChatCommand, ChatCommandMessage};

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
    pub finish_reason: Option<String>,
    pub message: ChatCompletionMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionChunkDelta {
    pub role: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionChunkChoice {
    pub index: u32,
    pub delta: ChatCompletionChunkDelta,
    pub finish_reason: Option<String>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionChunkResponse {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<ChatCompletionChunkChoice>,
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
    pub stream: Option<bool>,
}

impl From<ChatCommand> for ChatCompletionRequest {
    fn from(value: ChatCommand) -> Self {
        let system_message = if let Some(t) = value.topic {
            ChatCompletionMessage::system(t)
        } else {
            ChatCompletionMessage::system("You're concise")
        };
        let mut messages = vec![system_message];
        for msg in value.messages.iter() {
            match msg {
                ChatCommandMessage::User { content } => {
                    messages.push(ChatCompletionMessage::user(content))
                }
                ChatCommandMessage::Assistant { content } => {
                    messages.push(ChatCompletionMessage::assistant(content))
                }
            }
        }
        Self {
            model: "gpt-3.5-turbo".to_string(),
            messages,
            stream: Some(true),
        }
    }
}
